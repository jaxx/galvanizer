#[macro_use]
extern crate log;
extern crate env_logger;
#[macro_use]
extern crate nickel;
extern crate mustache;
extern crate galvanizer;


use std::env;
use std::path::Path;
use std::collections::HashMap;
use nickel::{Nickel, HttpRouter, StaticFilesHandler, Request, Response, MiddlewareResult};
use mustache::Template;

use galvanizer::configuration::Configuration;

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

    debug!("main: Galvanizer started.");

    let mut conf_file = env::current_dir().unwrap();
    conf_file.push("application.conf");

    let configuration = Configuration::open(conf_file.as_path());

    let host = configuration.lookup_str_or("application.host", "localhost");
    let port = configuration.lookup_integer32_or("application.port", 3000);

    let mut server = Nickel::new();
    let mut router = Nickel::router();

    router.get("/", show_index);

    server.utilize(router);
    server.utilize(StaticFilesHandler::new("public/assets"));

    server.listen(format!("{}:{}", host, port).as_str());

    debug!("main: Galvanizer stopped.");
}