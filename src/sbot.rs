use std::process::Command;
use std::process::Child;
use std::error::Error;


pub fn new_sbot_server(network_name: &str) -> Option<Child> {
    let sbot_server =
        Command::new("sbot")
            .env("ssb_appname", network_name)
            .arg("server")
            .spawn()
            .expect("sbot failed to start");

    Some(sbot_server)
}