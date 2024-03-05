// Get a list of pipelines events returns "OK" response
use chrono::prelude::*;
use datadog_api_client::datadog::configuration::Configuration;
use datadog_api_client::datadogV2::api::api_ci_visibility_pipelines::*;

#[tokio::main]
async fn main() {
    let configuration = Configuration::new();
    let api = CIVisibilityPipelinesAPI::with_config(configuration);
    let resp =
        api
            .list_ci_app_pipeline_events(
                ListCIAppPipelineEventsOptionalParams::default()
                    .filter_query("@ci.provider.name:circleci".to_string())
                    .filter_from((Utc::now() + chrono::Duration::minutes(-30)).to_rfc3339())
                    .filter_to((Utc::now()).to_rfc3339())
                    .page_limit(5),
            )
            .await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
