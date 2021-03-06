// use crate::{
//     api::Message, block_broadcaster::broadcast, config::verification_key, constants::BLOCK_TIME,
//     helpers::current_miner, models, models::Block, state::State,
// };
// use async_std::{sync, task::sleep};
//
// use futures::{future::FutureExt, pin_mut, select, stream::StreamExt};
//
use crate::state::State;
use broadcaster::BroadcastChannel;

pub async fn _run(
    _state: State,
    _new_block_broadcaster: BroadcastChannel<u32>,
    // _api_receiver: sync::Receiver<Message>,
) {
    // loop {
    // winning_blocks()
    //     .for_each(|block| println!("I won a block!"))
    //     .await;
    // }
    // 'run: loop {
    //     if current_miner().await.address.eq(&verification_key()) {
    //         let block = Block::insert();
    //         println!("Won block #{}", &block.number);
    //         let sleep_fused = sleep(*BLOCK_TIME).fuse();
    //         pin_mut!(sleep_fused);
    //         loop {
    //             let mut transaction_position = 0;
    //             let next_message_fused = api_receiver.next().map(Option::unwrap).fuse();
    //             pin_mut!(next_message_fused);
    //             select! {
    //                 () = sleep_fused => {
    //                     let transactions = block.seal(&mut state, transaction_position + 1).await;
    //                     broadcast((block.clone(), transactions.clone())).await;
    //                     let _ = new_block_broadcaster.send(&(block.number as u32)).await;
    //                     continue 'run;
    //                 },
    //                 (message) = next_message_fused => {
    //                     match message {
    //                         Message::Block(block) => {
    //                             println!("Got block while mining");
    //                         },
    //                         Message::Transaction(transaction, responder) => {
    //                             let completed_transaction =
    //                                 models::Transaction::run(&mut state, &block, transaction, transaction_position as i32);
    //                             transaction_position += 1;
    //                             responder.send(completed_transaction).unwrap();
    //                         },
    //                     }
    //                 },
    //             }
    //         }
    //     }
    //     if let Message::Block((block, transactions)) = api_receiver.next().map(Option::unwrap).await
    //     {
    //         block.clone().apply(&mut state, transactions.clone()).await;
    //         let _ = new_block_broadcaster.send(&(block.number as u32)).await;
    //     }
    // }
}
