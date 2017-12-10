extern crate reqwest;
extern crate threadpool;
use reqwest::Client;
use std::time::Duration;
use self::threadpool::ThreadPool;
use std::thread;
use serde_json;
use serde_json::Value;
use error::check_for_error;
use std::sync::mpsc::{Sender, Receiver, channel};
use objects::Update;
use std::io::Read;
use reqwest::Response;

fn construct_get_updates_url(bot_url : &str) -> String{
    let limit = 5;
    let timeout = 30;
    let path = ["getUpdates"];
    let path_url = ::construct_api_url(bot_url, &path);
    let url = format!("{}?limit={}&timeout={}",
                      path_url,
                      limit,
                      timeout,
    );
    url
}

#[derive(Debug)]
pub struct UpdatesReceiver{
    client: Client,
    updates_sender: Sender<Vec<Update>>,
    updates_receiver: Receiver<Vec<Update>>,
    url: String
}

impl UpdatesReceiver{
    pub fn new(url: String)-> Self{
        let (updates_sender, updates_receiver) = channel();
        UpdatesReceiver{
            client: Client::builder()
                .timeout(Duration::from_secs(40))
                .build().unwrap(),
            updates_sender,
            updates_receiver,
            url: construct_get_updates_url(&url)
        }
    }

    pub fn get_updates_channel(&self) -> &Receiver<Vec<Update>>{
        return &self.updates_receiver;
    }

    pub fn start_receiving(&self){
        info!("Starting to receive!");
        let url = self.url.clone();
        let mut offset = 0;
        let client_clone = self.client.clone();
        let sender_clone = self.updates_sender.clone();
        thread::spawn(move ||{
            info!("In a new thread!");
            loop {
                info!("Sending the request!");
                let mut data: Response;
                let url = format!("{}&offset={}", url,offset);
                match client_clone.get(&url).send() {
                    Ok(response) => data = response,
                    Err(e) => {
                        error!("{:?}", e);
                        continue;
                    },
                }
                let mut content = String::new();
                data.read_to_string(&mut content);
                info!("Got valid response: {}", content);
                let json = serde_json::from_str(content.as_str());
                let json = match json {
                    Ok(value) => value,
                    Err(e) => {
                        error!("{:?}", e);
                        continue;
                    },
                };

                let rjson: Value = match check_for_error(json) {
                    Ok(rjson) => rjson,
                    Err(e) => {
                        error!("{:?}", e);
                        continue;
                    },
                };

                info!("Parsing JSON!");
                let updates_json = rjson.get("result");
                info!("Checking the result key in the JSON!");
                if let Some(result) = updates_json {
                    info!("Found key, parsing it into a Vec<Update>!");
                    let updates: Vec<Update> = match serde_json::from_value(result.clone()){
                        Ok(recv_updates) => {
                            info!("Got updates! : {:?}", recv_updates);
                            recv_updates
                        },
                        Err(e) => {
                            error!("{}", e);
                            continue;
                        },
                    };
                    if updates.is_empty() {
                        info!("It was empty, no real updates!");
                        continue;
                    }
                    info!("Setting offset");
                    offset = (updates.last().unwrap().update_id + 1) as i32;
                    sender_clone.send(updates);
                } else {
                    info!("No key found!");
                    continue;
                }
            };
        });

    }
}
