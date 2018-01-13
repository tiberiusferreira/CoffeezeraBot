extern crate sysfs_gpio;
use self::sysfs_gpio::{Pin, Direction};
const RELAY_PIN_NUMBER: u64 = 26;
const SHUNT_PIN_NUMBER: u64 = 21;
use std::thread::sleep;
use std::time::Duration;
pub struct Grinder {
    relay: Pin,
    shunt: Pin
}

impl Grinder {
    pub fn new() -> Self{
        let relay = Pin::new(RELAY_PIN_NUMBER);
        relay.export().expect(&format!("Could not export pin {} to user space.", RELAY_PIN_NUMBER));
        sleep(Duration::from_millis(500));
        relay.set_direction(Direction::Out).expect(&format!("Could not set pin {} direction to Out", RELAY_PIN_NUMBER));
        let shunt = Pin::new(SHUNT_PIN_NUMBER);
        shunt.export().expect(&format!("Could not export pin {} to user space.", SHUNT_PIN_NUMBER));
        sleep(Duration::from_millis(500));
        shunt.set_direction(Direction::In).expect(&format!("Could not set pin {} direction to In", SHUNT_PIN_NUMBER));
        Grinder {
            relay,
            shunt
        }
    }

    pub fn turn_on(&self){
        self.relay.set_value(1).expect(&format!("Could not set RELAY_PIN_NUMBER {} to 1", RELAY_PIN_NUMBER));
    }
    pub fn turn_off(&self){
        self.relay.set_value(0).expect(&format!("Could not set RELAY_PIN_NUMBER {} to 0", RELAY_PIN_NUMBER));
    }
    pub fn is_grinding(&self) -> bool{
        let value = self.shunt.get_value().expect(&format!("Could not get value of SHUNT_PIN_NUMBER {}", SHUNT_PIN_NUMBER));
        return value == 1;
    }
}
