extern crate coffeezerabot;
extern crate teleborg;
use self::coffeezerabot::*;
mod current_user_context;
mod telegram_replier;
mod grinder;
use std::{thread};
use self::telegram_replier::TelegramHandler;
use self::teleborg::TelegramInterface;
use self::current_user_context::CurrentUserContext;
use std;
extern crate time;

pub struct CoffeezeraBot <T: TelegramInterface>{
    telegram_handler: TelegramHandler<T>,
    context: Option<CurrentUserContext>,
    grinder: grinder::Grinder,
}

impl <T: TelegramInterface> CoffeezeraBot<T>{
    pub fn new() -> CoffeezeraBot<T>{
        info!("Creating new CoffeezeraBot");
        CoffeezeraBot{
            telegram_handler: TelegramHandler::new(),
            context: None,
            grinder: grinder::Grinder::new(),
        }
    }

    pub fn emergency_turn_off(){
        grinder::Grinder::new().turn_off();
    }

    fn should_remove_user(&mut self) -> bool{
        match self.context {
            Some(ref context) => {
                return context.should_be_removed || context.get_time_left_turn_off() == 0.0 || context.current_user.account_balance == 0.0
            },
            None => false
        }
    }

    fn update_context_times(&mut self){
        if let Some(ref mut some_context) = self.context {
            some_context.tick(self.grinder.is_grinding());
        }
    }

    fn remove_user_if_appropriate_and_sync_to_db(&mut self){
        if self.should_remove_user(){
            self.sync_to_db();
            info!("Removing user!");
            self.context.take();
        }
    }

    fn update_grinder_state(&self){
        if self.context.is_some(){
            self.grinder.turn_on();
        }else {
            self.grinder.turn_off();
        }
    }

    pub fn sync_to_db(&mut self){
        if let Some(ref mut context) = self.context {
            let beginning = time::now();
            update_user(&self.telegram_handler.database_connection,
                        context.current_user.id,
                        context.current_user.account_balance);
            context.last_db_sync_time = time::now();
            info!("Synced to DB, took {} ms", (time::now() - beginning).num_milliseconds());
        }else {
            error!("Tried to sync to DB without context!");
        }
    }

    fn update_database_with_current_context_if_needed(&mut self){
        let mut needs_to_sync = false;
        if let Some(ref mut context) = self.context {
            needs_to_sync = context.needs_to_sync_to_db();
        }
        if needs_to_sync {
            self.sync_to_db();
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
            self.update_database_with_current_context_if_needed();
            self.remove_user_if_appropriate_and_sync_to_db();
            self.update_grinder_state();
            if self.context.is_none(){
                thread::sleep(std::time::Duration::from_millis(500));
            }else {
                thread::sleep(std::time::Duration::from_millis(30));
            }
        }
    }
}