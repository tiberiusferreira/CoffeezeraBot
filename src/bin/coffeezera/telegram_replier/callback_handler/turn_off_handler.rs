use super::*;
use coffeezera::IS_OPEN;
impl<'a> CallbackHandler<'a> {
    pub fn handle_turn_off_command(&self) -> Response {
        if let &Some(ref some_context) = self.context {
            info!("Turn off command with grinder in use");
            if some_context.current_user.telegram_id == self.callback.sender_id{
                info!("Turn off command with grinder in use by the current user");
                return self.get_response_for_turn_off_by_current_user(some_context);
            } else {
                if let Some(ref sender_db_info) = self.sender_db_info {
                    info!("Turn off command with grinder in use but NOT by current user, but a registered one");
                    return self.get_response_for_turn_off_by_not_current_user_but_a_registered_one(some_context, sender_db_info);
                }else {
                    info!("Turn off command with grinder in use but NOT by current user and a NOT registered one");
                    return self.get_response_for_not_registered_user();
                }
            }
        } else {
            if let Some(ref sender_db_info) = self.sender_db_info {
                info!("Turn off command from someone in DB and with grinder available");
                return self.get_response_for_turn_off_by_someone_registered_while_available(sender_db_info);
            } else {
                info!("Turn off command from someone NOT in DB and with grinder available");
                return self.get_response_for_not_registered_user();
            }
        }
    }

    fn get_response_for_turn_off_by_current_user(&self, context: &CurrentUserContext) -> Response{
        if context.get_time_since_context_creation_as_sec() < 7{
            let reply_text = format!("Para evitar pessoas ligando e desligando o moedor repetidamente, você precisa esperar ao menos 7 segundos antes de desliga-lo. Só se passaram {}s .", context.get_time_since_context_creation_as_sec());
            return Response {
                reply: reply_text,
                reply_markup: Some(vec![vec![TURN_OFF.to_string()]]),
                action: UpdateImpact::DoNothing,
            };
        }
        let reply_text;
        if IS_OPEN{
            reply_text = format!("Desligado! Bom café! O moedor está OPEN, mas você ainda tem: {:.2} segundos de crédito pra usar depois.", context.current_user.account_balance);
        }else {
            reply_text = format!("Desligado! Bom café! Você ainda tem: {:.2} segundos", context.current_user.account_balance);
        }
        Response {
            reply: reply_text,
            reply_markup: Some(vec![vec![TURN_ON.to_string()]]),
            action: UpdateImpact::TurnOff,
        }
    }

    pub fn get_response_for_auto_turn_off(context: &CurrentUserContext) -> Response{
        let reply_text;
        if IS_OPEN{
            reply_text = format!("O moedor foi desligado automaticamente. Bom café! O moedor está OPEN, mas você ainda tem: {:.2} segundos de crédito para usar depois.", context.current_user.account_balance);
        }else {
            reply_text = format!("O moedor foi desligado automaticamente. Bom café! Você ainda tem: {:.2} segundos", context.current_user.account_balance);
        }
        Response {
            reply: reply_text,
            reply_markup: Some(vec![vec![TURN_ON.to_string()]]),
            action: UpdateImpact::DoNothing,
        }
    }

    pub fn get_response_for_no_credits_auto_turn_off_message(context: &CurrentUserContext) -> Response{
        let reply_text = format!("Você está sem créditos. Recarrege automaticamente enviando um pagamento para @tiberio.ferreira no PicPay com SOMENTE seu apelido na forma exata: \"{}\" sem aspas no comentario do pagamento. Se você não tem PicPay ainda ganhe reembolso na primeira recarga de R$10 ou mais no cartao usando o codigo CTNLRE ate 2h após inscrição em Ajustes -> Usar codigo promocional", context.current_user.name);
        Response {
            reply: reply_text,
            reply_markup: Some(vec![vec![TURN_ON.to_string()]]),
            action: UpdateImpact::DoNothing,
        }
    }




    fn get_response_for_turn_off_by_not_current_user_but_a_registered_one(&self, context: &CurrentUserContext, sender_db_info: &CoffeezeraUser) -> Response{
        let reply_text;
        if IS_OPEN{
            reply_text = format!("O moedor está em uso por {}. Só ele pode desliga-lo :(.",
                                 context.current_user.name);
        }else {
            reply_text = format!("O moedor está em uso por {}. Só ele pode desliga-lo :(. Você pode tentar liga-lo depois usando seus créditos: {:.2} segundos",
                                 context.current_user.name, sender_db_info.account_balance);
        }
        Response {
            reply: reply_text,
            reply_markup: Some(vec![vec![TURN_ON.to_string()]]),
            action: UpdateImpact::DoNothing,
        }
    }

    fn get_response_for_turn_off_by_someone_registered_while_available(&self, sender_db_info: &CoffeezeraUser) -> Response{
        let reply_text;
        if IS_OPEN{
            reply_text = format!("O moedor não estava ligado, mas você pode liga-lo. O moedor está OPEN e você ainda tem tem {:.2} segundos de crédito para usar depois.", sender_db_info.account_balance);
        }else {
            reply_text = format!("O moedor não estava ligado, mas você pode liga-lo. Você tem {:.2} segundos de crédito.", sender_db_info.account_balance);
        }
        Response {
            reply: reply_text,
            reply_markup: Some(vec![vec![TURN_ON.to_string()]]),
            action: UpdateImpact::DoNothing,
        }
    }
}