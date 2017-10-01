extern crate telegram_bot;
extern crate tokio_core;
extern crate futures;
use std::time::Instant;
use std::thread;
use std::sync::Arc;
use std::sync::RwLock;
use telegram_bot::*;
use std::thread::sleep;
use std::sync::RwLockWriteGuard;
use std::time::{Duration, SystemTime};
extern crate time;


struct CoffeezeraUser {
    name: String,
    id: i64,
    time_left: f64,
    chat_id: i64
}

impl CoffeezeraUser {
    fn set_user<'a>(&mut self, id: i64, name: &'a str, chat_id: i64) {
        self.id = id;
        self.name = name.to_string();
        self.time_left = 10.0;
        self.chat_id = chat_id;
    }

    fn clear_user<'a>(&mut self) {
        self.id = -1;
        self.name = "".to_string();
        self.time_left = 0.0;
        self.chat_id = -1;
    }
}


//
//fn handle_text_msg<'a>(api: &Api, msg: &'a Message, current_user: Arc<RwLock<CoffeezeraUser>>) {
//    const NEW_USER_MSG: &'static str = "Todo uso da máquina será creditado a sua conta durante os \
//                         próximos 5 minutos ou até você clicar o botão Desligar abaixo.";
//    const YOU_ARE_ALREADY_USING_MSG: &'static str = "A máquina já está em uso por você, ";
//    const TURN_ON_COMMAND: &'static str = "Ligar";
//    const TURN_OFF_COMMAND: &'static str = "Desligar";
//    const WRONG_MSG_TYPE_ERROR: &'static str = "Got message which was not text";
//    let text_msg: &str = match msg.msg {
//        MessageType::Text(ref text) => text.as_str(),
//        _ => {
//            println!("{}", WRONG_MSG_TYPE_ERROR);
//            return;
//        }
//    };
//    match current_user.write() {
//        Ok(mut user) => {
//            if text_msg == TURN_ON_COMMAND && user.id == -1 {
//                send_msg(&api,
//                         msg.chat.id(),
//                         &NEW_USER_MSG,
//                         vec![vec!["Desligar".to_string()]]);
//                println!("{} {}", user.id, msg.from.id);
//                set_user(msg.from.id, &msg.from.first_name.as_str(), msg.chat.id(), &mut user);
//                println!("Current user name = {}", user.name);
//                println!("User name = {}", msg.from.first_name.as_str());
//            } else if text_msg == TURN_ON_COMMAND && user.id == msg.from.id {
//                println!("{} {}", user.id, msg.from.id);
//                send_msg(&api,
//                         msg.chat.id(),
//                         &format!("{}{}", &YOU_ARE_ALREADY_USING_MSG, user.name),
//                         vec![vec!["Desligar".to_string()]]
//                );
//            } else if text_msg == TURN_ON_COMMAND && user.id != msg.from.id {
//                send_msg(&api,
//                         msg.chat.id(),
//                         &format!("A máquina já está em uso por {}. Por favor, aguarde.", user.name),
//                         vec![vec![TURN_ON_COMMAND.to_string()]]
//                );
//            } else if text_msg == TURN_OFF_COMMAND && user.id == msg.from.id {
//                clear_user(&mut user);
//                send_msg(&api,
//                         msg.chat.id(),
//                         &format!("A máquina foi desligada e seus créditos não serão mais gastos"),
//                         vec![vec![TURN_ON_COMMAND.to_string()]]
//                );
//            } else {
//                send_msg(&api,
//                         msg.chat.id(),
//                         &format!("Tente \"{}\" como comando.", TURN_ON_COMMAND),
//                         vec![vec![TURN_ON_COMMAND.to_string()]]
//                );
//            }
//        },
//        Err(e) => {
//            println!("Failed to get a lock: {}", e);
//        }
//
//    }
//}

struct CoffeezeraBot {
    api: Api,
    current_user: CoffeezeraUser,
}

impl CoffeezeraBot {
    pub fn new() -> CoffeezeraBot {
        CoffeezeraBot{
            api: Api::from_token("477595004:AAGjB-rB5Mwkc75D5Q9E_2RqUBi2ioinCSk").unwrap(),
            current_user: CoffeezeraUser::default()
        }
    }


    fn send_msg(&self, chat_id: i64, message: &str,keyboard_options: Vec<Vec<String>>){
        self.api.send_message(
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

    fn is_older_than(message: &Message, seconds: i64) -> bool{
        if message.date < time::now().to_timespec().sec-seconds {
            println!("Time now: {} Message time: {}", time::now().to_timespec().sec, message.date);
            println!("Message is more than {} s old. Discarding", seconds);
            return true;
        }
        return false;
    }

    fn is_text_msg(msg: &Message) -> bool{
        match msg.msg {
            MessageType::Text(_) => return true,
            _ => return false
        }
    }
    pub fn get_msg_text(msg: &Message) -> &str{
        match msg.msg {
            MessageType::Text(ref content) => return content,
            _ => panic!("Not a text msg")
        }
    }

    fn handle_msg(&mut self, message: &Message){
        const NEW_USER_MSG: &'static str = "Todo uso da máquina será creditado a sua conta durante os \
                                 próximos 5 minutos ou até você clicar o botão Desligar abaixo.";
        const YOU_ARE_ALREADY_USING_MSG: &'static str = "A máquina já está em uso por você, ";
        const TURN_ON_COMMAND: &'static str = "Ligar";
        const TURN_OFF_COMMAND: &'static str = "Desligar";
        let msg_text = CoffeezeraBot::get_msg_text(&message);
        if msg_text == TURN_ON_COMMAND && self.current_user.id == -1 {
            self.send_msg(message.chat.id(),
                          &NEW_USER_MSG,
                          vec![vec!["Desligar".to_string()]]);
            self.current_user.set_user(message.from.id, &message.from.first_name.as_str(), message.chat.id());
            println!("{} {}", self.current_user.id, message.from.id);

            println!("Current user name = {}", self.current_user.name);
            println!("User name = {}", message.from.first_name.as_str());
        }
            else if msg_text == TURN_ON_COMMAND && self.current_user.id == message.from.id {
                self.send_msg(message.chat.id(),
                              &format!("{}{}", &YOU_ARE_ALREADY_USING_MSG, self.current_user.name),
                              vec![vec!["Desligar".to_string()]]
                );
            } else if msg_text == TURN_ON_COMMAND && self.current_user.id != message.from.id {
                self.send_msg(message.chat.id(),
                              &format!("A máquina já está em uso por {}. Por favor, aguarde.", self.current_user.name),
                              vec![vec![TURN_ON_COMMAND.to_string()]]
                );
            } else if msg_text == TURN_OFF_COMMAND && self.current_user.id == message.from.id {
                self.current_user.clear_user();
                self.send_msg(message.chat.id(),
                              &format!("A máquina foi desligada e seus créditos não serão mais gastos"),
                              vec![vec![TURN_ON_COMMAND.to_string()]]
                );
            } else {
                self.send_msg(message.chat.id(),
                              &format!("Tente \"{}\" como comando.", TURN_ON_COMMAND),
                              vec![vec![TURN_ON_COMMAND.to_string()]]
                );
            }
    }

    pub fn start(&mut self){
        let mut updates_listener = self.api.listener(ListeningMethod::LongPoll(None));
        updates_listener.listen(|u| {
            let actual_message = u.message.unwrap();
            if CoffeezeraBot::is_older_than(&actual_message, 10) {
                return Ok(ListeningAction::Continue)
            }
            if !CoffeezeraBot::is_text_msg(&actual_message) {
                return Ok(ListeningAction::Continue)
            }
            self.handle_msg(&actual_message);
            Ok(ListeningAction::Continue)
        });
    }


}

impl Default for CoffeezeraUser {
    fn default() -> CoffeezeraUser {
        CoffeezeraUser {
            name: "".to_string(),
            id: -1,
            time_left: 0.0,
            chat_id: 0
        }
    }
}


fn main() {
    let mut coffezera = CoffeezeraBot::new();
    coffezera.start();
}




