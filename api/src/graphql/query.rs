use async_graphql::{Context, Object, Result as GQLResult};
use entity::{download::Model, prelude::Download};
use sea_orm::{DatabaseConnection, EntityTrait};

#[derive(Default)]
pub struct QueryRoot;

#[Object]
impl QueryRoot {
    async fn liveness_probe(&self) -> Result<bool, std::io::Error> {
        Ok(true)
    }

    async fn get_downloads<'ctx>(&self, ctx: &Context<'ctx>) -> GQLResult<Vec<Model>> {
        let db = ctx.data::<DatabaseConnection>().unwrap();
        let res = Download::find().all(db).await.unwrap();
        Ok(res)
    }
}
