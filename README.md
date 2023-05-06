# synchro.rs

This is a package that can be used to synchronize the audio across all the devices connected to a LAN.

A custom protocol will be used to swnd the data over to the client systems by the server. This protocol is under testing and developmen.


## COMPLETED:

+ Hello world

+ Organized the codebase

## TODO:

+ Implement a TUI in the tui lib to serve a graphical interface for the user (Use tui-rs already imported)

+ Add support for sources other than synthically generated sin waves

+ Setup sinks

+ Make do proper error handling

+ Set up std::net (TCP) for both server and client

+ Make a trait for the protocol so it can be easily implemented by other structs

+ Serialize and Deserialize the protocol struct using serde.rs (already imported)

+ Figure out a way to get data from a source and add that corresponding type into the protocol struct

+ Document the code base


## Rules to follow

+ Fork the repo

+ Dont push src/main.rs without concent 

+ If any crates need to be added into the project ask me directly or make sure noone is working on that purticular lib
