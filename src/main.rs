#[macro_use]
extern crate log;
extern crate env_logger;
extern crate daemon;

#[macro_use]
extern crate nickel;

use daemon::{Daemon, DaemonRunner, State};
use std::sync::mpsc::Receiver;

use nickel::{Nickel, HttpRouter};

fn main() {
    env_logger::init().unwrap();

    debug!("main: Catalyst started.");

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