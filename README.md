# datadog-api-client-rust

This repository contains a Rust API client for the [Datadog API](https://docs.datadoghq.com/api/).

For more information, please visit [https://www.datadoghq.com/support/](https://www.datadoghq.com/support/)

## Overview

This API client was generated from OpenAPI specs by code and templates in the `.generator/` folder.

## Requirements

Building this API client library requires:  
``TODO``

## Installation

Add the following to `Cargo.toml` under `[dependencies]`:

```
datadog-api-client = { git = "ssh://git@github.com/DataDog/datadog-api-client-rust.git", branch = "master" }
```

## Getting Started

Please follow the [installation](#installation) instruction and execute the following Rust code:

```Rust
use datadog_api_client::datadog::configuration::Configuration;
use datadog_api_client::datadogV1::api::api_monitors::{MonitorsAPI, GetMonitorOptionalParams};

#[tokio::main]
async fn main() {
    env_logger::init();
    let configuration = Configuration::new();
    let monitors_api = MonitorsAPI::with_config(configuration.clone());

    let mut monitor_payload = GetMonitorOptionalParams::default();
    monitor_payload.group_states("all".into());

    let resp = monitors_api.get_monitor(1234567, monitor_payload).await;
    if let Ok(Some(value)) = resp {
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

This client includes access to Datadog API endpoints while they are in an unstable state and may undergo breaking changes. An extra configuration step is required to enable these endpoints:

```rust
configuration.set_unstable_operation_enabled(<OPERATION_NAME>, True )
```
where <OPERATION_NAME> is the API version and name of the method used to interact with that endpoint. For example: `v2.list_incidents`, or `v2.query_timeseries_data`

## Documentation for API Endpoints

To get access to the crate's generated documentation, use:

```
cargo doc --open
```

## Author

support@datadoghq.com

