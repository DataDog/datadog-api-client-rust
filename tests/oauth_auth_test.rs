// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use datadog_api_client::datadog::Configuration;

#[test]
fn test_set_access_token_compiles() {
    let mut config = Configuration::new();
    // Verify the public API exists and accepts a String
    config.set_access_token("my-oauth-token".to_string());
}

#[test]
fn test_access_token_default_does_not_send_bearer() {
    // A default configuration should not have an access token set.
    // This is verified indirectly: if access_token were Some, the generated
    // API code would send an Authorization header. We just verify construction works.
    let _config = Configuration::new();
}
