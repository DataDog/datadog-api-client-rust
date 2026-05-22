// Subscribe to sample log generation returns "OK" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_security_monitoring::SecurityMonitoringAPI;
use datadog_api_client::datadogV2::model::SampleLogGenerationDuration;
use datadog_api_client::datadogV2::model::SampleLogGenerationSubscriptionCreateAttributes;
use datadog_api_client::datadogV2::model::SampleLogGenerationSubscriptionCreateData;
use datadog_api_client::datadogV2::model::SampleLogGenerationSubscriptionCreateRequest;
use datadog_api_client::datadogV2::model::SampleLogGenerationSubscriptionRequestType;

#[tokio::main]
async fn main() {
    let body = SampleLogGenerationSubscriptionCreateRequest::new(
        SampleLogGenerationSubscriptionCreateData::new(
            SampleLogGenerationSubscriptionCreateAttributes::new("aws-cloudtrail".to_string())
                .duration(SampleLogGenerationDuration::THREE_DAYS),
            SampleLogGenerationSubscriptionRequestType::SUBSCRIPTION_REQUESTS,
        ),
    );
    let mut configuration = datadog::Configuration::new();
    configuration.set_unstable_operation_enabled("v2.CreateSampleLogGenerationSubscription", true);
    let api = SecurityMonitoringAPI::with_config(configuration);
    let resp = api.create_sample_log_generation_subscription(body).await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
