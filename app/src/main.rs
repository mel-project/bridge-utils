use std::{str::FromStr, process::Command};
use std::convert::TryFrom;
use std::ops::{Range, Deref};
use std::process::ExitStatus;
use std::sync::Arc;

use bindings::themelio_bridge::ThemelioBridge;
use blake3;
use ed25519_compact::{KeyPair, PublicKey, Signature, Seed, Noise};
use ethers::prelude::{Http, LocalWallet, Middleware, SignerMiddleware, Wallet};
use ethers::providers::Provider;
use ethers::signers::Signer;
use ethers::types::{Address as EthersAddress, Bytes, TransactionReceipt, TransactionRequest, U256};
use eyre::Result;
use rand::Rng;
use rayon::iter::{IntoParallelIterator, ParallelIterator};
use rs_merkle::{MerkleTree, MerkleProof, Hasher};
use sha3::{Digest, Keccak256};
use themelio_structs::{Address as ThemelioAddress, CoinData, CoinID, Denom, Header, NetID, BlockHeight, CoinValue, Transaction, TxKind, TxHash};
use tmelcrypt::HashVal;

const DATA_BLOCK_HASH_KEY: &[u8; 13] = b"smt_datablock";
const NODE_HASH_KEY: &[u8; 8] = b"smt_node";
const GAS_LIMIT: u32 = 20_000_000;

#[derive(Clone)]
pub struct Blake3Algorithm {}

impl Hasher for Blake3Algorithm {
    type Hash = [u8; 32];

    fn hash(data: &[u8]) -> [u8; 32] {
        *blake3::keyed_hash(blake3::hash(NODE_HASH_KEY).as_bytes(), data).as_bytes()
    }
}

// struct StakerInfo<'a> {
//     public_key: PublicKey,
//     syms_staked: u32,
//     header_signature: &'a [u8]
// }

// impl<T: Send> FromParallelIterator<T> for StakerInfo {
//     fn from_par_iter<I>(par_iter: I) -> self
//         where I: IntoParallelIterator<Item = T>
//     {
//         let par_iter = par_iter.into_par_iter();
//         StakerInfo {
//             public_key
//         }
//     }
// }

fn random_transaction() -> Transaction {
    let inputs: Vec<CoinID> = vec![ CoinID {
        txhash: TxHash(HashVal::random()),
        index: rand::thread_rng().gen(),
    }];

    let range: Range<u32> = 0..20;

    let recipient_address = range.into_par_iter().map(|_index| {
        rand::thread_rng().gen()
    }).collect::<Vec<u8>>();

    let output_one: CoinData = CoinData {
        covhash: ThemelioAddress(HashVal::random()),
        value: CoinValue(rand::thread_rng().gen()),
        denom: Denom::Mel,
        additional_data: recipient_address,
    };

    let output_two: CoinData = CoinData {
        covhash: ThemelioAddress(HashVal::random()),
        value: CoinValue(rand::thread_rng().gen()),
        denom: Denom::Mel,
        additional_data: vec![]
    };

    let outputs: Vec<CoinData> = vec![output_one, output_two];

    let covenants: Vec<Vec<u8>> = vec![vec![rand::thread_rng().gen()]];

    let sigs: Vec<Vec<u8>> = vec![vec![rand::thread_rng().gen()]];

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

    range.into_par_iter().map(|_index| {
        random_transaction()
    }).collect::<Vec<Transaction>>()
}

fn random_header() -> Header {
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

fn encode_header(header: Header) -> String {
    let network: u8 = header.network.into();
    let network: String = hex::encode([network]);
    let network: String = format!("{:0<64}", network);

    let previous = header.previous.to_string();

    let height: String = header.height.to_string();
    let height: String = format!("{:0>64}", height);

    let history_hash: String = header.history_hash.to_string();
    let coins_hash: String = header.coins_hash.to_string();
    let transactions_hash: String = header.transactions_hash.to_string();

    let fee_pool: String = header.fee_pool.to_string();
    let fee_pool: String = format!("{:0>64}", (fee_pool));
    let fee_pool: String = fee_pool.replace(".", "0");

    let fee_multiplier: String = header.fee_multiplier.to_string();
    let fee_multiplier: String = format!("{:0>64}", fee_multiplier);

    let dosc_speed: String = header.dosc_speed.to_string();
    let dosc_speed: String = format!("{:0>64}", dosc_speed);

    let pools_hash: String = header.pools_hash.to_string();
    let stakes_hash: String = header.stakes_hash.to_string();

    let address: &str = &String::from(hex::encode(HashVal::random()))[0..40];
    let address: String = format!("{:0>64}", address);

    format!(
        "{}{}{}{}{}{}{}{}{}{}{}{}",
        network,
        previous,
        height,
        history_hash,
        coins_hash,
        transactions_hash,
        fee_pool,
        fee_multiplier,
        dosc_speed,
        pools_hash,
        stakes_hash,
        address
    )
}

fn format_verify_tx_args(
    datablock_to_prove: &Transaction,
    index: usize,
    height: BlockHeight,
    merkle_proof: MerkleProof<Blake3Algorithm>
) -> (String, String, String, String) {
    let serialized_datablock: String = String::from("0x") + &hex::encode(stdcode::serialize(datablock_to_prove).unwrap());
    let height: String = height.to_string();

    let mut proof: String = hex::encode(merkle_proof.to_bytes());

    if proof.is_empty() {
        proof = format!("[{}]", proof);
    } else {
        proof = proof
            .chars()
            .collect::<Vec<char>>()
            .chunks(64)
            .map(|c| c.iter().collect::<String>())
            .collect::<Vec<String>>()
            .join(",0x");
        proof = format!("[0x{}]", proof);
    }

    (serialized_datablock, index.to_string(), height, proof)
}

fn submit_header_and_verify_tx() {
    // Prompt user for number of leaves desired in Merkle tree
    let mut num_datablocks: String = String::new();
    println!("How many leaves?:");
    std::io::stdin()
        .read_line(&mut num_datablocks)
        .expect("Failed to read line.");
    let num_datablocks: u32 = num_datablocks.trim().parse().expect("Please type a number.");

    // create required number of random Transaction type datablocks to turn into leaves
    let datablocks: Vec<Transaction> = create_datablocks(num_datablocks);
    let leaves: Vec<[u8; 32]> = datablocks
        .iter()
        .map(|x| *blake3::keyed_hash(blake3::hash(DATA_BLOCK_HASH_KEY).as_bytes(), &stdcode::serialize(&x).unwrap()).as_bytes())
        .collect();

    // use leaves to create Merkle tree and extract a random datablock to create its proof
    let merkle_tree: MerkleTree<Blake3Algorithm> = MerkleTree::<Blake3Algorithm>::from_leaves(&leaves);
    let index: usize = rand::thread_rng().gen_range(0..num_datablocks).try_into().unwrap();
    let datablock_to_prove: &Transaction = datablocks.get(index).ok_or("Can't get datablocks to prove.").unwrap();
    let merkle_proof: MerkleProof<Blake3Algorithm> = merkle_tree.proof(&vec![index]);
    let merkle_root: [u8; 32] = merkle_tree.root().ok_or("Couldn't get the merkle root.").expect("Fill in a reason");

    // hash function signature and save the first 4 bytes to derive function selector
    let mut hasher: Keccak256 = sha3::Keccak256::new();
    let func_signature = "relayHeader((bytes1,bytes32,uint64,bytes32,bytes32,bytes32,uint128,uint128,uint128,bytes32,bytes32,address))";
    hasher.update(func_signature);
    let function_hash = hex::encode(hasher.finalize());
    let function_selector = &function_hash[0..8];

    // instantiate a random Header and save the merkle root of tree in header.transactions_hash for proof verification
    let mut header: Header = random_header();
    header.transactions_hash = HashVal::from_str(&hex::encode(merkle_root)).unwrap();

    // encode header and format it and function selector into calldata
    let encoded_header: String = encode_header(header);
    let calldata: String = format!("{}{}{}", "0x", function_selector, encoded_header);
    
    // send `cast send <contract> <calldata>` to relayHeader()
    let output: ExitStatus = Command::new("seth")
        .args(["send", "0xd9741b289a7a00761a2edb16b793ece448efb374", "--password", "/home/marco/password", &calldata])
        .status()
        .expect("Failed to send tx to relayHeader()");
    println!("{}", output);

    // encode raw_tx, tx_index, block_height, and proof
    let (datablock, index, height, proof): (String, String, String, String) = format_verify_tx_args(datablock_to_prove, index, header.height, merkle_proof);
    
    // send RPC to verifyTx()
    let output: ExitStatus = Command::new("seth")
        .args(["send", "0xd9741b289a7a00761a2edb16b793ece448efb374", "--password", "/home/marco/password", "verifyTx(bytes,uint256,uint256,bytes32[])", &datablock, &index, &height, &proof])
        .status()
        .expect("Failed to send tx to verifyTx()");
    println!("{}", output);
}

fn random_staker(message: &[u8]) -> ([u8; 32], U256, [u8; 64]) {
    let keypair: KeyPair = KeyPair::from_seed(Seed::default());
    
    let syms_staked: u32 = rand::thread_rng().gen_range(0..1024);
    let syms_staked: U256 = U256::from(syms_staked);

    let signature: Signature = keypair.sk.sign(message, Some(Noise::generate()));

    (*keypair.pk, syms_staked, *signature.deref())
}

fn create_stakers(num: u32, message: &[u8]) -> (Vec<[u8; 32]>, Vec<U256>, Vec<[u8; 32]>) {
    let mut public_keys: Vec<[u8; 32]> = vec![];
    let mut staked_syms: Vec<U256> = vec![];
    let mut signatures: Vec<[u8; 32]> = vec![];

    for _i in 0..num {
        let staker = random_staker(message);
        let mut signature_slice: [u8; 32] = [0; 32];

        public_keys.push(staker.0);
        staked_syms.push(staker.1);

        signature_slice.copy_from_slice(&staker.2[0..32]);
        signatures.push(signature_slice);

        signature_slice.copy_from_slice(&staker.2[32..64]);
        signatures.push(signature_slice);
    }

    (public_keys, staked_syms, signatures)
}

// TESTS
async fn setup_bridge() -> ThemelioBridge<SignerMiddleware<Provider<Http>, LocalWallet>>{
    let provider = Provider::<Http>::try_from(
        "https://rinkeby.infura.io/v3/0771c92c5c6c42669665a80daa68b848",
    ).expect("Provider unable to be initialized.");

    let chain_id = provider.get_chainid().await.unwrap().as_u64();

    let wallet: LocalWallet = "acba26d579163e4780ed8212feedd5ca4dfd381df47d7a6d9b20076ba3ddefe2".parse().unwrap();
    let wallet = wallet.with_chain_id(chain_id);

    let client = SignerMiddleware::new(provider.clone(), wallet);
    let client = Arc::new(client);

    let address = "0x77653c46fbbadb73a389f99bc2a19ab5efb2ec01".parse::<EthersAddress>()
        .expect("Address unable to be parsed");

    let bridge = ThemelioBridge::new(address, client.clone());

    bridge
}

async fn test_e2e() -> Result<()> {
    let bridge = setup_bridge().await;

    // create a random block header
    let mut header: Header = random_header();

    let block_height = U256::from(header.height.0);

    let header_epoch = header.height.epoch();
    let header_epoch = U256::from(header_epoch);

    // create random transactions with ethereum addresses in additional_data of first output
    let num_datablocks = rand::thread_rng().gen_range(0..1024);
    let datablocks = create_datablocks(num_datablocks);

    // create a merkle tree and get the root, replace transactions_hash with it
    // create merkle proof for a random tx to verify
    let leaves: Vec<[u8; 32]> = datablocks
        .iter()
        .map(|x| *blake3::keyed_hash(blake3::hash(DATA_BLOCK_HASH_KEY).as_bytes(), &stdcode::serialize(&x).unwrap()).as_bytes())
        .collect();

    let merkle_tree: MerkleTree<Blake3Algorithm> = MerkleTree::<Blake3Algorithm>::from_leaves(&leaves);

    let index: usize = rand::thread_rng().gen_range(0..num_datablocks).try_into().unwrap();
    let index_u256 = U256::from(index);

    let tx_to_prove: &Transaction = datablocks.get(index).ok_or("Unable to get datablocks to prove.").unwrap();

    let mel_amount: u128 = tx_to_prove.outputs[0].value.into();
    let mel_amount = U256::from(mel_amount);

    let serialized_tx = stdcode::serialize(&tx_to_prove).expect("Unable to serialize tx.");
    let serialized_tx_bytes = Bytes::from(serialized_tx);

    let merkle_proof: MerkleProof<Blake3Algorithm> = merkle_tree.proof(&vec![index]);
    let merkle_proof_vec: Vec<[u8; 32]> = merkle_proof.proof_hashes().to_vec();

    let merkle_root: [u8; 32] = merkle_tree.root().ok_or("Unable to get merkle root.").expect("Fill in a reason");

    // replace the random 'transactions_hash' from 'header' with the root of our merkle tree
    header.transactions_hash = HashVal(merkle_root);

    // create random staker keypairs, serialize header and sign with each sk, store in vecs
    let num_stakers = 1;//rand::thread_rng().gen_range(1..16);

    let serialized_header = stdcode::serialize(&header).expect("Unable to serialize header.");
    let serialized_header_bytes = Bytes::from(serialized_header.clone());

    let stakers_info = create_stakers(num_stakers, &serialized_header);

    let stakers: Vec<[u8; 32]> = stakers_info.0;
    let staker_syms: Vec<U256> = stakers_info.1;
    let signatures: Vec<[u8; 32]> = stakers_info.2;

    // send tx to relayStakers()
    let call = bridge.relay_stakers(header_epoch, stakers.clone(), staker_syms);
    let pending_tx = call.send().await?;
    let receipt = pending_tx.await?;

    println!("\n*** relayStakers() receipt ***\n{:#?}\n", receipt.unwrap());
    //assert

    // send tx to relayHeader
    let call = bridge
        .relay_header(serialized_header_bytes, stakers.clone(), signatures)
        .gas(GAS_LIMIT);
    let pending_tx = call.send().await?;
    let receipt = pending_tx.await?;

    println!("\n***** relayHeader() receipt *****\n{:#?}\n", receipt);
    // assert

    // send tx to verifyTx()
    let call = bridge
        .verify_tx(serialized_tx_bytes, index_u256, block_height, merkle_proof_vec)
        .gas(20000000);
    let pending_tx = call.send().await?;
    let receipt = pending_tx.await?;

    println!("\n***** verifyTx() receipt *****\n{:#?}", receipt.unwrap());
    //assert

    // send tx to burn()
    let call = bridge.burn(mel_amount).gas(20000000);
    let pending_tx = call.send().await?;
    let receipt = pending_tx.await?;

    println!("\n***** burn() receipt *****\n{:#?}", receipt.unwrap());
    //assert

    Ok(())
}

#[tokio::main]
async fn main() -> Result<()> {
    test_e2e().await?;

    Ok(())
}