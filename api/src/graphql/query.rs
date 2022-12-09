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

    async fn start_downloads(&self) -> String {
        "Starting Downloads".to_string()
    }

    async fn pause_downloads(&self) -> String {
        "Pausing Downloads".to_string()
    }

    async fn stop_downloads(&self) -> String {
        "Stopping Downloads".to_string()
    }
}
