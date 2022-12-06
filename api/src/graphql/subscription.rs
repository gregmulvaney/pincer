use async_graphql::{
    futures_util::{Stream, StreamExt},
    Subscription,
};
use std::time::Duration;

#[derive(Default)]
pub struct SubscriptionRoot;

#[Subscription]
impl SubscriptionRoot {
    async fn getDownloads(&self) -> impl Stream<Item = bool> {
        tokio_stream::wrappers::IntervalStream::new(tokio::time::interval(Duration::from_secs(1)))
            .map(move |_| true)
    }
}
