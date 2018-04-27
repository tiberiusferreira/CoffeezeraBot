extern crate teleborg;

use self::teleborg::*;
use super::{CurrentUserContext, CoffeezeraUser};
use coffeezera::telegram_replier::response::Response;
use super::TURN_OFF;
use super::TURN_ON;
use coffeezera::telegram_replier::update_outcome::UpdateImpact;
mod turn_on_handler;
mod turn_off_handler;



pub struct CallbackHandler<'a>{
    callback: CleanedCallbackQuery,
    context: &'a Option<CurrentUserContext>,
    sender_db_info: Option<CoffeezeraUser>
}


impl<'a> CallbackHandler <'a>{

    fn create_unknown_callback_data_response_text(&self, unknown_data: &str) -> String {
        format!("Esse callback não tinha dados: {}, envie essa mensagem para @TiberioFerreira.", unknown_data)
    }
    pub fn new(callback: CleanedCallbackQuery, context: &'a Option<CurrentUserContext>, sender_db_info: Option<CoffeezeraUser>) -> CallbackHandler<'a>{
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

    fn user_has_credits(&self, user_db_info: &CoffeezeraUser) -> bool{
        if user_db_info.account_balance > 0.0{
            return true;
        }else {
            return false;
        }
    }

    pub fn handle_callback(&self) -> Response {
        info!("Handling the callback");
        let callback = &self.callback.data;
        if self.is_turn_on_command(callback.as_str()) {
            return self.handle_turn_on_command();
        }   else if self.is_turn_off_command(callback.as_str()) {
            return self.handle_turn_off_command();
        } else{
            error!("Got a callback that was neither a turn-on or turn-off: {}", callback.as_str());
            return Response {
                reply:  self.create_unknown_callback_data_response_text(&callback),
                reply_markup: None,
                action: UpdateImpact::DoNothing,
            };
        }
    }
}
