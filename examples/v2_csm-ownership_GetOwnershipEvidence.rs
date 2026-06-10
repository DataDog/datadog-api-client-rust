// Get the evidence for an ownership inference returns "OK" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_csm_ownership::CSMOwnershipAPI;
use datadog_api_client::datadogV2::api_csm_ownership::GetOwnershipEvidenceOptionalParams;
use datadog_api_client::datadogV2::model::OwnershipOwnerType;

#[tokio::main]
async fn main() {
    let mut configuration = datadog::Configuration::new();
    configuration.set_unstable_operation_enabled("v2.GetOwnershipEvidence", true);
    let api = CSMOwnershipAPI::with_config(configuration);
    let resp = api
        .get_ownership_evidence(
            "test-resource".to_string(),
            OwnershipOwnerType::TEAM,
            GetOwnershipEvidenceOptionalParams::default(),
        )
        .await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
