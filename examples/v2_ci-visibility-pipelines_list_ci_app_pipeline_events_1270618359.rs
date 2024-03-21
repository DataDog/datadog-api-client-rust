// Get a list of pipelines events returns "OK" response with pagination
use datadog_api_client::datadog::configuration::Configuration;
use datadog_api_client::datadogV2::api::api_ci_visibility_pipelines::CIVisibilityPipelinesAPI;
use datadog_api_client::datadogV2::api::api_ci_visibility_pipelines::ListCIAppPipelineEventsOptionalParams;
use futures_util::pin_mut;
use futures_util::stream::StreamExt;

#[tokio::main]
async fn main() {
    let configuration = Configuration::new();
    let api = CIVisibilityPipelinesAPI::with_config(configuration);
    let response = api.list_ci_app_pipeline_events_with_pagination(
        ListCIAppPipelineEventsOptionalParams::default()
            .filter_from("2021-11-11T11:10:41+00:00".to_string())
            .filter_to("2021-11-11T11:11:11+00:00".to_string())
            .page_limit(2),
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
