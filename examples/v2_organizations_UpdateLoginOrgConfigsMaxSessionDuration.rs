// Update the maximum session duration returns "No Content" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_organizations::OrganizationsAPI;
use datadog_api_client::datadogV2::model::MaxSessionDurationType;
use datadog_api_client::datadogV2::model::MaxSessionDurationUpdateAttributes;
use datadog_api_client::datadogV2::model::MaxSessionDurationUpdateData;
use datadog_api_client::datadogV2::model::MaxSessionDurationUpdateRequest;

#[tokio::main]
async fn main() {
    let body = MaxSessionDurationUpdateRequest::new(MaxSessionDurationUpdateData::new(
        MaxSessionDurationUpdateAttributes::new(604800),
        MaxSessionDurationType::MAX_SESSION_DURATION,
    ));
    let configuration = datadog::Configuration::new();
    let api = OrganizationsAPI::with_config(configuration);
    let resp = api
        .update_login_org_configs_max_session_duration(body)
        .await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
