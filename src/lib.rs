#[macro_use] extern crate diesel_infer_schema;
pub mod schema;
pub mod models;

#[macro_use] extern crate diesel;
extern crate dotenv;
use diesel::prelude::*;
use diesel::pg::PgConnection;
use dotenv::dotenv;
use std::env;

pub use diesel::result;



pub fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url))
}

use self::models::{CoffeezeraUser, NewUser};



pub fn create_user<'a>(conn: &PgConnection, name: &'a str, telegram_id: i64, account_balance: f64) -> CoffeezeraUser {
    use schema::coffeezera_users;


    let new_user = NewUser {
        name,
        telegram_id,
        account_balance,
    };

    diesel::insert(&new_user)
        .into(coffeezera_users::table)
        .get_result(conn).expect(format!("Error creating user {:?}", new_user.name).as_str())
}

pub fn update_user<'a>(conn: &PgConnection, user_id: i32, input_account_balance: f64) {
    use self::schema::coffeezera_users::dsl::{coffeezera_users, account_balance};

    diesel::update(coffeezera_users.find(user_id)).set(account_balance.eq(input_account_balance))
        .get_result::<CoffeezeraUser>(conn)
        .expect(&format!("Could not find user with id {}", user_id));
}

pub fn get_user<'a>(conn: &PgConnection, input_telegram_id: i64) -> Result<CoffeezeraUser, diesel::result::Error> {
    use self::schema::coffeezera_users::dsl::{coffeezera_users, id, telegram_id};

    coffeezera_users.filter(telegram_id.eq(input_telegram_id))
        .get_result::<CoffeezeraUser>(conn)
}