// Create an API GRPC test returns "OK - Returns the created test details." response
use datadog_api_client::datadog::configuration::Configuration;
use datadog_api_client::datadogV1::api::api_synthetics::SyntheticsAPI;
use datadog_api_client::datadogV1::model::*;
use std::ops::Add;
use std::time::{
    Duration,
    SystemTime,
    UNIX_EPOCH,
};

#[tokio::main]
async fn main() {
    let body =
        SyntheticsAPITest::new(
            SyntheticsAPITestConfig::new()
                .assertions(
                    vec![
                        SyntheticsAssertion::SyntheticsAssertionTarget(
                            Box::new(
                                SyntheticsAssertionTarget::new(
                                    SyntheticsAssertionOperator::IS,
                                    1,
                                    SyntheticsAssertionType::GRPC_HEALTHCHECK_STATUS,
                                ),
                            ),
                        ),
                        SyntheticsAssertion::SyntheticsAssertionTarget(
                            Box::new(
                                SyntheticsAssertionTarget::new(
                                    SyntheticsAssertionOperator::IS,
                                    "proto target".to_string(),
                                    SyntheticsAssertionType::GRPC_PROTO,
                                ),
                            ),
                        ),
                        SyntheticsAssertion::SyntheticsAssertionTarget(
                            Box::new(
                                SyntheticsAssertionTarget::new(
                                    SyntheticsAssertionOperator::IS,
                                    "123".to_string(),
                                    SyntheticsAssertionType::GRPC_METADATA,
                                ).property("property".to_string()),
                            ),
                        )
                    ],
                )
                .request(
                    SyntheticsTestRequest::new()
                        .host("localhost".to_string())
                        .message("".to_string())
                        .metadata(std::collections::BTreeMap::from([]))
                        .method("GET".to_string())
                        .port(50051)
                        .service("Hello".to_string()),
                ),
            vec!["aws:us-east-2".to_string()],
            "BDD test payload: synthetics_api_grpc_test_payload.json".to_string(),
            "Example-Synthetic".to_string(),
            SyntheticsTestOptions::new()
                .min_failure_duration(0)
                .min_location_failed(1)
                .monitor_name("Example-Synthetic".to_string())
                .monitor_options(SyntheticsTestOptionsMonitorOptions::new().renotify_interval(0))
                .tick_every(60),
            SyntheticsAPITestType::API,
        )
            .subtype(SyntheticsTestDetailsSubType::GRPC)
            .tags(vec!["testing:api".to_string()]);
    let configuration = Configuration::new();
    let api = SyntheticsAPI::with_config(configuration);
    let resp = api.create_synthetics_api_test(body).await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
