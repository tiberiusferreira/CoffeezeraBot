extern crate teleborg;

use self::teleborg::{Message, User, CallBackQuery};
use super::{CurrentUserContext, CoffeezeraUser};
use coffeezera::telegram_replier::response::Response;
use coffeezera::telegram_replier::grinder_action::GrinderAction;
mod turn_on_handler;

const TURN_OFF: &'static str  = "Desligar";
const TURN_ON: &'static str  = "Ligar";
const CALLBACK_NO_DATA: &'static str  = "Esse callback não tinha dados, envie essa mensagem para @TiberioFerreira.";

pub struct CallbackHandler<'a>{
    callback: CallBackQuery,
    context: &'a Option<CurrentUserContext>,
    sender_db_info: Option<CoffeezeraUser>
}


impl<'a> CallbackHandler <'a>{

    fn create_unknown_callback_data_response_text(&self, unknown_data: &str) -> String {
        format!("Esse callback não tinha dados: {}, envie essa mensagem para @TiberioFerreira.", unknown_data)
    }
    pub fn new(callback: CallBackQuery, context: &'a Option<CurrentUserContext>, sender_db_info: Option<CoffeezeraUser>) -> CallbackHandler<'a>{
        CallbackHandler{
            callback,
            context,
            sender_db_info
        }
    }

    fn is_turn_on_command(&self, command: &str) -> bool{
        return command.eq(TURN_ON);
    }

    fn is_turn_off_command(&self, command: &str) -> bool{
        return command.eq(TURN_OFF);
    }


//    fn update_msg_with_user_credits_and_on_option(&self) -> RequestResponse {
//        info!("Updating msg with user credits and turn on msg");
//        let reply_text = format!("Créditos: {:.2} segundos", self.sender_db_info.account_balance);
//        RequestResponse{
//            reply: String::new(),
//            markup: Some(vec![vec![self.turn_on_command.to_string()]]),
//            action: GrinderAction::DoNothing,
//        }
//    }
//

//




 






    fn user_has_credits(&self, user_db_info: &CoffeezeraUser) -> bool{
        if user_db_info.account_balance > 0.0{
            return true;
        }else {
            return false;
        }
    }


//    fn handle_turn_off_command(&self, context: &mut Option<CurrentUserContext>, message: &Message, sender_telegram_id: i64){
//        let should_turn_off_grinder: bool;
//        if let &mut Some(ref some_context) = context {
//            info!("Turn off command with grinder in use");
//            if some_context.current_user.telegram_id == sender_telegram_id{
//                info!("Turn off command with grinder in use by the current user");
//                should_turn_off_grinder = true;
//                self.update_msg_with_user_credits_and_on_option(&some_context.current_user,
//                                                                message.chat.id, message.message_id);
//            }else{
//                info!("Turn off command with grinder in use but NOT by current user");
//                self.update_msg_with_already_in_use_and_on_option(&some_context, message.chat.id,
//                                                                  message.message_id);
//                should_turn_off_grinder = false;
//            }
//        }else {
//            info!("Turn off command and grinder available");
//            should_turn_off_grinder = false;
//            if let Ok(user) = get_user(&self.database_connection, sender_telegram_id){
//                info!("Turn off command from someone in DB and with grinder available");
//                self.update_msg_with_user_credits_and_on_option(&user, message.chat.id, message.message_id);
//            }else{
//                info!("Turn off command from someone NOT in DB and with grinder available");
//                self.update_msg_with_not_registered(sender_telegram_id, message.chat.id,message.message_id);
//            }
//
//        }
//        if should_turn_off_grinder{
//            self.turn_off_grinder(context);
//        }
//    }


    fn handle_callback(&self) -> Response {
        info!("Handling the callback");
        let callback = match self.callback.data {
            Some(ref data) => data,
            None => {
                error!("Callback had no data");
                return Response {
                    reply: CALLBACK_NO_DATA.to_string(),
                    reply_markup: None,
                    action: GrinderAction::DoNothing,
                };
            }
        };

        if self.is_turn_on_command(callback.as_str()) {
            return self.handle_turn_on_command()
        }
//            else if self.is_turn_off_command(callback.as_str()) {
//            return self.handle_turn_off_command(context, message, callback_query.from.id);
//        }
            else{
                error!("Got a callback that was neither a turn-on or turn-off: {}", callback.as_str());
                return Response {
                    reply:  self.create_unknown_callback_data_response_text(&callback),
                    reply_markup: None,
                    action: GrinderAction::DoNothing,
                };
            }
    }

}
