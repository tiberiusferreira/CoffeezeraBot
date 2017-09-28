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

struct CurrentUser {
    name: String,
    id: i64,
    time_left: f64,
    chat_id: i64
}

fn set_user<'a>(id: i64, name: &'a str, chat_id: i64, user: &'a mut RwLockWriteGuard<CurrentUser<>>){
    user.id = id;
    user.name = name.to_string();
    user.time_left = 10.0;
    user.chat_id = chat_id;
}

fn clear_user<'a>(user: &'a mut RwLockWriteGuard<CurrentUser<>>){
    user.id = -1;
    user.name = "".to_string();
    user.time_left = 0.0;
    user.chat_id = -1;
}



fn handle_text_msg<'a>(api: &Api, msg: &'a Message, current_user: Arc<RwLock<CurrentUser>>) {
    const NEW_USER_MSG: &'static str = "Todo uso da máquina será creditado a sua conta durante os \
                         próximos 5 minutos ou até você clicar o botão Desligar abaixo.";
    const YOU_ARE_ALREADY_USING_MSG: &'static str = "A máquina já está em uso por você, ";
    const TURN_ON_COMMAND: &'static str = "Ligar";
    const TURN_OFF_COMMAND: &'static str = "Desligar";
    const WRONG_MSG_TYPE_ERROR: &'static str = "Got message which was not text";
    let text_msg: &str = match msg.msg {
        MessageType::Text(ref text) => text.as_str(),
        _ => {
            println!("{}", WRONG_MSG_TYPE_ERROR);
            return;
        }
    };
    match current_user.write() {
        Ok(mut user) => {
            if text_msg == TURN_ON_COMMAND && user.id == -1 {
                send_msg(&api,
                         msg.chat.id(),
                         &NEW_USER_MSG,
                         vec![vec!["Desligar".to_string()]]);
                println!("{} {}", user.id, msg.from.id);
                set_user(msg.from.id, &msg.from.first_name.as_str(), msg.chat.id(), &mut user);
                println!("Current user name = {}", user.name);
                println!("User name = {}", msg.from.first_name.as_str());
            } else if text_msg == TURN_ON_COMMAND && user.id == msg.from.id {
                println!("{} {}", user.id, msg.from.id);
                send_msg(&api,
                         msg.chat.id(),
                         &format!("{}{}", &YOU_ARE_ALREADY_USING_MSG, user.name),
                         vec![vec!["Desligar".to_string()]]
                );
            } else if text_msg == TURN_ON_COMMAND && user.id != msg.from.id {
                send_msg(&api,
                         msg.chat.id(),
                         &format!("A máquina já está em uso por {}. Por favor, aguarde.", user.name),
                         vec![vec![TURN_ON_COMMAND.to_string()]]
                );
            } else if text_msg == TURN_OFF_COMMAND && user.id == msg.from.id {
                clear_user(&mut user);
                send_msg(&api,
                         msg.chat.id(),
                         &format!("A máquina foi desligada e seus créditos não serão mais gastos"),
                         vec![vec![TURN_ON_COMMAND.to_string()]]
                );
            } else {
                send_msg(&api,
                         msg.chat.id(),
                         &format!("Tente \"{}\" como comando.", TURN_ON_COMMAND),
                         vec![vec![TURN_ON_COMMAND.to_string()]]
                );
            }
        },
        Err(e) => {
            println!("Failed to get a lock: {}", e);
        }

    }
}

fn main() {
    let api = Api::from_token("477595004:AAGjB-rB5Mwkc75D5Q9E_2RqUBi2ioinCSk").unwrap();

    const TURN_ON_COMMAND: &'static str = "Ligar";

    let mut current_user_id: i64 = -1;
    let current_user_name = String::new();


    impl Default for CurrentUser {
        fn default() -> CurrentUser {
            CurrentUser {
                name: "".to_string(),
                id: -1,
                time_left: 0.0,
                chat_id: 0
            }
        }
    }

    let shared_user = Arc::new(
        RwLock::new(CurrentUser::default())
    );
    let shared_api = Arc::new(
        RwLock::new(api)
    );
    let mut listener = match shared_api.read() {
        Ok(api) => {
            api.listener(ListeningMethod::LongPoll(None))
        },
        Err(_) => {
            println!("Could not get Read lock for listener");
            panic!("Could not get Read lock for listener");
        }
    };


    let shared_user_child_clone = shared_user.clone();
    let shared_api_child_clone = shared_api.clone();
    thread::spawn(move|| {
        let mut start = Instant::now();;
        println!("Starting timer");
        loop{
            println!("Sleeping for 500 ms");
            sleep(Duration::from_millis(500));
            match shared_user_child_clone.write() {
                Ok(mut shared_user) => {
                    println!("Got a lock on shared_user");
                    // here we have exclusive access to the data

                    if shared_user.time_left > 0.0 {
                        println!("User still has {} seconds", shared_user.time_left);
                        shared_user.time_left -= start.elapsed().subsec_nanos() as f64/(1000000000.0) ;
                    }else if shared_user.id != -1 {
                        println!("User has not time left, trying to get API read lock");
                        match shared_api_child_clone.read() {
                            Ok(api) => {
                                println!("Got API lock");
                                send_msg(&api,
                                         shared_user.chat_id,
                                         &format!("A máquina foi desligada e seus créditos não serão mais gastos"),
                                         vec![vec![TURN_ON_COMMAND.to_string()]]
                                );
                                println!("Clearing user");
                                clear_user(&mut shared_user);
                            }
                            Err(_) => {
                                println!("Could not get API read lock");
                            }
                        }

                    }
                },
                Err(e) => {
                    println!("Failed to get a lock: {}", e);
                }
            };
            println!("Elapsed time = {}", start.elapsed().as_secs());
            start = Instant::now();
        }
    });



    listener.listen(|u| {

        match shared_api.read() {
            Ok(api) => {
                if let Some(m) = u.message {
                    if m.date < time::now().to_timespec().sec-10 {
                        println!("Time now: {} Message time: {}", time::now().to_timespec().sec, m.date);
                        println!("Message is more than 10s old. Discarding");
                        return Ok(ListeningAction::Continue);
                    }
                    match m.msg {
                        MessageType::Text(_) => handle_text_msg(&api, &m, shared_user.clone()),
                        _ => ()
                    }
                }
            },
            Err(_) => {
                println!("Could not get Read lock for listener");
                panic!("Could not get Read lock for listener");
            }
        }


        match shared_user.read() {
            Ok(data) => {
                println!("Name: {:?}", data.name);
                println!("Id: {:?}", data.id);
                println!("Time left: {:?}", data.time_left);

            },
            Err(e) => {
                println!("Failed to get a lock: {}", e);
            }
        };
        Ok(ListeningAction::Continue)
    });
}




