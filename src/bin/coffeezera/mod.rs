extern crate teleborg;
mod database;
mod current_user_context;
mod telegram_replier;
mod grinder;

use self::database::*;
use std::{thread};
use self::telegram_replier::TelegramHandler;
use self::teleborg::TelegramInterface;
use self::current_user_context::CurrentUserContext;
use std;
use std::time;
static IS_OPEN: bool = false;
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
                return context.should_be_removed || context.get_time_left_turn_off() == 0.0 ||
                    (context.current_user.account_balance == 0.0 && !IS_OPEN)
            },
            None => false
        }
    }

    fn update_context_times(&mut self){
        if let Some(ref mut some_context) = self.context {
            some_context.tick(self.grinder.is_grinding());
        }
    }

    fn remove_user_if_appropriate_and_sync_to_db_and_msg_him(&mut self){
        if self.should_remove_user(){
            self.sync_to_db();
            if let Some(context) = self.context.as_ref(){
                if context.current_user.account_balance == 0.0{
                    self.telegram_handler.send_no_credits_auto_turned_off_message(&context);
                }else if context.get_time_left_turn_off() == 0.0{
                    self.telegram_handler.send_auto_turned_off_message(&context);
                }
            }
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
            let beginning = time::Instant::now();
            update_user_balance(&self.telegram_handler.database_connection,
                                context.current_user.id,
                                context.current_user.account_balance);
            context.last_db_sync_time = time::Instant::now();
            if CurrentUserContext::elapse_as_f64_seconds(beginning)*1000.0 > 20.0{
                warn!("Synced to DB, took {} ms", CurrentUserContext::elapse_as_f64_seconds(beginning)*1000.0);
            }
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
        loop {
            if let Ok(updates) = self.telegram_handler.telegram_interface.get_updates_channel().try_recv() {
                for update in updates{
                    self.telegram_handler.handle_update(update, &mut self.context);
                }
            }
            self.update_context_times();
            self.update_database_with_current_context_if_needed();
            self.remove_user_if_appropriate_and_sync_to_db_and_msg_him();
            self.update_grinder_state();
            if self.context.is_none(){
                thread::sleep(std::time::Duration::from_millis(200));
            }else {
                thread::sleep(std::time::Duration::from_millis(1));
            }
        }
    }
}