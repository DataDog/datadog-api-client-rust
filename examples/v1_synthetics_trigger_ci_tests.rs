// Trigger tests from CI/CD pipelines returns "OK" response
use chrono::prelude::{DateTime, Utc};
use datadog_api_client::datadog::configuration::Configuration;
use datadog_api_client::datadogV1::api::api_synthetics::*;
use datadog_api_client::datadogV1::model::*;
use std::collections::BTreeMap;

#[tokio::main]
async fn main() {
    let body =
        SyntheticsCITestBody::new().tests(vec![SyntheticsCITest::new("aaa-aaa-aaa".to_string())
            .basic_auth(SyntheticsBasicAuth::SyntheticsBasicAuthWeb(Box::new(
                SyntheticsBasicAuthWeb::new("PaSSw0RD!".to_string(), "my_username".to_string())
                    .type_(SyntheticsBasicAuthWebType::WEB),
            )))
            .device_ids(vec![SyntheticsDeviceID::LAPTOP_LARGE])
            .locations(vec!["aws:eu-west-3".to_string()])
            .metadata(
                SyntheticsCIBatchMetadata::new()
                    .ci(SyntheticsCIBatchMetadataCI::new()
                        .pipeline(SyntheticsCIBatchMetadataPipeline::new())
                        .provider(SyntheticsCIBatchMetadataProvider::new()))
                    .git(SyntheticsCIBatchMetadataGit::new()),
            )
            .retry(SyntheticsTestOptionsRetry::new())]);
    let configuration = Configuration::new();
    let api = SyntheticsAPI::with_config(configuration);
    let resp = api.trigger_ci_tests(body).await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
