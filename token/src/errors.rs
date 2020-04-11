pub use wasm_rpc::error::Error;

pub const NOT_CURRENTLY_MINING: Error = Error {
    code: 1,
    message: "Sender is not currently mining",
};

pub const INVALID_VALUE: Error = Error {
    code: 2,
    message: "Invalid value",
};
pub const INSUFFICIENT_FUNDS: Error = Error {
    code: 3,
    message: "Insufficient funds",
};
// lazy_static! {
//     pub static ref BLOCK_ALREADY_MINTED: Error =
//         (1, "this block has already been minted".to_string());
//     pub static ref NOT_BLOCK_WINNER: Error = (2, "sender is not the block winner".into(),);
//     pub static ref INSUFFICIENT_FUNDS: Error = (3, "insufficient funds".into(),);
// }
