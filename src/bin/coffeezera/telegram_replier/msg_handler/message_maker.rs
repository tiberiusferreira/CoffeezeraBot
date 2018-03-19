use super::*;
use super::picpay_command_handler::*;
pub struct MessageMaker;

impl MessageMaker{
    pub fn make_not_registered_response(user: &Option<User>) -> Response {
        match user {
            &Some(ref user) => {
                Response {
                    reply: format!("Você não está registrado ainda, envie essa mensagem para o @TiberioFerreira com o seu ID: {}", user.id),
                    action: UpdateImpact::DoNothing,
                    reply_markup: None,
                }
            },
            &None => {
                Response {
                    reply: format!("Essa mensagem não continha remetente. Envie isso para @TiberioFerreira"),
                    action: UpdateImpact::DoNothing,
                    reply_markup: None,
                }
            }
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

    pub fn make_add_picpay_account_instruction() -> Response{
        return Response{
            reply: format!("Envie {} mais o seu usuário PicPay no formato @meu.picpay. Ex: {} @tiberio.ferreira", ADD_PICPAY_ACCOUNT, ADD_PICPAY_ACCOUNT),
            reply_markup: None,
            action: UpdateImpact::DoNothing,
        }
    }



    pub fn make_successfully_got_picpay_account(db_user: CoffeezeraUser) -> Response{
        match db_user.picpay_username {
            Some(username) =>{
                return Response{
                    reply: format!("Conta PicPay vinculada: {} ! Pagamentos feitos dela para @tiberio.ferreira *SEM NENHUMA MENSAGEM ASSOCIADA* serão creditados ao Coffeezera", username),
                    action: UpdateImpact::DoNothing,
                    reply_markup: Some(vec![vec![TURN_ON.to_string()]]),
                }
            },
            None => {
                return Response{
                    reply: format!("Nenhuma conta PicPay vinculada."),
                    action: UpdateImpact::DoNothing,
                    reply_markup: Some(vec![vec![TURN_ON.to_string()]]),
                }
            }
        }

    }


    pub fn make_successfully_added_picpay_account(picpay_account: String, db_user: CoffeezeraUser) -> Response{
        return Response{
            reply: format!("Conta PicPay {} vinculada! Pagamentos feitos dela para @tiberio.ferreira *SEM NENHUMA MENSAGEM ASSOCIADA* serão creditados ao Coffeezera", picpay_account),
            action: UpdateImpact::AddPicpayAccount {
                user: db_user,
                picpay_name: picpay_account,
            },
            reply_markup: Some(vec![vec![TURN_ON.to_string()]]),
        }
    }

    pub fn make_successfully_modified_picpay_account(picpay_account: String, db_user: CoffeezeraUser) -> Response{
        return Response{
            reply: format!("Conta PicPay alterada. Conta: {} vinculada! Pagamentos feitos dela para @tiberio.ferreira *SEM NENHUMA MENSAGEM ASSOCIADA* serão creditados ao Coffeezera", picpay_account),
            action: UpdateImpact::AddPicpayAccount {
                user: db_user,
                picpay_name: picpay_account,
            },
            reply_markup: Some(vec![vec![TURN_ON.to_string()]]),
        }
    }

    pub fn make_successfully_removed_picpay_account(db_user: CoffeezeraUser) -> Response{
        return Response{
            reply: format!("Conta PicPay removida."),
            action: UpdateImpact::RemovePicpayAccount {
                user: db_user,
            },
            reply_markup: Some(vec![vec![TURN_ON.to_string()]]),
        }
    }


    pub fn make_modify_picpay_account_instruction() -> Response{
        return Response{
            reply: format!("Envie {} mais o seu usuário PicPay no formato @meu.picpay para modificar sua conta PicPay vinculada atualmente. Ex: {} @tiberio.ferreira", MODIFY_PICPAY_ACCOUNT, MODIFY_PICPAY_ACCOUNT),
            reply_markup: None,
            action: UpdateImpact::DoNothing,
        }
    }

    pub fn make_get_picpay_account_instruction() -> Response{
        return Response{
            reply: format!("Envie {} para ver a conta PicPay vinculada (se houver).", GET_PICPAY_ACCOUNT),
            reply_markup: None,
            action: UpdateImpact::DoNothing,
        }
    }



    pub fn make_remove_picpay_account_instruction() -> Response{
        return Response{
            reply: format!("Envie somente {} para desvincular a sua conta PicPay.", REMOVE_PICPAY_ACCOUNT),
            reply_markup: None,
            action: UpdateImpact::DoNothing,
        }
    }

    pub fn return_possible_commands() -> Response{
        let reply_text = format!("Os comandos possíveis são: {}  @meu.picpay {} @meu.novo.picpay {}",
                                 ADD_PICPAY_ACCOUNT, MODIFY_PICPAY_ACCOUNT, REMOVE_PICPAY_ACCOUNT);

        Response {
            reply: reply_text,
            action: UpdateImpact::DoNothing,
            reply_markup: None
        }
    }

    pub fn make_no_credits_response(sender_db_info: &CoffeezeraUser) -> Response {
        let credits = sender_db_info.account_balance;
        Response {
            reply: format!("Você está sem créditos: {:.2} segundos, fale com @TiberioFerreira para adicionar mais.", credits),
            action: UpdateImpact::DoNothing,
            reply_markup: None
        }
    }
}

