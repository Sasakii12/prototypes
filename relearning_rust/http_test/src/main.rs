use http::{Request, Response};



fn send(req: Request<()>) -> http::Result<Response<()>> {
    
}

fn main() {
    let mut request = Request::builder().uri("https://pokeapi.co/api/v2/pokemon/ditto");
    println!("Hello, world!");
}

