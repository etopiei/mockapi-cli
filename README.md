# Mock API - CLI

![Build Status](https://travis-ci.org/etopiei/mockapi-cli.png)
https://travis-ci.org/etopiei/mockapi-cli

This project was inspired by Mockoon, which is a really cool project.
Check it out here: <https://mockoon.com>

However this project was a closed source electron app, so I thought it would
be cool to create a open source cli version.

This app is perfect for hackathons or quickly creating a local testing environment for frontend development. It allows you to thoroughly test that the code will integrate with the backend all from your local workspace.

### Building for Development

To build this project run:

	$ cargo build

Then from /target/debug run:

	$ ./mockapi ...

### Installing

To install clone the repo and run:

	$ chmod +x install.sh
	$ ./install.sh

Run it from command line with:
	$ mockapi \[subcommand] <servername>

Or for help run:

	$ mockapi --help

Or for help with a particular command run:

	$ mockapi [command] --help

### Sample Usage:
To create a sever:

	mockapi create demo

Then add new data:

	mockapi new --type=GET -r application/json demo testResponse
	mockapi new -t POST demo testResponse2

Edit the data:

	mockapi edit --editor=vi demo testResponse2 

Then start the server:

	mockapi start demo

Then it can be queried through the following:

 	localhost:port/[name]
