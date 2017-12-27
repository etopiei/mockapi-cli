extern crate clap;
extern crate iron;
extern crate chrono;
extern crate router;
extern crate rustc_serialize;

use clap::{App, SubCommand};
use iron::prelude::*;
use iron::status;
use iron::mime::Mime;
use chrono::prelude::*;
use router::Router;
use rustc_serialize::json;

#[derive(RustcEncodable, RustcDecodable)]
struct JsonResponse {
    response: String
}

fn handler(_: &mut Request) -> IronResult<Response> {

    let dt = Local::now();
    println!("Receieved request at: {}", dt.to_string());

    let response = JsonResponse { response: "Hello there, General Kenobi".to_string()};
    let out = json::encode(&response).unwrap();
    let content_type = "application/json".parse::<Mime>().unwrap();
    Ok(Response::with((content_type, status::Ok, out)))
}

fn main() {
    let matches = App::new("mockapi")
        .version("0.1")
        .author("etopiei lj343@icloud.com")
        .about("Creates API for testing from command line")
        .args_from_usage(
            "-h, --help 'Show help message'
            <servername> 'Current server'"
        )
        .subcommand(SubCommand::with_name("start")
            .about("Start server")
            .arg_from_usage(
                "-p [PORT_NUMBER] --port=[PORT_NUMBER] 'Set the port number'"
            )
        )
        .subcommand(SubCommand::with_name("stop")
            .about("Stop server")
            .arg_from_usage(
                "-f 'Force server to stop'"
            )
        )
        .subcommand(SubCommand::with_name("restart")
            .about("Restarts server")
            .args_from_usage(
                "-p [PORT_NUMBER] --port=[PORT_NUMBER] 'Set the port number'"
            )
        )
        .subcommand(SubCommand::with_name("delete")
            .about("Deletes a response")
            .args_from_usage(
                "<response_name> 'Name of response to be deleted'"
            )
        )
        .subcommand(SubCommand::with_name("create")
            .about("Creates a new server")
        )
        .subcommand(SubCommand::with_name("new")
            .about("Creates a new response for a server")
            .args_from_usage(
                "-t [TYPE] --type=[TYPE] 'Sets the type, GET or POST (defualt is GET)'
                 <response_name> 'Response Name'"
            )
        )
        .subcommand(SubCommand::with_name("edit")
            .about("Edits a server response")
            .args_from_usage(
                "-e [EDITOR] --editor=[EDITOR] 'Sets the editor to edit the response (default is nano)'
                 <response_name> 'Name of response to edit"
            )
        )
        .get_matches();

        let servername = matches.value_of("servername").unwrap();
        println!("Using server: {}", servername);

        if matches.is_present("start") {

            let mut router = Router::new();
            router.get("/", handler, "index");

            //TODO: Here create more routes dynamically from server settings
            //Some sort of structure must hold the responses.

            println!("Starting server");
            Iron::new(router).http("localhost:4848").unwrap();

        } else if matches.is_present("stop") {
            println!("Stopping server");
        } else if matches.is_present("restart") {
            println!("Restarting server");
        } else if matches.is_present("delete") {
            println!("Deleting Response");
        } else if matches.is_present("create") {
            println!("Creating server");
        } else if matches.is_present("new") {
            println!("New response")
        } else if matches.is_present("edit") {
            println!("Editing file");
        }
    
}
