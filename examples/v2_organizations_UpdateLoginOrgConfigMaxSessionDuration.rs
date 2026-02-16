// Update maximum session duration returns "No Content - The maximum session
// duration was successfully updated." response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_organizations::OrganizationsAPI;
use datadog_api_client::datadogV2::model::MaxSessionDurationUpdateAttributes;
use datadog_api_client::datadogV2::model::MaxSessionDurationUpdateRequest;
use datadog_api_client::datadogV2::model::MaxSessionDurationUpdateRequestData;
use datadog_api_client::datadogV2::model::MaxSessionDurationUpdateRequestDataType;

#[tokio::main]
async fn main() {
    let body = MaxSessionDurationUpdateRequest::new(MaxSessionDurationUpdateRequestData::new(
        MaxSessionDurationUpdateAttributes::new(60),
        MaxSessionDurationUpdateRequestDataType::MAX_SESSION_DURATION,
    ));
    let mut configuration = datadog::Configuration::new();
    configuration.set_unstable_operation_enabled("v2.UpdateLoginOrgConfigMaxSessionDuration", true);
    let api = OrganizationsAPI::with_config(configuration);
    let resp = api.update_login_org_config_max_session_duration(body).await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
