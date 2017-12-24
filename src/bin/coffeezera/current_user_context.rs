extern crate time;
use super::coffeezerabot::models::CoffeezeraUser;

pub struct CurrentUserContext {
    pub current_user: CoffeezeraUser,
    pub current_user_chat_id: i64,
    pub current_user_message_id: i64,
    time_left_auto_turn_off: f64,
    last_update_time: time::Tm
}

impl CurrentUserContext {

    fn delta_time_ms(&self) -> f64{
        ((time::now() - self.last_update_time).num_milliseconds() as f64) / 1000.0
    }

    pub fn tick(&mut self){
        let delta = self.delta_time_ms();
        info!("Delta since last update = {}", delta);
        self.last_update_time = time::now();
        if self.time_left_auto_turn_off - delta > 0.0{
            self.time_left_auto_turn_off = self.time_left_auto_turn_off - delta;
        }else{
            self.time_left_auto_turn_off = 0.0;
        }

        if self.current_user.account_balance - delta > 0.0{
            self.current_user.account_balance = self.current_user.account_balance - delta;
        }else{
            self.current_user.account_balance = 0.0;
        }
    }

    pub fn get_time_left_turn_off(&self) -> f64 {
        self.time_left_auto_turn_off
    }

    pub fn new(db_user: CoffeezeraUser, chat_id: i64, message_id: i64) -> CurrentUserContext{
        CurrentUserContext{
            current_user: db_user,
            current_user_chat_id: chat_id,
            current_user_message_id: message_id,
            time_left_auto_turn_off: 60.0,
            last_update_time: time::now()
        }
    }
}

