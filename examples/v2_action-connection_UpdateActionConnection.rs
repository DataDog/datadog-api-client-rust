// Update an existing Action Connection returns "Successfully updated Action
// Connection" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_action_connection::ActionConnectionAPI;
use datadog_api_client::datadogV2::model::AWSAssumeRoleType;
use datadog_api_client::datadogV2::model::AWSAssumeRoleUpdate;
use datadog_api_client::datadogV2::model::AWSCredentialsUpdate;
use datadog_api_client::datadogV2::model::AWSIntegrationType;
use datadog_api_client::datadogV2::model::AWSIntegrationUpdate;
use datadog_api_client::datadogV2::model::ActionConnectionAttributesUpdate;
use datadog_api_client::datadogV2::model::ActionConnectionDataType;
use datadog_api_client::datadogV2::model::ActionConnectionDataUpdate;
use datadog_api_client::datadogV2::model::ActionConnectionIntegrationUpdate;
use datadog_api_client::datadogV2::model::UpdateActionConnectionRequest;

#[tokio::main]
async fn main() {
    let body = UpdateActionConnectionRequest::new(ActionConnectionDataUpdate::new(
        ActionConnectionAttributesUpdate::new()
            .integration(ActionConnectionIntegrationUpdate::AWSIntegrationUpdate(
                Box::new(
                    AWSIntegrationUpdate::new(AWSIntegrationType::AWS).credentials(
                        AWSCredentialsUpdate::AWSAssumeRoleUpdate(Box::new(
                            AWSAssumeRoleUpdate::new(AWSAssumeRoleType::AWSASSUMEROLE)
                                .account_id("123456789123".to_string())
                                .role("MyRoleUpdated".to_string()),
                        )),
                    ),
                ),
            ))
            .name("Cassette Connection".to_string()),
        ActionConnectionDataType::ACTION_CONNECTION,
    ));
    let configuration = datadog::Configuration::new();
    let api = ActionConnectionAPI::with_config(configuration);
    let resp = api
        .update_action_connection("cb460d51-3c88-4e87-adac-d47131d0423d".to_string(), body)
        .await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
