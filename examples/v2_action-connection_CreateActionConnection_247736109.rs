// Create a new Action Connection with HTTPMtlsAuth returns "Successfully created
// Action Connection" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_action_connection::ActionConnectionAPI;
use datadog_api_client::datadogV2::model::ActionConnectionAttributes;
use datadog_api_client::datadogV2::model::ActionConnectionData;
use datadog_api_client::datadogV2::model::ActionConnectionDataType;
use datadog_api_client::datadogV2::model::ActionConnectionIntegration;
use datadog_api_client::datadogV2::model::CreateActionConnectionRequest;
use datadog_api_client::datadogV2::model::HTTPCredentials;
use datadog_api_client::datadogV2::model::HTTPIntegration;
use datadog_api_client::datadogV2::model::HTTPIntegrationType;
use datadog_api_client::datadogV2::model::HTTPMtlsAuth;
use datadog_api_client::datadogV2::model::HTTPMtlsAuthType;

#[tokio::main]
async fn main() {
    let body = CreateActionConnectionRequest::new(ActionConnectionData::new(
        ActionConnectionAttributes::new(
            ActionConnectionIntegration::HTTPIntegration(Box::new(HTTPIntegration::new(
                "https://api.example.com".to_string(),
                HTTPCredentials::HTTPMtlsAuth(Box::new(HTTPMtlsAuth::new(
                    r#"-----BEGIN CERTIFICATE-----
MIICXjCCAUYCCQDOGcCfCHfhPzANBgkqhkiG9w0BAQsFADAzMQswCQYDVQQGEwJV
UzELMAkGA1UECAwCQ0ExFTATBgNVBAoMDERhdGFkb2cgSW5jLjAeFw0yNDA1MTQw
MDA1NThaFw0yNTA1MTQwMDA1NThaMDMxCzAJBgNVBAYTAlVTMQswCQYDVQQIDAJD
QTEVMBMGA1UECgwMRGF0YWRvZyBJbmMuMIIBIjANBgkqhkiG9w0BAQEFAAOCAQ8A
MIIBCgKCAQEAwQDTmLqWv2L6YhzBcjKgPEzd3kE+9dZ4hBXBCjK6HgF/3aOKOSYq
M9KPFHgJj6SjUJ+8TqX4sV6yW5xGX8dKjOpTYQfExEjGYcVrqKoOGg2k6dGkHyGm
2VzL4zKyK1C3zJ4KpJnMYK8dPPcgzJvO7jGxGKMgLVU3VNdxKGTrqKmC6RbZLQOz
M3fLp7bF2VcJ6VkGKW+yBK6vVMbQKMvjTbGqr3vIRd8SZzKRTsyIzXQDKgOv2vPn
SqYJjKFJ8vJ7JeH6zGyLjQ1cGVy9jJ3+TjJoJhCGOyGzJpBGIcXfYjFDLcSRh7KE
QIDAQABo1MwUTAdBgNVHQ4EFgQU/V8vJkPJ8b9yQnC/9bJ2kJGJ5MjoyEwHwYDVR0j
BBgwFoAU/V8vJkPJ8b9yQnC/9bJ2kJGJ5Mjo
-----END CERTIFICATE-----"#
                        .to_string(),
                    r#"-----BEGIN PRIVATE KEY-----
MIIEvwIBADANBgkqhkiG9w0BAQEFAASCBKkwggSlAgEAAoIBAQDBaNOMV7V8T7gR
OmNcNfqGHxPrLLo1w2J7J8h6S8bVD9yCH2JKV8J5G2J8J0V8J3Jg8b3Jg8LxJgV
V8J6JgV8J9JgJg8LJg8VJgV5JgLLJgVVJg8V4Jg8VLJg8V6JgV8JqJgVV8J3JgV
V8J7JgVV8J5JgVV8J8JgVVJg8LJgJVLJgLVJgVVJgLJgVVJgVVJgVVJgLVJgVV
JgVVJgVVJgLJgVVJgLVJgLLJgVVJgVLJgVVJgVLJgVVJgVVJgVVJgVVJgVVJg
VVJgVVJgVVJgVVJgVVJgVVJgVVJgVVJgVVJgVVJgVVJgVVJgVVJgVVJgVVJgVVJ
AgMBAAECggEBAKoJkJJJJkJJkJ
-----END PRIVATE KEY-----"#
                        .to_string(),
                    HTTPMtlsAuthType::HTTPMTLSAUTH,
                ))),
                HTTPIntegrationType::HTTP,
            ))),
            "HTTP mTLS Connection exampleactionconnection".to_string(),
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
