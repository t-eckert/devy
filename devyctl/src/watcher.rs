use crate::queue::{Job, Message, Queue};
use futures::{stream, StreamExt};
use std::{sync::Arc, time::Duration};

pub struct Watcher {
    pub queue: Arc<dyn Queue>,
    pub threads: usize,
}

impl Watcher {
    pub fn new(queue: Arc<dyn Queue>, threads: usize) -> Self {
        Self { queue, threads }
    }

    pub async fn run(&self) {
        info!("Starting {} workers", self.threads);
        loop {
            let jobs = match self.queue.pull(self.threads as u32).await {
                Ok(jobs) => jobs,
                Err(err) => {
                    error!("run: pulling jobs: {}", err);
                    tokio::time::sleep(Duration::from_millis(500)).await;
                    Vec::new()
                }
            };

            let number_of_jobs = jobs.len();
            if number_of_jobs > 0 {
                info!("Fetched {} jobs", number_of_jobs);
            }

            stream::iter(jobs)
                .for_each_concurrent(self.threads, |job| async {
                    let job_id = job.id;

                    let res = match handle_job(job).await {
                        Ok(_) => self.queue.delete_job(job_id).await,
                        Err(err) => {
                            error!("run: handling job({}): {}", job_id, &err);
                            self.queue.fail_job(job_id).await
                        }
                    };

                    match res {
                        Ok(_) => {}
                        Err(err) => {
                            error!("run: deleting / failing job: {}", &err);
                        }
                    }
                })
                .await;

            tokio::time::sleep(Duration::from_millis(1000)).await;
        }
    }
}

async fn handle_job(job: Job) -> Result<(), crate::Error> {
    match job.message {
        Message::Upload { .. } => {
            info!("Uploading!")
        }
    };

    Ok(())
}
