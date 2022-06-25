## archived
SS-DB was hastily made to be ready before the 2021-2022 FRC season.
With newfound time, a rewrite is to be made.

# ss-db

SS-DB is a singlularized rewrite of the previous SuperScout platform. 

Frankly, the old one had the issue of numerous instances of "yeah this will probably be fine if i don't do this" that were kept unchecked and unproved. 

This rewrite aims to stay within one program, using Actix-Web to deliver and parse HTTP responses/requests, Tera to render pages on the server-side to prevent massive amounts of JSON data from reaching the client, and SeaORM to enforce synchronization between structs and the SQLite Database.

## Running
Everything is able to be run directly through Cargo.
Configuration can be done through the `.env` file for through environment variables.
```bash
cargo run
```

Pull requests and issues are more than welcome.
