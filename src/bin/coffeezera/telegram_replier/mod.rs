extern crate diesel;
extern crate teleborg;
use std::env;
use self::diesel::{PgConnection};
use super::establish_connection;
mod msg_handler;
mod callback_handler;
mod response;
mod update_outcome;
use self::update_outcome::UpdateImpact;
use self::msg_handler::MessageHandler;
use super::{get_user};

use super::database::models::CoffeezeraUser;
use teleborg::*;
use super::current_user_context::CurrentUserContext;

const TURN_OFF: &'static str  = "Desligar";
const TURN_ON: &'static str  = "Ligar";

pub struct TelegramHandler<T> where T: TelegramInterface{
    pub telegram_interface: T,
    pub database_connection: PgConnection,
}


impl <T: TelegramInterface> TelegramHandler<T> {
    pub fn new() -> TelegramHandler<T> {
        let bot_token = env::var("TELEGRAM_BOT_TOKEN")
            .ok()
            .expect("Can't find TELEGRAM_BOT_TOKEN env variable")
            .parse::<String>()
            .unwrap();
        TelegramHandler {
            telegram_interface: T::new(bot_token).unwrap(),
            database_connection: establish_connection(),
        }
    }





    pub fn handle_update(&self, update: Update, context: &mut Option<CurrentUserContext>){
        match clean_update(update){
            Ok(cleaned_update) => {
                match cleaned_update {
                    CleanedUpdate::CleanedMessage(cleaned_msg) => self.handle_msg(cleaned_msg, context),
                    CleanedUpdate::CleanedCallbackQuery(cleaned_callback) => self.handle_callback(cleaned_callback, context)
                };
            },
            Err(err) => {
                error!("{}", err);
            }
        }
    }


    fn handle_msg(&self, message: CleanedMessage, context: &Option<CurrentUserContext>){
        let sender_db_info = get_user(&self.database_connection, message.sender_id).ok();
        let chat_id = message.chat_id;
        let message_handler = MessageHandler::new(message, context, sender_db_info);
        let response = message_handler.get_response();

        let mut message = OutgoingMessage::new(chat_id, &response.reply);
        if let Some(markup) = response.reply_markup {
            message.with_reply_markup(markup);
        };
        self.telegram_interface.send_msg(message);
    }



    pub fn send_auto_turned_off_message(&self, context: &CurrentUserContext){
        let response = callback_handler::CallbackHandler::get_response_for_auto_turn_off(context);
        let mut message = OutgoingEdit::new(context.current_user_chat_id, context.current_user_message_id,&response.reply);
        if let Some(markup) = response.reply_markup {
            message.with_reply_markup(markup);
        };
        self.telegram_interface.edit_message_text(message);
    }

    fn handle_callback(&self, callback_query: CleanedCallbackQuery, context: &mut Option<CurrentUserContext>){
        let original_message_id = callback_query.original_msg_id;
        let chat_id = callback_query.original_msg_chat_id;

        let sender_db_info=
            get_user(&self.database_connection, callback_query.sender_id).ok();

        let response = callback_handler::CallbackHandler::new(callback_query.clone(), context, sender_db_info.clone())
            .handle_callback();
        match response.action {
            UpdateImpact::TurnOn => {
                if let Some(sender_db_info) = sender_db_info{
                    self.turn_on_grinder(context, sender_db_info, chat_id, original_message_id);
                }else{
                    error!("Tried to allow non-registered user to access the grinder!!")
                }
            },
            UpdateImpact::TurnOff => self.turn_off_grinder(context),
            UpdateImpact::DoNothing => {}
        }
        let mut message = OutgoingEdit::new(chat_id, original_message_id,&response.reply);
        if let Some(markup) = response.reply_markup {
            message.with_reply_markup(markup);
        };
        self.telegram_interface.edit_message_text(message);

    }

    fn turn_off_grinder(&self, context: &mut Option<CurrentUserContext>){
        if let &mut Some(ref mut actual_context) = context{
                actual_context.should_be_removed = true;
        };
    }
    fn turn_on_grinder(&self, context: &mut Option<CurrentUserContext>, new_user: CoffeezeraUser, user_chat_id: i64, user_message_id: i64){
        *context = Some(CurrentUserContext::new(new_user, user_chat_id, user_message_id));
    }

}
