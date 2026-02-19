// Create a change request branch returns "OK" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_change_management::ChangeManagementAPI;
use datadog_api_client::datadogV2::model::ChangeRequestBranchCreateAttributes;
use datadog_api_client::datadogV2::model::ChangeRequestBranchCreateData;
use datadog_api_client::datadogV2::model::ChangeRequestBranchCreateRequest;
use datadog_api_client::datadogV2::model::ChangeRequestBranchResourceType;

#[tokio::main]
async fn main() {
    let body = ChangeRequestBranchCreateRequest::new(ChangeRequestBranchCreateData::new(
        ChangeRequestBranchCreateAttributes::new(
            "chm/CHM-1234".to_string(),
            "DataDog/dd-source".to_string(),
        ),
        ChangeRequestBranchResourceType::CHANGE_REQUEST_BRANCH,
    ));
    let mut configuration = datadog::Configuration::new();
    configuration.set_unstable_operation_enabled("v2.CreateChangeRequestBranch", true);
    let api = ChangeManagementAPI::with_config(configuration);
    let resp = api
        .create_change_request_branch("change_request_id".to_string(), body)
        .await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
