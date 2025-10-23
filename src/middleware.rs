use crate::{Request, response::ResponseResult};

pub struct PreMiddleware {
    handle: Box<dyn Fn(Request) -> Request + Send + Sync + 'static>,
}
impl PreMiddleware {
    pub fn new(handle: impl Fn(Request) -> Request + Send + Sync + 'static) -> Self {
        Self {
            handle: Box::new(handle),
        }
    }
    pub fn use_mw(&self, req: Request) -> Request {
        (self.handle)(req)
    }
}

pub struct PostMiddleware {
    handle: Box<dyn Fn(ResponseResult) -> ResponseResult + Send + Sync + 'static>,
}
impl PostMiddleware {
    pub fn new(handle: impl Fn(ResponseResult) -> ResponseResult + Send + Sync + 'static) -> Self {
        Self {
            handle: Box::new(handle),
        }
    }
    pub fn use_mw(&self, res: ResponseResult) -> ResponseResult {
        (self.handle)(res)
    }
}
