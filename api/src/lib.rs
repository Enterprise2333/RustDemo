use axum::{
    routing::get,
    Router,
};

#[tokio::main]
pub async fn start() {
    
    //获取port值
    let port = std::env::var("PORT").expect("PORT is not set in file .env");
    
    println!("端口为{}", port);

    // build our application with a single route
    let app = Router::new().route("/", get(|| async { "Hello, World!" }));

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind(&format!("0.0.0.0:{port}")).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}