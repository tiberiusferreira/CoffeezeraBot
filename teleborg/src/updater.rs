use std::{env, thread, time};
use std::sync::mpsc;
use std::sync::Arc;
use bot;
use objects::Update;
use std::time::Duration;
const BASE_URL: &'static str = "https://api.telegram.org/bot";

/// An `Updater` which will request updates from the API.
///
/// The `Updater` is the entry point of this library and will start threads
/// which will poll for updates and dispatch them to the handlers.
pub struct Updater {
    token: String,
}

