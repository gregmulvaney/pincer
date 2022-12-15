use async_graphql::{Context, Object, Result as GQLResult};

#[derive(Default)]
pub struct MutationRoot;

#[Object]
impl MutationRoot {
    async fn add_download(&self, ctx: &Context<'_>) -> GQLResult<bool> {
        Ok(true)
    }
}
