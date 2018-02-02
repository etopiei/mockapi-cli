# Mock API - CLI

This project was inspired by Mockoon, which is a really cool project.
Check it out here: <https://mockoon.com>

However this project was a closed source electron app, so I thought it would
be cool to create a open source cli version.

This app is perfect for hackathons or quickly creating a local testing environment for frontend development. It allows you to thoroughly test that the code will integrate with the backend all from your local workspace.

### Building

To build this project run:

	$ cargo build

Then from /target/debug run:

	$ ./mockapi-cli ...

Run it from command line with:
	$ mockapi-cli <servername> \[subcommand]

Or for help run:

	$ mockapi-cli help

Or for help with a particular command run:

	$ mockapi-cli [command] --help

### Sample Usage:
To create a sever:

	mockapi-cli create demo

Then add new data:

	mockapi-cli new demo --type=GET -r application/json testResponse
	mockapi-cli new demo -t POST testResponse2

Edit the data:

	mockapi-cli edit demo testResponse2 --editor=vi

Then start the server:

	mockapi-cli start demo

Then it can be queried through the following:

 	localhost:port/[name]
