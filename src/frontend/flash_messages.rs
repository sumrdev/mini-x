use actix_session::Session;
use actix_session::SessionExt;

use actix_web::dev;
use actix_web::error::ErrorBadRequest;
use actix_web::Error;
use actix_web::FromRequest;
use actix_web::HttpRequest;
use futures::future::{err, ok, Ready};
use serde::Deserialize;

#[derive(Debug, Deserialize, Default)]
pub struct FlashMessages {
    pub messages: Vec<String>,
}

impl FromRequest for FlashMessages {
    type Error = Error;
    type Future = Ready<Result<Self, Self::Error>>;

    fn from_request(req: &HttpRequest, _: &mut dev::Payload) -> Self::Future {
        let s = req.get_session();
        if let Ok(Some(message)) = s.get::<String>("_flash") {
            s.remove("_flash");
            ok(FlashMessages {
                messages: message.split(',').map(|s| s.to_string()).collect(),
            })
        } else {
            err(ErrorBadRequest("no flash message"))
        }
    }
}

pub fn add_flash(session: Session, message: &str) {
    if let Ok(Some(messages)) = session.get::<String>("_flash") {
        let new_message = format!("{},{}", messages, message);
        session.insert("_flash", new_message).unwrap();
    } else {
        session.insert("_flash", message).unwrap();
    }
}
