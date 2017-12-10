pub use self::parse_mode::ParseMode;
pub use self::chat_action::ChatAction;

mod parse_mode;
mod request_sender;
mod chat_action;
mod updates_receiver;
use reqwest::Client;
use serde_json;
use serde_json::Value;
use std::io::Read;
use std;
use self::request_sender::RequestSender;
use self::updates_receiver::UpdatesReceiver;
extern crate reqwest;
extern crate threadpool;
use bot::chat_action::get_chat_action;
use self::reqwest::Response;
use bot::parse_mode::get_parse_mode;
use error::{Result, check_for_error};
use error::Error::{JsonNotFound, RequestFailed};
use objects::{Update, Message, Contact, InlineKeyboardMarkup};
use std::time::Duration;
use self::request_sender::post_parameters;
use value_extension::ValueExtension;
use std::sync::{Arc};
use std::sync::mpsc::{Receiver, Sender, channel};
use std::thread;
use self::threadpool::ThreadPool;
/// A `Bot` which will do all the API calls.
///
/// The `Bot` will be given access to in a `Command` with which you can do all
/// the API interactions in your `Command`s.


#[derive(Debug)]
pub struct Bot {
    pub id: i64,
    pub first_name: String,
    pub last_name: Option<String>,
    pub username: String,
    pub bot_url: String,
    request_sender: RequestSender,
    updates_receiver: UpdatesReceiver,
}

impl Bot {

    /// Constructs a new `Bot`.
    pub fn new(bot_token: String) -> Result<Self> {
        let base_url = "https://api.telegram.org/bot".to_string();
        let bot_url = [base_url.clone(), bot_token].concat();
        let temp_client = Client::builder()
            .timeout(Duration::from_secs(40))
            .build().unwrap();
        let rjson = Bot::get_me(&temp_client, &bot_url)?;
        let id = rjson.as_required_i64("id")?;
        let first_name = rjson.as_required_string("first_name")?;
        let last_name = rjson.as_optional_string("last_name");
        let username = rjson.as_required_string("username")?;
        Ok(Bot {
               id: id,
               first_name: first_name,
               last_name: last_name,
               username: username,
               bot_url: bot_url.clone(),
               request_sender: RequestSender::new(),
               updates_receiver: UpdatesReceiver::new(bot_url),
           })
    }

    /// API call which gets the information about your bot.
    pub fn get_me(client: &Client, bot_url: &str) -> Result<Value> {
        let path = ["getMe"];
        let url = ::construct_api_url(bot_url, &path);
        let mut resp = client.get(&url).send()?;

        if resp.status().is_success() {
            let rjson: Value = resp.json()?;
            rjson.get("result").cloned().ok_or(JsonNotFound)
        } else {
            Err(RequestFailed(resp.status()))
        }
    }



    /// API call which will get called to get the updates for your bot.
    pub fn start_getting_updates(&self){
        info!("Asking for bot updates!");
        self.updates_receiver.start_receiving();
    }

    pub fn get_updates_channel(&self) -> &Receiver<Vec<Update>>{
        self.updates_receiver.get_updates_channel()
    }

    /// API call which will send a message to a chat which your bot participates in.
    pub fn send_message(&self,
                        chat_id: &i64,
                        text: &str,
                        parse_mode: Option<&ParseMode>,
                        disable_web_page_preview: Option<&bool>,
                        disable_notification: Option<&bool>,
                        reply_to_message_id: Option<&i64>,
                        reply_markup: Option<&InlineKeyboardMarkup>){
        let chat_id: &str = &chat_id.to_string();
        let parse_mode = &get_parse_mode(parse_mode.unwrap_or(&ParseMode::Text));
        let disable_web_page_preview: &str =
            &disable_web_page_preview.unwrap_or(&false).to_string();
        let disable_notification: &str = &disable_notification.unwrap_or(&false).to_string();
        let reply_to_message_id: &str = &reply_to_message_id
            .map(|i| i.to_string())
            .unwrap_or("None".to_string());
        let reply_markup =
            &reply_markup
                .map(|r| serde_json::to_string(r).unwrap_or("".to_string()))
                .unwrap_or("".to_string());

        let path = ["sendMessage"];
        let params = [("chat_id", chat_id),
            ("text", text),
            ("parse_mode", parse_mode),
            ("disable_web_page_preview", disable_web_page_preview),
            ("disable_notification", disable_notification),
            ("reply_to_message_id", reply_to_message_id),
            ("reply_markup", reply_markup)];
        self.post_message(&path, &params)
    }

    /// API call which will edit the text of a message send by the bot
    pub fn edit_message_text(&self,
                        chat_id: &i64,
                        message_id: &i64,
                        inline_message_id: Option<&str>,
                        text: &str,
                        parse_mode: Option<&ParseMode>,
                        disable_web_page_preview: Option<&bool>,
                        reply_markup: Option<&InlineKeyboardMarkup>){
        let chat_id: &str = &chat_id.to_string();
        let parse_mode = &get_parse_mode(parse_mode.unwrap_or(&ParseMode::Text));
        let disable_web_page_preview: &str =
            &disable_web_page_preview.unwrap_or(&false).to_string();
        let reply_markup =
            &reply_markup
                .map(|r| serde_json::to_string(r).unwrap_or("".to_string()))
                .unwrap_or("".to_string());

        let path = ["editMessageText"];
        let params = [("chat_id", chat_id),
            ("message_id", &message_id.to_string()),
            ("inline_message_id", inline_message_id.unwrap_or("")),
            ("text", text),
            ("parse_mode", parse_mode),
            ("disable_web_page_preview", disable_web_page_preview),
            ("reply_markup", reply_markup)];
        self.post_message(&path, &params)
    }

    /// API call which will reply to a message directed to your bot.
    pub fn reply_to_message(&self, update: &Update, text: &str) {
        let message = update.clone().message.unwrap();
        let message_id = message.message_id;
        let chat_id = message.chat.id;
        self.send_message(&chat_id, text, None, None, None, Some(&message_id), None)
    }



    /// The actual networking done for sending messages.
    fn post_message(&self, path: &[&str], params: &[(&str, &str)]){
        let url = ::construct_api_url(&self.bot_url, path);
        let vec = params.to_vec();
        let vec = vec.iter().map(|str1| {
            let &(string_1, string_2) = str1;
            (string_1.to_string(), string_2.to_string())
        }).collect();
        self.request_sender.send(post_parameters{
            path: url,
            params: vec
        });
    }
}
