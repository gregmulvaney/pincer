use async_graphql::{Context, Object};
use entity::{download::Model, prelude::Download};
use sea_orm::{DatabaseConnection, DbErr, EntityTrait, QuerySelect};

struct DownloadItem {
    id: String,
}

#[derive(Default)]
pub struct QueryRoot;

#[Object]
impl QueryRoot {
    async fn greet<'ctx>(&self, name: String) -> String {
        format!("Hello, {name}")
    }
    async fn downloads<'ctx>(&self, ctx: &Context<'ctx>) -> Result<Vec<Model>, DbErr> {
        let db = ctx.data::<DatabaseConnection>().unwrap();
        Ok(Download::find()
            .all(db)
            .await
            .map_err(|e| e.to_string())
            .unwrap())
    }
}
