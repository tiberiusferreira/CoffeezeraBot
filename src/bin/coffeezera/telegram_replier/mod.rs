extern crate diesel;
extern crate teleborg;
extern crate time;
use std::env;
use self::diesel::{PgConnection};
use super::establish_connection;
mod msg_handler;
use self::msg_handler::MessageHandler;
use super::{get_user};
use super::coffeezerabot::models::CoffeezeraUser;
use self::teleborg::{Update, Message, CallBackQuery, TelegramInterface, OutgoingMessage, OutgoingEdit};
use super::current_user_context::CurrentUserContext;
pub struct TelegramHandler<T> where T: TelegramInterface{
    pub telegram_interface: T,
    pub database_connection: PgConnection,
    you_are_not_registered_plus_id: &'static str,
    turn_on_command: &'static str,
    turn_off_command: &'static str,
}

pub struct RequestResponse {
    reply: String,
    markup: Option<Vec<Vec<String>>>,
    action: GrinderAction
}

enum GrinderAction {
    TurnOff,
    TurnOn,
    DoNothing
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
            you_are_not_registered_plus_id: "Você não está registrado para uso do moedor. Peça para o Tibério registrar seu id: ",
            turn_on_command: "Ligar",
            turn_off_command: "Desligar",
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
        let response = response.get_response();
        let mut message = OutgoingMessage::new(chat_id, &response.reply);
        if let Some(markup) = response.markup {
            message.with_reply_markup(markup);
        };
        self.telegram_interface.send_msg(message);
    }

    fn make_already_in_use_msg(&self, context: &CurrentUserContext) -> String{
        format!("O moedor já está em uso por {}. Tempo restante: {} s",
                context.current_user.name,
                context.get_time_left_turn_off())
    }


    fn user_has_credits(&self, user_telegram_id: i64) -> bool{
        if let Ok(user) = get_user(&self.database_connection, user_telegram_id){
            if user.account_balance > 0.0{
                return true;
            }
        }
        return false;
    }


    fn reply_with_not_enough_credits(&self, db_user: CoffeezeraUser, chat_id: i64) {
        info!("Returning not enough credits");
        let status = format!("Você está sem créditos: Créditos: {:.2} segundos", db_user.account_balance);
        let message = OutgoingMessage::new(chat_id, &status);
        self.telegram_interface.send_msg(message);
    }


    fn update_msg_with_user_credits_and_on_option(&self, db_user: &CoffeezeraUser, chat_id: i64, msg_id: i64) {
        info!("Updating msg with user credits and turn on msg");
        let status = format!("Créditos: {:.2} segundos", db_user.account_balance);
        let mut edit = OutgoingEdit::new(chat_id, msg_id, &status);
        edit.with_reply_markup(vec![vec![self.turn_on_command.to_string()]]);
        self.telegram_interface.edit_message_text(edit);
    }

    fn update_msg_with_user_credits_and_off_option_and_time_left(&self, db_user: &CoffeezeraUser, time_left: f64, chat_id: i64, msg_id: i64) {
        info!("Updating msg with user credits and turn off msg");
        let status = format!("Créditos: {:.2} segundos. Auto desligar: {}", db_user.account_balance, time_left as i64);
        let mut edit = OutgoingEdit::new(chat_id, msg_id, &status);
        edit.with_reply_markup(vec![vec![self.turn_off_command.to_string()]]);
        self.telegram_interface.edit_message_text(edit);
    }

    fn update_msg_with_already_in_use_and_on_option(&self, context: &CurrentUserContext, chat_id: i64, msg_id: i64) {
        info!("Sending already in use message update and ON option");
        let status = self.make_already_in_use_msg(context);
        let mut edit = OutgoingEdit::new(chat_id, msg_id, &status);
        edit.with_reply_markup(vec![vec![self.turn_on_command.to_string()]]);
        self.telegram_interface.edit_message_text(edit);
    }

    fn update_msg_with_not_registered(&self, sender_id: i64, chat_id: i64, msg_id: i64) {
        info!("Updating msg with not registered msg");
        let status = format!("{} {}" , self.you_are_not_registered_plus_id, sender_id);
        let edit = OutgoingEdit::new(chat_id, msg_id, &status);
        self.telegram_interface.edit_message_text(edit);
    }






    fn is_turn_on_command(&self, command: &str) -> bool{
        return command.eq(self.turn_on_command);
    }

    fn is_turn_off_command(&self, command: &str) -> bool{
        return command.eq(self.turn_off_command);
    }


    fn handle_turn_on_command(&self, context: &mut Option<CurrentUserContext>, message: &Message, sender_telegram_id: i64){
        if let &mut Some(ref some_context) = context {
            info!("Turn on command with grinder in use");
            if some_context.current_user.telegram_id == sender_telegram_id {
                info!("Turn on command by same user already using it");
                self.update_msg_with_user_credits_and_off_option_and_time_left(&some_context.current_user, some_context.get_time_left_turn_off(),message.chat.id, message.message_id);
            }else {
                info!("Turn on command by other user than the one using it");
                self.update_msg_with_already_in_use_and_on_option(some_context, message.chat.id, message.message_id);
                return;
            }
        }else {
            info!("Turn on command and grinder available");
            if let Ok(user_db_info) = get_user(&self.database_connection, sender_telegram_id){
                info!("Turn on command and grinder available and user is in DB");
                self.turn_on_grinder(context, user_db_info.clone(), message.chat.id, message.message_id);
                if !self.user_has_credits(sender_telegram_id){
                    info!("User has no credits!");
                    self.reply_with_not_enough_credits(user_db_info, message.chat.id);
                    return;
                }
                match context {
                    &mut Some(ref some_context) => {
                        self.update_msg_with_user_credits_and_off_option_and_time_left(&some_context.current_user,
                                                                                       some_context.get_time_left_turn_off(),
                                                                                       message.chat.id,
                                                                                       message.message_id);
                    },
                    &mut None => error!("Turned on grinder, but context is still NONE!")
                }
                return;
            }else {
                self.update_msg_with_not_registered(sender_telegram_id, message.chat.id, message.message_id);
                return;
            }
        }
    }

    fn handle_turn_off_command(&self, context: &mut Option<CurrentUserContext>, message: &Message, sender_telegram_id: i64){
        let should_turn_off_grinder: bool;
        if let &mut Some(ref some_context) = context {
            info!("Turn off command with grinder in use");
            if some_context.current_user.telegram_id == sender_telegram_id{
                info!("Turn off command with grinder in use by the current user");
                should_turn_off_grinder = true;
                self.update_msg_with_user_credits_and_on_option(&some_context.current_user, message.chat.id, message.message_id);
            }else{
                info!("Turn off command with grinder in use but NOT by current user");
                self.update_msg_with_already_in_use_and_on_option(&some_context, message.chat.id, message.message_id);
                should_turn_off_grinder = false;
            }
        }else {
            info!("Turn off command and grinder available");
            should_turn_off_grinder = false;
            if let Ok(user) = get_user(&self.database_connection, sender_telegram_id){
                info!("Turn off command from someone in DB and with grinder available");
                self.update_msg_with_user_credits_and_on_option(&user, message.chat.id, message.message_id);
            }else{
                info!("Turn off command from someone NOT in DB and with grinder available");
                self.update_msg_with_not_registered(sender_telegram_id, message.chat.id,message.message_id);
            }

        }
        if should_turn_off_grinder{
            self.turn_off_grinder(context);
        }
    }

    fn turn_off_grinder(&self, context: &mut Option<CurrentUserContext>){
        context.take();
    }
    fn turn_on_grinder(&self, context: &mut Option<CurrentUserContext>, new_user: CoffeezeraUser, user_chat_id: i64, user_message_id: i64){
        *context = Some(CurrentUserContext::new(new_user, user_chat_id, user_message_id));
    }


    fn handle_callback(&mut self, callback_query: CallBackQuery, context: &mut Option<CurrentUserContext>){
        info!("Handling the callback");

        let data = match callback_query.data{
            Some(data) => data,
            None => {
                error!("Callback had no data");
                return;
            }
        };

        let message = match callback_query.message.as_ref() {
            Some(message) => message,
            None => {
                error!("Callback had no message");
                return;
            }
        };

        if self.is_turn_on_command(data.as_str()) {
            self.handle_turn_on_command(context, message, callback_query.from.id);
        }else if self.is_turn_off_command(data.as_str()) {
            self.handle_turn_off_command(context, message, callback_query.from.id);
        }else{
            error!("Got a callback that was neither a turn-on or turn-off: {}", data.as_str());
        }
    }


}
