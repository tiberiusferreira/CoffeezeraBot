extern crate reqwest;
extern crate threadpool;
use reqwest::Client;
use std::time::Duration;
use self::threadpool::ThreadPool;

#[derive(Debug)]
pub struct RequestSender{
    client: Client,
    thread_pool: ThreadPool,
}

pub struct post_parameters{
    pub path: String,
    pub params: Vec<(String, String)>
}

impl RequestSender{
    pub fn new() -> RequestSender{
        RequestSender{
            client: Client::builder()
                .timeout(Duration::from_secs(5))
                .build().unwrap(),
            thread_pool: ThreadPool::new(8),
        }
    }
    pub fn send(&self, post_parameters: post_parameters){
        let client_clone = self.client.clone();
        self.thread_pool.execute(move ||{
            match client_clone.post(&post_parameters.path).form(&post_parameters.params).send() {
                Ok(_) => {},
                Err(e) => error!("Error sending: {}",e),
            }
        });
    }
}
