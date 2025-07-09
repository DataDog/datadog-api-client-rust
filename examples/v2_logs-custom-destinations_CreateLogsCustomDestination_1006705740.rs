// Create a Google Security Operations custom destination returns "OK" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_logs_custom_destinations::LogsCustomDestinationsAPI;
use datadog_api_client::datadogV2::model::CustomDestinationAttributeTagsRestrictionListType;
use datadog_api_client::datadogV2::model::CustomDestinationCreateRequest;
use datadog_api_client::datadogV2::model::CustomDestinationCreateRequestAttributes;
use datadog_api_client::datadogV2::model::CustomDestinationCreateRequestDefinition;
use datadog_api_client::datadogV2::model::CustomDestinationForwardDestination;
use datadog_api_client::datadogV2::model::CustomDestinationForwardDestinationGoogleSecurityOperations;
use datadog_api_client::datadogV2::model::CustomDestinationForwardDestinationGoogleSecurityOperationsType;
use datadog_api_client::datadogV2::model::CustomDestinationGoogleSecurityOperationsDestinationAuth;
use datadog_api_client::datadogV2::model::CustomDestinationGoogleSecurityOperationsDestinationAuthType;
use datadog_api_client::datadogV2::model::CustomDestinationType;

#[tokio::main]
async fn main() {
    let body =
        CustomDestinationCreateRequest
        ::new().data(
            CustomDestinationCreateRequestDefinition::new(
                CustomDestinationCreateRequestAttributes::new(
                    CustomDestinationForwardDestination::CustomDestinationForwardDestinationGoogleSecurityOperations(
                        Box::new(
                            CustomDestinationForwardDestinationGoogleSecurityOperations::new(
                                CustomDestinationGoogleSecurityOperationsDestinationAuth::new(
                                    "client@example.com".to_string(),
                                    "def123456".to_string(),
                                    r#"-----BEGIN PRIVATE KEY-----
MIIEvAIBADANBgkqhkiG9w0BAQEFAASCBK...
-----END PRIVATE KEY-----
"#.to_string(),
                                    "abc12345678".to_string(),
                                    "gcp-project".to_string(),
                                    CustomDestinationGoogleSecurityOperationsDestinationAuthType::GCP_PRIVATE_KEY,
                                ),
                                "123-456-7890".to_string(),
                                "google-security-operations-namespace".to_string(),
                                "https://malachiteingestion-pa.googleapis.com".to_string(),
                                CustomDestinationForwardDestinationGoogleSecurityOperationsType
                                ::GOOGLE_SECURITY_OPERATIONS,
                            ),
                        ),
                    ),
                    "Nginx logs".to_string(),
                )
                    .enabled(false)
                    .forward_tags(false)
                    .forward_tags_restriction_list(vec!["datacenter".to_string(), "host".to_string()])
                    .forward_tags_restriction_list_type(CustomDestinationAttributeTagsRestrictionListType::ALLOW_LIST)
                    .query("source:nginx".to_string()),
                CustomDestinationType::CUSTOM_DESTINATION,
            ),
        );
    let configuration = datadog::Configuration::new();
    let api = LogsCustomDestinationsAPI::with_config(configuration);
    let resp = api.create_logs_custom_destination(body).await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
