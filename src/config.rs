extern crate clap;
use clap::Clap;
use dotenv::dotenv;
use ed25519_dalek::Keypair;
use serde::{Deserialize, Deserializer};
use std::{
    env,
    net::{IpAddr, SocketAddr},
};

#[derive(Clap, Debug)]
pub struct Opts {
    #[clap(short = "a", long = "bind-address", default_value = "0.0.0.0")]
    pub bind_address: String,
    #[clap(short = "b", long = "bootnodes")]
    pub bootnodes: Option<String>,
    #[clap(short = "d", long = "database-url")]
    pub database_url: Option<String>,
    #[clap(short = "n", long = "network-id", default_value = "1793045504")]
    pub network_id: u32,
    #[clap(short = "p", long = "port", default_value = "80")]
    pub port: u16,
    #[clap(long = "rocksdb-path", default_value = "./db")]
    pub rocksdb_path: String,
    #[clap(long = "save-state")]
    pub save_state: bool,
    #[clap(long = "redis-url", default_value = "redis://127.0.0.1")]
    pub redis_url: String,
    #[clap(subcommand)]
    pub subcmd: Option<SubCommand>,
    #[clap(long = "websocket-port", default_value = "81")]
    pub websocket_port: u16,
}

#[derive(Clap, Debug)]
pub enum SubCommand {
    #[clap(name = "generate-keypair")]
    GenerateKeypair,
}

lazy_static! {
    pub static ref OPTS: Opts = {
        dotenv().ok();
        Opts::parse()
    };
    pub static ref HOST: String = env::var("HOST").unwrap();
    pub static ref GENESIS_NODE: bool = env::var("GENESIS_NODE").is_ok();
    pub static ref ENABLE_MINER: bool = env::var("ENABLE_MINER").is_ok();
    pub static ref BURN_PER_BLOCK: u64 = {
        if *ENABLE_MINER {
            env::var("BURN_PER_BLOCK")
                .expect("BURN_PER_BLOCK not set")
                .parse()
                .unwrap()
        } else {
            0
        }
    };
}

#[derive(Deserialize, Debug)]
pub struct Bootnode {
    pub host: String,
    #[serde(deserialize_with = "decode_base64")]
    public_key: Vec<u8>,
}

pub fn decode_base64<'de, D>(deserializer: D) -> Result<Vec<u8>, D::Error>
where
    D: Deserializer<'de>,
{
    use serde::de::Error;
    String::deserialize(deserializer)
        .and_then(|string| base64::decode(&string).map_err(|err| Error::custom(err.to_string())))
}

pub fn bootnodes() -> Vec<Bootnode> {
    let path = OPTS
        .bootnodes
        .clone()
        .unwrap_or("dist/bootnodes.yaml".to_string());
    let string = std::fs::read_to_string(path).unwrap();
    serde_yaml::from_str(&string).unwrap()
}

pub fn database_url() -> String {
    OPTS.database_url
        .clone()
        .unwrap_or(env::var("DATABASE_URL").expect("DATABASE_URL must be set"))
}

pub fn socket() -> SocketAddr {
    (OPTS.bind_address.parse::<IpAddr>().unwrap(), OPTS.port).into()
}

pub fn keypair() -> Keypair {
    Keypair::from_bytes(&base64::decode(&env::var("PRIVATE_KEY").unwrap()).unwrap()).unwrap()
}