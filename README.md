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

### Sample Usage:
To create a sever:

	mockapi-cli demo create

Then add new data:

	mockapi-cli demo new testResponse --type=GET
	mockapi-cli demo new testResponse2 --type=POST

Edit the data:

	mockapi-cli demo edit testResponse2 --editor=vi

Then start the server:

	mockapi-cli demo start --port=8000

Then it can be queried through the following:

 	localhost:port/[name]
