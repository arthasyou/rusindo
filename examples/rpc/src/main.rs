use rusindo::network::grpc::route;
// use rusindo::network::grpc::route::Handle;

fn main() {
    let mut router = route::Router::new();
    router
        .add_route(20, b)
        .add_route(10, a);

    router.route(10, 30);
    router.route(20, "Hello".to_string());

    println!("Hello, world!");
}


fn a(data: u32) -> Vec<u8> {
    println!("{}", data);
    Vec::new()
}

fn b(data: String) -> Vec<u8> {
    println!("{}", data);
    Vec::new()
}


