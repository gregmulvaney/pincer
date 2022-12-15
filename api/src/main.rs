mod graphql;

use crate::graphql::{query::QueryRoot, GraphQLSchema};
use async_graphql::{http::GraphiQLSource, EmptyMutation, EmptySubscription, Schema};
use async_graphql_axum::{GraphQLRequest, GraphQLResponse, GraphQLSubscription};
use axum::{
    extract::State,
    response::{Html, IntoResponse},
    routing::get,
    Router, Server,
};
use migration::{Migrator, MigratorTrait};
use sea_orm::Database;
use std::io::Write;

// GraphIQL playground
#[axum::debug_handler]
async fn graphiql() -> impl IntoResponse {
    Html(
        GraphiQLSource::build()
            .endpoint("http://localhost:4000/graphql")
            .subscription_endpoint("ws://localhost:4000/ws")
            .finish(),
    )
}

// GraphQL Handler
#[axum::debug_handler]
async fn graphql_handler(schema: State<GraphQLSchema>, req: GraphQLRequest) -> GraphQLResponse {
    schema.execute(req.0).await.into()
}

fn dump_schema(schema: &GraphQLSchema) {
    let sdl = schema.sdl();
    let mut file = std::fs::File::create("../graphql/schema.gql").unwrap();
    file.write_all(sdl.as_bytes()).unwrap()
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();
    dotenvy::dotenv().ok();

    // TODO: Change to environment variable
    let addr = std::net::SocketAddr::from(([0, 0, 0, 0], 4000));

    let db = Database::connect(std::env::var("DATABASE_URL").unwrap())
        .await
        .unwrap();

    Migrator::up(&db, None).await.unwrap();

    let schema: GraphQLSchema = Schema::build(QueryRoot, EmptyMutation, EmptySubscription)
        .data(db)
        .finish();

    dump_schema(&schema);

    let app = Router::new()
        .route("/graphql", get(graphiql).post(graphql_handler))
        .route_service("/ws", GraphQLSubscription::new(schema.clone()))
        .with_state(schema);

    println!("Listening on {}", &addr);

    let server = Server::bind(&addr).serve(app.into_make_service());

    if let Err(e) = server.await {
        eprintln!("Server error: {e}")
    }
}
