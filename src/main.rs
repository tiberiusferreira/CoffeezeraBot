extern crate telegram_bot;
extern crate tokio_core;
extern crate futures;
use std::thread;
use telegram_bot::*;
use std::thread::sleep;
use std::time::{Duration, SystemTime};
use std::sync::mpsc::{Sender, Receiver};
use std::sync::mpsc;
extern crate time;


struct CoffeezeraUser {
    name: String,
    id: i64,
    time_left: f64,
    chat_id: i64,
}

impl CoffeezeraUser {

    fn default() -> CoffeezeraUser {
        CoffeezeraUser {
            name: "".to_string(),
            id: -1,
            time_left: 0.0,
            chat_id: 0,
        }
    }

    fn set_user<'a>(&mut self, id: i64, name: &'a str, chat_id: i64) {
        self.id = id;
        self.name = name.to_string();
        self.time_left = 10.0;
        self.chat_id = chat_id;
    }

    fn clear_user(&mut self) {
        self.id = -1;
        self.name = "".to_string();
        self.time_left = 0.0;
        self.chat_id = -1;
    }
}



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
        match self.api.send_message(
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
        ){
            Err(e) => {
                println!("Error sending message {:?}", e);
            },
            Ok(_) => ()
        }
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
        let (tx, rx): (Sender<Message>, Receiver<Message>) = mpsc::channel();
        let thread_tx = tx.clone();
        let thread_api = self.api.clone();
        thread::spawn(move || {
            let mut updates_listener = thread_api.listener(ListeningMethod::LongPoll(None));
            match updates_listener.listen(|u| {
                let actual_message = u.message.unwrap();
                if CoffeezeraBot::is_older_than(&actual_message, 30) {
                    return Ok(ListeningAction::Continue)
                }
                if !CoffeezeraBot::is_text_msg(&actual_message) {
                    return Ok(ListeningAction::Continue)
                }
                match thread_tx.send(actual_message) {
                    Err(e) => println!("{}", e),
                    Ok(_) => ()
                }
                Ok(ListeningAction::Continue)
            }) {
                Err(why) => println!("Error: {}", why),
                Ok(_) => ()
            }
        });
        let now = SystemTime::now();
        loop {
            match rx.try_recv() {
                Ok(message) => self.handle_msg(&message),
                _ => ()
            }
            let time_passed_ms: f64 = now.elapsed().unwrap().subsec_nanos() as f64/1000000000.0;
            if !self.current_user.id != -1 {
                if (self.current_user.time_left-time_passed_ms) > 0.0 {
                    self.current_user.time_left = self.current_user.time_left-time_passed_ms;
                }else {
                    self.current_user.time_left = 0.0;
                    self.current_user.clear_user();
                }
            }
            sleep(Duration::from_millis(500));
        }
    }
}


fn main() {
    let mut coffeezera = CoffeezeraBot::new();
    coffeezera.start();
}




