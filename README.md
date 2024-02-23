# Getting started 
## Clone the repository
Having git on the machine is neccesary 

Navigate a terminal to the desired directory. Then use the following command 
```
https://github.com/sumrdev/mini-x.git
```

## Installing rust 
 Follow the guide to installing the rust language 


[Rust installation guide](https://www.rust-lang.org/learn/get-started)

## Installing sqlite3
Follow the guide to installing sqlite3

[sqlite3 installation guide](https://www.sqlitetutorial.net/download-install-sqlite/)

## (Optional) Installing libsqlite3-dev
Follow the guide to installing libsqlite3-dev in case of build errors with cargo run

[libsqlite3 installation guide](https://ubuntu.pkgs.org/20.04/ubuntu-main-amd64/libsqlite3-dev_3.31.1-4_amd64.deb.html)

## Running the program 
First you need to build the project. This is done from the root foulder in the project running the build command.
```
cargo build
```

After it has successfully build and no errors occoured run the project.
```
cargo run --bin mini-x
```
In the case of of dependencies not being installed run 
```
cargo install --path ./
```
Then  in your bowser of choice connect to [localhost:5000](http://localhost:5000)

## Running the apicargo run --bin mini-x
```
cargo run --bin api
```

# Dependencies
- actix-files - version "0.6.5"
- actix-web - version "4"
- actix-session - version "0.9.0"
- askama - version "0.12.1"
- askama_actix - version "0.14.0"
- chrono - version "0.4.34"
- filters - version "0.4.0"
- rusqlite - version "0.30.0"
- actix-web-flash-messages - version  "0.4"
- serde - version "1.0.196"
- pwhash - version "1"
- actix-identity - version "0.7.0"
- md-5 - version "0.10.6"
- uuid - version "1.7.0"

# Resources 
## Frameworks 
- Actix web framework 

## Important libraries 
- Askama for rendering templates
- rusqlite for database handling
- pwhash to verify and create user secrets 