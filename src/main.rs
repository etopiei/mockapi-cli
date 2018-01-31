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

fn get_list_of_routes(server_name: &String) -> Vec<String> {
    let mut a = vec!["".to_string()];

    let pathname = env::var("HOME").unwrap() + "/mockapi-servers/" + server_name + "/.server-config";
    let path = Path::new(&pathname);
    let display = path.display();

    if ! Path::new(&path).exists() {
        return a;
    }

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
    let split = file_contents.split("\n");
    for s in split {
        if test == false {
            test = true;
        } else {
            a.push(s.to_string());
        }
    }

    a
}

fn create_server(servername: &String) -> bool {

    match fs::create_dir_all(env::var("HOME").unwrap() + "/mockapi-servers/" + servername) {
        Err(why) => println!("Server directory already created: {}", why.description()),
        Ok(_) => println!("Directory created"),
    }

    let pathname = env::var("HOME").unwrap() + "/mockapi-servers/" + servername + "/.server-config";

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

    true

}

fn get_query(query_url: &String) -> String {
    let split = query_url.split("/");
    let vec = split.collect::<Vec<&str>>();
    vec[3].to_string()
}

fn get_server_name() -> String {
    //Read servername from file
    let pathname = env::var("HOME").unwrap() + "/mockapi-servers/.current-server";
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

    let servername = file_contents.to_string();
    servername
}

fn write_server_name(servername: &String) {

    let pathname = env::var("HOME").unwrap() + "/mockapi-servers/.current-server";

    let path = Path::new(&pathname);
    let display = path.display();

    let mut file = match File::create(&path) {
        Err(why) => panic!("Couldn't create {}: {}", display, why.description()),
            Ok(file) => file,
    };

    match file.write_all(servername.as_bytes()) {
        Err(why) => panic!("Couldn't write to {}: {}", display, why.description()),
        Ok(_) => println!("Updated current server: {}", display),
    }
}

fn handle(req: &mut Request) -> IronResult<Response> {
        let dt = Local::now();
        println!("Received request: {} at: {}",  req.url, dt.to_string());

        let servername = get_server_name();
        let query = get_query(&req.url.to_string());

        //println!("Servername is: {} and query is: {}", servername, query);
        
        //TODO: Go to servername folder and find query response
        //also get query type from .server-config

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
                 -r [RESPONSE_TYPE] 'Sets the type of response, default is text/plain, application/json, text/csv and application/xml are also supported.'
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

        if matches.is_present("start") {

            write_server_name(&servername.to_string());

            let mut router = Router::new();
            let routes = get_list_of_routes(&servername.to_string());

            if routes.len() > 1 {

                for route in routes {
                    router.get("/".to_string() + &route, handle, route);
                }

                println!("Starting server");
                //TODO: Get the port from the config file
                Iron::new(router).http("localhost:4848").unwrap();

            } else {
                println!("No server data found. Ensure server exists and it has at least 1 response.");
            } 

        } else if matches.is_present("delete") {
            println!("Deleting Response");
        } else if matches.is_present("create") {

            println!("Creating server");
            if create_server(&servername.to_string()) {
                println!("Server created succesfully.");
            } else {
                println!("Server creation failed.");
            }

        } else if matches.is_present("new") {
            println!("New response")
        } else if matches.is_present("edit") {
            println!("Editing file");
        }

}