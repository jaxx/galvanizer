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
use nickel::{Nickel, HttpRouter, StaticFilesHandler};
use config::reader as config_reader;
use config::types::Config;

use std::env;

fn main() {
    env_logger::init().unwrap();

    debug!("main: Galvanizer started.");

    let mut conf_file = env::current_dir().unwrap();
    conf_file.push("application.conf");

    let configuration = read_configuration_file(conf_file.as_path());

    let service_name = match configuration.lookup_str("application.service_name") {
        Some(name) => name,
        None => "galvanizer"
    };

    let daemon = Daemon {
        name: service_name.into()
    };

    daemon.run(move |rx: Receiver<State>| {
        debug!("main: Galvanizer service started.");

        for signal in rx.iter() {
            match signal {
                State::Start => {
                    debug!("main: Service -> start().");

                    let mut server = Nickel::new();

                    server.utilize(StaticFilesHandler::new("assets"));
                    server.utilize(StaticFilesHandler::new("assets/templates"));

                    server.get("**", middleware!("Hello world!"));
                    server.listen("127.0.0.1:3000");
                },
                State::Reload => debug!("main: Service -> restart()."),
                State::Stop => debug!("main: Service -> stop().")
            }
        }

        debug!("main: Galvanizer service finished.");
    }).unwrap();

    debug!("main: Galvanizer stopped.");
}

fn read_configuration_file(path: &Path) -> Config {
    let config = match config_reader::from_file(path) {
        Ok(c) => c,
        Err(e) => panic!("Can't read configuration file from '{}'. Error: {}.", path.display(), e.description()),
    };

    config
}