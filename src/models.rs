#[derive(Queryable, Clone, AsChangeset)]
#[changeset_options(treat_none_as_null = "true")]
#[table_name = "coffeezera_users"]
pub struct CoffeezeraUser {
    pub id: i32,
    pub name: String,
    pub telegram_id: i64,
    pub account_balance: f64,
    pub picpay_username: Option<String>,
}

use super::schema::coffeezera_users;

#[derive(Insertable)]
#[table_name="coffeezera_users"]
pub struct NewUser<'a> {
    pub name: &'a str,
    pub telegram_id: i64,
    pub account_balance: f64,
    pub picpay_username: &'a str,
}