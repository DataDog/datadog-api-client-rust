// Search pipelines events returns "OK" response with pagination
use datadog_api_client::datadog::configuration::Configuration;
use datadog_api_client::datadogV2::api::api_ci_visibility_pipelines::*;
use datadog_api_client::datadogV2::model::*;

#[tokio::main]
async fn main() {
    let body =
        CIAppPipelineEventsRequest::new()
            .filter(CIAppPipelinesQueryFilter::new().from("now-30s".to_string()).to("now".to_string()))
            .options(CIAppQueryOptions::new().timezone("GMT".to_string()))
            .page(CIAppQueryPageOptions::new().limit(2))
            .sort(CIAppSort::TIMESTAMP_ASCENDING);
    let configuration = Configuration::new();
    let api = CIVisibilityPipelinesAPI::with_config(configuration);
    let resp =
        api.search_ci_app_pipeline_events(SearchCIAppPipelineEventsOptionalParams::default().body(body)).await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
