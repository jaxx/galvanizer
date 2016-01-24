#[macro_use]
extern crate log;
extern crate env_logger;
extern crate daemon;
#[macro_use]
extern crate nickel;
extern crate config;


use std::sync::mpsc::Receiver;
use std::path::Path;
use std::error::Error;
use daemon::{Daemon, DaemonRunner, State};
use nickel::{Nickel, HttpRouter};
use config::reader as config_reader;
use config::types::Config;

fn main() {
    env_logger::init().unwrap();

    debug!("main: Catalyst started.");

    let configuration = read_configuration_file("");

    let daemon = Daemon {
        name: "catalyst".into()
    };

    daemon.run(move |rx: Receiver<State>| {
        debug!("main: Catalyst service started.");

        for signal in rx.iter() {
            match signal {
                State::Start => {
                    debug!("main: Service -> start().");

                    let mut server = Nickel::new();

                    server.get("**", middleware!("Hello world!"));
                    server.listen("127.0.0.1:3000");
                },
                State::Reload => debug!("main: Service -> restart()."),
                State::Stop => debug!("main: Service -> stop().")
            }
        }

        debug!("main: Catalyst service finished.");
    }).unwrap();

    debug!("main: Catalyst stopped.");
}

fn read_configuration_file(path: &str) -> Config {
    let config = match config_reader::from_file(Path::new(path)) {
        Ok(c) => c,
        Err(e) => panic!("Can't read configuration file from '{}'. Error: {}.", path, e.description()),
    };
    config
}