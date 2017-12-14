extern crate clap;
use clap::{Arg, App, SubCommand};

fn main() {
    let matches = App::new("mockapi")
        .version("0.1")
        .author("etopiei lj343@icloud.com")
        .about("Creates API for testing from command line")
        .args_from_usage(
            "-a, --action=[ACT] 'Specifies and action, start, stop or restart'
             -p, --port=[PORT_NUMBER] 'Sets the port of the server'
             -n, --new 'Creates new response for the server'
             -c, --create 'Makes new server'
             -d, --delete=[RESPONSE_NAME] 'Delete response'
             -e, --edit=[RESPONSE_NAME] 'Edits a response'
             -l, --list 'List response for given server'
             -f, --file 'Create file for new response'
             --editor 'Specify editor to change response'
             -h, --help 'Show help message'
             <servername> 'Sets the server to operate on'")
        .get_matches();
    
}
