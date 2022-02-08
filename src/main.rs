use themelio_stf::{Header, NetID, BlockHeight, CoinValue, Transaction, TxKind, TxHash, HexBytes, Denom};
use tmelcrypt::HashVal;
use std::{str::FromStr, process::Command};
use themelio_stf::melvm::{Address, Covenant, CoinID, CoinData};
use blake3;
use rs_merkle::{MerkleTree, MerkleProof, Hasher};
use rand::Rng;
use sha3::Digest;

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

fn main() {
    submit_header_and_verify_tx();
}

fn create_datablocks(num: u32) -> Vec<Transaction> {
    let mut datablocks: Vec<Transaction> = Vec::new();

    for _i in 0..num {
        datablocks.push(
            Transaction {
                kind: TxKind::Swap,
                inputs: vec![ CoinID {
                    txhash: TxHash(HashVal::random()),
                    index: rand::thread_rng().gen(),
                }],
                outputs: vec![ CoinData {
                    covhash: Address(HashVal::random()),
                    value: CoinValue(rand::thread_rng().gen()),
                    denom: Denom::Mel,
                    additional_data: vec![],
                }, CoinData {
                    covhash: Address(HashVal::random()),
                    value: CoinValue(rand::thread_rng().gen()),
                    denom: Denom::Mel,
                    additional_data: vec![]
                }],
                fee: CoinValue(rand::thread_rng().gen()),
                scripts: vec![Covenant((0..162).map(|_| { rand::thread_rng().gen::<u8>() }).collect())],
                data: (0..2).map(|_| { rand::thread_rng().gen::<u8>() }).collect(),
                sigs: vec![HexBytes((0..128).map(|_| { rand::thread_rng().gen::<u8>() }).collect())]
            }
        );
    }
    datablocks
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
    let network = hex::encode([network]);
    let network = format!("{:0<64}", network);

    let previous = header.previous.to_string();

    let height = header.height.to_string();
    let height = format!("{:0>64}", height);

    let history_hash = header.history_hash.to_string();
    let coins_hash = header.coins_hash.to_string();
    let transactions_hash = header.transactions_hash.to_string();

    let fee_pool = header.fee_pool.to_string();
    let fee_pool = format!("{:0>64}", (fee_pool));
    let fee_pool = fee_pool.replace(".", "0");

    let fee_multiplier = header.fee_multiplier.to_string();
    let fee_multiplier = format!("{:0>64}", fee_multiplier);

    let dosc_speed = header.dosc_speed.to_string();
    let dosc_speed = format!("{:0>64}", dosc_speed);

    let pools_hash = header.pools_hash.to_string();
    let stakes_hash = header.stakes_hash.to_string();

    let address = &String::from(hex::encode(HashVal::random()))[0..40];
    let address = format!("{:0>64}", address);

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
    let serialized_datablock = String::from("0x") + &hex::encode(stdcode::serialize(datablock_to_prove).unwrap());
    let height = height.to_string();
    let mut proof = hex::encode(merkle_proof.to_bytes());

    if proof.len() == 0 {
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
    let mut num_datablocks = String::new();
    println!("How many leaves?:");
    std::io::stdin()
        .read_line(&mut num_datablocks)
        .expect("Failed to read line.");
    let num_datablocks: u32 = num_datablocks.trim().parse().expect("Please type a number.");

    // create required number of random Transaction type datablocks to turn into leaves
    let datablocks = create_datablocks(num_datablocks);
    let leaves: Vec<[u8; 32]> = datablocks
        .iter()
        .map(|x| *blake3::keyed_hash(blake3::hash(DATA_BLOCK_HASH_KEY).as_bytes(), &stdcode::serialize(&x).unwrap()).as_bytes())
        .collect();

    // use leaves to create Merkle tree and extract a random datablock to create its proof
    let merkle_tree = MerkleTree::<Blake3Algorithm>::from_leaves(&leaves);
    let index: usize = rand::thread_rng().gen_range(0..num_datablocks).try_into().unwrap();
    let datablock_to_prove = datablocks.get(index).ok_or("Can't get datablocks to prove.").unwrap();
    let merkle_proof = merkle_tree.proof(&vec![index]);
    let merkle_root = merkle_tree.root().ok_or("Couldn't get the merkle root.").unwrap();

    // hash function signature and save the first 4 bytes to derive function selector
    let mut hasher = sha3::Keccak256::new();
    let func_signature = "relayHeader((bytes1,bytes32,uint64,bytes32,bytes32,bytes32,uint128,uint128,uint128,bytes32,bytes32,address))";
    hasher.update(func_signature);
    let function_hash = hex::encode(hasher.finalize());
    let function_selector = &function_hash[0..8];

    // instantiate a random Header and save the merkle root of tree in header.transactions_hash for proof verification
    let mut header = random_header();
    header.transactions_hash = HashVal::from_str(&hex::encode(merkle_root)).unwrap();

    // encode header and format it and function selector into calldata
    let encoded_header = encode_header(header);
    let calldata = format!("{}{}{}", "0x", function_selector, encoded_header);
    
    // send `cast send <contract> <calldata>` to relayHeader()
    let output = Command::new("seth")
        .args(["send", "0xd9741b289a7a00761a2edb16b793ece448efb374", "--password", "/home/marco/password", &calldata])
        .status()
        .expect("Failed to send tx to relayHeader()");
    println!("{}", output);

    // encode raw_tx, tx_index, block_height, and proof
    let (datablock, index, height, proof) = format_verify_tx_args(datablock_to_prove, index, header.height, merkle_proof);
    
    // send RPC to verifyTx()
    let output = Command::new("seth")
        .args(["send", "0xd9741b289a7a00761a2edb16b793ece448efb374", "--password", "/home/marco/password", "verifyTx(bytes,uint256,uint256,bytes32[])", &datablock, &index, &height, &proof])
        .status()
        .expect("Failed to send tx to verifyTx()");
    println!("{}", output);
}