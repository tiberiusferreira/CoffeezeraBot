extern crate telegram_bot;
extern crate tokio_core;
extern crate futures;


use telegram_bot::*;

fn main() {
    let api = Api::from_token("477595004:AAGjB-rB5Mwkc75D5Q9E_2RqUBi2ioinCSk").unwrap();
    let new_user_msg = "Todo uso da máquina será creditado a sua conta durante os \
                         próximos 5 minutos ou até você clicar o botão Desligar abaixo.";
    let you_are_already_using_msg = "A máquina já está em uso por você.";
    let turn_on_command = "Ligar";
    // We want to listen for new updates via LongPoll
    let mut listener = api.listener(ListeningMethod::LongPoll(None));

    let mut current_user_id: i64 = -1;
    let mut current_user_name = String::new();

    fn send_msg(api: &Api, chat_id: i64, message: &str,keyboard_options: Vec<Vec<String>>){
        api.send_message(
            chat_id,
            message.to_string(),
            None,
            None,
            None,
            Some(ReplyMarkup::from(ReplyKeyboardMarkup {
                keyboard: keyboard_options,
                resize_keyboard: Some(false),
                one_time_keyboard: Some(true),
                selective: Some(false)
            }
            )
            )
        );
    }
    listener.listen(|u| {
        if let Some(m) = u.message {
            // if the message was a text message:
            if let MessageType::Text(text) = m.msg {
                if text.eq(turn_on_command) && current_user_id == -1{
                    send_msg(&api,
                             m.chat.id(),
                             &new_user_msg,
                             vec![vec!["Desligar".to_string()]]);
                    current_user_id = m.from.id;
                    current_user_name = m.from.first_name;
                    return Ok(ListeningAction::Continue);
                }else if text.eq("Ligar") && current_user_id == m.from.id{
                    send_msg(&api,
                             m.chat.id(),
                             &you_are_already_using_msg,
                             vec![vec!["Desligar".to_string()]]
                    );
                    return Ok(ListeningAction::Continue);
                }else if text.eq("Ligar") && current_user_id != m.from.id{
                    send_msg(&api,
                             m.chat.id(),
                             &format!("A máquina já está em uso por {}. Por favor, aguarde.", current_user_name),
                             vec![vec![turn_on_command.to_string()]]
                    );
                    return Ok(ListeningAction::Continue);
                }
                api.send_message(
                    m.chat.id(),
                    format!("Tente \"{}\" como comando.", turn_on_command),
                    None,
                    None,
                    None,
                    Some(ReplyMarkup::from(ReplyKeyboardMarkup {
                        keyboard: vec![vec![turn_on_command.to_string()]],
                        resize_keyboard: Some(false),
                        one_time_keyboard: Some(true),
                        selective: Some(false)
                    }
                    )
                    )
                );
            }
        }
        Ok(ListeningAction::Continue)
    });
}




