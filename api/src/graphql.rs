use async_graphql::{Context, EmptyMutation, EmptySubscription, Object, Schema, SimpleObject};
use entity::{download::Model, prelude::Download};
use sea_orm::{DatabaseConnection, DbErr, EntityTrait};

pub type GraphQLSchema = Schema<QueryRoot, EmptyMutation, EmptySubscription>;

#[derive(SimpleObject)]
struct DownloadItem {
    id: String,
    name: String,
}

#[derive(Default)]
pub struct QueryRoot;

#[Object]
impl QueryRoot {
    async fn greet<'ctx>(&self, name: String) -> String {
        format!("Hello, {name}")
    }

    async fn downloads<'ctx>(
        &self,
        ctx: &Context<'ctx>,
        limits: Option<String>,
    ) -> Result<Vec<Model>, DbErr> {
        let db = ctx.data::<DatabaseConnection>().unwrap();
        Ok(Download::find()
            .all(db)
            .await
            .map_err(|e| e.to_string())
            .unwrap())
    }
}
