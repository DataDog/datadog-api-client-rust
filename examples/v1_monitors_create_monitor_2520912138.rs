// Create a ci-tests monitor returns "OK" response
use datadog_api_client::datadog::configuration::Configuration;
use datadog_api_client::datadogV1::api::api_monitors::*;
use datadog_api_client::datadogV1::model::*;

#[tokio::main]
async fn main() {
    let body =
        Monitor::new(
            r#"ci-tests("type:test @git.branch:staging* @test.status:fail").rollup("count").by("@test.name").last("5m") >= 1"#.to_string(),
            MonitorType::CI_TESTS_ALERT,
        )
            .message("some message Notify: @hipchat-channel".to_string())
            .name("Example-Monitor".to_string())
            .options(MonitorOptions::new().thresholds(MonitorThresholds::new().critical(1 as f64)))
            .priority(Some(3))
            .tags(vec!["test:examplemonitor".to_string(), "env:ci".to_string()]);
    let configuration = Configuration::new();
    let api = MonitorsAPI::with_config(configuration);
    let resp = api.create_monitor(body).await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}