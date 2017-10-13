#Mock API - CLI

This project was inspired by Mockoon, which is a really cool project.
Check it out here: <https://mockoon.com>

However this project was a closed source electron app, so I thought it would
be cool to create a open source cli version.

Run it from command line with:

mockapi-cli --server \[servername] \[options]

###Options:
-a \[start/stop/restart/port] : action

-n : new server

-c \[post/get] --file\[file/text] --name\[filename] (only if file specified) \[name] : create a new request response for the mock-api

-d \[name] : delete a request response

-e \[name] --editor\[vi/nano] : edit a request response

-l \[post/get/all/name]: list endpoints

###Details:

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

###Sample Usage:
To create a sever:

	mockapi-cli --server demo -n

Then add new data:

	mockapi-cli --server demo -c get --file demo.json --name data
	mockapi-cli --server demo -e data --editor vi

Then start the server:

	mockapi-cli --server demo -a start

Then it can be queried by:

	curl localhost:port/data

Or more generally through the endpoint:

 	localhost:port/[name]