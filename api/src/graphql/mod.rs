use crate::graphql::{mutation::MutationRoot, query::QueryRoot, subscription::SubscriptionRoot};
use async_graphql::Schema;

pub mod mutation;
pub mod query;
pub mod subscription;

pub type GraphQLSchema = Schema<QueryRoot, MutationRoot, SubscriptionRoot>;
