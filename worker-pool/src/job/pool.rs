use std::sync::Arc;

use tokio::sync::mpsc::{self};
use tokio_util::sync::CancellationToken;

#[async_trait::async_trait]
pub trait AsyncJobHandler {
    type Input: Send + 'static;
    type Output: Send + 'static;

    async fn handle(&self, input: Self::Input) -> Self::Output;
}

struct Worker<H: AsyncJobHandler> {
    result_chan: mpsc::Sender<H::Output>,
    task_chan: mpsc::Receiver<H::Input>,
    handler: Arc<H>,
    cancell: CancellationToken,
}

// right now it's an unordered subscription based model: you send some data -> you get some unordered output
pub struct Pool<H: AsyncJobHandler> {
    workers_channels: Vec<mpsc::Sender<H::Input>>,
    pub worker_result_receiver: tokio::sync::Mutex<mpsc::Receiver<H::Output>>,
}

impl<H: AsyncJobHandler> Worker<H>
where
    H::Input: std::fmt::Debug,
    H::Output: std::fmt::Debug,
{
    pub async fn run(&mut self) {
        loop {
            tokio::select! {
                _ = self.cancell.cancelled() => {
                    break;
                }
                input = self.task_chan.recv()=> {
                    if let Some(input) = input {
                        let output = self.handler.handle(input).await;
                        self.result_chan.send(output).await.unwrap();
                    }
                }
            }
        }
    }
}
impl<H: AsyncJobHandler> Pool<H>
where
    H::Input: std::fmt::Debug,
    H::Output: std::fmt::Debug,
    H: Sync + Send + 'static,
{
    pub fn new(handler: H, num_workers: usize, cancell: CancellationToken) -> Self {
        let (worker_result_sender, worker_result_receiver) =
            mpsc::channel::<H::Output>(num_workers * 2);
        let worker_arc = Arc::new(handler);

        let mut workers_channels: Vec<mpsc::Sender<H::Input>> = vec![];

        for _ in 0..num_workers {
            let (task_sender, task_receiver) = mpsc::channel::<H::Input>(num_workers * 2);
            workers_channels.push(task_sender);

            let mut new_worker = Worker {
                result_chan: worker_result_sender.clone(),
                handler: worker_arc.clone(),
                cancell: cancell.child_token(),
                task_chan: task_receiver,
            };
            tokio::spawn(async move {
                new_worker.run().await;
            });
        }
        Self {
            worker_result_receiver: tokio::sync::Mutex::new(worker_result_receiver),
            workers_channels: workers_channels,
        }
    }

    pub async fn queue(&self, input: H::Input) {
        let index = fastrand::usize(..self.workers_channels.len());
        let _ = self.workers_channels[index].send(input).await;
    }
}
