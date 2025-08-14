// Update an existing Action Connection with HTTPBasicAuth returns "Successfully
// updated Action Connection" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_action_connection::ActionConnectionAPI;
use datadog_api_client::datadogV2::model::ActionConnectionAttributesUpdate;
use datadog_api_client::datadogV2::model::ActionConnectionDataType;
use datadog_api_client::datadogV2::model::ActionConnectionDataUpdate;
use datadog_api_client::datadogV2::model::ActionConnectionIntegrationUpdate;
use datadog_api_client::datadogV2::model::HTTPBasicAuthType;
use datadog_api_client::datadogV2::model::HTTPBasicAuthUpdate;
use datadog_api_client::datadogV2::model::HTTPCredentialsUpdate;
use datadog_api_client::datadogV2::model::HTTPIntegrationType;
use datadog_api_client::datadogV2::model::HTTPIntegrationUpdate;
use datadog_api_client::datadogV2::model::UpdateActionConnectionRequest;

#[tokio::main]
async fn main() {
    let body = UpdateActionConnectionRequest::new(ActionConnectionDataUpdate::new(
        ActionConnectionAttributesUpdate::new()
            .integration(ActionConnectionIntegrationUpdate::HTTPIntegrationUpdate(
                Box::new(
                    HTTPIntegrationUpdate::new(HTTPIntegrationType::HTTP)
                        .base_url("https://api.updated.com".to_string())
                        .credentials(HTTPCredentialsUpdate::HTTPBasicAuthUpdate(Box::new(
                            HTTPBasicAuthUpdate::new(HTTPBasicAuthType::HTTPBASICAUTH)
                                .password("updated-password".to_string())
                                .username("updated-user".to_string()),
                        ))),
                ),
            ))
            .name("HTTP Basic Auth Updated".to_string()),
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
