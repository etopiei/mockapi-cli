#[macro_use]
extern crate clap;
extern crate iron;
extern crate chrono;
extern crate router;
extern crate rustc_serialize;
extern crate exec;

use clap::App;
use iron::prelude::*;
use iron::status;
use iron::mime::Mime;
use iron::{Response, Request, IronResult};
use chrono::prelude::*;
use router::Router;
use std::io::prelude::*;
use std::fs::File;
use std::io::{Write, BufWriter};
use std::fs::OpenOptions;
use std::fs;
use std::env;
use std::fs::metadata;
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

    let f = File::create(&path).expect("Unable to create file");
    let mut f = BufWriter::new(f);

    let parts = content.split("\n");
    for line in parts {
        let mut add_line = String::from(line);
        add_line.push('\n');
        f.write(add_line.as_bytes()).expect("Unable to write data");
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

fn set_port(servername: &String, port: &String) {
    //Open the server config and change the port number
    let pathname = env::var("HOME").unwrap() + "/mockapi-servers/" + servername + "/.server-config";
    let file_contents = read_file(&pathname);

    let split = file_contents.split("\n");
    let mut vec = split.collect::<Vec<&str>>();
    vec[0] = port;

    let mut new_string = String::new();
    for s in vec {
        new_string.push_str(s);
        new_string.push_str("\n");
    }

    write_string_to_file(&new_string, &pathname);
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

fn create_response(routename: &String, request_type: &String, response_type: &String, servername: &String) -> bool {
    
    let content_to_write = "\n".to_string() + routename + ":" + response_type + ":" + request_type;

    let path = env::var("HOME").unwrap() + "/mockapi-servers/" + servername + "/.server-config";
    let mut file = OpenOptions::new()
        .write(true)
        .append(true)
        .open(path)
        .unwrap();

    if let Err(e) = file.write_all(content_to_write.as_bytes()) {
        eprintln!("Failed to add new response: {}", e);
    }
    
    let pathname = env::var("HOME").unwrap() + "/mockapi-servers/" + servername + "/" + routename;
    let text = "[Dummy new response text]";
    write_string_to_file(&text.to_string(), &pathname);
    true
}

fn open_for_edit(editor: &String, route_name: &String, servername: &String) {
    let mut using_editor = String::new();
    let file_path = env::var("HOME").unwrap() + "/mockapi-servers/" + servername + "/" + route_name;
    if editor == "vi" || editor == "nano" || editor == "emacs" {

        using_editor.push_str(editor);
        let mut full_command = String::from(using_editor);
        full_command.push(' ');
        full_command.push_str(&file_path);

        let err = exec::Command::new("sh")
            .arg("-c")
            .arg(full_command)
            .exec();
        
        println!("Error: {}", err);

    } else {
        println!("Editor {} is not supported.", editor);
    }
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
        if ! s.contains(response) {
            new_contents.push_str(s);
            new_contents.push('\n');
        }
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

fn list_responses(_servername: &String, _response_type: &String) {
    //go through responses for given server in .server-config file
    //ensure it matches type requirements
}

fn list_servers() {
    //search for folders in the server directory
    println!("Servers:");

    let pathname = env::var("HOME").unwrap() + "/mockapi-servers/";
    let paths = fs::read_dir(pathname).unwrap();

    for path in paths {
        let current_path = path.unwrap().path().display().to_string();
        let md = metadata(&current_path).unwrap();
        if md.is_dir() {
            let v: Vec<&str> = current_path.split("/mockapi-servers/").collect();
            if v.len() > 1 {
                println!("{}", v[1]);
            }
        }
    }
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

    let yml = load_yaml!("app.yml");
    let matches = App::from_yaml(yml).get_matches();

    let mut servername = String::new();

    if let Some(matches) = matches.subcommand_matches("start") {
        if matches.is_present("servername") {
            servername.push_str(matches.value_of("servername").unwrap());
    
            if matches.is_present("port") {
                let port = matches.value_of("port").unwrap().to_string();
                set_port(&servername, &port);
            }

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
                println!("Serving at: localhost:{}", &port_number);
                Iron::new(router).http(host + &port_number).unwrap();
            } else {
                println!("No server data found. Ensure server exists and it has at least 1 response.");
            } 
        } else {
            panic!("Failed to set servername.");
        }
    } else if let Some(matches) = matches.subcommand_matches("delete") {
        if matches.is_present("servername") {
            servername.push_str(matches.value_of("servername").unwrap());
            let response = matches.value_of("route_name").unwrap();
            delete_response_from_server(&response.to_string(), &servername.to_string());
        } else {
            panic!("Failed to set servername");
        }
    } else if let Some(matches) = matches.subcommand_matches("create") {
        if matches.is_present("servername") {
            servername.push_str(matches.value_of("servername").unwrap());
            if create_server(&servername.to_string()) {
                println!("Server created succesfully.");
            } else {
                println!("Server creation failed.");
            }
        } else {
            panic!("Failed to set servername");
        }
    } else if let Some(matches) = matches.subcommand_matches("new") {

        if matches.is_present("servername") {
            servername.push_str(matches.value_of("servername").unwrap());

            let mut request_type = String::new();
            let mut response_type = String::new();

            if matches.is_present("TYPE") {
                request_type.push_str(matches.value_of("TYPE").unwrap());
            } else {
                request_type.push_str("GET");
            }

            if matches.is_present("RESPONSE_TYPE") {
                response_type.push_str(matches.value_of("RESPONSE_TYPE").unwrap());
            } else {
                response_type.push_str("text/plain");
            }

            if matches.is_present("routename") {
                let mut route_name = matches.value_of("routename").unwrap();
                while route_name.len() > 0 && &route_name[0..1] == "/" {
                    route_name = &route_name[1..];
                }
                if route_name.len() > 0 {
                    create_response(&route_name.to_string(), &request_type, &response_type, &servername.to_string());
                } else {
                    println!("Failed to get route name.");
                }
            } else {
                println!("Failed to get route name.");
            }
        } else {
            panic!("Failed to set servername");
        }

    } else if let Some(matches) = matches.subcommand_matches("edit") {
        if matches.is_present("servername") {
            servername.push_str(matches.value_of("servername").unwrap());
            let mut editor = String::new();

            if matches.is_present("editor") {
                editor.push_str(matches.value_of("editor").unwrap());
            } else {
                editor.push_str("nano");
            }

            let mut route_name;

            if matches.is_present("routename") {
                route_name = matches.value_of("routename").unwrap();
                //TODO: Check route exists
            } else {
                panic!("Failed to get routename");
            }

            open_for_edit(&editor, &route_name.to_string(), &servername.to_string())

        } else {
            panic!("Failed to set servername");
        }
    } else if let Some(matches) = matches.subcommand_matches("list") {

        let mut response_type = String::new();
        let mut servername;

        if matches.is_present("type") {
            response_type = matches.value_of("type").unwrap().to_string();
        }

        if matches.is_present("servername") {
            //list responses from the given server
            servername = matches.value_of("servername").unwrap().to_string();
            list_responses(&servername, &response_type);
        } else {
            //list all servers
            list_servers();
        }
    }
}
