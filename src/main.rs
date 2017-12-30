extern crate clap;
extern crate iron;
extern crate chrono;
extern crate router;
extern crate rustc_serialize;

use clap::{App, SubCommand};
use iron::prelude::*;
use iron::status;
use iron::mime::Mime;
use iron::{Response, Request, IronResult};
use chrono::prelude::*;
use router::Router;
use rustc_serialize::json;
use std::io::Write;
use std::io::prelude::*;
use std::fs::File;
use std::fs;
use std::env;
use std::error::Error;
use std::path::Path;

#[derive(RustcEncodable, RustcDecodable)]
struct JsonResponse {
    response: String
}

struct Server {
    name: String,
}

impl Server {
    pub fn new<'a>(server_name: &'a str) -> Server {
        Server { name: server_name.to_string()}
    }
}

fn handler(req: &mut Request) -> IronResult<Response> {

    let dt = Local::now();
    println!("Received request: {} at: {}",  req.url, dt.to_string());

    //TODO: 3. Take the URL and match it with response name, then return relevant data

    let response = JsonResponse { response: "Hello there, General Kenobi".to_string()};
    let out = json::encode(&response).unwrap();
    let content_type = "application/json".parse::<Mime>().unwrap();
    Ok(Response::with((content_type, status::Ok, out)))
}

fn get_list_of_routes(server_name: &str) -> Vec<String> {
    let mut a = vec!["".to_string()];

    let pathname = env::var("HOME").unwrap() + "/mockapi-servers/" + server_name + "/server.conf";
    let path = Path::new(&pathname);
    let display = path.display();

    let mut file = match File::open(&path) {
        Err(why) => panic!("Couldn't open {}: {}", display, why.description()),
        Ok(file) => file,
    };

    let mut file_contents = String::new();
    match file.read_to_string(&mut file_contents) {
        Err(why) => panic!("Couldn't read {}: {}", display, why.description()),
        Ok(_) => (),
    }

    let mut test = false; //this is to skip the port in the config file.
    let mut split = file_contents.split("\n");
    for s in split {
        if test == false {
            test = true;
        } else {
            a.push(s.to_string());
        }
    }

    return a;
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
        .subcommand(SubCommand::with_name("delete")
            .about("Deletes a route")
            .args_from_usage(
                "<route_name> 'Name of route to be deleted'"
            )
        )
        .subcommand(SubCommand::with_name("create")
            .about("Creates a new server")
        )
        .subcommand(SubCommand::with_name("new")
            .about("Creates a new route for a server")
            .args_from_usage(
                "-t [TYPE] --type=[TYPE] 'Sets the type, GET or POST (defualt is GET)'
                 <route_name> 'Route Name'"
            )
        )
        .subcommand(SubCommand::with_name("edit")
            .about("Edits a server response")
            .args_from_usage(
                "-e [EDITOR] --editor=[EDITOR] 'Sets the editor to edit the response (default is nano)'
                 <route_name> 'Name of route to edit"
            )
        )
        .get_matches();

        let servername = matches.value_of("servername").unwrap();
        let my_server = Server{ name: servername.to_string() };

        if matches.is_present("start") {

            let mut router = Router::new();
            let routes = get_list_of_routes(servername);

            for route in routes {
                router.get("/".to_string() + &route, handler, route);
            }

            println!("Starting server");
            //TODO: Get the port from the config file
            Iron::new(router).http("localhost:4848").unwrap();

        } else if matches.is_present("delete") {
            println!("Deleting Response");
        } else if matches.is_present("create") {

            println!("Creating server");

            match fs::create_dir_all(env::var("HOME").unwrap() + "/mockapi-servers/" + servername) {
                Err(why) => println!("Server directory already create: {}", why.description()),
                Ok(_) => println!("Directory created"),
            }

            let pathname = env::var("HOME").unwrap() + "/mockapi-servers/" + servername + "/server.conf";

            let path = Path::new(&pathname);
            let display = path.display();

            let mut file = match File::create(&path) {
                Err(why) => panic!("Couldn't create {}: {}", display, why.description()),
                Ok(file) => file,
            };

            match file.write_all("4848".as_bytes()) {
                Err(why) => panic!("couldn't write to {}: {}", display, why.description()),
                Ok(_) => println!("successfully wrote to {}", display),
            }

        } else if matches.is_present("new") {
            println!("New response")
        } else if matches.is_present("edit") {
            println!("Editing file");
        }
    
}
