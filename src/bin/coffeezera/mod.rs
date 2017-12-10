extern crate coffeezerabot;
use self::coffeezerabot::*;
pub mod telegram_impl;
mod current_user_context;
mod telegram_handler;
mod telegram_interface;
use std::thread;
use self::telegram_handler::TelegramHandler;
use self::telegram_interface::TelegramInterface;
use self::current_user_context::CurrentUserContext;

pub struct CoffeezeraBot <T: TelegramInterface>{
    telegram_handler: TelegramHandler<T>,
    telegram_interface: T,
    context: Option<CurrentUserContext>
}

impl <T: TelegramInterface> CoffeezeraBot<T>{
    pub fn new() -> CoffeezeraBot<T>{
        info!("Creating new CoffeezeraBot");
        CoffeezeraBot{
            telegram_handler: TelegramHandler::new(),
            telegram_interface: T::new(),
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

    pub fn start(&mut self){
        let mut time_left_timeout: f64 = 0.0;
        self.telegram_interface.start_getting_updates();
        loop{
            if let Ok(updates) = self.telegram_interface.get_updates_channel().try_recv() {
                    for update in updates{
                        self.telegram_handler.handle_update(update, &mut self.context);
                    }
            }
            thread::sleep_ms(100);
            self.update_context_times();
            self.remove_user_if_appropriate();
        }
    }
}