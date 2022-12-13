use async_graphql::{
    futures_util::{Stream, StreamExt},
    Subscription,
};
use std::time::Duration;

#[derive(Default)]
pub struct SubscriptionRoot;

#[Subscription]
impl SubscriptionRoot {
    async fn integers(&self, #[graphql(default = 1)] step: i32) -> impl Stream<Item = i32> {
        println!("Generated");
        let mut value = 0;
        tokio_stream::wrappers::IntervalStream::new(tokio::time::interval(Duration::from_secs(3)))
            .map(move |_| {
                value += step;

                value
            })
    }
}
