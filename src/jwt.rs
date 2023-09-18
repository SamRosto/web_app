#![allow(warnings)]

use actix_web::dev::Payload;
use actix_web::{Error, FromRequest, HttpRequest};
use futures::future::{Ready, ok};

// Payload struct houses requests raw data stream
// FromRequest trait which is what we are going to implement to extract the data before it hits the view
// Ready, ok - used to wrap the result of our data extraction to create a future that is immediately ready with a success value, which is a value from our header extraction.

// Look for the `token` key and if it is there, we return the JwToken struct wth the message
pub struct JwToken {
    pub message: String,
}

impl FromRequest for JwToken {
    type Error = Error;
    type Future = Ready<Result<JwToken, Error>>;

    fn from_request(req: &HttpRequest, _ : &mut Payload) -> Self::Future {
        // can extract data from the header
        match req.headers().get("token") {
            Some(data) => {
                let token = JwToken {
                    message: data.to_str().unwrap().to_string(),
                };
                ok(token)
            },

            None => {
                let token = JwToken {
                    message: String::from("nothing found")
                };
                ok(token)
            }
        }
    }
}
