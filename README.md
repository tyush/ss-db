# ss-db

SS-DB is a singlularized rewrite of the previous SuperScout platform. 

Frankly, the old one had the issue of numerous instances of "yeah this will probably be fine if i don't do this" that were kept unchecked and unproved. 

This rewrite aims to stay within one program, using Actix-Web to deliver and parse HTTP responses/requests, Tera to render pages on the server-side to prevent massive amounts of JSON data from reaching the client, and SeaORM to enforce synchronization between structs and the SQLite Database.

Pull requests and issues are more than welcome.
