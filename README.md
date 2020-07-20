# language-random-quizer
A random Actix-rs/ Diesel-ORM/ BackEnd skill exercise by making a backend server with users guessing words and achieving points, which might show at the index page or something.

# Requirements 
* MySQL Server
* Rust (currently using 1.44.0-nightly, but actix-rs uses stable rust)
* Diesel-cli (download instructions can be found in https://diesel.rs/)

# Running
* Make a .env file with your MySQL information
* Run the diesel-cli migrations with: 
```diesel migration run```
* Start the local server by running: 
```cargo run```
