use std::sync::Arc;

use crate::{Request, Response, Route, route::HandlerError};

#[derive(Debug)]
pub struct Router {
    index: Route,
}

impl Router {
    pub fn builder() -> RouterBuilder {
        RouterBuilder::new()
    }

    pub fn route(&self, req: Request) -> Response {
        let path = req.get_path();
        let mut path_iter = path.iter();
        let _ = path_iter.next();
        match self.index.route(req, path_iter) {
            Ok(r) => r,
            Err(_e) => Response::not_found(),
        }
    }
}

pub struct RouterBuilder {
    index: Route,
}

impl RouterBuilder {
    pub fn new() -> Self {
        RouterBuilder {
            index: Route {
                name: "".to_string().into_boxed_str(),
                children: Vec::new(),
                handlers: Vec::new(),
            },
        }
    }

    pub fn get(
        mut self,
        path: &'static str,
        handle: impl Fn(Request) -> Result<Response, HandlerError> + Send + Sync + 'static,
    ) -> Self {
        self.index.insert(0b00000001, path.split("/"), handle);
        self
    }

    pub fn build(self) -> Result<Arc<Router>, ()> {
        Ok(Arc::new(Router { index: self.index }))
    }
}
