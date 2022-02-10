use std::{str::FromStr, process::Command};
use std::ops;
use std::process::ExitStatus;

use blake3;
use ethers::core::k256::ecdsa::SigningKey;
use ethers::prelude::{Http, LocalWallet, Provider, SignerMiddleware, Wallet};
use ethers::providers::Middleware;
use ethers::utils::{Ganache, GanacheInstance};
use ethers::signers::Signer;
use ethers::types::{TransactionReceipt, TransactionRequest};
use rand::Rng;
use rayon::iter::{IntoParallelIterator, ParallelIterator};
use rs_merkle::{MerkleTree, MerkleProof, Hasher};
use sha3::{Digest, Keccak256};
use themelio_structs::{Address, CoinData, CoinID, Denom, Header, NetID, BlockHeight, CoinValue, Transaction, TxKind, TxHash};
use tmelcrypt::HashVal;

const DATA_BLOCK_HASH_KEY: &[u8; 13] = b"smt_datablock";
const NODE_HASH_KEY: &[u8; 8] = b"smt_node";

#[derive(Clone)]
pub struct Blake3Algorithm {}

impl Hasher for Blake3Algorithm {
    type Hash = [u8; 32];
    fn hash(data: &[u8]) -> [u8; 32] {
        *blake3::keyed_hash(blake3::hash(NODE_HASH_KEY).as_bytes(), data).as_bytes()
    }
}

fn create_datablocks(num: u32) -> Vec<Transaction> {
    let range: ops::Range<u32> = 0..num;

    range.into_par_iter().map(|_index| {
        let inputs: Vec<CoinID> = vec![ CoinID {
            txhash: TxHash(HashVal::random()),
            index: rand::thread_rng().gen(),
        }];

        let output_one: CoinData = CoinData {
            covhash: Address(HashVal::random()),
            value: CoinValue(rand::thread_rng().gen()),
            denom: Denom::Mel,
            additional_data: vec![],
        };

        let output_two: CoinData = CoinData {
            covhash: Address(HashVal::random()),
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

    }).collect::<Vec<Transaction>>()
}

fn random_header() -> Header {
    Header {
        network: NetID::Mainnet,
        previous: HashVal::random(),
        height: BlockHeight(rand::thread_rng().gen_range(0..1000)),
        history_hash: HashVal::random(),
        coins_hash: HashVal::random(),
        transactions_hash: HashVal::random(),
        fee_pool: CoinValue(rand::thread_rng().gen_range(0..1000)),
        fee_multiplier: rand::thread_rng().gen_range(0..1000),
        dosc_speed: rand::thread_rng().gen_range(0..1000),
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

#[tokio::main]
async fn main() {
    // submit_header_and_verify_tx();

    let ganache: GanacheInstance = Ganache::new().spawn();

    let wallet_one: LocalWallet = ganache.keys()[0].clone().into();
    let wallet_two: LocalWallet = ganache.keys()[1].clone().into();


    // In progress
    //
    // let wallet_one_address: String = String::from("a8bc40dc835d1a6ae6a33211f5c056e7d6ce8c25b8d1b57876488f5808da5570");
    //
    // let thingy = wallet_one_address.as_bytes();
    //
    // let other_thingy = SigningKey::from_bytes(thingy).expect("This is horrible.");
    //
    // let wallet_one: LocalWallet = LocalWallet::new(&mut other_thingy);
    // let wallet_two: LocalWallet = LocalWallet::new(&mut rand::thread_rng());

    let endpoint: String = String::from("https://rinkeby.infura.io/v3/0771c92c5c6c42669665a80daa68b848");

    // connect to the network
    // let provider = Provider::<Http>::try_from(ganache.endpoint()).expect("Failed to create provider.");
    let provider = Provider::<Http>::try_from(endpoint).expect("Failed to create provider.");

    let client: SignerMiddleware<Provider<Http>, Wallet<SigningKey>> = SignerMiddleware::new(provider, wallet_one);

    let send_transaction: TransactionRequest = TransactionRequest::new().to(wallet_two.address()).value(10000);

    let pending_transaction = client.send_transaction(send_transaction, None).await.expect("Could not send transaction.");

    let receipt_option: Option<TransactionReceipt> = pending_transaction.await.expect("Connection to network failed.");

    let receipt: TransactionReceipt = receipt_option.expect("Transaction returned None.");

    let receipt_transaction: Option<ethers::prelude::Transaction> = client.get_transaction(receipt.transaction_hash).await.expect("Could not receive transaction.");

    let receipt_transaction_string: String = serde_json::to_string(&receipt_transaction).expect("Could not convert receipt transaction to a string.");

    let receipt_string: String = serde_json::to_string(&receipt).expect("Could not convert receipt to a string.");

    println!("Sent transaction: {}\n", receipt_transaction_string);
    println!("Transaction receipt: {}", receipt_string);
}