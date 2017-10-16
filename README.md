# Mock API - CLI

This project was inspired by Mockoon, which is a really cool project.
Check it out here: <https://mockoon.com>

However this project was a closed source electron app, so I thought it would
be cool to create a open source cli version.

### Building

Please note: This requires using rust nightly compiler.
If you are not using rust nightly run:
	> rustup default nightly

To build this project run:
	> cargo build

Then from /target/debug run:
	>./mockapi-cli ...

Run it from command line with:
mockapi-cli \[servername] \[options]

### Options:
-a/--action \[start/stop/restart/port] : action

-n : new server

-c \[post/get] --file\[file/text] --name\[filename] (only if file specified) \[name] : create a new request response for the mock-api

-d \[name] : delete a request response

-e \[name] --editor\[vi/nano] : edit a request response

-l \[post/get/all/name]: list endpoints

### Details:

start : starts the server specified

stop : stops the specified server

resart : restarts the specified server

port : this specifies the port to use (default is 3080)

post : specifies a post response

get : speciifes a get response

file : specifies the response should be a file

text : specifies the response should be text

name : this is the name of the endpoint

vi/nano : specify the editor to use

### Sample Usage:
To create a sever:

	mockapi-cli demo -n

Then add new data:

	mockapi-cli demo -c get demoData --f demo.json
	mockapi-cli demo -c get demoText

Edit the data:

	mockapi-cli demo -e demoText --editor vi

Then start the server:

	mockapi-cli demo -a start

Then it can be queried by:

	curl localhost:port/data

Or more generally through the endpoint:

 	localhost:port/[name]
