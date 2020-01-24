/*
written by jongwhan lee
@2020.01.19

<= for the future =>
*/
use super::database::{Database, MemoryDatabase};
use super::merkletrie_interface::MerkletrieDatabase;
use super::merkletrie_interface::MerkletrieInterface;
use failure::Error;
use serde::Deserialize;
use serde::Serialize;
use std::collections::BTreeMap;
use std::time::{Duration, Instant};
#[derive(Serialize, Deserialize, Debug, Clone, Default, PartialEq)]
pub struct SparseMerkletrie<T>
where
    T: MerkletrieDatabase,
{
    database: T,
    root: Node,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default, PartialEq)]
pub struct Node {
    pub children: BTreeMap<Vec<u8>, Vec<u8>>,
    pub value: Vec<u8>,
}

impl<T> MerkletrieInterface for SparseMerkletrie<T>
where
    T: MerkletrieDatabase,
{
    fn load(&mut self, hash: &[u8]) -> Result<(), Error> {
        let node_found = self.read_node(&hash)?;
        self.root = node_found;
        Ok(())
    }

    fn put(&mut self, key: &[u8], value: &[u8]) -> Result<(), Error> {
        let mut output = "".to_string();
        self.put(key, value, &mut output);
        Ok(())
    }

    fn get(&mut self, key: &[u8]) -> Result<Vec<u8>, Error> {
        let mut output = "".to_string();
        self.get(key, &mut output)
    }

    fn get_roothash(&self) -> Result<Vec<u8>, Error> {
        self.get_hash(&self.root)
    }
}

impl<T> SparseMerkletrie<T>
where
    T: MerkletrieDatabase,
{
    pub fn new(database: T) -> Self {
        SparseMerkletrie {
            database,
            root: Node::default(),
        }
    }

    // encoded, hash
    fn get_hash(&self, n: &Node) -> Result<Vec<u8>, Error> {
        Ok(self.get_encoded_hash(n)?.1)
    }

    // encoded, hash
    pub fn get_encoded_hash(&self, n: &Node) -> Result<(Vec<u8>, Vec<u8>), Error> {
        let encoded: Vec<u8> = bincode::serialize(&n)?;
        let hash = self.database.compute_hash(&encoded.as_slice());
        Ok((encoded.to_vec(), hash))
    }

    fn write_node(&mut self, n: &Node) -> Result<Vec<u8>, Error> {
        let (encoded, hash) = self.get_encoded_hash(n)?;
        self.database.write(&hash, &encoded[..])?;
        Ok(hash)
    }

    fn read_node(&self, key: &[u8]) -> Result<Node, Error> {
        let data = self.database.read(key)?;
        let decoded: Node = bincode::deserialize(&data[..])?;
        Ok(decoded)
    }

    pub fn show_root(&self) {
        let (encoded, hash) = self.get_encoded_hash(&self.root).expect("compute hash");
        //println!("hash= {}", hex::encode(&hash));
    }

    pub fn convert_to_bits(&self, key: &[u8]) -> Vec<u8> {
        let mut index = 8 * key.len() as i32 - 1;
        let mut ret: Vec<u8> = vec![];
        while index >= 0 {
            println!("{}", index);
            let which_byte = key.len() as i32 - 1 - index / 8;
            let byte_value = key[which_byte as usize];
            let bit = index % 8;
            let flag_value = byte_value & 1 << bit;
            let flag = if flag_value > 0 { 1 } else { 0 };
            ret.push(flag);
            index = index - 1;
        }
        ret
    }
    pub fn show_bits(&self, bits: &[u8]) {
        for i in 0..bits.len() {
            print!("{}", bits[i]);
        }
        println!("");
    }
    pub fn put(&mut self, key: &[u8], value: &[u8], output: &mut String) {
        let mut root = self.root.clone();
        let bits = self.convert_to_bits(key);
        println!("{:?}", bits);
        self.show_bits(&bits);
        let roothash = self
            .do_put(&bits, value, 8 * key.len() as i32 - 1, output, &mut root)
            .expect("ok");
        //  let (encoded,hash)=self.get_encoded_hash(&root).expect("compute hash");
        // assert!(hash== roothash);
        // self.root = root;
    }

    pub fn do_put(
        &mut self,
        key_bits: &[u8],
        value: &[u8],
        index: i32,
        output: &mut String,
        parent: &mut Node,
    ) -> Result<Vec<u8>, Error> {
        let mut index= key_bits.len();
        println!("index= {}", index);

        let mut ret = Node::default();
        ret.value= value.to_vec();
        self.write_node(&ret);
        
        
        Ok(vec![])
    }

    pub fn get(&mut self, key: &[u8], output: &mut String) -> Result<Vec<u8>, Error> {
        self.do_get(&key, 8 * key.len() as i32 - 1, output, &self.root)
    }

    pub fn do_get(
        &self,
        key: &[u8],
        index: i32,
        output: &mut String,
        parent: &Node,
    ) -> Result<Vec<u8>, Error> {
        Ok(vec![])
    }
}

use bitvec::prelude::*;

pub fn dynamic_sparse_main() -> Result<(), failure::Error> {
    let database = MemoryDatabase::default();
    let mut smt = SparseMerkletrie::new(MemoryDatabase::default());
    //let database = Database::new("./data");
    //let mut smt = SparseMerkletrie::new(database.clone());
    let mut i: i32 = 0;
    let n = 1000;
    let now = Instant::now();
    for i in 0..n {
        let b = i as i32;
        let value = b.to_le_bytes();
        //let key= database.compute_hash(&value);
        let key = hex::decode("f081").unwrap();

        let mut output = "".to_string();
        println!("{} {} ( {} )", i, hex::encode(&key), key.len());

        smt.put(&key, &value, &mut output);
    }
    println!("sparse merkletrie= {}", now.elapsed().as_millis());
    Ok(())
}
