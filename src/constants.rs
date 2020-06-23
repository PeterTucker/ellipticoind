pub const GENESIS_STATE_PATH: &'static str = "./dist/genesis.cbor";
pub const TOKEN_WASM_PATH: &'static str = "./contracts/token/dist/token.wasm";
pub const SYSTEM_ADDRESS: [u8; 32] = [0; 32];
// vQMn3JvS3ATITteQ+gOYfuVSn2buuAH+4e8NY/CvtwA= in hex
pub const GENESIS_ADDRESS: [u8; 32] =
    hex!("bd0327dc9bd2dc04c84ed790fa03987ee5529f66eeb801fee1ef0d63f0afb700");
lazy_static! {
    pub static ref TOKEN_CONTRACT: Vec<u8> =
        [&SYSTEM_ADDRESS.to_vec(), "Ellipticoin".as_bytes()].concat();
    pub static ref SYSTEM_CONTRACT_ADDRESS: Vec<u8> = vec![0; 32];
}
pub enum Namespace {
    _Allowences,
    _Balances,
    CurrentMiner,
    EthereumBalances,
    Miners,
    _RandomSeed,
    _UnlockedEthereumBalances,
}
