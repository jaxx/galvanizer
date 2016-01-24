#[macro_use]
extern crate log;
extern crate env_logger;
extern crate daemon;
#[macro_use]
extern crate nickel;
extern crate config;
extern crate mustache;

use std::collections::HashMap;
use std::sync::mpsc::Receiver;
use std::path::Path;
use std::error::Error;
use daemon::{Daemon, DaemonRunner, State};
use nickel::{Nickel, HttpRouter, StaticFilesHandler, Request, Response, MiddlewareResult};
use config::reader as config_reader;
use config::types::Config;
use mustache::Template;

use std::env;

fn render_to_string(template: &Template, data: &mut HashMap<&str, String>) -> String {
    let mut bytes = vec![];
    template.render(&mut bytes, data).unwrap();
    String::from_utf8(bytes).unwrap()
}

fn show_index<'a>(_: &mut Request, res: Response<'a>) -> MiddlewareResult<'a> {
    let template = mustache::compile_path(Path::new("app/assets/templates/index.html")).unwrap();
    let mut data = HashMap::<&str, String>::new();
    let value = render_to_string(&template, &mut data);
    data.insert("content", value);
    res.render("app/assets/templates/layout.html", &data)
}

fn main() {
    env_logger::init().unwrap();

    debug!("main: Catalyst started.");

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
        debug!("main: Catalyst service started.");

        for signal in rx.iter() {
            match signal {
                State::Start => {
                    debug!("main: Service -> start().");

                    let mut server = Nickel::new();
                    let mut router = Nickel::router();
                    
                    router.get("/", show_index);

                    server.utilize(router);
                    server.utilize(StaticFilesHandler::new("public/assets"));

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

fn read_configuration_file(path: &Path) -> Config {
    let config = match config_reader::from_file(path) {
        Ok(c) => c,
        Err(e) => panic!("Can't read configuration file from '{}'. Error: {}.", path.display(), e.description()),
    };

    config
}