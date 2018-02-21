extern crate teleborg;

use self::teleborg::{Message, User};
use super::{CurrentUserContext, CoffeezeraUser};
use coffeezera::telegram_replier::response::Response;
use super::grinder_action::GrinderAction;
const TURN_OFF: &'static str  = "Desligar";
const TURN_ON: &'static str  = "Ligar";
use coffeezera::IS_OPEN;
pub struct MessageHandler<'a>{
    message: Message,
    context: &'a Option<CurrentUserContext>,
    sender_db_info: Option<CoffeezeraUser>
}


impl<'a> MessageHandler <'a>{
    pub fn new(message: Message, context: &'a Option<CurrentUserContext>, sender_db_info: Option<CoffeezeraUser>) -> MessageHandler<'a>{
        MessageHandler{
            message,
            context,
            sender_db_info
        }
    }

    fn make_not_registered_response(user: &Option<User>) -> Response {
        match user {
            &Some(ref user) => {
                Response {
                    reply: format!("Você não está registrado ainda, envie essa mensagem para o @TiberioFerreira com o seu ID: {}", user.id),
                    action: GrinderAction::DoNothing,
                    reply_markup: None,
                }
            },
            &None => {
                Response {
                    reply: format!("Essa mensagem não continha remetente. Envie isso para @TiberioFerreira"),
                    action: GrinderAction::DoNothing,
                    reply_markup: None,
                }
            }
        }
    }

    fn make_you_are_already_using_response(context: &CurrentUserContext) -> Response {
        if IS_OPEN{
            Response {
                reply: format!("Você já está usando o moedor. Ele está OPEN e você ainda tem {:.2} segundos de crédito.",
                               context.current_user.account_balance),
                action: GrinderAction::DoNothing,
                reply_markup: Some(vec![vec![TURN_OFF.to_string()]])
            }
        }else {
            Response {
                reply: format!("Você já está usando o moedor. Créditos: {:.2} segundos",
                               context.current_user.account_balance),
                action: GrinderAction::DoNothing,
                reply_markup: Some(vec![vec![TURN_OFF.to_string()]])
            }
        }
    }

    fn make_someone_is_already_using_response(context: &CurrentUserContext) -> Response {
        let reply_text = format!("O moedor já está em uso por {}. Por favor, espere ele desligar o moedor ou ser removido automaticamente em: {:.2} segundos",
                                 context.current_user.name,
                                 context.get_time_left_turn_off());
        Response {
            reply: reply_text,
            action: GrinderAction::DoNothing,
            reply_markup: None
        }
    }

    fn make_default_response(sender_db_info: &CoffeezeraUser) -> Response {
        let credits = sender_db_info.account_balance;
        let reply_text;
        if IS_OPEN{
            reply_text = format!("O moedor está OPEN e você ainda tem {:.2} segundos de crédito para usar depois.", credits);
        }else {
            reply_text = format!("Créditos: {:.2} segundos", credits);
        }
        Response {
            reply: reply_text,
            action: GrinderAction::DoNothing,
            reply_markup: Some(vec![vec![TURN_ON.to_string()]])
        }
    }

    fn make_no_credits_response(sender_db_info: &CoffeezeraUser) -> Response {
        let credits = sender_db_info.account_balance;
        Response {
            reply: format!("Você está sem créditos: {:.2} segundos, fale com @TiberioFerreira para adicionar mais.", credits),
            action: GrinderAction::DoNothing,
            reply_markup: None
        }
    }


    fn make_reply_to_db_user_msg(&self, sender_db_info: &CoffeezeraUser) -> Response {
        let context = self.context;
        if let &Some(ref context) = context{
            info!("There was already an user using the grinder.");
            if sender_db_info.telegram_id == context.current_user.telegram_id{
                info!("It was the sender!");
                return MessageHandler::make_you_are_already_using_response(context);
            }else {
                info!("It was NOT the sender!");
                return MessageHandler::make_someone_is_already_using_response(context);
            }
        }else if sender_db_info.account_balance > 0.0 || IS_OPEN{
            return MessageHandler::make_default_response(sender_db_info);
        }else {
            return MessageHandler::make_no_credits_response(sender_db_info);
        }
    }

    pub fn get_request_response(&self) -> Response {
        match &self.sender_db_info {
            &None => {
                return MessageHandler::make_not_registered_response(&self.message.from);
            },
            &Some(ref sender_db_info) => {
                return self.make_reply_to_db_user_msg(sender_db_info)
            }
        }
    }

}