extern crate coffeezerabot;
extern crate teleborg;
use self::coffeezerabot::*;
mod current_user_context;
mod telegram_replier;
use std::{thread, time};
use self::telegram_replier::TelegramHandler;
use self::teleborg::TelegramInterface;
use self::current_user_context::CurrentUserContext;
use std::env;

pub struct CoffeezeraBot <T: TelegramInterface>{
    telegram_handler: TelegramHandler<T>,
    context: Option<CurrentUserContext>
}

impl <T: TelegramInterface> CoffeezeraBot<T>{
    pub fn new() -> CoffeezeraBot<T>{
        info!("Creating new CoffeezeraBot");
        let bot_token = env::var("TELEGRAM_BOT_TOKEN")
            .ok()
            .expect("Can't find TELEGRAM_BOT_TOKEN env variable")
            .parse::<String>()
            .unwrap();
        CoffeezeraBot{
            telegram_handler: TelegramHandler::new(),
            context: None
        }
    }

    fn should_remove_user(&mut self) -> bool{
        match self.context {
            Some(ref context) => {
                return context.get_time_left_turn_off() == 0.0 || context.current_user.account_balance == 0.0
            },
            None => false
        }
    }

    fn update_context_times(&mut self){
        if let Some(ref mut some_context) = self.context {
            some_context.tick();
        }
    }

    fn remove_user_if_appropriate(&mut self){
        if self.should_remove_user(){
            info!("Removing user!");
            self.context.take();
        }
    }

    fn update_database_with_current_context(&self){
        if let Some(ref context) = self.context {
            update_user(&self.telegram_handler.database_connection,
                        context.current_user.id,
                        context.current_user.account_balance);
        }
    }
    pub fn start(&mut self){
        self.telegram_handler.telegram_interface.start_getting_updates();
        loop{
            if let Ok(updates) = self.telegram_handler.telegram_interface.get_updates_channel().try_recv() {
                for update in updates{
                    self.telegram_handler.handle_update(update, &mut self.context);
                }
            }
            self.update_context_times();
            self.update_database_with_current_context();
            self.remove_user_if_appropriate();
            thread::sleep(time::Duration::from_millis(100));
        }
    }
}