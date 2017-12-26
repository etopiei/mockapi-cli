extern crate clap;
use clap::{Arg, App, SubCommand};

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
                "-p --port=[PORT_NUMBER] 'Set the port number'"
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
                "-p --port=[PORT_NUMBER] 'Set the port number'"
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
                "-t --type 'Sets the type, GET or POST (defualt is GET)'
                 <response_name> 'Response Name'"
            )
        )
        .subcommand(SubCommand::with_name("edit")
            .about("Edits a server response")
            .args_from_usage(
                "--editor 'Sets the editor to edit the response (default is nano)'
                 <response_name> 'Name of response to edit"
            )
        )
        .get_matches();

        let servername = matches.value_of("servername").unwrap();
        println!("Using server: {}", servername);

        if matches.is_present("start") {
            println!("Starting server");
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
