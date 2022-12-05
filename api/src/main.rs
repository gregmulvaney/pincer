use crate::graphql::{GraphQLSchema, QueryRoot};
use async_graphql::{
    http::{playground_source, GraphQLPlaygroundConfig},
    EmptyMutation, EmptySubscription, Request, Response, Schema,
};
use axum::{
    extract::State,
    response::{Html, IntoResponse},
    routing::get,
    Json, Router, Server,
};
use migration::{Migrator, MigratorTrait};
use sea_orm::Database;

mod graphql;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();
    dotenvy::dotenv().ok();
    let addr = std::net::SocketAddr::from(([0, 0, 0, 0], 4001));
    let db = Database::connect(std::env::var("DATABASE_URL").unwrap())
        .await
        .unwrap();
    Migrator::up(&db, None).await.unwrap();
    let schema = Schema::build(QueryRoot, EmptyMutation, EmptySubscription)
        .data(db)
        .finish();
    let app = Router::new()
        .route("/", get(graphiql).post(graphql_handler))
        .with_state(schema);
    let server = Server::bind(&addr).serve(app.into_make_service());

    if let Err(e) = server.await {
        eprintln!("Error starting server: {e}")
    }
}

async fn graphiql() -> impl IntoResponse {
    Html(playground_source(GraphQLPlaygroundConfig::new("/")))
}

async fn graphql_handler(schema: State<GraphQLSchema>, req: Json<Request>) -> Json<Response> {
    schema.execute(req.0).await.into()
}
