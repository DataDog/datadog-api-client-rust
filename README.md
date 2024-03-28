# Datadog API Client

Rust HTTP client for the [Datadog API](https://docs.datadoghq.com/api/).

To submit support or feature requests, please visit [https://www.datadoghq.com/support/](https://www.datadoghq.com/support/)

## Overview

This API client was generated from Datadog's public OpenAPI specs. Generator code and templates can be found in the repository's `.generator/` folder.

## Installation

Run `cargo add datadog-api-client` or add the following to `Cargo.toml` under `[dependencies]`:

```
datadog-api-client = "0.0.1"
```

## Getting Started

Please follow the [installation](#installation) instructions and try the following snippet to validate your Datadog API key:

```Rust
use datadog_api_client::datadog::configuration::Configuration;
use datadog_api_client::datadogV1::api::api_authentication::AuthenticationAPI;

#[tokio::main]
async fn main() {
    let configuration = Configuration::new();
    let api = AuthenticationAPI::with_config(configuration);
    let resp = api.validate().await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
```

### Authentication

By default the library will use the `DD_API_KEY` and `DD_APP_KEY` environment variables to authenticate against the Datadog API.  
To provide your own set of credentials, you need to set some keys on the configuration:

```rust
configuration.set_auth_key(
    "apiKeyAuth",
    APIKey {
        key: "<DD-API-KEY>".to_string(),
        prefix: "".to_owned(),
    },
);
configuration.set_auth_key(
    "appKeyAuth",
    APIKey {
        key: "<DD-APP-KEY>".to_string(),
        prefix: "".to_owned(),
    },
);
```

### Unstable Endpoints

This client includes access to Datadog API endpoints while they are in an unstable state and may undergo breaking changes. An extra configuration step is required to use these endpoints:

```rust
configuration.set_unstable_operation_enabled("<OPERATION_NAME>", True)
```
where `<OPERATION_NAME>` is the API version and name of the method used to interact with that endpoint. For example: `v2.list_incidents`, or `v2.query_timeseries_data`

## Author

support@datadoghq.com

