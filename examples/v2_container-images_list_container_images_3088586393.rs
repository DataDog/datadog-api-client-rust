// Get all Container Images returns "OK" response with pagination
use chrono::prelude::{DateTime, Utc};
use datadog_api_client::datadog::configuration::Configuration;
use datadog_api_client::datadogV2::api::api_container_images::*;
use datadog_api_client::datadogV2::model::*;
use futures_util::pin_mut;
use futures_util::stream::StreamExt;
use std::collections::BTreeMap;

#[tokio::main]
async fn main() {
    let configuration = Configuration::new();
    let api = ContainerImagesAPI::with_config(configuration);
    let response = api.list_container_images_with_pagination(
        ListContainerImagesOptionalParams::default().page_size(2),
    );
    pin_mut!(response);
    while let Some(resp) = response.next().await {
        if let Ok(value) = resp {
            println!("{:#?}", value);
        } else {
            println!("{:#?}", resp.unwrap_err());
        }
    }
}
