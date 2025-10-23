use std::sync::Arc;

use crate::{Request, Response, Route, middleware, response::ResponseResult, route::HandlerError};

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
                pre_middleware: Vec::new(),
                post_middleware: Vec::new(),
            },
        }
    }

    pub fn get(
        mut self,
        path: &'static str,
        handle: impl Fn(Request) -> ResponseResult + Send + Sync + 'static,
    ) -> Self {
        let mut path_iter = path.split("/");
        if let Some("") = path_iter.next() {
        } else {
            panic!("Expected path of format '/...'")
        }

        self.index.insert(2, path_iter, handle);
        self
    }

    pub fn premiddleware(
        mut self,
        handle: impl Fn(Request) -> Request + Send + Sync + 'static,
    ) -> Self {
        self.index
            .pre_middleware
            .push(middleware::PreMiddleware::new(handle));
        self
    }

    pub fn postmiddleware(
        mut self,
        handle: impl Fn(ResponseResult) -> ResponseResult + Send + Sync + 'static,
    ) -> Self {
        self.index
            .post_middleware
            .push(middleware::PostMiddleware::new(handle));
        self
    }

    pub fn build(self) -> Result<Arc<Router>, ()> {
        Ok(Arc::new(Router { index: self.index }))
    }
}
