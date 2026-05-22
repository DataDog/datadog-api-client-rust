// Bulk subscribe to sample log generation returns "OK" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_security_monitoring::SecurityMonitoringAPI;
use datadog_api_client::datadogV2::model::SampleLogGenerationBulkSubscriptionAttributes;
use datadog_api_client::datadogV2::model::SampleLogGenerationBulkSubscriptionData;
use datadog_api_client::datadogV2::model::SampleLogGenerationBulkSubscriptionRequest;
use datadog_api_client::datadogV2::model::SampleLogGenerationBulkSubscriptionRequestType;
use datadog_api_client::datadogV2::model::SampleLogGenerationDuration;

#[tokio::main]
async fn main() {
    let body = SampleLogGenerationBulkSubscriptionRequest::new(
        SampleLogGenerationBulkSubscriptionData::new(
            SampleLogGenerationBulkSubscriptionAttributes::new(vec!["aws-cloudtrail".to_string()])
                .duration(SampleLogGenerationDuration::THREE_DAYS),
            SampleLogGenerationBulkSubscriptionRequestType::BULK_SUBSCRIPTION_REQUESTS,
        ),
    );
    let mut configuration = datadog::Configuration::new();
    configuration
        .set_unstable_operation_enabled("v2.BulkCreateSampleLogGenerationSubscriptions", true);
    let api = SecurityMonitoringAPI::with_config(configuration);
    let resp = api
        .bulk_create_sample_log_generation_subscriptions(body)
        .await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
