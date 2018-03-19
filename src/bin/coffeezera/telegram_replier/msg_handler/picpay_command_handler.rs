use super::*;

pub const ADD_PICPAY_ACCOUNT: &'static str  = "/vincular_conta_picpay";
pub const MODIFY_PICPAY_ACCOUNT: &'static str  = "/modificar_conta_picpay_vinculada";
pub const GET_PICPAY_ACCOUNT: &'static str  = "/ver_conta_picpay_vinculada";
pub const REMOVE_PICPAY_ACCOUNT: &'static str  = "/desvincular_conta_picpay";

pub struct PicPayCommandHandler{
    message_text: String,
    sender_db_info: CoffeezeraUser
}

impl PicPayCommandHandler{
    pub fn new(message_text: String, sender_db_info: CoffeezeraUser) -> PicPayCommandHandler{
        PicPayCommandHandler{
            message_text,
            sender_db_info
        }
    }

    fn handle_add_account(&self, arguments: Vec<&str>) -> Response{
        if arguments.len() != 1{
            return MessageMaker::make_add_picpay_account_instruction();
        }
        if let Some(picpay_name) = arguments.first() {
            return MessageMaker::make_successfully_added_picpay_account(
                picpay_name.to_string(), self.sender_db_info.clone());
        }else{
            return MessageMaker::make_add_picpay_account_instruction();
        }

    }

    fn handle_modify_account(&self, arguments: Vec<&str>) -> Response{
        if arguments.len() != 1{
            return MessageMaker::make_modify_picpay_account_instruction();
        }
        if let Some(picpay_name) = arguments.first() {
            return MessageMaker::make_successfully_modified_picpay_account(
                picpay_name.to_string(), self.sender_db_info.clone());
        }else{
            return MessageMaker::make_modify_picpay_account_instruction();
        }
    }

    fn handle_get_account(&self, arguments: Vec<&str>) -> Response{
        if arguments.len() != 0{
            return MessageMaker::make_get_picpay_account_instruction();
        }
        return MessageMaker::make_successfully_got_picpay_account(self.sender_db_info.clone());
    }

    fn handle_remove_account(&self, arguments: Vec<&str>) -> Response{
        if arguments.len() != 0{
            return MessageMaker::make_remove_picpay_account_instruction();
        }
        return MessageMaker::make_successfully_removed_picpay_account(self.sender_db_info.clone());
    }


    pub fn get_response(&self) -> Response {
        let mut split_message = self.message_text.split_whitespace().collect::<Vec<&str>>();
        if split_message.len() == 0{
            return MessageMaker::return_possible_commands();
        }else if let Some(command) = split_message.clone().first() {
            match command {
                &ADD_PICPAY_ACCOUNT => {
                    return self.handle_add_account(split_message.drain(1..).collect())
                },
                &MODIFY_PICPAY_ACCOUNT => {
                    return self.handle_modify_account(split_message.drain(1..).collect())
                },
                &REMOVE_PICPAY_ACCOUNT => {
                    return self.handle_remove_account(split_message.drain(1..).collect())
                },
                _ => return MessageMaker::return_possible_commands()
            }
        }
        return MessageMaker::return_possible_commands();
    }
}