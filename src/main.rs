extern crate docopt;

use docopt::Docopt;

const USAGE: &'static str = "
mockapi-cli.

Usage:
  mockapi-cli <servername> ...
  mockapi-cli <servername> [(-a | --action)] <action>
  mockapi-cli <servername> (-p | --port) <portNumber>
  mockapi-cli <servername> (-n | --new) [<portNumber>]
  mockapi-cli <servername> (-c | --create) <type> <name> [((-f | --file) <filename>)]
  mockapi-cli <servername> (-d | --delete) <name>
  mockapi-cli <servername> (-e | --edit) <name> [(--editor <editorName>)]
  mockapi-cli <servername> (-l | --list) <listType>
  mockapi-cli <servername> (-h | --help)


Options:
  -a --action   Do some action, start, stop, restart
  -p --port     Change port number of server
  -n --new      Create new server with name and optional portNumber
  -c --create   Create a new response with type post/get, name and filename (if responding with file)
  -d --delete   Delete an entry named with name
  -e --edit     Edit an entry with name given.
  -l --list     List entries for server
  -f --file     Specify filename, optional can respond with raw text
  --editor      Specify the editor to use
  -h --help     Show this screen

";

fn main() {
    let args = Docopt::new(USAGE)
                      .and_then(|dopt| dopt.parse())
                      .unwrap_or_else(|e| e.exit());

    // You can conveniently access values with `get_{bool,count,str,vec}`
    // functions. If the key doesn't exist (or if, e.g., you use `get_str` on
    // a switch), then a sensible default value is returned.
    let servername = args.get_str("<servername>");
    println!("{}", servername);
	println!("Valid arguments passed.");
    //println!("  Request Type: {}", args.get_str("<type>"));
    //println!("  Request Name: {}", args.get_str("<name>"));
}