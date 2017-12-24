extern crate teleborg;
use std::env;
use super::telegram_interface::TelegramInterface;
use self::teleborg::objects::{InlineKeyboardMarkup, Update, OutgoingMessage, OutgoingEdit};
use self::teleborg::Bot;
use std::sync::mpsc::Receiver;
pub struct TelegramInterfaceImpl {
    bot: Bot,
}


impl TelegramInterface for TelegramInterfaceImpl {

    fn start_getting_updates(&mut self){
        self.bot.start_getting_updates()
    }

    fn get_updates_channel(&self) -> &Receiver<Vec<Update>>{
        self.bot.get_updates_channel()
    }

    fn new() -> TelegramInterfaceImpl {
        info!("Creating new TelegramInterfaceImpl");
        let bot_token = env::var("TELEGRAM_BOT_TOKEN")
            .ok()
            .expect("Can't find TELEGRAM_BOT_TOKEN env variable")
            .parse::<String>()
            .unwrap();
        TelegramInterfaceImpl {
            bot: Bot::new(bot_token).unwrap(),
        }
    }

    fn send_msg(&self, chat_id: i64, message: &str, keyboard_options: Option<Vec<Vec<String>>>){
        let mut msg: OutgoingMessage = OutgoingMessage::new(chat_id, message);
        if let Some(keyboard_options) = keyboard_options{
            msg.with_reply_markup(InlineKeyboardMarkup::new(keyboard_options));
        }
        self.bot.send_msg(msg);
    }




    fn edit_message_text(&self,
                         chat_id: i64,
                         message_id: i64,
                         text: &str,
                         keyboard_options: Option<Vec<Vec<String>>>){
        let mut msg: OutgoingEdit = OutgoingEdit::new(chat_id,message_id, text);
        if let Some(keyboard_options) = keyboard_options{
            msg.with_reply_markup(InlineKeyboardMarkup::new(keyboard_options));
        }
        self.bot.edit_message_text(msg);
    }

}


