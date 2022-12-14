use crate::graphql::{
    mutation::MutationRoot, query::QueryRoot, subscription::SubscriptionRoot, GraphQLSchema,
};
use async_graphql::{
    http::{playground_source, GraphQLPlaygroundConfig, GraphiQLSource},
    Schema,
};
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
use tokio::sync::mpsc;
use tower_http::cors::CorsLayer;

mod graphql;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();
    dotenvy::dotenv().ok();

    let addr = std::net::SocketAddr::from(([0, 0, 0, 0], 4000));

    let db = Database::connect(std::env::var("DATABASE_URL").unwrap())
        .await
        .unwrap();

    Migrator::up(&db, None).await.unwrap();

    let schema = Schema::build(QueryRoot, MutationRoot, SubscriptionRoot)
        .data(db)
        .finish();

    // Dump SDL to a file for IDE compatibility
    dump_sdl(&schema);

    let app = Router::new()
        .route("/", get(graphiql).post(graphql_handler))
        .route_service("/ws", GraphQLSubscription::new(schema.clone()))
        .layer(CorsLayer::permissive())
        .with_state(schema);

    println!("Listening on {}", &addr);

    let server = Server::bind(&addr).serve(app.into_make_service());

    if let Err(e) = server.await {
        eprintln!("Error starting server: {e}")
    }
}

async fn graphiql() -> impl IntoResponse {
    Html(
        GraphiQLSource::build()
            .endpoint("http://localhost:4000")
            .subscription_endpoint("ws://localhost:4000/ws")
            .finish(),
    )
}

async fn graphql_handler(schema: State<GraphQLSchema>, req: GraphQLRequest) -> GraphQLResponse {
    schema.execute(req.0).await.into()
}

fn dump_sdl(schema: &GraphQLSchema) {
    let sdl = schema.sdl();
    let mut file = std::fs::File::create("../graphql/schema.gql").unwrap();
    file.write_all(sdl.as_bytes()).unwrap();
}
