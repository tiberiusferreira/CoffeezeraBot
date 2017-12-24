extern crate teleborg;
use self::teleborg::objects::{Update};
use std::sync::mpsc::Receiver;
pub trait TelegramInterface {
    fn new() -> Self;
    fn start_getting_updates(&mut self);
    fn get_updates_channel(&self) -> &Receiver<Vec<Update>>;
    fn send_msg(&self,
                chat_id: i64,
                message: &str,
                keyboard_options: Option<Vec<Vec<String>>>);
    fn edit_message_text(&self,
                         chat_id: i64,
                         message_id: i64,
                         text: &str,
                         keyboard_options: Option<Vec<Vec<String>>>);
}
