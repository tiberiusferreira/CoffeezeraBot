extern crate diesel;
extern crate teleborg;
extern crate time;
use std::env;
use self::diesel::{PgConnection};
use super::establish_connection;
mod msg_handler;
mod callback_handler;
mod response;
mod grinder_action;
use self::msg_handler::MessageHandler;
use super::{get_user};
use super::coffeezerabot::models::CoffeezeraUser;
use self::teleborg::{Update, Message, CallBackQuery, TelegramInterface, OutgoingMessage, OutgoingEdit};
use super::current_user_context::CurrentUserContext;
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


    pub fn handle_update(&mut self, update: Update, context: &mut Option<CurrentUserContext>){
        if let Some(message) = update.message {
            info!("This was a message update");
            self.handle_msg(message, context);
            return;
        }
        if let Some(callback_query) = update.callback_query {
            info!("This was a callback update");
            self.handle_callback(callback_query, context);
            return;
        }
        error!("This was neither a Message or Callback update. Weird.");
    }


    fn handle_msg(&mut self, message: Message, context: &Option<CurrentUserContext>){
        let sender_db_info = message.from.as_ref().and_then(|user| {
            get_user(&self.database_connection, user.id).ok()
        });
        let chat_id = message.chat.id;
        let response = MessageHandler::new(message, context, sender_db_info);
        let response = response.get_request_response();
        let mut message = OutgoingMessage::new(chat_id, &response.reply);
        if let Some(markup) = response.reply_markup {
            message.with_reply_markup(markup);
        };
        self.telegram_interface.send_msg(message);
    }


    fn handle_callback(&mut self, callback_query: CallBackQuery, context: &mut Option<CurrentUserContext>){
        let sender_db_info =
            get_user(&self.database_connection, callback_query.from.id).ok();
        callback_handler::CallbackHandler::new(callback_query, context, sender_db_info);
    }

    fn turn_off_grinder(&self, context: &mut Option<CurrentUserContext>){
        context.take();
    }
    fn turn_on_grinder(&self, context: &mut Option<CurrentUserContext>, new_user: CoffeezeraUser, user_chat_id: i64, user_message_id: i64){
        *context = Some(CurrentUserContext::new(new_user, user_chat_id, user_message_id));
    }

}
