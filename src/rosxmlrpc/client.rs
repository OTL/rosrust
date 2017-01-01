use hyper;
use rustc_serialize::{Encodable, Decodable};
use super::error::Error;
use super::serde;

pub struct Client {
    http_client: hyper::Client,
    server_uri: String,
}

impl Client {
    pub fn new(server_uri: &str) -> Client {
        Client {
            http_client: hyper::Client::new(),
            server_uri: String::from(server_uri),
        }
    }

    pub fn request_tree(&self, request: Request) -> ClientResult<serde::XmlRpcValue> {
        let mut body = Vec::<u8>::new();
        request.encoder.write_request(&request.name, &mut body)?;

        let body = String::from_utf8(body)?;
        let res = self.http_client
            .post(&self.server_uri)
            .body(&body)
            .send()?;

        let mut res = serde::Decoder::new_response(res)?;

        Ok(res.pop()
            .ok_or(Error::Serde(serde::ErrorKind::Decoding("request tree".into()).into()))?
            .value())
    }

    pub fn request<T: Decodable>(&self, request: Request) -> ClientResult<T> {
        let mut body = Vec::<u8>::new();
        request.encoder.write_request(&request.name, &mut body)?;

        let body = String::from_utf8(body)?;
        let res = self.http_client
            .post(&self.server_uri)
            .body(&body)
            .send()?;

        let mut res = serde::Decoder::new_response(res)?;

        Ok(T::decode(&mut res.pop()
            .ok_or(Error::Serde(serde::ErrorKind::Decoding("request".into()).into()))?)?)
    }
}

pub struct Request {
    name: String,
    encoder: serde::Encoder,
}

impl Request {
    pub fn new(function_name: &str) -> Request {
        Request {
            name: String::from(function_name),
            encoder: serde::Encoder::new(),
        }
    }

    pub fn add<T: Encodable>(&mut self, parameter: &T) -> Result<(), serde::Error> {
        parameter.encode(&mut self.encoder)
    }
}

pub type ClientResult<T> = Result<T, Error>;
