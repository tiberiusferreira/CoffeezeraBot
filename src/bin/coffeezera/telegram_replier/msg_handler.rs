extern crate teleborg;

use self::teleborg::{Message, User};
use super::{CurrentUserContext, CoffeezeraUser, RequestResponse, GrinderAction};
const TURN_OFF: &'static str  = "Desligar";
const TURN_ON: &'static str  = "Ligar";
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

    fn make_not_registered_response(user: &Option<User>) -> RequestResponse{
        match user {
            &Some(ref user) => {
                RequestResponse {
                    reply: format!("Você não está registrado ainda, envie essa mensagem para o @TiberioFerreira com o seu ID: {}", user.id),
                    action: GrinderAction::DoNothing,
                    markup: None,
                }
            },
            &None => {
                RequestResponse {
                    reply: format!("Essa mensagem não continha remetente. Envie isso para @TiberioFerreira"),
                    action: GrinderAction::DoNothing,
                    markup: None,
                }
            }
        }
    }

    fn make_you_are_already_using_response() -> RequestResponse{
        RequestResponse{
            reply: format!("Você já está usando o moedor."),
            action: GrinderAction::DoNothing,
            markup: Some(vec![vec![TURN_OFF.to_string()]])
        }
    }

    fn make_someone_is_already_using_response(context: &CurrentUserContext) -> RequestResponse{
        RequestResponse{
            reply: format!("{} já está usando o moedor.", context.current_user.name),
            action: GrinderAction::DoNothing,
            markup: None
        }
    }

    fn make_default_response(sender_db_info: &CoffeezeraUser) -> RequestResponse{
        let credits = sender_db_info.account_balance;
        RequestResponse{
            reply: format!("Créditos: {:.2} segundos", credits),
            action: GrinderAction::DoNothing,
            markup: Some(vec![vec![TURN_ON.to_string()]])
        }
    }

    fn make_no_credits_response(sender_db_info: &CoffeezeraUser) -> RequestResponse{
        let credits = sender_db_info.account_balance;
        RequestResponse{
            reply: format!("Você está sem créditos: {:.2} segundos, fale com @TiberioFerreira para adicionar mais.", credits),
            action: GrinderAction::DoNothing,
            markup: None
        }
    }


    fn make_reply_to_db_user_msg(&self, sender_db_info: &CoffeezeraUser) -> RequestResponse{
        let context = self.context;
        if let &Some(ref context) = context{
            info!("There was already an user using the grinder.");
            if sender_db_info.telegram_id == context.current_user.telegram_id{
                info!("It was the sender!");
                return MessageHandler::make_you_are_already_using_response()
            }else {
                info!("It was NOT the sender!");
                return MessageHandler::make_someone_is_already_using_response(context);
            }
        }else if sender_db_info.account_balance > 0.0 {
            return MessageHandler::make_default_response(sender_db_info);
        }else {
            return MessageHandler::make_no_credits_response(sender_db_info);
        }
    }

    pub fn get_response(&self) -> RequestResponse{
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