// Create an API SSL test returns "OK - Returns the created test details." response
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
                                    SyntheticsAssertionOperator::IS_IN_MORE_DAYS_THAN,
                                    10,
                                    SyntheticsAssertionType::CERTIFICATE,
                                ),
                            ),
                        )
                    ],
                )
                .request(SyntheticsTestRequest::new().host("datadoghq.com".to_string()).port(443)),
            vec!["aws:us-east-2".to_string()],
            "BDD test payload: synthetics_api_ssl_test_payload.json".to_string(),
            "Example-Synthetic".to_string(),
            SyntheticsTestOptions::new().accept_self_signed(true).check_certificate_revocation(true).tick_every(60),
            SyntheticsAPITestType::API,
        )
            .subtype(SyntheticsTestDetailsSubType::SSL)
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
