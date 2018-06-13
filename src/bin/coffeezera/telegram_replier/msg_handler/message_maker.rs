use super::*;
pub struct MessageMaker;

impl MessageMaker{
    pub fn make_not_registered_response(sender_telegram_id: i64) -> Response {
        Response {
            reply: format!("Você não está registrado ainda, envie essa mensagem para o @TiberioFerreira com o seu ID: {}", sender_telegram_id),
            action: UpdateImpact::DoNothing,
            reply_markup: None,
        }
    }

pub fn make_you_are_already_using_response(context: &CurrentUserContext) -> Response {
    if IS_OPEN{
        Response {
            reply: format!("Você já está usando o moedor. Ele está OPEN e você ainda tem {:.2} segundos de crédito.",
                           context.current_user.account_balance),
            action: UpdateImpact::DoNothing,
            reply_markup: Some(vec![vec![TURN_OFF.to_string()]])
        }
    }else {
        Response {
            reply: format!("Você já está usando o moedor. Créditos: {:.2} segundos",
                           context.current_user.account_balance),
            action: UpdateImpact::DoNothing,
            reply_markup: Some(vec![vec![TURN_OFF.to_string()]])
        }
    }
}

pub fn make_someone_is_already_using_response(context: &CurrentUserContext) -> Response {
    let reply_text = format!("O moedor já está em uso por {}. Por favor, espere ele desligar o moedor ou ser removido automaticamente em: {:.2} segundos",
                             context.current_user.name,
                             context.get_time_left_turn_off());
    Response {
        reply: reply_text,
        action: UpdateImpact::DoNothing,
        reply_markup: Some(vec![vec![TURN_ON.to_string()]])
    }
}

pub fn make_default_response(sender_db_info: &CoffeezeraUser) -> Response {
    let credits = sender_db_info.account_balance;
    let reply_text;
    if IS_OPEN{
        reply_text = format!("O moedor está OPEN e você ainda tem {:.2} segundos de crédito para usar depois.", credits);
    }else {
        reply_text = format!("Créditos: {:.2} segundos", credits);
    }
    Response {
        reply: reply_text,
        action: UpdateImpact::DoNothing,
        reply_markup: Some(vec![vec![TURN_ON.to_string()]])
    }
}





pub fn make_no_credits_response(sender_db_info: &CoffeezeraUser) -> Response {
    Response {
        reply: format!("Você está sem créditos. Recarrege automaticamente enviando um pagamento para @tiberio.ferreira no PicPay com SOMENTE seu apelido: \"{}\" sem aspas, no comentario do pagamento. Se você nao tem PicPay ainda ganhe reembolso na primeira recarga de R$10 ou mais no cartao usando o codigo CTNLRE ate 2h apos inscricao em Ajustes -> Usar codigo promocional", sender_db_info.name),
        action: UpdateImpact::DoNothing,
        reply_markup: None
    }
}


}

