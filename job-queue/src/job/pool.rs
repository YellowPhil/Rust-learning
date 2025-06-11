use std::sync::Arc;

use tokio::sync::{
    mpsc::{self, Receiver, Sender},
    watch,
};
use tokio_util::sync::CancellationToken;

struct Worker<I, R, F>
where
    I: Send,
    R: Send,
    F: AsyncFn(I) -> R + Send + Sync + 'static,
{
    result_chan: mpsc::Sender<R>,
    task_chan: mpsc::Receiver<I>,
    handler: Arc<F>,
    cancell: CancellationToken,
}


// right now it's an unordered subscription based model: you send some data -> you get some unordered output
struct Pool<I, R, F>
where
    I: Send,
    R: Send,
    F: AsyncFn(I) -> R + Send + Sync + 'static,
{
    workers: Vec<Worker<I, R, F>>,
    workers_channels: Vec<mpsc::Sender<I>>,
    pub result_chan: mpsc::Receiver<R>,
    cancell: CancellationToken,
}

impl<I, R, F> Worker<I, R, F>
where
    I: Send,
    R: Send,
    F: AsyncFn(I) -> R + Send + Sync + 'static,
{
    pub async fn run(mut self) {
        loop {
            tokio::select! {
                _ = self.cancell.cancelled() => {
                    break;
                }
                input = self.task_chan.recv() => {
                    if let Some(input) = input {
                        let output = (self.handler)(input).await;
                        self.result_chan.send(output).await.unwrap();
                    }
                }
            }
        }
    }
}
impl<I, R, F> Pool<I, R, F>
where
    I: Send,
    R: Send,
    F: AsyncFn(I) -> R + Send + Sync + 'static,
{
    pub fn new(worker_function: F, num_workers: usize, cancell: CancellationToken) -> Self {
        let (result_sender, result_receiver) = mpsc::channel(num_workers * 2);
        let worker_arc = Arc::new(worker_function);

        let mut workers_channels: Vec<mpsc::Sender<I>> = vec![];

        let workers: Vec<Worker<I, R, F>> = (0..num_workers)
            .map(|_| {
                let (task_sender, task_receiver) = mpsc::channel(num_workers * 2);
                workers_channels.push(task_sender);
                Worker {
                    result_chan: result_sender.clone(),
                    handler: worker_arc.clone(),
                    cancell: cancell.child_token(),
                    task_chan: task_receiver,
                }
            })
            .collect();

        Self {
            workers: workers,
            workers_channels: workers_channels,
            result_chan: result_receiver,
            cancell: cancell.clone(),
        }
    }

    pub fn queue(&self, input: I) {
        let index = fastrand::usize(..self.workers.len());
        let _ = self.workers_channels[index].send(input);
    }
}
