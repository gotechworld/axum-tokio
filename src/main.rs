// This is a simple example of an Axum web server that listens on port 3000 and responds to requests to the home page ("/") with a 200 OK status code.
use axum::http::StatusCode;
use axum::Router;
use axum::routing::get;
use tokio::net::TcpListener;

// Handles requests to the home page and returns a simple success status.
async fn index_handler() -> StatusCode {
    StatusCode::OK
}

#[tokio::main]
async fn main() {
    // Build the router and connect the "/" path to the handler above.
    let app:Router = Router::new()
        .route("/", get(index_handler));

    // Open the server on port 3000 so it can accept incoming requests.
    let listener:TcpListener =
        tokio::net::TcpListener::bind("0.0.0.0:3000")
            .await
            .unwrap();

    // Print a small message so we know the server started.
    println!("Listening...!");

    // Start serving requests with Axum.
    axum::serve(listener, app)
        .await
        .unwrap();

}
