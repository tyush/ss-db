# The `POST` Post

In HTTP-request land, there exists multiple different methods based on the CRUD (`CREATE`, `REPLACE`, `UPDATE`, `DELETE`) idea of working with a Database. The HTTP methods are as follows:

| Method   | Description | Limitations | Expected Use Case
| :------: | ----------- | ----------- | - 
| `GET`    | Get data from a server, usually with in-URL parameters | Maximum 2048 character URL limit | Getting data from an API, making SQL queries, stuff that can be safely cached
| `HEAD`   | Only send metadata for a response | Doesn't send data | Checking size of a file before downloading, making sure a server is alive (responds `200 OK`)
| `POST`   | Send data to a server, with the expectation that the server will commit it to something | Usually limited to around 128KB | Giving data to an API such as user preferences or details of an action
| `PUT`    | Send data to a server, with a location defined by the client | Client-controlled location | Uploading a file to a cloud file storage server
| `DELETE` | Ask a server to remove data | Can be refused | Deleting a file to a cloud-based file storage server
| `CONNECT`| Establish a secure TCP/IP tunnel | Falliable | Upgrading an `HTTP` connection to an `HTTPS` connection, establishing an uplink for a realtime game
| `OPTIONS`| Ask a server for what HTTP methods it supports |  | Checking to ensure a user-provided URL supports certain methods before connecting
| `TRACE`  | Ask the server to respond with the request the client sent | Limited size | Check to see if any middlemen servers are adding/removing data
| `PATCH`  | Ask server to modify state | See `PUT` | Updating only part of a file to save bandwidth, ala git

 > That's... that's a lot of methods. 

Yep. On the bright side, we only care about 3:
`GET`, `HEAD`, and `POST`. Even the specification itself only requires a web server to implement `GET` and `HEAD`, but we'll use `POST` for something down the line.

There's a lot of other distinctions I didn't list between `GET` and `POST`, but for the most part, think of them like this:
 - `GET` implies the client sends parameters and expects a response from the server based on those parameters. If you've ever done a Google Search and wondered why the URL was so long, that's `GET`.
 - `POST` implies the client sends large data and expects a _status_ from the server, detailing whether the server was successful in processing/saving the data.

 > Why does this matter? We just care about forms, and forms can use any method with the `method=` parameter.

The issue is the limitations of each method.

By default, a form is `GET`. 
`GET` only allows 2048 characters. Sure, that sounds like a lot, but think about it this way: a scouter decides to go all AP Lang essay on us and writes a paragraph detailing every aspect of a team's robot. What is our server to do? If we fail, the scouter is confused.

So we move to `POST`. 

For us, `POST` has no disadvantages that we care about. 
 > So we can slap it on everything, right?

Unfortunately we can't. We use the Actix-Web Flash Messages middleware in order to handle alerts. The Alerts are only sent with the next, and only next request. So when we attach a `saving successful!` alert to our next request following a form submission, it ends up being consumed by the `POST` on the `TemporaryRedirect` we send to the client to get them to go back to the form.
Instead, what we have to do is make submitting the form a `POST`, and getting the form both `POST` and `GET`.

Previously, we've been defining paths using Actix-web's proc-macros, which look a little like this:

```rust
#[post("/the/path/and/{data}/we/want")]
async fn function_that_renders_page(
    ...
) -> WebResult {
    ...
```

However, this only allows us to choose one method at a time. In order to choose two, we have to give it a route at runtime, which we can do by not defining a method on a runtime-created route in our configuration function.
```rust
pub fn add_paths(
    conf: &mut ServiceConfig
) {
    conf.service(index)
        ...
        .route("/form/2022/pit",  web::to(forms2022::pit))
}
```

Now, both `GET` and `POST` requests are handled by the same function, that renders our Pit form.