mod routes;
mod handlers;
mod models;

#[tokio::main]
async fn main() {
    
    let app = routes::create_router();

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();

    println!("LOCAL server running at http//127.0.0.1:3000");

    axum::serve(listener, app)
        .await
        .unwrap();

}
