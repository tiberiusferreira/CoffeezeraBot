#[derive(Queryable, Clone)]
pub struct CoffeezeraUser {
    pub id: i32,
    pub name: String,
    pub telegram_id: i64,
    pub account_balance: f64,
}

use super::schema::coffeezera_users;

#[derive(Insertable)]
#[table_name="coffeezera_users"]
pub struct NewUser<'a> {
    pub name: &'a str,
    pub telegram_id: i64,
    pub account_balance: f64,
}