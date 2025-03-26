// Create a new Action Connection returns "Successfully created Action Connection"
// response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_action_connection::ActionConnectionAPI;
use datadog_api_client::datadogV2::model::AWSAssumeRole;
use datadog_api_client::datadogV2::model::AWSAssumeRoleType;
use datadog_api_client::datadogV2::model::AWSCredentials;
use datadog_api_client::datadogV2::model::AWSIntegration;
use datadog_api_client::datadogV2::model::AWSIntegrationType;
use datadog_api_client::datadogV2::model::ActionConnectionAttributes;
use datadog_api_client::datadogV2::model::ActionConnectionData;
use datadog_api_client::datadogV2::model::ActionConnectionDataType;
use datadog_api_client::datadogV2::model::ActionConnectionIntegration;
use datadog_api_client::datadogV2::model::CreateActionConnectionRequest;

#[tokio::main]
async fn main() {
    let body = CreateActionConnectionRequest::new(ActionConnectionData::new(
        ActionConnectionAttributes::new(
            ActionConnectionIntegration::AWSIntegration(Box::new(AWSIntegration::new(
                AWSCredentials::AWSAssumeRole(Box::new(AWSAssumeRole::new(
                    "123456789123".to_string(),
                    "MyRoleUpdated".to_string(),
                    AWSAssumeRoleType::AWSASSUMEROLE,
                ))),
                AWSIntegrationType::AWS,
            ))),
            "Cassette Connection exampleactionconnection".to_string(),
        ),
        ActionConnectionDataType::ACTION_CONNECTION,
    ));
    let configuration = datadog::Configuration::new();
    let api = ActionConnectionAPI::with_config(configuration);
    let resp = api.create_action_connection(body).await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
