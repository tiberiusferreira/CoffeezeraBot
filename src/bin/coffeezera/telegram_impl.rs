extern crate teleborg;
use std::sync::Arc;
use std::env;
use super::telegram_interface::TelegramInterface;
use self::teleborg::ParseMode;
use self::teleborg::objects::{InlineKeyboardMarkup, Message, Update};
use self::teleborg::error::Error;
use std::sync::mpsc::Receiver;
pub struct TelegramInterfaceImpl {
    bot: teleborg::Bot,
}

impl TelegramInterfaceImpl{
    fn convert_vec_vec_to_keyboard(&self, keyboard_options: Vec<Vec<String>>) -> InlineKeyboardMarkup{
        let mut vec = Vec::new();
        for keyboard_line in keyboard_options{
            let mut line_vec = Vec::new();
            for keyboard_column in keyboard_line{
                line_vec.push(teleborg::objects::InlineKeyboardButton::new(
                    keyboard_column.clone(),
                    None,
                    Some(keyboard_column.to_string()),
                    None,
                    None));
            }
            vec.push(line_vec);
        }
        InlineKeyboardMarkup::new(vec)
    }
}

impl TelegramInterface for TelegramInterfaceImpl {

    fn start_getting_updates(&self){
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
            bot: teleborg::Bot::new(bot_token).unwrap(),
        }
    }

    fn send_msg(&self, chat_id: i64, message: &str, keyboard_options: Option<Vec<Vec<String>>>){
        let actual_keyboard;
        let keyboard;
        match keyboard_options{
            None => {
                keyboard = None
            },
            Some(some_keyboard_options) => {
                actual_keyboard = self.convert_vec_vec_to_keyboard(some_keyboard_options);
                keyboard = Some(&actual_keyboard);
            }
        }

        self.bot.send_message(
            &chat_id,
            message,
            None,
            None,
            None,
            None,
            keyboard
        )
    }




    fn edit_message_text(&self,
                         chat_id: i64,
                         message_id: i64,
                         text: &str,
                         keyboard_options: Option<Vec<Vec<String>>>){
        let actual_keyboard;
        let keyboard;
        if keyboard_options.is_none(){
            keyboard = None;
        }else {
            actual_keyboard = self.convert_vec_vec_to_keyboard(keyboard_options.unwrap());
            keyboard = Some(&actual_keyboard);
        }
        self.bot.edit_message_text(&chat_id,
                                   &message_id,
                                   None,
                                   text,
                                   Some(&ParseMode::Markdown),
                                   None,
                                   keyboard)
    }

}


