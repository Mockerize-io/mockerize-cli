use core::fmt;
use serde::{Deserialize, Serialize};

#[allow(clippy::upper_case_acronyms)]
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub enum Method {
    GET,
    DELETE,
    POST,
    PUT,
    HEAD,
    CONNECT,
    OPTIONS,
    TRACE,
    PATCH,
}

impl fmt::Display for Method {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Method::GET => write!(f, "GET"),
            Method::DELETE => write!(f, "DELETE"),
            Method::POST => write!(f, "POST"),
            Method::PUT => write!(f, "PUT"),
            Method::HEAD => write!(f, "HEAD"),
            Method::CONNECT => write!(f, "CONNECT"),
            Method::OPTIONS => write!(f, "OPTIONS"),
            Method::TRACE => write!(f, "TRACE"),
            Method::PATCH => write!(f, "PATCH"),
        }
    }
}

impl From<actix_web::http::Method> for Method {
    fn from(method: actix_web::http::Method) -> Self {
        match method {
            actix_web::http::Method::GET => Method::GET,
            actix_web::http::Method::DELETE => Method::DELETE,
            actix_web::http::Method::POST => Method::POST,
            actix_web::http::Method::PUT => Method::PUT,
            actix_web::http::Method::HEAD => Method::HEAD,
            actix_web::http::Method::CONNECT => Method::CONNECT,
            actix_web::http::Method::OPTIONS => Method::OPTIONS,
            actix_web::http::Method::TRACE => Method::TRACE,
            actix_web::http::Method::PATCH => Method::PATCH,
            _ => unimplemented!("Method not supported"),
        }
    }
}

impl From<Method> for actix_web::http::Method {
    fn from(method: Method) -> Self {
        match method {
            Method::GET => actix_web::http::Method::GET,
            Method::DELETE => actix_web::http::Method::DELETE,
            Method::POST => actix_web::http::Method::POST,
            Method::PUT => actix_web::http::Method::PUT,
            Method::HEAD => actix_web::http::Method::HEAD,
            Method::CONNECT => actix_web::http::Method::CONNECT,
            Method::OPTIONS => actix_web::http::Method::OPTIONS,
            Method::TRACE => actix_web::http::Method::TRACE,
            Method::PATCH => actix_web::http::Method::PATCH,
        }
    }
}
