// Create or update HAMR organization connection returns "OK" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_high_availability_multi_region::HighAvailabilityMultiRegionAPI;
use datadog_api_client::datadogV2::model::HamrOrgConnectionAttributesRequest;
use datadog_api_client::datadogV2::model::HamrOrgConnectionDataRequest;
use datadog_api_client::datadogV2::model::HamrOrgConnectionRequest;
use datadog_api_client::datadogV2::model::HamrOrgConnectionStatus;
use datadog_api_client::datadogV2::model::HamrOrgConnectionType;

#[tokio::main]
async fn main() {
    let body = HamrOrgConnectionRequest::new(HamrOrgConnectionDataRequest::new(
        HamrOrgConnectionAttributesRequest::new(
            HamrOrgConnectionStatus::ACTIVE,
            true,
            "admin@example.com".to_string(),
            "us1".to_string(),
            "Production Backup Org".to_string(),
            "660f9511-f3ac-52e5-b827-557766551111".to_string(),
        ),
        "550e8400-e29b-41d4-a716-446655440000".to_string(),
        HamrOrgConnectionType::HAMR_ORG_CONNECTIONS,
    ));
    let mut configuration = datadog::Configuration::new();
    configuration.set_unstable_operation_enabled("v2.CreateHamrOrgConnection", true);
    let api = HighAvailabilityMultiRegionAPI::with_config(configuration);
    let resp = api.create_hamr_org_connection(body).await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
