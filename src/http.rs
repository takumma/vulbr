use crate::url::ParsedUrl;
use dns_lookup::lookup_host;
use std::io::prelude::*;
use std::io::Read;
use std::net::TcpStream;
use std::string::String;
use std::vec::Vec;

/*
struct Header {
    key: String,
    value: String,
}

impl Header {
    fn new(key: String, value: String) -> Self {
        Self { key, value }
    }
}
*/

pub struct HttpClient {}

impl HttpClient {
    pub fn new() -> Self {
        Self {}
    }

    pub fn get(&self, url: &ParsedUrl) -> std::io::Result<HttpResponse> {
        let ips = lookup_host(&url.host)?.into_iter();
        let ipv4s: Vec<std::net::IpAddr> = ips.filter(|ip| ip.is_ipv4()).collect();

        let mut stream = TcpStream::connect((ipv4s[0], url.port))?;

        let mut request = String::from("GET /");
        request.push_str(&url.path);
        request.push_str(" HTTP/1.1\n");

        // headers
        request.push_str("Host: ");
        request.push_str(&url.host);
        request.push('\n');
        request.push_str("Accept: */*\n");
        request.push_str("Connection: close\n");

        request.push('\n');

        println!("request: {:?}", request);

        stream.write(request.as_bytes())?;

        let mut buf = String::new();
        stream.read_to_string(&mut buf)?;
        println!("---------------buf string----------------");
        println!("{}", buf);

        Ok(HttpResponse::new(buf))
    }

    // TODO: support correctly
    pub fn _post(&self, url: &ParsedUrl, _body: String) -> std::io::Result<HttpResponse> {
        let ips: Vec<std::net::IpAddr> = lookup_host(&url.host)?;

        let mut stream = TcpStream::connect((ips[0], url.port))?;

        let mut request = String::from("POST ");
        request.push_str(&url.path);
        request.push_str(" HTTP/1.1\n");

        /*
        // headers
        for h in &url.headers {
            request.push_str(&h.key);
            request.push_str(": ");
            request.push_str(&h.value);
            request.push('\n');
        }
        */

        request.push('\n');

        stream.write(request.as_bytes())?;

        let mut buf = String::new();
        stream.read_to_string(&mut buf)?;

        Ok(HttpResponse::new(buf))
    }
}

#[derive(Debug)]
pub struct HttpResponse {
    _version: String,
    status_code: u32,
    _reason: String,
    // TODO: replace String with Vec<Header>.
    _headers: Headers,
    body: String,
}

impl HttpResponse {
    pub fn new(raw_response: String) -> Self {
        let preprocessed_response = raw_response.replace("\n\r", "\n");

        let (status_line, remaining) = match preprocessed_response.split_once("\n") {
            Some((s, r)) => (s, r),
            None => panic!("http response doesn't have a new line"),
        };

        let (headers_str, body) = match remaining.split_once("\n\n") {
            Some((h, b)) => (h, b),
            None => ("", remaining),
        };

        let statuses: Vec<&str> = status_line.split(" ").collect();

        let mut headers = Headers::new();

        // split headers with indention.
        let preprocessed_response = headers_str.replace("\r", "");
        let header_string_vec = preprocessed_response.split("\n").collect::<Vec<&str>>();

        for h in header_string_vec {
            /*
            split a header with name & value
            (ex: "Location: example.com" -> ("Location", "example.com")
            */
            let header = h.split(": ").collect::<Vec<&str>>();
            headers.append(header[0].to_string(), header[1].to_string());
        }

        Self {
            _version: statuses[0].to_string(),
            status_code: match statuses[1].parse() {
                Ok(s) => s,
                Err(_) => 404,
            },
            _reason: statuses[2].to_string(),
            _headers: headers,
            body: body.to_string(),
        }
    }

    pub fn status_code(&self) -> u32 {
        self.status_code
    }

    pub fn headers(&self) -> Headers {
        self._headers.clone()
    }

    pub fn body(&self) -> String {
        self.body.clone()
    }
}

#[derive(Debug, Clone)]
pub struct Header {
    _name: String,
    _value: String,
}

impl Header {
    pub fn new(name: String, value: String) -> Self {
        Self {
            _name: name,
            _value: value,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Headers {
    _values: Vec<Header>,
}

impl Headers {
    pub fn new() -> Self {
        Self { _values: Vec::new() }
    }

    pub fn get(&self, name: String) -> String {
        let index = self
            ._values
            .iter()
            .position(|header| header._name == name)
            .unwrap();
        self._values[index]._value.to_string()
    }

    pub fn append(&mut self, name: String, value: String) {
        self._values.push(Header::new(name, value));
    }
}
