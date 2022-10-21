use std::{
    convert::TryFrom,
    env,
    fs,
    fs::File,
    io::{Read, Write},
    ops::Range,
    path::Path,
    process::{Command, Output},
    sync::Arc,
};
use bindings::themelio_bridge::ThemelioBridge;
use clap::Parser;
use dotenv::dotenv;
use ethers::abi::{ethereum_types::BigEndianHash, Token};
use ethers::prelude::*;
use ethers::prelude::k256::ecdsa::SigningKey;
use ethers::utils::*;
use ethers_solc::artifacts::{
    BytecodeObject,
    Optimizer,
    Settings,
    Source,
    Sources,
};
use ethers_solc::{
    CompilerInput,
    remappings::Remapping,
    Solc,
};
use ethers::{
    providers::Provider,
    signers::Signer,
    utils::keccak256,
};
use ethers::types::{
    Address as EthersAddress,
    Bytes,
    H160,
    U256
};
use eyre::Result;
use melorun::{SpendContext};
use novasmt::dense::DenseMerkleTree;
use rand::Rng;
use rayon::iter::{
    IntoParallelIterator,
    ParallelIterator
};
use regex::Regex;
use serde::{Deserialize, Serialize};
use themelio_structs::{
    Address as ThemelioAddress,
    BlockHeight,
    CoinData,
    CoinID,
    CoinValue,
    Denom,
    Header,
    NetID,
    StakeDoc,
    Transaction,
    TxKind,
    TxHash, STAKE_EPOCH
};
use tmelcrypt::{
    ed25519_keygen,
    Ed25519PK,
    HashVal,
};

struct Config {
    pub wallet: LocalWallet,
    pub rpc: String,
}

impl Config {
    pub fn new() -> Result<Config, &'static str> {
        dotenv().ok();

        let secret_key = env::var("SECRET_KEY")
            .expect("Missing SECRET_KEY env variable.");

        let wallet: LocalWallet = secret_key
            .parse()
            .unwrap();

        let rpc: String = env::var("RPC_URL")
            .expect("Unable to parse RPC_URL.");

        Ok(
            Config {
                wallet,
                rpc
            }
        )
    }
}

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    #[clap(long, default_value = "")]
    num_stakedocs: String,

    #[clap(long, default_value = "")]
    num_transactions: String,

    #[clap(long, default_value = "")]
    themelio_recipient: String
}

struct ProofArgs {
    coins_slot: H256,
    contract_address: EthersAddress,
    tx_hash: H256,
    coin: CoinData,
    block_id: BlockId,
}

#[derive(Debug, Default, Clone, PartialEq, Eq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PubEIP1186ProofResponse {
    address: Address,
    balance: U256,
    code_hash: H256,
    nonce: U256,
    storage_hash: H256,
    account_proof: Vec<Bytes>,
    storage_proof: Vec<StorageProof>,
}

const BRIDGE_COVHASH: ThemelioAddress = ThemelioAddress(HashVal([0; 32]));
const COINS_SLOT: H256 = H256([0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 254]);
const DATA_BLOCK_HASH_KEY: &[u8; 13] = b"smt_datablock";
const GAS_LIMIT: u32 = 29_000_000;

fn random_header(modifier: u128) -> Header {
    if modifier == 0 || modifier == u8::MAX as u128 || modifier == u16::MAX as u128 || modifier == u32::MAX as u128 ||
        modifier == u32::MAX as u128 || modifier == u64::MAX as u128 || modifier == u128::MAX {
        let block_height = if modifier > u64::MAX as u128 {
            u64::MAX
        } else {
            modifier as u64
        };

        Header {
            network: NetID::Mainnet,
            previous: HashVal::random(),
            height: BlockHeight(block_height),
            history_hash: HashVal::random(),
            coins_hash: HashVal::random(),
            transactions_hash: HashVal::random(),
            fee_pool: CoinValue(modifier),
            fee_multiplier: modifier,
            dosc_speed: modifier,
            pools_hash: HashVal::random(),
            stakes_hash: HashVal::random(),
        }
    } else {
        Header {
            network: NetID::Mainnet,
            previous: HashVal::random(),
            height: BlockHeight(rand::thread_rng().gen()),
            history_hash: HashVal::random(),
            coins_hash: HashVal::random(),
            transactions_hash: HashVal::random(),
            fee_pool: CoinValue(rand::thread_rng().gen()),
            fee_multiplier: rand::thread_rng().gen(),
            dosc_speed: rand::thread_rng().gen(),
            pools_hash: HashVal::random(),
            stakes_hash: HashVal::random(),
        }
    }
}

fn random_coin_id() -> CoinID {
    CoinID {
        txhash: TxHash(HashVal::random()),
        index: rand::thread_rng().gen(),
    }
}

fn random_coindata() -> CoinData {
    let additional_data: Vec<u8> = (0..20)
        .into_par_iter()
        .map(|_| {
            rand::thread_rng().gen::<u8>()
        })
        .collect();

    CoinData {
        covhash: ThemelioAddress(HashVal::random()),
        value: CoinValue(rand::thread_rng().gen()),
        denom: random_denom(),
        additional_data
    }
}

fn random_denom() -> Denom {
    let denom_int = rand::thread_rng().gen_range(0..4);

    match denom_int {
        0 => Denom::Mel,
        1 => Denom::Sym,
        2 => Denom::Erg,
        _ => Denom::Custom(TxHash(HashVal::random()))
    }
}

fn random_stakedoc(epoch: u64) -> StakeDoc {
    let e_start: u64;
    if epoch == 0 {
        e_start = 0;
    } else {
        e_start = rand::thread_rng()
            .gen_range(0..epoch + 1);
    }

    let e_post_end: u64 = rand::thread_rng()
        .gen_range(epoch + 2..u64::MAX);

    StakeDoc {
        pubkey: ed25519_keygen().0,
        e_start,
        e_post_end,
        syms_staked: CoinValue(rand::thread_rng().gen_range(0..u32::MAX as u128)),
    }
}

fn random_transaction() -> Transaction {
    let limit: u32 = 32;

    let num_inputs: u32 = rand::thread_rng().gen_range(1..limit);
    let inputs = (0..num_inputs)
        .into_par_iter()
        .map(|_| {
            random_coin_id()
        })
        .collect();

    let num_outputs: u32 = rand::thread_rng().gen_range(1..limit);
    let outputs = (0..num_outputs)
        .into_par_iter()
        .map(|_| {
            random_coindata()
        })
        .collect();

    let num_covenants: u32 = rand::thread_rng().gen_range(1..limit);
    let covenants = (0..num_covenants)
        .into_par_iter()
        .map(|_| {
            let size = rand::thread_rng().gen_range(0..limit);
            let range = 0..size;
            let covenant = range
                .into_iter()
                .map(|_| {
                    rand::thread_rng().gen::<u8>()
                })
                .collect();

            covenant
        })
        .collect();

    let num_sigs: u32 = rand::thread_rng().gen_range(1..limit);
    let sigs = (0..num_sigs)
        .into_par_iter()
        .map(|_| {
            let size = rand::thread_rng().gen_range(0..limit);
            let range = 0..size;
            let sig = range
                .into_iter()
                .map(|_| {
                    rand::thread_rng().gen::<u8>()
                })
                .collect();

            sig
        })
        .collect();

    Transaction {
        kind: TxKind::Swap,
        inputs,
        outputs,
        fee: CoinValue(rand::thread_rng().gen()),
        covenants,
        data: (0..2).map(|_| { rand::thread_rng().gen::<u8>() }).collect(),
        sigs,
    }
}

fn create_datablocks(num: u32) -> Vec<Transaction> {
    let range: Range<u32> = 0..num;

    range
        .into_par_iter()
        .map(|_index| {
            random_transaction()
        })
        .collect::<Vec<Transaction>>()
}

fn create_stakes_and_sign_header(num_stakedocs: u32, epoch: u64, header: &[u8]) -> (Vec<u8>, Vec<[u8; 32]>) {
    let mut epoch_syms = CoinValue(0);
    let mut next_epoch_syms = CoinValue(0);
    let mut stakes: Vec<u8> = vec!();
    let mut signatures: Vec<[u8; 32]> = vec!();

    for _ in 0..num_stakedocs {
        let mut stakedoc = random_stakedoc(epoch);
        println!("{:?}\n", stakedoc);

        let keypair = ed25519_keygen();
        stakedoc.pubkey = Ed25519PK::from_bytes(&keypair.0.0).unwrap();

        let signature: &[u8] = &keypair.1.sign(header);

        signatures.push(signature[0..32].try_into().unwrap());
        signatures.push(signature[32..64].try_into().unwrap());

        if stakedoc.e_start <= epoch && stakedoc.e_post_end >= epoch + 1 {
            epoch_syms += stakedoc.syms_staked;
        }

        if stakedoc.e_start <= epoch + 1 && stakedoc.e_post_end >= epoch + 2 {
            next_epoch_syms += stakedoc.syms_staked;
        }

        let mut stakedoc_vec = stdcode::serialize(&stakedoc).unwrap();

        stakes.append(&mut stakedoc_vec);
    }
    for i in 0..signatures.len() {
        println!("signatures[{}] = {}", i, hex::encode(signatures[i]));
    };
    println!("\nGenesis Header Epoch Syms: {}", epoch_syms);
    println!("Next Epoch Syms: {}\n", next_epoch_syms);

    let mut stakes_structured = stdcode::serialize(&epoch_syms).unwrap();
    stakes_structured.append(&mut stdcode::serialize(&next_epoch_syms).unwrap());
    stakes_structured.append(&mut stakes);

    (stakes_structured, signatures)
}

async fn link_libraries(
    bytecode: &mut BytecodeObject,
) -> Result<()> {
    let mut source_ancestors = Path::new(&env!("CARGO_MANIFEST_DIR"))
        .ancestors();

    source_ancestors.next();

    let source = source_ancestors
        .next()
        .expect("Error accessing path.");

    let config = Config::new()
        .expect("Error initializing configuration.");

    let provider = Provider::<Http>::try_from(config.rpc)
        .expect("Provider unable to be initialized.");

    let wallet: LocalWallet = config.wallet;

    let chain_id = provider
        .get_chainid()
        .await
        .unwrap()
        .as_u64();

    let wallet = wallet
        .with_chain_id(chain_id);

    let client = SignerMiddleware::new(provider.clone(), wallet);
    let client = Arc::new(client);

    let blake3_address = match env::var("BLAKE3_ADDRESS") {
        Ok(val) => val.parse::<EthersAddress>()?,
        Err(_) => {
            let blake3_source = source
                .join("contracts/lib/blake3-sol/src/Blake3Sol.sol");

            let mut settings = Settings::default()
                .with_via_ir();
        
            settings.optimizer = Optimizer{
                enabled: Some(true),
                runs: Some(100),
                details: None
            };

            let compiler_input = CompilerInput {
                language: "Solidity".to_string(),
                sources: Sources::from([(
                    "contracts/Blake3Sol.sol".into(),
                    Source {
                        content: fs::read_to_string(blake3_source)
                            .expect("Unable to read file.")
                    },
                )]),
                settings: settings,
            };

            let compiled = Solc::default()
                .compile_exact(&compiler_input)
                .expect("Could not compile contracts.");

            let compiled = compiled
                .find("Blake3Sol")
                .expect("Could not find contract.");

            let (abi, bytecode, _runtime_bytecode) = compiled
                .into_parts_or_default();

            let factory = ContractFactory::new(abi, bytecode, client.clone());

            let blake3 = factory
                .deploy(())
                .expect("Unable to deploy bridge.")
                .send()
                .await
                .expect("Error awaiting bridge contract creation.");

            blake3.address()
        },
    };

    bytecode
        .link(
            "/home/marco/dev/bridge-utils/contracts/lib/blake3-sol/src/Blake3Sol.sol",
            "Blake3Sol",
            blake3_address
        )
        .resolve();

    let ed25519_address = match env::var("ED25519_ADDRESS") {
        Ok(val) => val.parse::<EthersAddress>()?,
        Err(_) => {
            let ed25519_source = source
                .join("contracts/lib/ed25519-sol/src/Ed25519.sol");

            let mut settings = Settings::default()
                .with_via_ir();

            settings.optimizer = Optimizer{
                enabled: Some(true),
                runs: Some(10),
                details: None
            };

            let compiler_input = CompilerInput {
                language: "Solidity".to_string(),
                sources: Sources::from([(
                    "contracts/lib/ed25519-sol/src/Ed25519.sol".into(),
                    Source {
                        content: fs::read_to_string(ed25519_source)
                            .expect("Unable to read file.")
                    },
                )]),
                settings,
            };

            let compiled = Solc::default()
                .compile_exact(&compiler_input)
                .expect("Could not compile Ed25519 contract.");

            let compiled = compiled
                .find("Ed25519")
                .expect("Could not find Ed25519 contract.");

            let (abi, bytecode, _runtime_bytecode) = compiled
                .into_parts_or_default();

            let factory = ContractFactory::new(abi, bytecode, client.clone());

            let ed25519 = factory
                .deploy(())
                .expect("Unable to deploy Ed25519 library.")
                .send()
                .await
                .expect("Error awaiting Ed25519 library creation.");

            ed25519.address()
        },
    };

    bytecode
        .link(
            "/home/marco/dev/bridge-utils/contracts/lib/ed25519-sol/src/Ed25519.sol",
            "Ed25519",
            ed25519_address
        )
        .resolve();

    Ok(())
}

async fn setup_bridge() -> H160 {
    let mut source_ancestors = Path::new(&env!("CARGO_MANIFEST_DIR"))
        .ancestors();

    source_ancestors.next();
    
    let source = source_ancestors
        .next()
        .expect("Error accessing path.");

    let themelio_bridge_source = source
        .join("contracts/src/ThemelioBridge.sol");

    let remappings = vec!(
        Remapping {
            name: String::from("blake3-sol/"),
            path: source
                    .join("contracts/lib/blake3-sol/src/")
                    .to_str()
                    .unwrap()
                    .to_string()
        },
        Remapping {
            name: String::from("ed25519-sol/"),
            path: source
                    .join("contracts/lib/ed25519-sol/src/")
                    .to_str()
                    .unwrap()
                    .to_string()
        },
        Remapping {
            name: String::from("openzeppelin-contracts-upgradeable/contracts/token/ERC1155/"),
            path: source
                    .join("contracts/lib/openzeppelin-contracts-upgradeable/contracts/token/ERC1155/")
                    .to_str()
                    .unwrap()
                    .to_string()
        },
        Remapping {
            name: String::from("openzeppelin-contracts-upgradeable/contracts/proxy/utils/"),
            path: source
                    .join("contracts/lib/openzeppelin-contracts-upgradeable/contracts/proxy/utils/")
                    .to_str()
                    .unwrap()
                    .to_string()
        },
    );

    let mut settings = Settings::default()
        .with_via_ir();

    settings.optimizer = Optimizer{
        enabled: Some(true),
        runs: Some(100),
        details: None
    };

    settings.remappings = remappings;

    let compiler_input = CompilerInput {
        language: "Solidity".to_string(),
        sources: Sources::from([(
            "contracts/ThemelioBridge.sol".into(),
            Source {
                content: fs::read_to_string(themelio_bridge_source)
                    .expect("Unable to read file.")
            },
        )]),
        settings,
    };

    let compiled = Solc::default()
        .compile_exact(&compiler_input)
        .expect("Could not compile contracts.");

    let compiled = compiled
        .find("ThemelioBridge")
        .expect("Could not find contract.");

    let mut bytecode = compiled.bin.unwrap().clone();

    link_libraries(&mut bytecode)
        .await
        .expect("Error linking libraries.");

    let (abi, _bytecode, _runtime_bytecode) = compiled
        .into_parts_or_default();

    let config = Config::new()
        .expect("Error initializing configuration.");

    let provider = Provider::<Http>::try_from(config.rpc)
        .expect("Provider unable to be initialized.");

    let wallet: LocalWallet = config.wallet;

    let chain_id = provider
        .get_chainid()
        .await
        .unwrap()
        .as_u64();

    let wallet = wallet
        .with_chain_id(chain_id);

    let client = SignerMiddleware::new(provider.clone(), wallet);
    let client = Arc::new(client);

    let factory = ContractFactory::new(abi, bytecode.into_bytes().unwrap(), client.clone());

    let bridge = factory
        .deploy(())
        .expect("Unable to deploy bridge.")
        .send()
        .await
        .expect("Error awaiting bridge contract creation.");
    
    bridge.address()
}

async fn setup_bridge_proxy(
    genesis_height: u64,
    genesis_transactions_hash: Vec<u8>,
    genesis_stakes_hash: Vec<u8>
) -> ThemelioBridge<SignerMiddleware<Provider<Http>, Wallet<SigningKey>>> {
    let bridge_address = match env::var("BRIDGE_ADDRESS") {
        Ok(val) => val.parse::<EthersAddress>().expect("Error parsing bridge address."),
        Err(_) => setup_bridge().await
    };

    let mut source_ancestors = Path::new(&env!("CARGO_MANIFEST_DIR"))
        .ancestors();

    source_ancestors.next();
    
    let source = source_ancestors
        .next()
        .expect("Error accessing path.");

    let themelio_bridge_proxy_source = source
        .join("contracts/src/ThemelioBridgeProxy.sol");

    let remappings = vec!(
        Remapping {
            name: String::from("openzeppelin-contracts/contracts/proxy/ERC1967/"),
            path: source
                    .join("contracts/lib/openzeppelin-contracts/contracts/proxy/ERC1967/")
                    .to_str()
                    .unwrap()
                    .to_string()
        },
    );

    let mut settings = Settings::default()
        .with_via_ir();

    settings.optimizer = Optimizer{
        enabled: Some(true),
        runs: Some(100),
        details: None
    };

    settings.remappings = remappings;

    let compiler_input = CompilerInput {
        language: "Solidity".to_string(),
        sources: Sources::from([(
            "contracts/ThemelioBridgeProxy.sol".into(),
            Source {
                content: fs::read_to_string(themelio_bridge_proxy_source)
                    .expect("Unable to read file.")
            },
        )]),
        settings,
    };

    let compiled = Solc::default().compile_exact(&compiler_input)
        .expect("Could not compile contracts.");

    let (abi, bytecode, _runtime_bytecode) = compiled
        .find("ThemelioBridgeProxy")
        .expect("Could not find contract.")
        .into_parts_or_default();

    let config = Config::new()
        .expect("Error initializing configuration.");

    let wallet: LocalWallet = config.wallet;

    let provider = Provider::<Http>::try_from(config.rpc)
        .expect("Provider unable to be initialized.");

    let chain_id = provider
        .get_chainid()
        .await
        .unwrap()
        .as_u64();

    let wallet = wallet
        .with_chain_id(chain_id);

    let client = SignerMiddleware::new(provider.clone(), wallet);
    let client = Arc::new(client);

    let factory = ContractFactory::new(abi, bytecode, client.clone());

    let mut func_selector = hex::decode("09f10a3c").unwrap().to_vec();

    let genesis_height = format!("{:0>64x}", genesis_height);
    let mut genesis_height = hex::decode(genesis_height).unwrap();

    let mut genesis_transactions_hash = genesis_transactions_hash.to_vec();
    let mut genesis_stakes_hash = genesis_stakes_hash.to_vec();

    let mut initialization_data: Vec<u8> = vec!();
    initialization_data.append(&mut func_selector);
    initialization_data.append(&mut genesis_height);
    initialization_data.append(&mut genesis_transactions_hash);
    initialization_data.append(&mut genesis_stakes_hash);

    let constructor_data = (
        Token::Address(bridge_address),
        Token::Bytes(initialization_data.clone())
    );

    let bridge_proxy_call = factory
        .deploy(constructor_data)
        .expect("Unable to deploy bridge proxy.");
    let bridge_proxy_receipt = bridge_proxy_call
        .send()
        .await
        .expect("Error awaiting bridge proxy contract creation.");

    println!("\n***** Bridge Proxy deployment receipt *****\n{:#?}\n********************\n", bridge_proxy_receipt);

    let bridge_proxy_address = bridge_proxy_receipt.address();

    let wrapped_bridge_proxy = ThemelioBridge::new(bridge_proxy_address, client.clone());

    wrapped_bridge_proxy
}

async fn test_e2e(num_stakedocs: u32, num_transactions: u32, themelio_recipient: [u8; 32]) -> Result<ProofArgs, eyre::Error> {
    let config = Config::new().unwrap();

    let cross_epoch: bool = rand::thread_rng().gen();
    println!("Crossing epoch?: {}\n", cross_epoch);

    let modifier: u128 = rand::thread_rng().gen();
    let mut genesis_header = random_header(modifier);
    let genesis_header_epoch: u64 = rand::thread_rng().gen_range(0..u64::MAX / STAKE_EPOCH);
    let mut header: Header = random_header(modifier);

    if cross_epoch {
        genesis_header.height = BlockHeight((genesis_header_epoch + 1) * STAKE_EPOCH - 1);
        header.height = genesis_header.height + BlockHeight(rand::thread_rng().gen_range(1..STAKE_EPOCH));
    } else {
        genesis_header.height = BlockHeight(rand::thread_rng().gen_range(
            genesis_header_epoch * STAKE_EPOCH..((genesis_header_epoch + 1) * STAKE_EPOCH)
        ).into());
        header.height = BlockHeight(rand::thread_rng().gen_range(genesis_header.height.0 + 1..(genesis_header_epoch + 1) * STAKE_EPOCH));
    }
    println!("Genesis Header height: {}\nHeader Height: {}\n", genesis_header.height.0, header.height.0);

    // create random transactions with ethereum addresses in additional_data of first output
    let mut datablocks = create_datablocks(num_transactions);

    let index: usize = rand::thread_rng()
        .gen_range(0..num_transactions)
        .try_into()
        .expect("Error converting to usize.");

    datablocks[index].outputs[0].covhash = BRIDGE_COVHASH;
    datablocks[index].outputs[0].additional_data = config.wallet.address().0.to_vec();

    let tx_to_prove = datablocks
        .get(index)
        .ok_or("Unable to get tx datablock to prove.")
        .expect("Error getting tx to prove.");
    println!("{:?}\n", tx_to_prove);

    let denom = tx_to_prove
        .outputs[0]
        .denom;
    let denom: HashVal  = match denom {
        Denom::Mel => HashVal([0; 32]),
        Denom::Sym => HashVal([0 ,0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1]),
        Denom::Erg => HashVal([0 ,0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 2]),
        Denom::Custom(tx_hash) => tx_hash.0,
        _ => HashVal::random()
    };
    println!("Denom: {}\n", denom);

    let value: u128 = tx_to_prove
        .outputs[0]
        .value
        .into();

    println!("Value: {}\n", value);

    let serialized_tx = stdcode::serialize(&tx_to_prove)
        .expect("Unable to serialize tx.");

    let tx_hash = *blake3::keyed_hash(
        blake3::hash(DATA_BLOCK_HASH_KEY).as_bytes(),
        &serialized_tx
    ).as_bytes();

    let serialized_tx = Bytes::from(serialized_tx);

    let datablocks_serded = datablocks
        .iter()
        .map(|tx| {
            stdcode::serialize(&tx).unwrap().clone()
        })
        .collect::<Vec<_>>();

    let tree = DenseMerkleTree::new(&datablocks_serded);

    let proof = tree.proof(index);

    let root = tree.root_hash();
    println!("Transactions Hash: {}\n", hex::encode(&root));

    // replace the random 'transactions_hash' from 'header' with the root of our merkle tree
    header.transactions_hash = HashVal(root);
    println!("{:#?}\n", header);

    // create random staker keypairs, serialize header and sign with each sk, store in vecs
    let serialized_header = stdcode::serialize(&header).expect("Unable to serialize header.");
    let serialized_header_bytes = Bytes::from(serialized_header.clone());

    let stakes: Vec<u8>;
    let signatures: Vec<[u8; 32]>;
    (stakes, signatures) = create_stakes_and_sign_header(num_stakedocs, genesis_header_epoch, &serialized_header);
    let stakes = Bytes::from(stakes);

    let stakes_hash = blake3::keyed_hash(
        blake3::hash(DATA_BLOCK_HASH_KEY).as_bytes(),
        &stakes
    );
    println!("Stakes Hash: {}\n", stakes_hash);

    let wrapped_bridge_proxy = setup_bridge_proxy(genesis_header.height.0, vec!(0 as u8; 32), stakes_hash.as_bytes().to_vec())
        .await;

    // send tx to verifyStakes()
    let stakes_call = wrapped_bridge_proxy
        .verify_stakes(stakes.clone())
        .gas(GAS_LIMIT);
    let stakes_pending_tx = stakes_call
        .send()
        .await?;
    let stakes_receipt = stakes_pending_tx
        .await?
        .expect("Error with verifyStakes() call.");
    println!("\n*** verifyStakers() receipt ***\n{:#?}\n********************\n", stakes_receipt);

    // send tx to verifyHeader
    let header_call = wrapped_bridge_proxy
        .verify_header(U256::from(genesis_header.height.0), serialized_header_bytes, stakes.clone(), signatures)
        .gas(GAS_LIMIT);
    let header_pending_tx = header_call
        .send()
        .await?;
    let header_receipt = header_pending_tx
        .await?
        .expect("Error with verifyHeader() call.");
    println!("\n***** verifyHeader() receipt *****\n{:#?}\n********************\n", header_receipt);

    // send tx to verifyTx()
    let tx_call = wrapped_bridge_proxy
        .verify_tx(serialized_tx, U256::from(index), U256::from(header.height.0), proof)
        .gas(GAS_LIMIT);
    let tx_pending_tx = tx_call
        .send()
        .await?;
    let tx_receipt = tx_pending_tx
        .await?
        .expect("Error with verifyTx() call.");
    println!("\n***** verifyTx() receipt *****\n{:#?}\n********************\n", tx_receipt);

    // send tx to burn()
    let burn_call = wrapped_bridge_proxy
        .burn(config.wallet.address(), tx_hash, themelio_recipient)
        .gas(GAS_LIMIT);
    let burn_pending_tx = burn_call
        .send()
        .await?;
    let burn_receipt = burn_pending_tx
        .await?
        .unwrap();
    println!("\n***** burn() receipt *****\n{:#?}\n********************\n", &burn_receipt);

    let proof_args = ProofArgs{
        coins_slot: COINS_SLOT,
        contract_address: wrapped_bridge_proxy.address(),
        tx_hash: H256(tx_hash),
        coin: tx_to_prove.outputs[0].clone(),
        block_id: BlockId::Number(
            BlockNumber::Number(
                burn_receipt.block_number
                    .expect("Error unwrapping block height.")
            )
        ),
    };

    Ok(proof_args)
}

async fn get_state_root(block_height: BlockId) -> H256 {
    let config = Config::new()
        .expect("error initializing configuration.");

    let provider = Provider::<Http>::try_from(config.rpc)
        .expect("Provider unable to be instantiated.");

    let block = provider.get_block(block_height)
        .await
        .expect("Error getting block information.")
        .expect("Error unwrapping block info.");

    let state_root = block.state_root;

    state_root
}

async fn get_proof(proof_args: &ProofArgs) -> Result<Vec<u8>, eyre::Error> {
    let config = Config::new()
        .expect("Error initializing configuration.");

    let provider = Provider::<Http>::try_from(config.rpc)
        .expect("Provider unable to be instantiated.");

    let location = U256::from(keccak256([proof_args.tx_hash.as_bytes(), proof_args.coins_slot.as_bytes()].concat())) + 2;

    let locations: Vec<H256> = vec!(H256::from_uint(&location));

    let proof = provider.get_proof(proof_args.contract_address, locations, Some(proof_args.block_id))
        .await?;

    println!("{:#?}\n", proof);

    let pub_proof: PubEIP1186ProofResponse = stdcode::deserialize(&stdcode::serialize(&proof)?)?;

    let account_proof = pub_proof.account_proof.concat();
    let storage_proof = pub_proof.storage_proof[0].proof.concat();
    let total_proof = [account_proof, storage_proof].concat();

    Ok(total_proof)
}

fn write_to_cov(proof_args: &ProofArgs, state_root: H256) -> Result<(), eyre::Error> {
    let cov_filename = String::from("covenants/bridge.melo");
    let cov_path = Path::new(&cov_filename);
    let mut cov = File::open(cov_path)?;
    let mut cov_contents = String::new();
    cov.read_to_string(&mut cov_contents)?;
    drop(cov);

    // find "eth_bridge_addr" value and replace with proof_args.contract_address
    let addr_re = Regex::new(r#"eth_bridge_addr = x"([\da-f]{40})"#)?;

    let cov_contents = addr_re
        .replace(
            &cov_contents,
            String::from("eth_bridge_addr = x\"") + &hex::encode(proof_args.contract_address.as_bytes())
        );

    // find "def get_state_root() =" and replace with state_root
    let root_re = Regex::new(r#"get_state_root\(\): Hash =\s+x"([\da-f]+)"#)?;

    let cov_contents = root_re
        .replace(
            &cov_contents,
            String::from("get_state_root(): Hash =\n\tx\"") + &hex::encode(state_root)
        );

    // write updated contents to bridge.melo
    let mut new_cov = File::create(cov_path)?;
    new_cov.write(cov_contents.as_bytes())?;

    Ok(())
}

fn write_to_context(coin: CoinData, tx_hash: HashVal) -> Result<(), eyre::Error> {
    let context_filename = String::from("covenants/context.yaml");
    let mut env_file: SpendContext = serde_yaml::from_str(&std::fs::read_to_string(&context_filename)?)?;

    env_file.spender_outputs.insert(0, coin.clone());
    env_file.parent_denom = coin.denom;
    env_file.parent_value = coin.value;
    env_file.parent_fake_txhash = tx_hash;

    std::fs::write(&context_filename, serde_yaml::to_string(&env_file)?)?;

    Ok(())
}

fn spawn_covenant() -> Output {
    let output = Command::new("melorun")
        .arg("covenants/bridge.melo")
        .arg("-s")
        .arg("covenants/context.yaml")
        .output()
        .expect("Failed to execute command");

    output
}

#[tokio::main]
async fn main() -> Result<()> {
    let args = Args::parse();

    let num_stakedocs: u32 = args
        .num_stakedocs
        .parse()
        .expect("Please include '--num-stakedocs <int>' flag.");

    let num_transactions: u32 = args
        .num_transactions
        .parse()
        .expect("Please include '--num-transactions <int>' flag.");

    let themelio_recipient: String = args
        .themelio_recipient
        .parse::<String>()
        .expect("Please include '--themelio-recipient <address>' flag.")
        .strip_prefix("0x")
        .expect("Address must start with '0x'.")
        .to_string();
    let themelio_recipient: [u8; 32] = hex::decode(themelio_recipient)
        .expect("Invalid Themelio recipient address.")
        .try_into()
        .expect("Expected an address of length 32");

    let proof_vars = test_e2e(num_stakedocs, num_transactions, themelio_recipient)
        .await
        .expect("Error in e2e test.");

    let state_root = get_state_root(proof_vars.block_id)
        .await;

    println!("State root at proof height {:?} : {:#?}\n********************\n", proof_vars.block_id, state_root);

    write_to_cov(&proof_vars, state_root)?;

    let mut coin = proof_vars.coin.clone();
    let proof = get_proof(&proof_vars).await?;
    coin.additional_data = proof;

    write_to_context(coin, HashVal(proof_vars.tx_hash.0))?;

    let output = spawn_covenant();

    if output.status.success() {
        println!("Success:\n{}", String::from_utf8(output.stdout)?);
    } else {
        println!("Error:\n{}", String::from_utf8(output.stderr)?);
    }

    Ok(())
}