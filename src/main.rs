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
//use rustc_serialize::json;
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

fn read_file(pathname: &String) -> String {

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

    file_contents
}

fn write_string_to_file(content: &String, pathname: &String) {
    //write contents of file to path passed.
    let path = Path::new(&pathname);
    let display = path.display();

    let mut file = match File::create(&path) {
        Err(why) => panic!("Couldn't create {}: {}", display, why.description()),
        Ok(file) => file,
    };

    match file.write_all(content.as_bytes()) {
        Err(why) => panic!("couldn't write to {}: {}", display, why.description()),
        Ok(_) => println!("successfully wrote to {}", display),
    }
}

fn get_list_of_routes(server_name: &String) -> Vec<String> {
    let mut a = vec!["".to_string()];
    let pathname = env::var("HOME").unwrap() + "/mockapi-servers/" + server_name + "/.server-config";
    let file_contents = read_file(&pathname);

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

fn get_route_name(route_full: &String) -> String {
    let split = route_full.split(":");
    let vec = split.collect::<Vec<&str>>();
    vec[0].to_string()
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

fn get_port(servername: &String) -> String {
    //read server config file to get port number

    let pathname = env::var("HOME").unwrap() + "/mockapi-servers/" + servername + "/.server-config";
    let file_contents = read_file(&pathname);

    let split = file_contents.split("\n");
    let vec = split.collect::<Vec<&str>>();
    let port_number = vec[0];
    port_number.to_string()

}

fn get_query(query_url: &String) -> String {
    let split = query_url.split("/");
    let vec = split.collect::<Vec<&str>>();
    vec[3].to_string()
}

fn get_query_type(query: &String, servername: &String) -> String {
    //read file and return mime type of response
    let pathname = env::var("HOME").unwrap() + "/mockapi-servers/" + servername + "/.server-config";
    let file_contents = read_file(&pathname);

    let mut response_type = "";
    let split = file_contents.split("\n");
    let mut test = true;
    for s in split {
        if test {
            test = false;
        } else {
            if s.contains(query) {
                let details = s.split(":");
                let parts = details.collect::<Vec<&str>>();
                response_type = parts[1];
            }
        }
    }

    response_type.to_string()
}

fn delete_response_from_server(response: &String, servername: &String) -> bool {
    //delete file with response
    match fs::remove_file(env::var("HOME").unwrap() + "/mockapi-servers/" + servername + "/" + &response) {
            Err(why) => println!("Failed to delete response: {}. Because: {}", &response, why.description()),
            Ok(_) => println!("Succesfully deleted response")
    };

    //delete response from server file
    let pathname = env::var("HOME").unwrap() + "/mockapi-servers/" + servername + "/.server-config";
    let file_contents = read_file(&pathname);

    let mut new_contents = String::from("");
    let split = file_contents.split("\n");
    for s in split {
        new_contents.push_str(s);
        new_contents.push('\n');
    }

    write_string_to_file(&new_contents, &pathname);

    true
}

fn get_response_data(query: &String, servername: &String) -> String {
    //read from response file data as a string
    let pathname = env::var("HOME").unwrap() + "/mockapi-servers/" + servername + "/" + query;
    let file_contents = read_file(&pathname);

    file_contents
}

fn get_server_name() -> String {
    //Read servername from file
    let pathname = env::var("HOME").unwrap() + "/mockapi-servers/.current-server";
    let file_contents = read_file(&pathname);

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

        if query.capacity() > 0 {
            let content_type = get_query_type(&query, &servername).parse::<Mime>().unwrap();
            let out = get_response_data(&query, &servername);
            Ok(Response::with((content_type, status::Ok, out)))
        } else {
            let content_type = "text/plain".to_string().parse::<Mime>().unwrap();
            let out = "Testing Server";
            Ok(Response::with((content_type, status::Ok, out)))
        }  
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
                    let route_name = get_route_name(&route);
                    router.get("/".to_string() + &route_name, handle, route_name);
                }

                let port_number = get_port(&servername.to_string());
                let host = String::from("localhost:");
                Iron::new(router).http(host + &port_number).unwrap();

                println!("Serving at: localhost:{}", &port_number);

            } else {
                println!("No server data found. Ensure server exists and it has at least 1 response.");
            } 
        } else if matches.is_present("delete") {
            let response = matches.value_of("route_name").unwrap();
            delete_response_from_server(&response.to_string(), &servername.to_string());
        } else if matches.is_present("create") {
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