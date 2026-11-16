use axum::{response::Response,extract::Request,middleware::{self, Next}};


pub async fn logger(req: Request, next: Next)-> Response{ 
    println!("-> Incoming: {} {}", req.method(), req.uri());
    
    // Pass control to the next middleware or handler
    let response = next.run(req).await;
    
    println!("<- Outgoing: status {}", response.status());
    response
}