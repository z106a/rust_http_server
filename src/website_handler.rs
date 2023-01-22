use super::server::Handler;
use super::http::{Request, Response, StatusCode, Method};
use std::fs;

pub struct WebsiteHandler {
  public_path: String
}

impl WebsiteHandler {
    pub fn new(public_path: String) -> Self {
      Self { public_path }
    }

    fn read_file(&self, file_path: &str) -> Option<String> {
      let path = format!("{}/{}", self.public_path, file_path);
      println!("{}", path);
      fs::read_to_string(path).ok()
    }
}

impl Handler for WebsiteHandler {
    fn handle_request(&mut self, request: &Request) -> Response {
      match request.method() {
        Method::GET => match request.path() {
          "/" => Response::new(StatusCode::Ok, self.read_file("index.html")),
          "/test" => Response::new(StatusCode::Ok, Some("<h1>Test</h1>".to_string())),
          path => match self.read_file(path) {
            Some(contents) => Response::new(StatusCode::Ok, Some(contents)),
            None => Response::new(StatusCode::NotFound, None),
          },

        }
          _ => Response::new(StatusCode::NotFound, None),
      }
    }
}