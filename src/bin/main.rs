mod coffeezera;
use coffeezera::telegram_adaptor::TelegramInterfaceImpl;
use coffeezera::CoffeezeraBot;
extern crate reqwest;
#[macro_use]
extern crate log;
extern crate flexi_logger;

use flexi_logger::Logger;
use flexi_logger::opt_format;

fn main() {
    Logger::with_str("info")
        .format(opt_format)
        .start()
        .unwrap_or_else(|e| panic!("Logger initialization failed with {}", e));
    let mut coffeezera: coffeezera::CoffeezeraBot<TelegramInterfaceImpl> = CoffeezeraBot::new();
    coffeezera.start();



}




