use async_graphql::{EmptyMutation, EmptySubscription, Object, Schema};

pub type PincerSchema = Schema<QueryRoot, EmptyMutation, EmptySubscription>;

#[derive(Default)]
pub struct QueryRoot;

#[Object]
impl QueryRoot {
    async fn greet<'ctx>(&self, name: String) -> String {
        format!("Hello, {}", name)
    }
}
