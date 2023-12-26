use std::collections::HashMap;

pub struct SimpleRequest {
    pub cmd: u32,
    pub data: Vec<u8>,
}

type BoxedCallback = Box<dyn Fn(&SimpleRequest) -> Vec<u8>>;

pub struct Router {
    routes: HashMap<u32, BoxedCallback>,
}

impl Router {
    pub fn new() -> Router {
        Router {
            routes: HashMap::new(),
        }
    }

    pub fn add_route<C>(&mut self, cmd: u32, callback: C) -> &Self
    where
        C: Fn(&SimpleRequest) -> Vec<u8> + 'static,
    {
        self.routes.insert(cmd, Box::new(callback));
        self
    }

    pub fn route(&self, request: &SimpleRequest) -> Vec<u8> {
        match self.routes.get(&request.cmd) {
            Some(callback) => callback(request),
            _ => Vec::new(),
        }
    }
}

#[test]
fn test_router() {
    let mut router = Router::new();
    router.add_route(1002, |_| not_found_response());
    // router.add_route(1003, |req| get_gcd_response(req));
}

fn not_found_response() -> Vec<u8> {
    Vec::new()
}
