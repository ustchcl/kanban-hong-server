use crate::models::account::Account;
use actix_web::{
  delete, get, post, put, web, Error, FromRequest,
  HttpRequest, HttpResponse, Responder, dev::Payload
};
use actix_identity::Identity;
use futures::future::Ready;

pub type LoggedUser = Account;

impl FromRequest for LoggedUser {
  type Config = ();
  type Error = Error;
  type Future = Ready<Result<LoggedUser, Error>>;

  fn from_request(req: &HttpRequest, payload: &mut Payload) -> Self::Future {
    if let Ok(identity) = Identity::from_request(req, payload).into_inner() {
      if let Some(user_json) = identity.identity() {
        if let Ok(user) = serde_json::from_str(&user_json) {
          return ok(user);
        }
      }
    }
    err(ServiceError::Unauthorized.into())
  }
}