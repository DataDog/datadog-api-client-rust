// Send a failure event for DORA Metrics returns "OK - but delayed due to
// incident" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_dora_metrics::DORAMetricsAPI;
use datadog_api_client::datadogV2::model::DORAFailureRequest;
use datadog_api_client::datadogV2::model::DORAFailureRequestAttributes;
use datadog_api_client::datadogV2::model::DORAFailureRequestData;
use datadog_api_client::datadogV2::model::DORAGitInfo;

#[tokio::main]
async fn main() {
    let body = DORAFailureRequest::new(DORAFailureRequestData::new(
        DORAFailureRequestAttributes::new(1693491974000000000)
            .env("staging".to_string())
            .finished_at(1693491984000000000)
            .git(DORAGitInfo::new(
                "66adc9350f2cc9b250b69abddab733dd55e1a588".to_string(),
                "https://github.com/organization/example-repository".to_string(),
            ))
            .name("Webserver is down failing all requests.".to_string())
            .services(vec!["shopist".to_string()])
            .severity("High".to_string())
            .team("backend".to_string())
            .version("v1.12.07".to_string()),
    ));
    let configuration = datadog::Configuration::new();
    let api = DORAMetricsAPI::with_config(configuration);
    let resp = api.create_dora_failure(body).await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
