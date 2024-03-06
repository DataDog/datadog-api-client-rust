// Search events returns "OK" response with pagination
use datadog_api_client::datadog::configuration::Configuration;
use datadog_api_client::datadogV2::api::api_events::*;
use datadog_api_client::datadogV2::model::*;

#[tokio::main]
async fn main() {
    let body =
        EventsListRequest::new()
            .filter(EventsQueryFilter::new().from("now-15m".to_string()).to("now".to_string()))
            .options(EventsQueryOptions::new().timezone("GMT".to_string()))
            .page(EventsRequestPage::new().limit(2))
            .sort(EventsSort::TIMESTAMP_ASCENDING);
    let configuration = Configuration::new();
    let api = EventsAPI::with_config(configuration);
    let resp = api.search_events(SearchEventsOptionalParams::default().body(body)).await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
