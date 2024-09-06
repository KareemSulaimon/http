use std::fmt::{self, Display, Formatter};
use std::io;
use crate::http::request::{HttpRequest, Version}; // Import HttpRequest and Version

pub struct HttpResponse {
    pub version: Version,
    pub status: ResponseStatus,
    pub content_length: usize,
    pub accept_ranges: AcceptRanges,
    pub response_body: String,
    pub current_path: String,
    pub package_name: String,
    pub package_version: String,
}

impl HttpResponse {
    pub fn new(request: HttpRequest) -> io::Result<HttpResponse> {
        let version = Version::V2_0; // Or whatever logic you need to determine the version
        let mut status = ResponseStatus::NotFound;
        let mut content_length = 0;
        let mut accept_ranges = AcceptRanges::None;
        let current_path = request.resource.path.clone();
        let mut response_body = String::new();

        let server_path = std::env::current_dir()?;
        let resource = request.resource.path.clone();
        let new_path = server_path.join(resource);

        if new_path.exists() {
            if new_path.is_file() {
                let content = std::fs::read_to_string(new_path)?;
                response_body.push_str(&content);
                content_length = content.len();
                status = ResponseStatus::OK;
                accept_ranges = AcceptRanges::Bytes;
            }
        } else {
            response_body = format!(
                "<html>\
                <body>\
                <h1>NOT FOUND</h1>\
                </body>\
                </html>"
            );
            content_length = response_body.len();
        }

        let response_header = format!(
            "{} {}\r\n\
             Content-Length: {}\r\n\
             {}\r\n\
             X-Package-Name: {}\r\n\
             X-Package-Version: {}\r\n\r\n",
            version,
            status,
            content_length,
            accept_ranges,
            "simple-http",
            "0.1.0"
        );

        Ok(HttpResponse {
            version,
            status,
            content_length,
            accept_ranges,
            response_body: format!("{}{}", response_header, response_body),
            current_path,
            package_name: "simple-http".to_string(),
            package_version: "0.1.0".to_string(),
        })
    }
}

#[derive(Debug)]
pub enum ResponseStatus {
    OK = 200,
    NotFound = 404,
}

impl Display for ResponseStatus {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let msg = match self {
            ResponseStatus::OK => "200 OK",
            ResponseStatus::NotFound => "404 Not Found",
        };
        write!(f, "{}", msg)
    }
}

#[derive(Debug)]
pub enum AcceptRanges {
    Bytes,
    None,
}

impl Display for AcceptRanges {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let msg = match self {
            AcceptRanges::Bytes => "Accept-Ranges: bytes",
            AcceptRanges::None => "Accept-Ranges: none",
        };
        write!(f, "{}", msg)
    }
}

impl Display for HttpResponse {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{} {}\r\n\
             Content-Length: {}\r\n\
             {}\r\n\
             X-Package-Name: {}\r\n\
             X-Package-Version: {}\r\n\r\n\
             {}",
            self.version,
            self.status,
            self.content_length,
            self.accept_ranges,
            self.package_name,
            self.package_version,
            self.response_body
        )
    }
}
