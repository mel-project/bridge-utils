use std::{
    convert::TryFrom,
    env,
    fs,
    io,
    ops::Range,
    path::Path,
    sync::Arc,
};
use bindings::themelio_bridge::ThemelioBridge;
use clap::Parser;
use dotenv::dotenv;
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
    abi::Token,
};
use ethers::types::{
    Address as EthersAddress,
    Bytes,
    H160,
    U256
};
use eyre::Result;
use rand::Rng;
use rayon::iter::{
    IntoParallelIterator,
    ParallelIterator
};
use novasmt::dense::DenseMerkleTree;
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

const BRIDGE_COVHASH: ThemelioAddress = ThemelioAddress(HashVal([0; 32]));
const DATA_BLOCK_HASH_KEY: &[u8; 13] = b"smt_datablock";
const NODE_HASH_KEY: &[u8; 8] = b"smt_node";
const GAS_LIMIT: u32 = 29_000_000;

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
    #[clap(long)]
    initial_conditions: bool,
}

fn random_header(modifier: u128) -> Header {
    if modifier == 0 {
        Header {
            network: NetID::Mainnet,
            previous: HashVal::random(),
            height: BlockHeight(u64::MIN),
            history_hash: HashVal::random(),
            coins_hash: HashVal::random(),
            transactions_hash: HashVal::random(),
            fee_pool: CoinValue(u128::MIN),
            fee_multiplier: u128::MIN,
            dosc_speed: u128::MIN,
            pools_hash: HashVal::random(),
            stakes_hash: HashVal::random(),
        }
    } else if modifier == <u8 as Into<u128>>::into(u8::MAX) {
        Header {
            network: NetID::Mainnet,
            previous: HashVal::random(),
            height: BlockHeight(u8::MAX.into()),
            history_hash: HashVal::random(),
            coins_hash: HashVal::random(),
            transactions_hash: HashVal::random(),
            fee_pool: CoinValue(u8::MAX.into()),
            fee_multiplier: u8::MAX.into(),
            dosc_speed: u8::MAX.into(),
            pools_hash: HashVal::random(),
            stakes_hash: HashVal::random(),
        }
    } else if modifier == <u16 as Into<u128>>::into(u16::MAX) {
        Header {
            network: NetID::Mainnet,
            previous: HashVal::random(),
            height: BlockHeight(u16::MAX.into()),
            history_hash: HashVal::random(),
            coins_hash: HashVal::random(),
            transactions_hash: HashVal::random(),
            fee_pool: CoinValue(u16::MAX.into()),
            fee_multiplier: u16::MAX.into(),
            dosc_speed: u16::MAX.into(),
            pools_hash: HashVal::random(),
            stakes_hash: HashVal::random(),
        }
    } else if modifier == <u32 as Into<u128>>::into(u32::MAX) {
        Header {
            network: NetID::Mainnet,
            previous: HashVal::random(),
            height: BlockHeight(u32::MAX.into()),
            history_hash: HashVal::random(),
            coins_hash: HashVal::random(),
            transactions_hash: HashVal::random(),
            fee_pool: CoinValue(u32::MAX.into()),
            fee_multiplier: u32::MAX.into(),
            dosc_speed: u32::MAX.into(),
            pools_hash: HashVal::random(),
            stakes_hash: HashVal::random(),
        }
    } else if modifier == <u64 as Into<u128>>::into(u64::MAX) {
        Header {
            network: NetID::Mainnet,
            previous: HashVal::random(),
            height: BlockHeight(u64::MAX),
            history_hash: HashVal::random(),
            coins_hash: HashVal::random(),
            transactions_hash: HashVal::random(),
            fee_pool: CoinValue(u64::MAX.into()),
            fee_multiplier: u64::MAX.into(),
            dosc_speed: u64::MAX.into(),
            pools_hash: HashVal::random(),
            stakes_hash: HashVal::random(),
        }
    } else if modifier == u128::MAX {
        Header {
            network: NetID::Mainnet,
            previous: HashVal::random(),
            height: BlockHeight(u64::MAX),
            history_hash: HashVal::random(),
            coins_hash: HashVal::random(),
            transactions_hash: HashVal::random(),
            fee_pool: CoinValue(u128::MAX),
            fee_multiplier: u128::MAX,
            dosc_speed: u128::MAX,
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
            .gen_range(0..epoch);
    }

    let e_post_end: u64 = rand::thread_rng()
        .gen_range(epoch + 1..u64::MAX);

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
        let keypair = ed25519_keygen();
        stakedoc.pubkey = Ed25519PK::from_bytes(&keypair.0.0).unwrap();

        signatures.push(keypair.1.sign(header).as_slice().try_into().unwrap());

        epoch_syms += stakedoc.syms_staked;

        if stakedoc.e_start <= epoch + 1 && stakedoc.e_post_end > epoch + 1 {
            next_epoch_syms += stakedoc.syms_staked;
        }

        let mut stakedoc_vec = stdcode::serialize(&stakedoc).unwrap();

        stakes.append(&mut stakedoc_vec);
    }

    let mut stakes_structured = stdcode::serialize(&epoch_syms).unwrap();
    stakes_structured.append(&mut stdcode::serialize(&next_epoch_syms).unwrap());
    stakes_structured.append(&mut stakes);

    (stakes, signatures)
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
    genesis_stakes_hash: Vec<u8>,
    genesis_transactions_hash: Vec<u8>
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
    let mut genesis_height = hex::encode(stdcode::serialize(&genesis_height).unwrap());
    genesis_height = format!("{:0>64}", genesis_height);
    let mut genesis_height = hex::decode(genesis_height).unwrap();

    let mut genesis_stakes_hash = genesis_stakes_hash.to_vec();
    let mut genesis_transactions_hash = genesis_transactions_hash.to_vec();

    let mut initialization_data: Vec<u8> = vec!();
    initialization_data.append(&mut func_selector);
    initialization_data.append(&mut genesis_height);
    initialization_data.append(&mut genesis_stakes_hash);
    initialization_data.append(&mut genesis_transactions_hash);

    let constructor_data = (
        Token::Address(bridge_address),
        Token::Bytes(initialization_data)
    );

    let bridge_proxy = factory
        .deploy(constructor_data)
        .expect("Unable to deploy bridge proxy.")
        .send()
        .await
        .expect("Error awaiting bridge proxy contract creation.");
    
    
    let bridge_proxy_address = bridge_proxy.address();

    let wrapped_bridge_proxy = ThemelioBridge::new(bridge_proxy_address, client.clone());

    wrapped_bridge_proxy
}

async fn test_e2e(num_stakedocs: u32, num_transactions: u32) -> Result<()> {
    let config = Config::new().unwrap();

    let cross_epoch: bool = rand::thread_rng().gen();
    let modifier: u128 = rand::thread_rng().gen();
    let mut genesis_header = random_header(modifier);
    let mut header: Header = random_header(modifier);

    if cross_epoch {
        genesis_header.height = BlockHeight(rand::thread_rng().gen_range(0..u64::MAX / STAKE_EPOCH) - 1);
        header.height = genesis_header.height + BlockHeight(rand::thread_rng().gen_range(1..STAKE_EPOCH));
    } else {
        genesis_header.height = BlockHeight(rand::thread_rng().gen_range(0..u64::MAX / STAKE_EPOCH));
        header.height = genesis_header.height + BlockHeight(rand::thread_rng().gen_range(1..STAKE_EPOCH - (genesis_header.height.0 - genesis_header.height.epoch() * STAKE_EPOCH)))
    }

    let genesis_header_epoch = genesis_header.height.epoch();

    // create random transactions with ethereum addresses in additional_data of first output
    let mut datablocks = create_datablocks(num_transactions);

    let index: usize = rand::thread_rng()
        .gen_range(0..num_transactions)
        .try_into()
        .expect("Error converting to usize.");

    datablocks[index].outputs[0].covhash = BRIDGE_COVHASH;

    let tx_to_prove = datablocks
        .get(index)
        .ok_or("Unable to get tx datablock to prove.")
        .expect("Error getting tx to prove.");

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

    let value: u128 = tx_to_prove
        .outputs[0]
        .value
        .into();

    let serialized_tx = stdcode::serialize(&tx_to_prove)
        .expect("Unable to serialize tx.");
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

    // replace the random 'transactions_hash' from 'header' with the root of our merkle tree
    header.transactions_hash = HashVal(root);

    // create random staker keypairs, serialize header and sign with each sk, store in vecs
    let serialized_header = stdcode::serialize(&header).expect("Unable to serialize header.");
    let serialized_header_bytes = Bytes::from(serialized_header.clone());

    let stakes: Vec<u8>;
    let signatures: Vec<[u8; 32]>;
    (stakes, signatures) = create_stakes_and_sign_header(num_stakedocs, genesis_header_epoch, &serialized_header);
    let stakes = Bytes::from(stakes);

    let stakes_hash = blake3::keyed_hash(
        blake3::hash(DATA_BLOCK_HASH_KEY).as_bytes(),
        &hex::decode(&stakes).unwrap()
    );

    let wrapped_bridge_proxy = setup_bridge_proxy(genesis_header.height.0, stakes_hash.as_bytes().to_vec(), vec!(0 as u8; 32)).await;

    // send tx to verifyStakes()
    let call = wrapped_bridge_proxy.verify_stakes(stakes.clone());
    let pending_tx = call.send().await?;
    let receipt = pending_tx.await?;

    println!("\n*** verifyStakers() receipt ***\n{:#?}\n********************\n", receipt.unwrap());

    // send tx to verifyHeader
    let call = wrapped_bridge_proxy
        .verify_header(U256::from(genesis_header.height.0), serialized_header_bytes, stakes.clone(), signatures)
        .gas(GAS_LIMIT);
    let pending_tx = call.send().await?;
    let receipt = pending_tx.await?;

    println!("\n***** verifyHeader() receipt *****\n{:#?}\n********************\n", receipt.unwrap());

    // send tx to verifyTx()
    let call = wrapped_bridge_proxy
        .verify_tx(serialized_tx, U256::from(index), U256::from(header.height.0), proof)
        .gas(GAS_LIMIT);
    let pending_tx = call.send().await?;
    let receipt = pending_tx.await?;

    println!("\n***** verifyTx() receipt *****\n{:#?}\n********************\n", receipt.unwrap());

    // send tx to burn()
    let call = wrapped_bridge_proxy.burn(config.wallet.address(), U256::from(denom.0), U256::from(value), *b"00000000000000000000000000000000")
        .gas(GAS_LIMIT);
    let pending_tx = call.send().await?;
    let receipt = pending_tx.await?;

    println!("\n***** burn() receipt *****\n{:#?}\n********************\n", receipt.unwrap());

    Ok(())
}

#[tokio::main]
async fn main() -> Result<()> {
    let args = Args::parse();

    if args.initial_conditions {

    } else {
        let mut num_stakers = String::new();
        let mut merkle_tree_height = String::new();

        println!("Input the desired number of stakers: ");
        io::stdin()
            .read_line(&mut num_stakers)
            .expect("Failed to read number of stakers.");

        println!("Input the desired Merkle tree height: ");
        io::stdin()
            .read_line(&mut merkle_tree_height)
            .expect("Failed to read Merkle tree height.");

        let num_stakers: u32 = num_stakers
            .trim()
            .parse()
            .expect("Error parsing number of stakers.");

        let merkle_tree_height: u32 = merkle_tree_height
            .trim()
            .parse()
            .expect("Error parsing Merkle tree height.");

        test_e2e(num_stakers, merkle_tree_height)
            .await
            .expect("Error awaiting end-to-end test.");
    }
    
    Ok(())
}