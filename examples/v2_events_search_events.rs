// Search events returns "OK" response
use datadog_api_client::datadog::configuration::Configuration;
use datadog_api_client::datadogV2::api::api_events::*;
use datadog_api_client::datadogV2::model::*;

#[tokio::main]
async fn main() {
    let body =
        EventsListRequest::new()
            .filter(
                EventsQueryFilter::new()
                    .from("2020-09-17T11:48:36+01:00".to_string())
                    .query("datadog-agent".to_string())
                    .to("2020-09-17T12:48:36+01:00".to_string()),
            )
            .page(EventsRequestPage::new().limit(5))
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
