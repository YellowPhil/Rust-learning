mod job;

use std::{sync::Arc, time::Duration};
use tokio::time::timeout;
use tokio_util::sync::CancellationToken;

use crate::job::pool::{AsyncJobHandler, Pool};

struct SquareCalc;

#[async_trait::async_trait]
impl AsyncJobHandler for SquareCalc {
    type Input = i32;
    type Output = i32;

    async fn handle(&self, input: Self::Input) -> Self::Output {
        input * input
    }
}

#[tokio::main]
async fn main() {
    let token = CancellationToken::new();
    let pool = Arc::new(Pool::new(SquareCalc {}, 8, token.clone()));
    let pool_clone = pool.clone();
    tokio::spawn(async move {
        for _ in 0..10_000 {
            let _ = pool.queue(fastrand::i32(-46339..46340 - 1)).await;
        }
    });
    let f = tokio::spawn(async move {
        let mut receiver = pool_clone.worker_result_receiver.lock().await;
        let mut counter = 0;
        while counter < 10_000 {
            if let Some(_) = receiver.recv().await {
                counter += 1;
            }
        }
        println!("DONE!!!!!!");
        token.cancel();
    });
    if let Err(_) = timeout(Duration::from_secs(5), f).await {
        println!("timeout...;")
    }
}
