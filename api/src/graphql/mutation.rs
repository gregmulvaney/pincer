use crate::graphql::DownloadItem;
use async_graphql::{Context, Object, Result as GQLResult};
use byte_unit::Byte;
use entity::download::*;
use sea_orm::{ActiveModelTrait, DatabaseConnection, NotSet, Set};
use url::Url;

#[derive(Default)]
pub struct MutationRoot;

#[Object]
impl MutationRoot {
    async fn add_download(&self, ctx: &Context<'_>, url: String) -> GQLResult<bool> {
        let db = ctx.data::<DatabaseConnection>().unwrap();
        let download: DownloadItem = parse_url(url).await.unwrap();
        let item = ActiveModel {
            id: NotSet,
            name: Set(download.name),
            raw_size: Set(download.raw_size),
            adjusted_size: Set(download.adjusted_size),
            unit: Set(download.unit),
            url: Set(download.url),
            host: Set(download.host),
        };
        item.insert(db).await.unwrap();
        Ok(true)
    }
}

async fn parse_url(url: String) -> Result<DownloadItem, Box<dyn std::error::Error>> {
    let (url, raw_size, adjusted_size, unit, file_name, host) = tokio::spawn(async move {
        let res = reqwest::Client::new().get(&url).send().await.unwrap();
        // Get file size in Bytes and format to the appropriate unit
        let raw_size = res.content_length().unwrap();
        let parsed_size = Byte::from_bytes(raw_size as u128).get_appropriate_unit(false);
        let unit = parsed_size.get_unit();
        let parsed_url = Url::parse(&url).unwrap();
        let host = parsed_url.host_str().unwrap().to_string();
        let file_name = parsed_url
            .path_segments()
            .unwrap()
            .last()
            .unwrap()
            .to_string();

        // Reduce float precision to nearest 10th
        let size = format!("{:.2}", parsed_size.get_value());
        (url, raw_size, size, unit, file_name, host)
    })
    .await
    .unwrap();

    Ok(DownloadItem {
        name: file_name,
        url,
        raw_size: raw_size.to_string(),
        adjusted_size: adjusted_size,
        unit: unit.to_string(),
        host,
    })
}
