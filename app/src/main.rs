use std::convert::TryFrom;
use std::{env, io};
use std::ops::{Range, Deref};
use std::sync::Arc;

use bindings::themelio_bridge::ThemelioBridge;
use blake3;
use ed25519_compact::{KeyPair, Signature, Seed, Noise};
use ethers::prelude::{Http, LocalWallet, Middleware, SignerMiddleware};
use ethers::providers::Provider;
use ethers::signers::Signer;
use ethers::types::{Address as EthersAddress, Bytes, U256};
use eyre::Result;
use rand::Rng;
use rayon::iter::{IntoParallelIterator, ParallelIterator};
use rs_merkle::{MerkleTree, MerkleProof, Hasher};
use themelio_structs::{
    Address as ThemelioAddress,
    BlockHeight,
    CoinData,
    CoinID,
    Denom,
    Header,
    NetID,
    CoinValue,
    Transaction,
    TxKind,
    TxHash
};
use tmelcrypt::HashVal;

const DATA_BLOCK_HASH_KEY: &[u8; 13] = b"smt_datablock";
const NODE_HASH_KEY: &[u8; 8] = b"smt_node";
const GAS_LIMIT: u32 = 29_000_000;

pub struct Config {
    pub wallet: LocalWallet,
}

impl Config {
    pub fn new() -> Result<Config, &'static str> {
        let secret_key = env::var("SECRET_KEY").expect("Missing SECRET_KEY env variable.");
        let wallet: LocalWallet = secret_key.parse().unwrap();

        Ok(Config { wallet })
    }
}

#[derive(Clone)]
pub struct Blake3Algorithm {}

impl Hasher for Blake3Algorithm {
    type Hash = [u8; 32];

    fn hash(data: &[u8]) -> [u8; 32] {
        *blake3::keyed_hash(blake3::hash(NODE_HASH_KEY).as_bytes(), data).as_bytes()
    }
}

fn random_transaction() -> Transaction {
    let inputs: Vec<CoinID> = vec![ CoinID {
        txhash: TxHash(HashVal::random()),
        index: rand::thread_rng().gen(),
    }];

    //let range: Range<u32> = 0..20;
    //
    // let recipient_address = range.into_par_iter().map(|_index| {
    //     rand::thread_rng().gen()
    // }).collect::<Vec<u8>>();
    let recipient_address: Vec<u8> = Config::new()
        .expect("Missing env variables.")
        .wallet.address()
        .as_bytes()
        .to_vec();

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
async fn setup_bridge(address: &str) -> ThemelioBridge<SignerMiddleware<Provider<Http>, LocalWallet>>{
    let provider = Provider::<Http>::try_from(
        "https://rinkeby.infura.io/v3/0771c92c5c6c42669665a80daa68b848",
    ).expect("Provider unable to be initialized.");

    let chain_id = provider.get_chainid().await.unwrap().as_u64();

    let wallet: LocalWallet = Config::new()
        .expect("Missing env variables.")
        .wallet;

    let wallet = wallet.with_chain_id(chain_id);

    let client = SignerMiddleware::new(provider.clone(), wallet);
    let client = Arc::new(client);

    let address = address.parse::<EthersAddress>()
        .expect("Unable to parse address.");

    let bridge = ThemelioBridge::new(address, client.clone());

    bridge
}

async fn test_e2e(address: &str, num_stakers: u32, merkle_tree_height: u32) -> Result<()> {
    let bridge = setup_bridge(address).await;

    // create a random block header
    let mut header: Header = random_header();

    let block_height = U256::from(header.height.0);

    let header_epoch = header.height.epoch();
    let header_epoch = U256::from(header_epoch);

    // create random transactions with ethereum addresses in additional_data of first output
    let num_datablocks = rand::thread_rng().gen_range(2u32.pow(merkle_tree_height - 1)..2u32.pow(merkle_tree_height));
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

    let tx_to_prove: &Transaction = datablocks.get(index).ok_or("Unable to get tx datablock to prove.").unwrap();

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
    let serialized_header = stdcode::serialize(&header).expect("Unable to serialize header.");
    let serialized_header_bytes = Bytes::from(serialized_header.clone());

    let stakers_info = create_stakers(num_stakers, &serialized_header);

    let stakers: Vec<[u8; 32]> = stakers_info.0;
    let staker_syms: Vec<U256> = stakers_info.1;
    let signatures: Vec<[u8; 32]> = stakers_info.2;

    // send tx to relayStakers()
    let call = bridge.relay_stakers(header_epoch, stakers.clone(), staker_syms.clone());
    let pending_tx = call.send().await?;
    let receipt = pending_tx.await?;

    println!("\n*** relayStakers() receipt ***\n{:#?}\n********************\n", receipt.unwrap());
    //assert

    // send tx to relayHeader
    let call = bridge
        .relay_header(serialized_header_bytes.clone(), stakers.clone(), signatures.clone())
        .gas(GAS_LIMIT);
    let pending_tx = call.send().await?;
    let receipt = pending_tx.await?;

    println!("\n***** relayHeader() receipt *****\n{:#?}\n********************\n", receipt.unwrap());
    // assert

    // send tx to verifyTx()
    let call = bridge
        .verify_tx(serialized_tx_bytes.clone(), index_u256, block_height, merkle_proof_vec.clone())
        .gas(GAS_LIMIT);
    let pending_tx = call.send().await?;
    let receipt = pending_tx.await?;

    println!("\n***** verifyTx() receipt *****\n{:#?}\n********************\n", receipt.unwrap());
    //assert

    // send tx to burn()
    let call = bridge.burn(mel_amount).gas(GAS_LIMIT);
    let pending_tx = call.send().await?;
    let receipt = pending_tx.await?;

    println!("\n***** burn() receipt *****\n{:#?}\n********************\n", receipt.unwrap());
    //assert

    Ok(())
}
// blake3: 0xA2DAa21c41546aF27ecE20a5A3e08d40bAe3cF66
// ed25519: 0x8c77CF199f6cAD870b65eD559bc3bA1560b60587
// themeliobridge: 0x8e27C1C496dD6D850E62e0825eD120e1b6d0b560
#[tokio::main]
async fn main() -> Result<()> {
    let address = "0x77653c46FBbaDb73A389f99bc2A19Ab5EFB2ec01";

    let mut num_stakers = String::new();
    let mut merkle_tree_height = String::new();

    println!("Input the desired number of stakers: ");
    io::stdin()
        .read_line(&mut num_stakers)
        .expect("Failed to read number of stakers.");

    println!("Input the desired Merkle tree height: ");
    io::stdin()
        .read_line(&mut merkle_tree_height)
        .expect("Failed to read Merkle tree height.")
        ;

    let num_stakers: u32 = num_stakers
        .trim()
        .parse()?;

    let merkle_tree_height: u32 = merkle_tree_height
        .trim()
        .parse()?;

    test_e2e(address, num_stakers, merkle_tree_height).await?;

    Ok(())
}