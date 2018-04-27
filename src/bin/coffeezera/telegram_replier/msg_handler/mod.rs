extern crate teleborg;
mod message_maker;

use self::teleborg::*;
use super::{CurrentUserContext, CoffeezeraUser};
use coffeezera::telegram_replier::response::Response;
use super::update_outcome::UpdateImpact;
use super::TURN_ON;
use super::TURN_OFF;
use coffeezera::IS_OPEN;
use self::message_maker::*;

pub struct MessageHandler<'a>{
    message: CleanedMessage,
    context: &'a Option<CurrentUserContext>,
    sender_db_info: Option<CoffeezeraUser>
}

struct MessageHandlerForKnownUser<'a>{
    context: &'a Option<CurrentUserContext>,
    sender_db_info: CoffeezeraUser
}


impl<'a> MessageHandlerForKnownUser <'a>{
    fn make_reply(&self) -> Response {
        let context = self.context;
        if let &Some(ref context) = context{
            info!("There was already an user using the grinder.");
            if self.sender_db_info.telegram_id == context.current_user.telegram_id{
                info!("It was the sender!");
                return MessageMaker::make_you_are_already_using_response(context);
            }else {
                info!("It was NOT the sender!");
                return MessageMaker::make_someone_is_already_using_response(context);
            }
        }else if self.sender_db_info.account_balance > 0.0 || IS_OPEN{
            return MessageMaker::make_default_response(&self.sender_db_info);
        }else {
            return MessageMaker::make_no_credits_response(&self.sender_db_info);
        }
    }
}


impl<'a> MessageHandler <'a>{
    pub fn new(message: CleanedMessage, context: &'a Option<CurrentUserContext>, sender_db_info: Option<CoffeezeraUser>) -> MessageHandler<'a>{
        MessageHandler{
            message,
            context,
            sender_db_info
        }
    }

    pub fn get_response(self) -> Response {
        match self.sender_db_info {
            None => {
                return MessageMaker::make_not_registered_response(self.message.sender_id);
            },
            Some(sender_db_info) => {
                return MessageHandlerForKnownUser{
                    context: self.context,
                    sender_db_info
                }.make_reply();
            }
        }
    }

}