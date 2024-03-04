// Search pipelines events returns "OK" response
use datadog_api_client::datadog::configuration::Configuration;
use datadog_api_client::datadogV2::api::api_ci_visibility_pipelines::CIVisibilityPipelinesAPI;
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
        CIAppPipelineEventsRequest::new()
            .filter(
                CIAppPipelinesQueryFilter::new()
                    .from("now-15m".to_string())
                    .query("@ci.provider.name:github AND @ci.status:error".to_string())
                    .to("now".to_string()),
            )
            .options(CIAppQueryOptions::new().timezone("GMT".to_string()))
            .page(CIAppQueryPageOptions::new().limit(5))
            .sort(CIAppSort::TIMESTAMP_ASCENDING);
    let configuration = Configuration::new();
    let api = CIVisibilityPipelinesAPI::with_config(configuration);
    let resp = api.search_ci_app_pipeline_events(body).await;
    if let Ok(Some(value)) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
