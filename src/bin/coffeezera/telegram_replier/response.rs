use coffeezera::telegram_replier::grinder_action::GrinderAction;

pub struct Response {
    pub reply: String,
    pub reply_markup: Option<Vec<Vec<String>>>,
    pub action: GrinderAction
}
