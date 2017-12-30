mod coffeezera;
use coffeezera::CoffeezeraBot;
extern crate teleborg;
extern crate reqwest;
#[macro_use]
extern crate log;
extern crate flexi_logger;

use teleborg::Bot;
use flexi_logger::Logger;
use flexi_logger::opt_format;

fn main() {
    Logger::with_str("info")
        .format(opt_format)
        .start()
        .unwrap_or_else(|e| panic!("Logger initialization failed with {}", e));
    let mut coffeezera: coffeezera::CoffeezeraBot<Bot> = CoffeezeraBot::new();
    coffeezera.start();

}




