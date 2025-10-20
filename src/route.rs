use std::{path::Iter, str::Split};

use crate::{Request, Response};

pub struct Route {
    pub(crate) name: Box<str>,
    pub(crate) children: Vec<Box<Route>>,
    pub(crate) handlers:
        Vec<Box<dyn Fn(Request) -> Result<Response, HandlerError> + Send + Sync + 'static>>,
}

impl std::fmt::Debug for Route {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Route")
            .field("name", &self.name)
            .field("children", &self.children)
            .finish()?;
        for _ in self.handlers.iter() {
            write!(f, "A handler!! ")?;
        }
        Ok(())
    }
}

impl Route {
    pub fn route(&self, req: Request, mut path_iter: Iter<'_>) -> Result<Response, HandlerError> {
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
        method: u8,
        mut path_iter: Split<'static, &'static str>,
        handle: impl Fn(Request) -> Result<Response, HandlerError> + Send + Sync + 'static,
    ) {
        match path_iter.next() {
            Some("") => {
                self.handlers.push(Box::new(handle));
            }
            Some("_") => {
                if self.children.is_empty() {
                    let mut new_route = Route {
                        name: "".to_string().into_boxed_str(),
                        children: Vec::new(),
                        handlers: Vec::new(),
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
                };
                new_route.insert(method, path_iter, handle);

                self.children.push(Box::new(new_route));
            }
            None => {
                panic!()
            }
        }
    }

    fn serve(&self, req: Request) -> Result<Response, HandlerError> {
        self.handlers.get(0).unwrap()(req)
    }
}

pub enum HandlerError {
    NotFound,
}
