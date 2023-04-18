use std::collections::HashMap;

struct Route<T> 
{
    handler: fn(T) -> Vec<u8>
}


impl<T> Route<T> 
{
    fn new(handler: fn(T) -> Vec<u8>) -> Self {
        Route {
            handler
        }
    }

    fn handle(&self, data: T) -> Vec<u8> {
        (self.handler)(data)
    }

}

pub struct Router<T> 
{
    routes: HashMap<u32, Route<T>>
}

impl<T> Router<T> 
{
    pub fn new() -> Router<T> {
        Router {
            routes: HashMap::new(),
        }
    }

    pub fn add_route(&mut self, cmd: u32, handler: fn(T) -> Vec<u8>) -> &Self 
    {
        let route = Route::new(handler);
        self.routes.insert(cmd, route);
        self
    }

    pub fn route(&self, cmd: u32, data: T) -> Vec<u8> {
        match self.routes.get_key_value(&cmd) {
            Some((_, route)) => route.handle(data),
            _ => Vec::new()
        }        
    }
}


#[derive(Debug)]
struct Model1 {
    a: u8,
    b: u32
}

#[derive(Debug)]
struct Model2 {
    a: u64,
    b: String,
}

fn main() {
    let mut router = Router::new();
    router
        .add_route(10, a);
        // .add_route(20, b);

    let m1 = Model1{ a:1, b:2 };
    let m2 = Model2{ a:1, b:"hello".to_string() };
    router.route(10, m1);
    // router.route(20, m2);`

}


fn a(data: Model1) -> Vec<u8> {
    println!("{:?}", data);
    Vec::new()
}

fn b(data: Model2) -> Vec<u8> {
    println!("{:?}", data);
    Vec::new()
}

