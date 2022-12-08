use crate::graphql::{mutation::MutationRoot, query::QueryRoot, subscription::SubscriptionRoot};
use async_graphql::{Schema, SimpleObject};

pub mod mutation;
pub mod query;
pub mod subscription;

pub type GraphQLSchema = Schema<QueryRoot, MutationRoot, SubscriptionRoot>;

#[derive(Debug, Default, SimpleObject)]
pub struct DownloadItem {
    name: String,
    url: String,
    raw_size: String,
    adjusted_size: String,
    unit: String,
    host: String,
}
