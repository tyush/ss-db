# Errors and what used to be my biggest gripe with Actix-Web

In Actix, your "pages" usually return something that implements their Responder trait. Combined with Result, this makes it really easy and ergonomic to return diverse types on calls that may fall. However, failing is the issue.

Take this short example page:

```rust
async fn spaghetti(
    _req: HttpRequest,
    data: web::Data<AppState>
) -> actix_web::Result<impl Responder> {
    let template = &data.templates;
    let conn = &data.conn;

    let image = Image::find_by_id("spaghetti").one(conn).await?.with_context(|| "Could not find image!")?;

    let html = template.render("dbtest_index.html", &tera::Context::from_serialize(ImagePage { img_data: image.b64 })?)?;

    Ok(HttpResponse::Ok().content_type("text/html").body(html))
}
```

The example uses Tera to render a page template which acquires vital image data from a database with 2 types of errors that could happen (`Tera`/`Anyhow`), and uses the `?` operator to deal with all of them. This is great, right? Rust ergonomics allowing for easily falliable functions! However, when we try to compile...

```rust
error[E0277]: the trait bound `sea_orm::DbErr: ResponseError` is not satisfied
  --> src\paths\mod.rs:41:53
   |
41 |     let image = Image::find_by_id("spaghetti").one(conn).await?.with_context(|| "Could not find image!")?;
   |                                                     ^ the trait `ResponseError` is not implemented for `sea_orm::DbErr`
   |
   = note: required because of the requirements on the impl of `std::convert::From<sea_orm::DbErr>` for `actix_web::Error`
   = note: required because of the requirements on the impl of `FromResidual<Result<Infallible, sea_orm::DbErr>>` for `Result<_, actix_web::Error>`

error[E0277]: the trait bound `anyhow::Error: ResponseError` is not satisfied
  --> src\paths\mod.rs:41:95
   |
41 |     let image = Image::find_by_id(1).one(conn).await?.with_context(|| "Could not find image!")?;
   |                                                                                               ^ the trait `ResponseError` is not implemented for `anyhow::Error`
   |
   = note: required because of the requirements on the impl of `std::convert::From<anyhow::Error>` for `actix_web::Error`
   = note: required because of the requirements on the impl of `FromResidual<Result<Infallible, anyhow::Error>>` for `Result<_, actix_web::Error>`  

```

Oh boy. 

So what's happening here?

Through the magic of "reading what the compiler spits out," I have deduced that Rust doesn't know how to coerce a Tera/Anyhow error into an Actix error. 

Well, what now? We can't return an arbitrary error from our pages, so how do we deal with them now?

My first thought was to wrap our functions in another function that converts `Box<dyn Error>` into an Actix-friendly error. It would end up something like this:
```rust
#[get("/spaghetti")]
async fn spaghetti(
    req: HttpRequest,
    data: web::Data<AppState>
) -> impl Responder {
    match inner_spaghetti(req, data) {
        Ok(resp) => resp,
        Err(e) => HttpResponse::InternalServerError().body(e.to_string)
    }
}

async fn inner_spaghetti(
    _req: HttpRequest,
    data: web::Data<AppState>
) -> Result<HttpResponse, Box<dyn Error>> {
    let template = &data.templates;
    let conn = &data.conn;

    let image = Image::find_by_id("spaghetti").one(conn).await?.with_context(|| "Could not find image!")?;

    let html = template.render("dbtest_index.html", &tera::Context::from_serialize(ImagePage { img_data: image.b64 })?)?;

    Ok(HttpResponse::Ok().content_type("text/html").body(html))
}
```

Frankly, this worked much better than it should have. However, it would mean another ~10 lines on every page function we write and passing through every variable. 

As I was resigning myself to my new wrapped future, I looked a bit closer at the compiler error:
```rust
   = note: required because of the requirements on the impl of `std::convert::From<anyhow::Error>` for `actix_web::Error`
```

Wait a minute; Rust will coerce other errors to Actix errors!

Well, how does this help us?

 - Actix errors are anything that implement ResponseError
 - We can implement ResponseError on types we own

Oh, well that last qualifier is sad. We can only implement ResponseError on types that our crate has made. 

> Back to wrapping functions I guess

No, no, wait! What if we created a type that's an enum over all the errors we want to be able to return from our pages!

```rust
pub enum ErrToActix {
    Anyhow(anyhow::Error),
    Tera(tera::Error)
}
```

Then, we implement `From` on this type for all the types of error we want to care about

```rust
impl From<anyhow::Error> for ErrToActix {
    fn from(rhs: anyhow::Error) -> Self {
        Self::Anyhow(rhs)
    }
}

impl From<tera::Error> for ErrToActix {
    fn from(rhs: tera::Error) -> Self {
        Self::Tera(rhs)
    }
}
```

And now we just implement `ResponseError` (and `Display` by requirement) for our enum!

```rust
impl Display for ErrToActix {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Anyhow(e) => write!(f, "{}", e),
            Self::Tera(e) => write!(f, "{}", e)
        }
    }
}

impl actix_web::error::ResponseError for ErrToActix {
    fn status_code(&self) -> actix_web::http::StatusCode {
        match self {
            _ => actix_web::http::StatusCode::INTERNAL_SERVER_ERROR
        }
    }
}
```

Perfect! Now, we can just say `
async fn spaghetti(...) -> Result<HttpResponse, ErrToActix>` and Actix will happily use the `Display` implementation of whatever error we throw at it!