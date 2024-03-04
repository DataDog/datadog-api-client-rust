// Search spans returns "OK" response with pagination
use datadog_api_client::datadog::configuration::Configuration;
use datadog_api_client::datadogV2::api::api_spans::SpansAPI;
use datadog_api_client::datadogV2::model::*;
use std::ops::Add;
use std::time::{
    Duration,
    SystemTime,
    UNIX_EPOCH,
};

#[tokio::main]
async fn main() {
    let body =
        SpansListRequest
        ::new().data(
            SpansListRequestData::new()
                .attributes(
                    SpansListRequestAttributes::new()
                        .filter(
                            SpansQueryFilter::new()
                                .from("now-15m".to_string())
                                .query("service:python*".to_string())
                                .to("now".to_string()),
                        )
                        .options(SpansQueryOptions::new().timezone("GMT".to_string()))
                        .page(SpansListRequestPage::new().limit(2))
                        .sort(SpansSort::TIMESTAMP_ASCENDING),
                )
                .type_(SpansListRequestType::SEARCH_REQUEST),
        );
    let configuration = Configuration::new();
    let api = SpansAPI::with_config(configuration);
    let resp = api.list_spans(body).await;
    if let Ok(Some(value)) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
