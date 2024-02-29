
use actix_session::SessionExt;

use actix_web::dev;
use actix_web::error::ErrorBadRequest;
use actix_web::Error;
use actix_web::FromRequest;
use actix_web::HttpRequest;
use serde::Deserialize;
use futures::future::{ok, err, Ready};


#[derive(Debug, Deserialize, Default)]
pub struct FlashMessages {
    pub messages: Vec<String>,
}

impl FromRequest for FlashMessages {
    type Error = Error;
    type Future = Ready<Result<Self, Self::Error>>;

    fn from_request(req: &HttpRequest, _: &mut dev::Payload) -> Self::Future {
        let s = req.get_session();
        if let Ok(item) = s.get::<String>("_flash") {
            if let Some(message) = item {
                s.remove("_flash");
                println!("{}",message);
                ok(FlashMessages { messages:  message.split(",").map(|s| s.to_string()).collect()  })
            } else {
                err(ErrorBadRequest("no flash message"))
            }
        } else {
            err(ErrorBadRequest("no flash message"))
        }

    }
}