use async_graphql::Object;

#[derive(Default)]
pub struct QueryRoot;

#[Object]
impl QueryRoot {
    async fn greet<'ctx>(&self, name: String) -> String {
        format!("Hello, {name}")
    }
}
