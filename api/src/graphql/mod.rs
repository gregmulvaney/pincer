use crate::graphql::query::QueryRoot;
use async_graphql::{EmptyMutation, EmptySubscription, Schema};

mod mutation;
pub mod query;
mod subscription;

pub type GraphQLSchema = Schema<QueryRoot, EmptyMutation, EmptySubscription>;
