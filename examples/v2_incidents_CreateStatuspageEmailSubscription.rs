// Create a status page email subscription returns "Created" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_incidents::IncidentsAPI;
use datadog_api_client::datadogV2::model::IncidentStatuspageSubscriptionDataAttributesRequest;
use datadog_api_client::datadogV2::model::IncidentStatuspageSubscriptionDataRequest;
use datadog_api_client::datadogV2::model::IncidentStatuspageSubscriptionRequest;
use datadog_api_client::datadogV2::model::IncidentStatuspageSubscriptionType;

#[tokio::main]
async fn main() {
    let body =
        IncidentStatuspageSubscriptionRequest::new(IncidentStatuspageSubscriptionDataRequest::new(
            IncidentStatuspageSubscriptionDataAttributesRequest::new(
                "user@example.com".to_string(),
            ),
            IncidentStatuspageSubscriptionType::STATUSPAGE_EMAIL_SUBSCRIPTION,
        ));
    let mut configuration = datadog::Configuration::new();
    configuration.set_unstable_operation_enabled("v2.CreateStatuspageEmailSubscription", true);
    let api = IncidentsAPI::with_config(configuration);
    let resp = api
        .create_statuspage_email_subscription("page_id".to_string(), body)
        .await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
