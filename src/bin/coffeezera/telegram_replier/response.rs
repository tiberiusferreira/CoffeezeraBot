use coffeezera::telegram_replier::update_impact::UpdateImpact;

pub struct Response {
    pub reply: String,
    pub reply_markup: Option<Vec<Vec<String>>>,
    pub action: UpdateImpact
}
