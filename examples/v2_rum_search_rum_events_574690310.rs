// Search RUM events returns "OK" response with pagination
use chrono::prelude::{DateTime, Utc};
use datadog_api_client::datadog::configuration::Configuration;
use datadog_api_client::datadogV2::api::api_rum::*;
use datadog_api_client::datadogV2::model::*;
use futures_util::pin_mut;
use futures_util::stream::StreamExt;
use std::collections::BTreeMap;

#[tokio::main]
async fn main() {
    let body = RUMSearchEventsRequest::new()
        .filter(
            RUMQueryFilter::new()
                .from("now-15m".to_string())
                .query("@type:session AND @session.type:user".to_string())
                .to("now".to_string()),
        )
        .options(
            RUMQueryOptions::new()
                .time_offset(0)
                .timezone("GMT".to_string()),
        )
        .page(RUMQueryPageOptions::new().limit(2))
        .sort(RUMSort::TIMESTAMP_ASCENDING);
    let configuration = Configuration::new();
    let api = RUMAPI::with_config(configuration);
    let response = api.search_rum_events_with_pagination(body);
    pin_mut!(response);
    while let Some(resp) = response.next().await {
        if let Ok(value) = resp {
            println!("{:#?}", value);
        } else {
            println!("{:#?}", resp.unwrap_err());
        }
    }
}
