extern crate coffeezerabot;
use super::*;

impl<'a> CallbackHandler<'a> {

    pub fn handle_turn_on_command(&self) -> Response {
        if let &Some(ref some_context) = self.context {
            info!("Turn on command with grinder in use");
            if some_context.current_user.telegram_id == self.callback.from.id {
                info!("Turn on command by same user already using it");
                return self.get_response_for_turn_on_by_current_user(some_context);
            } else {
                info!("Turn on command by other user than the one using it");
                return self.get_response_for_turn_on_while_busy(some_context);
            }
        } else {
            info!("Turn on command and grinder available");
            if let Some(ref user_db_info) = self.sender_db_info {
                info!("Turn on command and grinder available and user is in DB");
                if !self.user_has_credits(&user_db_info) {
                    info!("User has no credits!");
                    return self.get_response_for_turn_on_no_credits(&user_db_info);
                }
                return self.get_response_for_turn_on_with_credits_while_available(&user_db_info);
            } else {
                return self.get_response_for_not_registered_user();
            }
        }
    }

    fn get_response_for_not_registered_user(&self) -> Response {
        info!("Updating msg with not registered msg");
        let reply_text = format!("Você não está registrado para usar o moedor. Envie essa mensagem para @TiberioFerreira com o seu ID: {}", self.callback.from.id);
        Response {
            reply: reply_text,
            reply_markup: None,
            action: GrinderAction::DoNothing,
        }
    }


    fn get_response_for_turn_on_with_credits_while_available(&self, sender_db_info: &CoffeezeraUser) -> Response {
        info!("Updating msg with user credits and turn off msg and turning on grinder");
        let reply_text = format!("Créditos: {:.2} segundos", sender_db_info.account_balance);
        Response {
            reply: reply_text,
            reply_markup: Some(vec![vec![TURN_OFF.to_string()]]),
            action: GrinderAction::TurnOn,
        }
    }

    fn get_response_for_turn_on_no_credits(&self, sender_db_info: &CoffeezeraUser) -> Response {
        info!("Updating msg with not enough credits!");
        let reply_text = format!("Você está sem créditos: {:.2} segundos. :( Fale com @TiberioFerreira.", sender_db_info.account_balance);
        Response {
            reply: reply_text,
            reply_markup: None,
            action: GrinderAction::DoNothing,
        }
    }

    fn get_response_for_turn_on_by_current_user(&self, context: &CurrentUserContext) -> Response {
        info!("Updating msg with user credits and turn off msg");
        let reply_text = format!("Créditos: {:.2} segundos", context.current_user.account_balance);
        Response {
            reply: reply_text,
            reply_markup: Some(vec![vec![TURN_OFF.to_string()]]),
            action: GrinderAction::DoNothing,
        }
    }

    fn make_already_in_use_msg(&self, context: &CurrentUserContext) -> String{
        format!("O moedor já está em uso por {}. Por favor, espere ele desligar o moedor ou ser removido automaticamente em: {} segundos",
                context.current_user.name,
                context.get_time_left_turn_off())
    }

    fn get_response_for_turn_on_while_busy(&self, context: &CurrentUserContext) -> Response {
        info!("Sending already in use message update and ON option");
        let reply_text = self.make_already_in_use_msg(context);
        Response {
            reply: reply_text,
            reply_markup: Some(vec![vec![TURN_ON.to_string()]]),
            action: GrinderAction::DoNothing,
        }
    }
}