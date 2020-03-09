use crate::transaction::*;
use crate::block::*;
use crate::crypto::hash::{H256, H160};

use rand::{Rng,thread_rng};
use rand::distributions::Distribution;

///Block
pub fn generate_random_block(parent: &H256) -> Block {
    let content = generate_random_content();
    let header = generate_random_header(parent, &content);
    Block::new(header, content)
}

pub fn generate_random_header(parent: &H256, content: &Content) -> Header {
    let mut rng = rand::thread_rng();
    let nonce: u32 = rng.gen();
    let timestamp: u128 = rng.gen();
    let difficulty = generate_random_hash();
    let merkle_root = content.merkle_root();
    Header::new(
        parent, nonce, timestamp,
        &difficulty, &merkle_root
    )
}

pub fn generate_random_content() -> Content {
    let mut content = Content::new();
    let mut rng = rand::thread_rng();
    let size: u32 = rng.gen_range(10, 20);
    for _ in 0..size {
        content.add_tran(generate_random_transaction());
    }
    content
}

pub fn generate_block(parent: &H256, nonce: u32, difficulty: &H256)
                      -> Block {
    let content = generate_content();
    let header = generate_header(parent, &content, nonce, difficulty);
    Block::new(header, content)
}

pub fn generate_header(parent: &H256, content: &Content, nonce: u32,
                   difficulty: &H256) -> Header {
    let ts = 100u128;
    let merkle_root = content.merkle_root();
    Header::new(
        parent, nonce, ts,
        difficulty, &merkle_root,
    )
}

fn generate_content() -> Content {
    let mut content = Content::new();
    let tran = generate_random_transaction();
    content.add_tran(tran);
    content
}

/// Transaction
pub fn generate_random_transaction() -> Transaction {
    let mut inputs = Vec::<TxInput>::new();
    let mut outputs = Vec::<TxOutput>::new();
    inputs.push(generate_random_txinput());
    outputs.push(generate_random_txoutput());
    Transaction {inputs, outputs}
}

pub fn generate_random_txinput() -> TxInput {
    let pre_hash = generate_random_hash();
    let mut rng = rand::thread_rng();
    let index: u32 = rng.gen_range(0, 10);
    TxInput {pre_hash, index}
}

pub fn generate_random_txoutput() -> TxOutput {
    let rec_address = generate_random_h160();
    let mut rng = rand::thread_rng();
    let val: u64 = rng.gen_range(0, 256);
    TxOutput {rec_address, val}
}

/// Hash
pub fn generate_random_hash() -> H256 {
    let mut rng = rand::thread_rng();
    let random_bytes: Vec<u8> = (0..32).map(|_| rng.gen()).collect();
    let mut raw_bytes = [0; 32];
    raw_bytes.copy_from_slice(&random_bytes);
    (&raw_bytes).into()
}

pub fn generate_random_h160() -> H160 {
    let mut rng = rand::thread_rng();
    let random_bytes: Vec<u8> = (0..20).map(|_| rng.gen()).collect();
    let mut raw_bytes = [0; 20];
    raw_bytes.copy_from_slice(&random_bytes);
    (&raw_bytes).into()
}

///Other
#[allow(dead_code)]
pub fn generate_random_str() -> String {
    let rng = thread_rng();
    rand::distributions::Alphanumeric.sample_iter(rng).take(10).collect()
}