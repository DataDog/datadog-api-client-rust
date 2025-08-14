// Create a new Action Connection with HTTPTokenAuth returns "Successfully created
// Action Connection" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_action_connection::ActionConnectionAPI;
use datadog_api_client::datadogV2::model::ActionConnectionAttributes;
use datadog_api_client::datadogV2::model::ActionConnectionData;
use datadog_api_client::datadogV2::model::ActionConnectionDataType;
use datadog_api_client::datadogV2::model::ActionConnectionIntegration;
use datadog_api_client::datadogV2::model::CreateActionConnectionRequest;
use datadog_api_client::datadogV2::model::HTTPCredentials;
use datadog_api_client::datadogV2::model::HTTPHeader;
use datadog_api_client::datadogV2::model::HTTPIntegration;
use datadog_api_client::datadogV2::model::HTTPIntegrationType;
use datadog_api_client::datadogV2::model::HTTPToken;
use datadog_api_client::datadogV2::model::HTTPTokenAuth;
use datadog_api_client::datadogV2::model::HTTPTokenAuthType;
use datadog_api_client::datadogV2::model::TokenType;

#[tokio::main]
async fn main() {
    let body = CreateActionConnectionRequest::new(ActionConnectionData::new(
        ActionConnectionAttributes::new(
            ActionConnectionIntegration::HTTPIntegration(Box::new(HTTPIntegration::new(
                "https://api.example.com".to_string(),
                HTTPCredentials::HTTPTokenAuth(Box::new(
                    HTTPTokenAuth::new(HTTPTokenAuthType::HTTPTOKENAUTH)
                        .headers(vec![HTTPHeader::new(
                            "Authorization".to_string(),
                            "Bearer token-value".to_string(),
                        )])
                        .tokens(vec![HTTPToken::new(
                            "ApiKey".to_string(),
                            TokenType::SECRET,
                            "secret-token-value".to_string(),
                        )]),
                )),
                HTTPIntegrationType::HTTP,
            ))),
            "HTTP Token Connection exampleactionconnection".to_string(),
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
