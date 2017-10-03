extern crate telegram_bot;
extern crate tokio_core;
extern crate futures;
extern crate diesel;
extern crate coffeezerabot;
use self::coffeezerabot::*;
use std::io::{stdin, Read};
use std::thread;
use telegram_bot::*;
use std::thread::sleep;
use std::time::{Duration, SystemTime};
use std::sync::mpsc::{Sender, Receiver};
use std::sync::mpsc;
use self::coffeezerabot::*;
use self::coffeezerabot::models::*;
use self::diesel::prelude::*;
extern crate time;









struct CoffeezeraBot {
    api: Api,
    current_user: Option<CoffeezeraUser>,
    current_user_chat_id: i64,
    is_grinding: bool,
    time_left_auto_turn_off: f64,
    database_connection: PgConnection
}

impl CoffeezeraBot {
    pub fn new() -> CoffeezeraBot {
        CoffeezeraBot{
            api: Api::from_token("477595004:AAGjB-rB5Mwkc75D5Q9E_2RqUBi2ioinCSk").unwrap(),
            current_user_chat_id: 0,
            current_user: None,
            is_grinding: false,
            time_left_auto_turn_off: 0.0,
            database_connection: establish_connection()
        }
    }

    fn clear_user(&mut self){
        self.send_msg(self.current_user_chat_id,
            "A máquina foi desligada e sua conta não será mais debitada",
                      vec![vec!["Ligar".to_string()]]
        );
        self.current_user = None;
        self.is_grinding = false;
        self.current_user_chat_id = 0;
        self.time_left_auto_turn_off = 0.0;
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
        const YOU_ARE_NOT_REGISTERED : &'static str = "Você não está registrado para uso do moedor. Peça para o Tibério registrar seu id: ";
        const TURN_ON_COMMAND: &'static str = "Ligar";
        const TURN_OFF_COMMAND: &'static str = "Desligar";
        let msg_text = CoffeezeraBot::get_msg_text(&message);
        if get_user(&self.database_connection, message.from.id).is_err() {
            self.send_msg(message.chat.id(),
                          format!("{} {}" ,&YOU_ARE_NOT_REGISTERED, message.from.id).as_str(),
                          vec![vec!["Ligar".to_string()]]);
            return;
        };
        if msg_text == TURN_ON_COMMAND && self.current_user.is_none() {
            self.send_msg(message.chat.id(),
                          &NEW_USER_MSG,
                          vec![vec!["Desligar".to_string()]]);
            self.current_user = Some(get_user(&self.database_connection, message.from.id).unwrap());
            self.current_user_chat_id = message.chat.id();
            self.time_left_auto_turn_off = 20.0;
            println!("Current user name = {}", self.current_user.as_ref().unwrap().name);
            println!("User name = {}", message.from.first_name.as_str());
        }
            else if msg_text == TURN_ON_COMMAND && self.current_user.as_ref().unwrap().telegram_id == message.from.id {
                self.send_msg(message.chat.id(),
                              &format!("{}{}", &YOU_ARE_ALREADY_USING_MSG, self.current_user.as_ref().unwrap().name),
                              vec![vec!["Desligar".to_string()]]
                );
            } else if msg_text == TURN_ON_COMMAND && self.current_user.as_ref().unwrap().telegram_id != message.from.id {
                self.send_msg(message.chat.id(),
                              &format!("A máquina já está em uso por {}. Por favor, aguarde.", self.current_user.as_ref().unwrap().name),
                              vec![vec![TURN_ON_COMMAND.to_string()]]
                );
            } else if msg_text == TURN_OFF_COMMAND && self.current_user.as_ref().unwrap().telegram_id == message.from.id {
                self.clear_user();
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
            match self.current_user {
                Some(_) => {
                    if (self.time_left_auto_turn_off - time_passed_ms) > 0.0 {
                        self.time_left_auto_turn_off = self.time_left_auto_turn_off - time_passed_ms;
                        println!("Time left until auto-off: {}", self.time_left_auto_turn_off);
                    } else {
                        self.clear_user();
                    }
                }
                None => ()
            }
            sleep(Duration::from_millis(500));
        }
    }
}


fn main() {
    let mut coffeezera = CoffeezeraBot::new();
    let connection = establish_connection();
    //    create_user(&connection, "teta2", 122, 1.22);
    //    update_user(&connection, 1, 10);

    coffeezera.start();
}




