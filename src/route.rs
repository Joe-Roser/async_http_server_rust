use std::{fmt::Debug, path::Iter, str::Split};

use crate::{
    Request, Response,
    middleware::{PostMiddleware, PreMiddleware},
    response::ResponseResult,
};

pub struct Route {
    pub(crate) name: Box<str>,
    pub(crate) children: Vec<Box<Route>>,
    pub(crate) handlers: Vec<Handler>,
    pub(crate) pre_middleware: Vec<PreMiddleware>,
    pub(crate) post_middleware: Vec<PostMiddleware>,
}

impl Debug for Route {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Route")
            .field("name", &self.name)
            .field("children", &self.children)
            .field("handlers", &self.handlers)
            .field("pre_middleware", &self.pre_middleware.len())
            .field("post_middleware", &self.post_middleware.len())
            .finish()
    }
}
impl Route {
    pub fn route(&self, mut req: Request, mut path_iter: Iter<'_>) -> ResponseResult {
        for mw in &self.pre_middleware {
            req = mw.use_mw(req);
        }
        match path_iter.next() {
            None => self.serve(req),
            Some(p) => {
                for c in self.children.iter() {
                    if *c.name == *p.to_str().unwrap() {
                        return c.route(req, path_iter);
                    }
                }
                Err(HandlerError::NotFound)
            }
        }
    }

    pub fn insert(
        &mut self,
        method: u16,
        mut path_iter: Split<'static, &'static str>,
        handle: impl Fn(Request) -> ResponseResult + Send + Sync + 'static,
    ) {
        match path_iter.next() {
            Some("") | None => {
                if let None = self.handlers.iter().find(|h| h.method & method != 0) {
                    self.handlers.push(Handler {
                        handle: Box::new(handle),
                        method: method,
                    });
                } else {
                    panic!("Method collision on this path")
                }
            }
            Some("_") => {
                if self.children.is_empty() {
                    let mut new_route = Route {
                        name: "".to_string().into_boxed_str(),
                        children: Vec::new(),
                        handlers: Vec::new(),
                        pre_middleware: Vec::new(),
                        post_middleware: Vec::new(),
                    };
                    new_route.insert(method, path_iter, handle);
                    self.children.push(Box::new(new_route));
                } else {
                }
            }
            Some(p) => {
                for child in self.children.iter_mut() {
                    if *child.name == *p {
                        return child.insert(method, path_iter, handle);
                    }
                }

                let mut new_route = Route {
                    name: p.to_string().into_boxed_str(),
                    children: Vec::new(),
                    handlers: Vec::new(),
                    pre_middleware: Vec::new(),
                    post_middleware: Vec::new(),
                };
                new_route.insert(method, path_iter, handle);

                self.children.push(Box::new(new_route));
            }
        }
    }

    fn serve(&self, req: Request) -> ResponseResult {
        if let Some(h) = self
            .handlers
            .iter()
            .find(|h| h.method & req.method as u16 != 0)
        {
            (h.handle)(req)
        } else {
            Err(HandlerError::NotFound)
        }
    }
}

pub enum HandlerError {
    NotFound,
}

pub struct Handler {
    handle: Box<dyn Fn(Request) -> ResponseResult + Send + Sync + 'static>,
    method: u16,
}
impl Debug for Handler {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        const METHODS: [&str; 9] = [
            "HEAD", "GET", "POST", "PUT", "DELETE", "CONNECT", "OPTIONS", "TRACE", "PATCH",
        ];

        for (i, m) in METHODS.iter().enumerate() {
            if (1 << i as u16) & self.method != 0 {
                write!(f, "{}, ", m)?;
            }
        }

        Ok(())
    }
}
