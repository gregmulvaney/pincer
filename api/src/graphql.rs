use async_graphql::{Context, EmptyMutation, EmptySubscription, Object, Schema};
use entity::prelude::Download;
use sea_orm::{DatabaseConnection, EntityTrait};

pub type GraphQLSchema = Schema<QueryRoot, EmptyMutation, EmptySubscription>;

#[derive(Default)]
pub struct QueryRoot;

#[Object]
impl QueryRoot {
    async fn greet<'ctx>(&self, name: String) -> String {
        format!("Hello, {name}")
    }

    async fn get_downloads<'ctx>(&self, ctx: &Context<'ctx>) -> String {
        let db = ctx.data::<DatabaseConnection>().unwrap();
        let res = Download::find().all(db).await.unwrap();
        println!("{res:?}");
        "Works".to_string()
    }
}
