// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Serialize, Deserialize};
use crate::datadog;
use reqwest::header::{HeaderMap, HeaderValue};
use std::io::Write;
use flate2::{write::{GzEncoder, ZlibEncoder}, Compression};




/// CreateDashboardError is a struct for typed errors of method [`DashboardsAPI::create_dashboard`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CreateDashboardError {
    APIErrorResponse(crate::datadogV1::model::APIErrorResponse),
    UnknownValue(serde_json::Value),
}


/// GetDashboardError is a struct for typed errors of method [`DashboardsAPI::get_dashboard`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetDashboardError {
    APIErrorResponse(crate::datadogV1::model::APIErrorResponse),
    UnknownValue(serde_json::Value),
}


/// UpdateDashboardError is a struct for typed errors of method [`DashboardsAPI::update_dashboard`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum UpdateDashboardError {
    APIErrorResponse(crate::datadogV1::model::APIErrorResponse),
    UnknownValue(serde_json::Value),
}


/// Manage all your dashboards, as well as access to your shared dashboards, through the API. See the [Dashboards page](<https://docs.datadoghq.com/dashboards/>) for more information.
#[derive(Debug, Clone)]
pub struct DashboardsAPI {
    config: datadog::Configuration,
    client: reqwest_middleware::ClientWithMiddleware,
}

impl Default for DashboardsAPI {
    fn default() -> Self {
        Self::with_config(datadog::Configuration::default())
    }
}

impl DashboardsAPI {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn with_config(config: datadog::Configuration) -> Self {
        let reqwest_client_builder = {
            let builder = config.apply_headers(reqwest::Client::builder());
            #[cfg(not(target_arch = "wasm32"))]
            let builder = if let Some(proxy_url) = &config.proxy_url {
                builder
                    .proxy(reqwest::Proxy::all(proxy_url).expect("Failed to parse proxy URL"))
            } else {
                builder
            };
            builder
        };

        let middleware_client_builder = {
            let builder =
                reqwest_middleware::ClientBuilder::new(reqwest_client_builder.build().unwrap());
            #[cfg(feature = "retry")]
            let builder = if config.enable_retry {
                struct RetryableStatus;
                impl reqwest_retry::RetryableStrategy for RetryableStatus {
                    fn handle(
                        &self,
                        res: &Result<reqwest::Response, reqwest_middleware::Error>,
                    ) -> Option<reqwest_retry::Retryable> {
                        match res {
                            Ok(success) => reqwest_retry::default_on_request_success(success),
                            Err(_) => None,
                        }
                    }
                }
                let backoff_policy = reqwest_retry::policies::ExponentialBackoff::builder()
                    .build_with_max_retries(config.max_retries);

                let retry_middleware =
                    reqwest_retry::RetryTransientMiddleware::new_with_policy_and_strategy(
                        backoff_policy,
                        RetryableStatus,
                    );

                builder.with(retry_middleware)
            } else {
                builder
            };
            builder
        };

        let client = middleware_client_builder.build();

        Self { config, client }
    }

    pub fn with_client_and_config(config: datadog::Configuration, client: reqwest_middleware::ClientWithMiddleware) -> Self {
        Self {config, client}
    }

    
    
    /// Create a dashboard using the specified options. When defining queries in your widgets, take note of which queries should have the `as_count()` or `as_rate()` modifiers appended.
/// Refer to the following [documentation](<https://docs.datadoghq.com/developers/metrics/type_modifiers/?tab=count#in-application-modifiers>) for more information on these modifiers.
    pub async fn create_dashboard(&self, body: crate::datadogV1_20270101::model::Dashboard) -> Result<crate::datadogV1_20270101::model::Dashboard, datadog::Error<CreateDashboardError>> {
        match self.create_dashboard_with_http_info(body).await {
            Ok(response_content) => {
                if let Some(e) = response_content.entity {
                    Ok(e)
                } else {
                    Err(datadog::Error::Serde(serde::de::Error::custom("response content was None")))
                }
            },
            Err(err) => Err(err),
        }
    }

    
    /// Create a dashboard using the specified options. When defining queries in your widgets, take note of which queries should have the `as_count()` or `as_rate()` modifiers appended.
/// Refer to the following [documentation](<https://docs.datadoghq.com/developers/metrics/type_modifiers/?tab=count#in-application-modifiers>) for more information on these modifiers.
    pub async fn create_dashboard_with_http_info(&self, body: crate::datadogV1_20270101::model::Dashboard) -> Result<datadog::ResponseContent<crate::datadogV1_20270101::model::Dashboard>, datadog::Error<CreateDashboardError>> {
        let local_configuration = &self.config;
        let local_operation_id = "v1_20270101.create_dashboard";

        

        let local_client = &self.client;

        let local_uri_str = format!(
            "{}/api/v1/dashboard",
            local_configuration.get_operation_host(local_operation_id));
        let mut local_req_builder = local_client.request(reqwest::Method::POST, local_uri_str.as_str());

        

        // build headers
        let mut headers = HeaderMap::new();
        headers.insert("Content-Type", HeaderValue::from_static("application/json"));
        headers.insert("Accept", HeaderValue::from_static("application/json"));
        headers.insert("DD-API-Version", HeaderValue::from_static("2027-01-01"));

        

        // build user agent
        match HeaderValue::from_str(local_configuration.user_agent.as_str()) {
            Ok(user_agent) => headers.insert(reqwest::header::USER_AGENT, user_agent),
            Err(e) => {
                log::warn!("Failed to parse user agent header: {e}, falling back to default");
                headers.insert(
                    reqwest::header::USER_AGENT,
                    HeaderValue::from_static(datadog::DEFAULT_USER_AGENT.as_str()),
                )
            }
        };
        
        // build auth
        if let Some(local_key) = local_configuration.auth_keys.get("apiKeyAuth") {
            headers.insert("DD-API-KEY", HeaderValue::from_str(local_key.key.as_str()).expect("failed to parse DD-API-KEY header"));
        };
        if let Some(local_key) = local_configuration.auth_keys.get("appKeyAuth") {
            headers.insert("DD-APPLICATION-KEY", HeaderValue::from_str(local_key.key.as_str()).expect("failed to parse DD-APPLICATION-KEY header"));
        };

        
        // build body parameters
        let output = Vec::new();
        let mut ser = serde_json::Serializer::with_formatter(output, datadog::DDFormatter);
        if body.serialize(&mut ser).is_ok() {
            if let Some(content_encoding) = headers.get("Content-Encoding") {
                match content_encoding.to_str().unwrap_or_default() {
                    "gzip" => {
                        let mut enc = GzEncoder::new(
                            Vec::new(),
                            Compression::default(),
                        );
                        let _ = enc.write_all(ser.into_inner().as_slice());
                        match enc.finish() {
                            Ok(buf) => {
                                local_req_builder = local_req_builder.body(buf);
                            }
                            Err(e) => return Err(datadog::Error::Io(e)),
                        }
                    }
                    "deflate" => {
                        let mut enc = ZlibEncoder::new(
                            Vec::new(),
                            Compression::default(),
                        );
                        let _ = enc.write_all(ser.into_inner().as_slice());
                        match enc.finish() {
                            Ok(buf) => {
                                local_req_builder = local_req_builder.body(buf);
                            }
                            Err(e) => return Err(datadog::Error::Io(e)),
                        }
                    }
                    #[cfg(feature = "zstd")]
                    "zstd1" => {
                        let mut enc = zstd::stream::Encoder::new(Vec::new(), 0).unwrap();
                        let _ = enc.write_all(ser.into_inner().as_slice());
                        match enc.finish() {
                            Ok(buf) => {
                                local_req_builder = local_req_builder.body(buf);
                            }
                            Err(e) => return Err(datadog::Error::Io(e)),
                        }
                    }
                    _ => {
                        local_req_builder = local_req_builder.body(ser.into_inner());
                    },
                }
            } else {
                local_req_builder = local_req_builder.body(ser.into_inner());
            }
        }

        local_req_builder = local_req_builder.headers(headers);
        let local_req = local_req_builder.build()?;
        log::debug!("request content: {:?}", local_req.body());
        let local_resp = local_client.execute(local_req).await?;

        let local_status = local_resp.status();
        let local_content = local_resp.text().await?;
        log::debug!("response content: {}", local_content);

        if !local_status.is_client_error() && !local_status.is_server_error() {
            match serde_json::from_str::<crate::datadogV1_20270101::model::Dashboard>(&local_content) {
                Ok(e) => return Ok(datadog::ResponseContent {
                    status: local_status,
                    content: local_content,
                    entity: Some(e),
                }),
                Err(e) => return Err(datadog::Error::Serde(e)),
            };
        } else {
            let local_entity: Option<CreateDashboardError> = serde_json::from_str(&local_content).ok();
            let local_error = datadog::ResponseContent { status: local_status, content: local_content, entity: local_entity };
            Err(datadog::Error::ResponseError(local_error))
        }
    }
    
    /// Get a dashboard using the specified ID.
    pub async fn get_dashboard(&self, dashboard_id: String) -> Result<crate::datadogV1_20270101::model::Dashboard, datadog::Error<GetDashboardError>> {
        match self.get_dashboard_with_http_info(dashboard_id).await {
            Ok(response_content) => {
                if let Some(e) = response_content.entity {
                    Ok(e)
                } else {
                    Err(datadog::Error::Serde(serde::de::Error::custom("response content was None")))
                }
            },
            Err(err) => Err(err),
        }
    }

    
    /// Get a dashboard using the specified ID.
    pub async fn get_dashboard_with_http_info(&self, dashboard_id: String) -> Result<datadog::ResponseContent<crate::datadogV1_20270101::model::Dashboard>, datadog::Error<GetDashboardError>> {
        let local_configuration = &self.config;
        let local_operation_id = "v1_20270101.get_dashboard";

        

        let local_client = &self.client;

        let local_uri_str = format!(
            "{}/api/v1/dashboard/{dashboard_id}",
            local_configuration.get_operation_host(local_operation_id), dashboard_id=
            datadog::urlencode(dashboard_id)
            );
        let mut local_req_builder = local_client.request(reqwest::Method::GET, local_uri_str.as_str());

        

        // build headers
        let mut headers = HeaderMap::new();
        headers.insert("Accept", HeaderValue::from_static("application/json"));
        headers.insert("DD-API-Version", HeaderValue::from_static("2027-01-01"));

        

        // build user agent
        match HeaderValue::from_str(local_configuration.user_agent.as_str()) {
            Ok(user_agent) => headers.insert(reqwest::header::USER_AGENT, user_agent),
            Err(e) => {
                log::warn!("Failed to parse user agent header: {e}, falling back to default");
                headers.insert(
                    reqwest::header::USER_AGENT,
                    HeaderValue::from_static(datadog::DEFAULT_USER_AGENT.as_str()),
                )
            }
        };
        
        // build auth
        if let Some(local_key) = local_configuration.auth_keys.get("apiKeyAuth") {
            headers.insert("DD-API-KEY", HeaderValue::from_str(local_key.key.as_str()).expect("failed to parse DD-API-KEY header"));
        };
        if let Some(local_key) = local_configuration.auth_keys.get("appKeyAuth") {
            headers.insert("DD-APPLICATION-KEY", HeaderValue::from_str(local_key.key.as_str()).expect("failed to parse DD-APPLICATION-KEY header"));
        };

        

        local_req_builder = local_req_builder.headers(headers);
        let local_req = local_req_builder.build()?;
        log::debug!("request content: {:?}", local_req.body());
        let local_resp = local_client.execute(local_req).await?;

        let local_status = local_resp.status();
        let local_content = local_resp.text().await?;
        log::debug!("response content: {}", local_content);

        if !local_status.is_client_error() && !local_status.is_server_error() {
            match serde_json::from_str::<crate::datadogV1_20270101::model::Dashboard>(&local_content) {
                Ok(e) => return Ok(datadog::ResponseContent {
                    status: local_status,
                    content: local_content,
                    entity: Some(e),
                }),
                Err(e) => return Err(datadog::Error::Serde(e)),
            };
        } else {
            let local_entity: Option<GetDashboardError> = serde_json::from_str(&local_content).ok();
            let local_error = datadog::ResponseContent { status: local_status, content: local_content, entity: local_entity };
            Err(datadog::Error::ResponseError(local_error))
        }
    }
    
    /// Update a dashboard using the specified ID.
    pub async fn update_dashboard(&self, dashboard_id: String, body: crate::datadogV1_20270101::model::Dashboard) -> Result<crate::datadogV1_20270101::model::Dashboard, datadog::Error<UpdateDashboardError>> {
        match self.update_dashboard_with_http_info(dashboard_id, body).await {
            Ok(response_content) => {
                if let Some(e) = response_content.entity {
                    Ok(e)
                } else {
                    Err(datadog::Error::Serde(serde::de::Error::custom("response content was None")))
                }
            },
            Err(err) => Err(err),
        }
    }

    
    /// Update a dashboard using the specified ID.
    pub async fn update_dashboard_with_http_info(&self, dashboard_id: String, body: crate::datadogV1_20270101::model::Dashboard) -> Result<datadog::ResponseContent<crate::datadogV1_20270101::model::Dashboard>, datadog::Error<UpdateDashboardError>> {
        let local_configuration = &self.config;
        let local_operation_id = "v1_20270101.update_dashboard";

        

        let local_client = &self.client;

        let local_uri_str = format!(
            "{}/api/v1/dashboard/{dashboard_id}",
            local_configuration.get_operation_host(local_operation_id), dashboard_id=
            datadog::urlencode(dashboard_id)
            );
        let mut local_req_builder = local_client.request(reqwest::Method::PUT, local_uri_str.as_str());

        

        // build headers
        let mut headers = HeaderMap::new();
        headers.insert("Content-Type", HeaderValue::from_static("application/json"));
        headers.insert("Accept", HeaderValue::from_static("application/json"));
        headers.insert("DD-API-Version", HeaderValue::from_static("2027-01-01"));

        

        // build user agent
        match HeaderValue::from_str(local_configuration.user_agent.as_str()) {
            Ok(user_agent) => headers.insert(reqwest::header::USER_AGENT, user_agent),
            Err(e) => {
                log::warn!("Failed to parse user agent header: {e}, falling back to default");
                headers.insert(
                    reqwest::header::USER_AGENT,
                    HeaderValue::from_static(datadog::DEFAULT_USER_AGENT.as_str()),
                )
            }
        };
        
        // build auth
        if let Some(local_key) = local_configuration.auth_keys.get("apiKeyAuth") {
            headers.insert("DD-API-KEY", HeaderValue::from_str(local_key.key.as_str()).expect("failed to parse DD-API-KEY header"));
        };
        if let Some(local_key) = local_configuration.auth_keys.get("appKeyAuth") {
            headers.insert("DD-APPLICATION-KEY", HeaderValue::from_str(local_key.key.as_str()).expect("failed to parse DD-APPLICATION-KEY header"));
        };

        
        // build body parameters
        let output = Vec::new();
        let mut ser = serde_json::Serializer::with_formatter(output, datadog::DDFormatter);
        if body.serialize(&mut ser).is_ok() {
            if let Some(content_encoding) = headers.get("Content-Encoding") {
                match content_encoding.to_str().unwrap_or_default() {
                    "gzip" => {
                        let mut enc = GzEncoder::new(
                            Vec::new(),
                            Compression::default(),
                        );
                        let _ = enc.write_all(ser.into_inner().as_slice());
                        match enc.finish() {
                            Ok(buf) => {
                                local_req_builder = local_req_builder.body(buf);
                            }
                            Err(e) => return Err(datadog::Error::Io(e)),
                        }
                    }
                    "deflate" => {
                        let mut enc = ZlibEncoder::new(
                            Vec::new(),
                            Compression::default(),
                        );
                        let _ = enc.write_all(ser.into_inner().as_slice());
                        match enc.finish() {
                            Ok(buf) => {
                                local_req_builder = local_req_builder.body(buf);
                            }
                            Err(e) => return Err(datadog::Error::Io(e)),
                        }
                    }
                    #[cfg(feature = "zstd")]
                    "zstd1" => {
                        let mut enc = zstd::stream::Encoder::new(Vec::new(), 0).unwrap();
                        let _ = enc.write_all(ser.into_inner().as_slice());
                        match enc.finish() {
                            Ok(buf) => {
                                local_req_builder = local_req_builder.body(buf);
                            }
                            Err(e) => return Err(datadog::Error::Io(e)),
                        }
                    }
                    _ => {
                        local_req_builder = local_req_builder.body(ser.into_inner());
                    },
                }
            } else {
                local_req_builder = local_req_builder.body(ser.into_inner());
            }
        }

        local_req_builder = local_req_builder.headers(headers);
        let local_req = local_req_builder.build()?;
        log::debug!("request content: {:?}", local_req.body());
        let local_resp = local_client.execute(local_req).await?;

        let local_status = local_resp.status();
        let local_content = local_resp.text().await?;
        log::debug!("response content: {}", local_content);

        if !local_status.is_client_error() && !local_status.is_server_error() {
            match serde_json::from_str::<crate::datadogV1_20270101::model::Dashboard>(&local_content) {
                Ok(e) => return Ok(datadog::ResponseContent {
                    status: local_status,
                    content: local_content,
                    entity: Some(e),
                }),
                Err(e) => return Err(datadog::Error::Serde(e)),
            };
        } else {
            let local_entity: Option<UpdateDashboardError> = serde_json::from_str(&local_content).ok();
            let local_error = datadog::ResponseContent { status: local_status, content: local_content, entity: local_entity };
            Err(datadog::Error::ResponseError(local_error))
        }
    }
}