use crate::graphql::PincerSchema;
use async_graphql::{Request, Response};
use axum::{extract::State, Json};

mod graphql;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();
    dotenvy::dotenv().ok();

    let addr = std::net::SocketAddr::from(([0, 0, 0, 0], 3000));
}

async fn graphql_handler(schema: State<PincerSchema>, req: Json<Request>) -> Json<Response> {
    todo!()
}

async fn graphiql() {
    todo!()
}
