// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use datadog_api_client::datadog::{APIKey, Configuration};
use std::env;
use std::sync::Mutex;

// Mutex to prevent env var tests from interfering with each other
static ENV_MUTEX: Mutex<()> = Mutex::new(());

#[test]
fn test_set_pat_stores_pat() {
    let mut config = Configuration::new();
    config.set_pat("my-pat-token".to_string());

    let headers = config.auth_headers();
    assert!(headers
        .iter()
        .any(|(k, v)| k == "Authorization" && v == "Bearer my-pat-token"));
}

#[test]
fn test_auth_headers_with_pat_returns_bearer() {
    let mut config = Configuration::new();
    config.set_pat("my-pat-token".to_string());

    let headers = config.auth_headers();
    assert!(headers
        .iter()
        .any(|(k, v)| k == "Authorization" && v == "Bearer my-pat-token"));
}

#[test]
fn test_auth_headers_with_pat_and_api_keys_sends_all() {
    let mut config = Configuration::new();
    config.set_auth_key(
        "apiKeyAuth",
        APIKey {
            key: "my-api-key".to_string(),
            prefix: "".to_string(),
        },
    );
    config.set_auth_key(
        "appKeyAuth",
        APIKey {
            key: "my-app-key".to_string(),
            prefix: "".to_string(),
        },
    );
    config.set_pat("my-pat-token".to_string());

    let headers = config.auth_headers();
    assert!(headers
        .iter()
        .any(|(k, v)| k == "Authorization" && v == "Bearer my-pat-token"));
    assert!(headers
        .iter()
        .any(|(k, v)| k == "DD-API-KEY" && v == "my-api-key"));
    assert!(headers
        .iter()
        .any(|(k, v)| k == "DD-APPLICATION-KEY" && v == "my-app-key"));
}

#[test]
fn test_auth_headers_without_pat_returns_api_keys() {
    let _lock = ENV_MUTEX.lock().unwrap();
    let old_pat = env::var("DD_BEARER_TOKEN").ok();
    env::remove_var("DD_BEARER_TOKEN");

    let mut config = Configuration::new();
    config.set_auth_key(
        "apiKeyAuth",
        APIKey {
            key: "my-api-key".to_string(),
            prefix: "".to_string(),
        },
    );
    config.set_auth_key(
        "appKeyAuth",
        APIKey {
            key: "my-app-key".to_string(),
            prefix: "".to_string(),
        },
    );

    let headers = config.auth_headers();
    assert!(headers
        .iter()
        .any(|(k, v)| k == "DD-API-KEY" && v == "my-api-key"));
    assert!(headers
        .iter()
        .any(|(k, v)| k == "DD-APPLICATION-KEY" && v == "my-app-key"));
    // No Authorization header should be present
    assert!(!headers.iter().any(|(k, _)| k == "Authorization"));

    match old_pat {
        Some(v) => env::set_var("DD_BEARER_TOKEN", v),
        None => env::remove_var("DD_BEARER_TOKEN"),
    }
}

#[test]
fn test_dd_bearer_token_env_var() {
    let _lock = ENV_MUTEX.lock().unwrap();
    let old_pat = env::var("DD_BEARER_TOKEN").ok();

    env::set_var("DD_BEARER_TOKEN", "env-pat-token");

    let config = Configuration::default();

    let headers = config.auth_headers();
    assert!(headers
        .iter()
        .any(|(k, v)| k == "Authorization" && v == "Bearer env-pat-token"));

    // Restore env
    match old_pat {
        Some(v) => env::set_var("DD_BEARER_TOKEN", v),
        None => env::remove_var("DD_BEARER_TOKEN"),
    }
}

#[test]
fn test_empty_dd_pat_does_not_set_pat() {
    let _lock = ENV_MUTEX.lock().unwrap();
    let old_pat = env::var("DD_BEARER_TOKEN").ok();
    let old_app_key = env::var("DD_APP_KEY").ok();

    env::set_var("DD_BEARER_TOKEN", "");
    env::set_var("DD_APP_KEY", "my-app-key");

    let config = Configuration::default();

    let headers = config.auth_headers();
    assert!(headers
        .iter()
        .any(|(k, v)| k == "DD-APPLICATION-KEY" && v == "my-app-key"));

    // Restore env
    match old_pat {
        Some(v) => env::set_var("DD_BEARER_TOKEN", v),
        None => env::remove_var("DD_BEARER_TOKEN"),
    }
    match old_app_key {
        Some(v) => env::set_var("DD_APP_KEY", v),
        None => env::remove_var("DD_APP_KEY"),
    }
}
