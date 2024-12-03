// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use crate::datadog;
use flate2::{
    write::{GzEncoder, ZlibEncoder},
    Compression,
};
use reqwest::header::{HeaderMap, HeaderValue};
use serde::{Deserialize, Serialize};
use std::io::Write;

/// GetDataDeletionRequestsOptionalParams is a struct for passing parameters to the method [`DataDeletionAPI::get_data_deletion_requests`]
#[non_exhaustive]
#[derive(Clone, Default, Debug)]
pub struct GetDataDeletionRequestsOptionalParams {
    /// The next page of the previous search. If the next_page parameter is included, the rest of the query elements are ignored.
    pub next_page: Option<String>,
    /// Retrieve only the requests related to the given product.
    pub product: Option<String>,
    /// Retrieve only the requests that matches the given query.
    pub query: Option<String>,
    /// Retrieve only the requests with the given status.
    pub status: Option<String>,
    /// Sets the page size of the search.
    pub page_size: Option<i64>,
}

impl GetDataDeletionRequestsOptionalParams {
    /// The next page of the previous search. If the next_page parameter is included, the rest of the query elements are ignored.
    pub fn next_page(mut self, value: String) -> Self {
        self.next_page = Some(value);
        self
    }
    /// Retrieve only the requests related to the given product.
    pub fn product(mut self, value: String) -> Self {
        self.product = Some(value);
        self
    }
    /// Retrieve only the requests that matches the given query.
    pub fn query(mut self, value: String) -> Self {
        self.query = Some(value);
        self
    }
    /// Retrieve only the requests with the given status.
    pub fn status(mut self, value: String) -> Self {
        self.status = Some(value);
        self
    }
    /// Sets the page size of the search.
    pub fn page_size(mut self, value: i64) -> Self {
        self.page_size = Some(value);
        self
    }
}

/// CancelDataDeletionRequestError is a struct for typed errors of method [`DataDeletionAPI::cancel_data_deletion_request`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CancelDataDeletionRequestError {
    APIErrorResponse(crate::datadogV2::model::APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// CreateDataDeletionRequestError is a struct for typed errors of method [`DataDeletionAPI::create_data_deletion_request`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CreateDataDeletionRequestError {
    APIErrorResponse(crate::datadogV2::model::APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// GetDataDeletionRequestsError is a struct for typed errors of method [`DataDeletionAPI::get_data_deletion_requests`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetDataDeletionRequestsError {
    APIErrorResponse(crate::datadogV2::model::APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// The Data Deletion API allows the user to target and delete data from the allowed products. It's currently enabled for Logs and RUM and depends on `logs_delete_data` and `rum_delete_data` permissions respectively.
#[derive(Debug, Clone)]
pub struct DataDeletionAPI {
    config: datadog::Configuration,
    client: reqwest_middleware::ClientWithMiddleware,
}

impl Default for DataDeletionAPI {
    fn default() -> Self {
        Self::with_config(datadog::Configuration::default())
    }
}

impl DataDeletionAPI {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn with_config(config: datadog::Configuration) -> Self {
        let mut reqwest_client_builder = reqwest::Client::builder();

        if let Some(proxy_url) = &config.proxy_url {
            let proxy = reqwest::Proxy::all(proxy_url).expect("Failed to parse proxy URL");
            reqwest_client_builder = reqwest_client_builder.proxy(proxy);
        }

        let mut middleware_client_builder =
            reqwest_middleware::ClientBuilder::new(reqwest_client_builder.build().unwrap());

        if config.enable_retry {
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

            middleware_client_builder = middleware_client_builder.with(retry_middleware);
        }

        let client = middleware_client_builder.build();

        Self { config, client }
    }

    pub fn with_client_and_config(
        config: datadog::Configuration,
        client: reqwest_middleware::ClientWithMiddleware,
    ) -> Self {
        Self { config, client }
    }

    /// Cancels a data deletion request by providing its ID.
    pub async fn cancel_data_deletion_request(
        &self,
        id: String,
    ) -> Result<
        crate::datadogV2::model::CancelDataDeletionResponseBody,
        datadog::Error<CancelDataDeletionRequestError>,
    > {
        match self.cancel_data_deletion_request_with_http_info(id).await {
            Ok(response_content) => {
                if let Some(e) = response_content.entity {
                    Ok(e)
                } else {
                    Err(datadog::Error::Serde(serde::de::Error::custom(
                        "response content was None",
                    )))
                }
            }
            Err(err) => Err(err),
        }
    }

    /// Cancels a data deletion request by providing its ID.
    pub async fn cancel_data_deletion_request_with_http_info(
        &self,
        id: String,
    ) -> Result<
        datadog::ResponseContent<crate::datadogV2::model::CancelDataDeletionResponseBody>,
        datadog::Error<CancelDataDeletionRequestError>,
    > {
        let local_configuration = &self.config;
        let operation_id = "v2.cancel_data_deletion_request";

        let local_client = &self.client;

        let local_uri_str = format!(
            "{}/api/v2/deletion/requests/{id}/cancel",
            local_configuration.get_operation_host(operation_id),
            id = datadog::urlencode(id)
        );
        let mut local_req_builder =
            local_client.request(reqwest::Method::PUT, local_uri_str.as_str());

        // build headers
        let mut headers = HeaderMap::new();
        headers.insert("Accept", HeaderValue::from_static("application/json"));

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
            headers.insert(
                "DD-API-KEY",
                HeaderValue::from_str(local_key.key.as_str())
                    .expect("failed to parse DD-API-KEY header"),
            );
        };
        if let Some(local_key) = local_configuration.auth_keys.get("appKeyAuth") {
            headers.insert(
                "DD-APPLICATION-KEY",
                HeaderValue::from_str(local_key.key.as_str())
                    .expect("failed to parse DD-APPLICATION-KEY header"),
            );
        };

        local_req_builder = local_req_builder.headers(headers);
        let local_req = local_req_builder.build()?;
        log::debug!("request content: {:?}", local_req.body());
        let local_resp = local_client.execute(local_req).await?;

        let local_status = local_resp.status();
        let local_content = local_resp.text().await?;
        log::debug!("response content: {}", local_content);

        if !local_status.is_client_error() && !local_status.is_server_error() {
            match serde_json::from_str::<crate::datadogV2::model::CancelDataDeletionResponseBody>(
                &local_content,
            ) {
                Ok(e) => {
                    return Ok(datadog::ResponseContent {
                        status: local_status,
                        content: local_content,
                        entity: Some(e),
                    })
                }
                Err(e) => return Err(datadog::Error::Serde(e)),
            };
        } else {
            let local_entity: Option<CancelDataDeletionRequestError> =
                serde_json::from_str(&local_content).ok();
            let local_error = datadog::ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(datadog::Error::ResponseError(local_error))
        }
    }

    /// Creates a data deletion request by providing a query and a time frame targeting the proper data.
    pub async fn create_data_deletion_request(
        &self,
        product: String,
        body: crate::datadogV2::model::CreateDataDeletionRequestBody,
    ) -> Result<
        crate::datadogV2::model::CreateDataDeletionResponseBody,
        datadog::Error<CreateDataDeletionRequestError>,
    > {
        match self
            .create_data_deletion_request_with_http_info(product, body)
            .await
        {
            Ok(response_content) => {
                if let Some(e) = response_content.entity {
                    Ok(e)
                } else {
                    Err(datadog::Error::Serde(serde::de::Error::custom(
                        "response content was None",
                    )))
                }
            }
            Err(err) => Err(err),
        }
    }

    /// Creates a data deletion request by providing a query and a time frame targeting the proper data.
    pub async fn create_data_deletion_request_with_http_info(
        &self,
        product: String,
        body: crate::datadogV2::model::CreateDataDeletionRequestBody,
    ) -> Result<
        datadog::ResponseContent<crate::datadogV2::model::CreateDataDeletionResponseBody>,
        datadog::Error<CreateDataDeletionRequestError>,
    > {
        let local_configuration = &self.config;
        let operation_id = "v2.create_data_deletion_request";

        let local_client = &self.client;

        let local_uri_str = format!(
            "{}/api/v2/deletion/data/{product}",
            local_configuration.get_operation_host(operation_id),
            product = datadog::urlencode(product)
        );
        let mut local_req_builder =
            local_client.request(reqwest::Method::POST, local_uri_str.as_str());

        // build headers
        let mut headers = HeaderMap::new();
        headers.insert("Content-Type", HeaderValue::from_static("application/json"));
        headers.insert("Accept", HeaderValue::from_static("application/json"));

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
            headers.insert(
                "DD-API-KEY",
                HeaderValue::from_str(local_key.key.as_str())
                    .expect("failed to parse DD-API-KEY header"),
            );
        };
        if let Some(local_key) = local_configuration.auth_keys.get("appKeyAuth") {
            headers.insert(
                "DD-APPLICATION-KEY",
                HeaderValue::from_str(local_key.key.as_str())
                    .expect("failed to parse DD-APPLICATION-KEY header"),
            );
        };

        // build body parameters
        let output = Vec::new();
        let mut ser = serde_json::Serializer::with_formatter(output, datadog::DDFormatter);
        if body.serialize(&mut ser).is_ok() {
            if let Some(content_encoding) = headers.get("Content-Encoding") {
                match content_encoding.to_str().unwrap_or_default() {
                    "gzip" => {
                        let mut enc = GzEncoder::new(Vec::new(), Compression::default());
                        let _ = enc.write_all(ser.into_inner().as_slice());
                        match enc.finish() {
                            Ok(buf) => {
                                local_req_builder = local_req_builder.body(buf);
                            }
                            Err(e) => return Err(datadog::Error::Io(e)),
                        }
                    }
                    "deflate" => {
                        let mut enc = ZlibEncoder::new(Vec::new(), Compression::default());
                        let _ = enc.write_all(ser.into_inner().as_slice());
                        match enc.finish() {
                            Ok(buf) => {
                                local_req_builder = local_req_builder.body(buf);
                            }
                            Err(e) => return Err(datadog::Error::Io(e)),
                        }
                    }
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
                    }
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
            match serde_json::from_str::<crate::datadogV2::model::CreateDataDeletionResponseBody>(
                &local_content,
            ) {
                Ok(e) => {
                    return Ok(datadog::ResponseContent {
                        status: local_status,
                        content: local_content,
                        entity: Some(e),
                    })
                }
                Err(e) => return Err(datadog::Error::Serde(e)),
            };
        } else {
            let local_entity: Option<CreateDataDeletionRequestError> =
                serde_json::from_str(&local_content).ok();
            let local_error = datadog::ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(datadog::Error::ResponseError(local_error))
        }
    }

    /// Gets a list of data deletion requests based on several filter parameters.
    pub async fn get_data_deletion_requests(
        &self,
        params: GetDataDeletionRequestsOptionalParams,
    ) -> Result<
        crate::datadogV2::model::GetDataDeletionsResponseBody,
        datadog::Error<GetDataDeletionRequestsError>,
    > {
        match self.get_data_deletion_requests_with_http_info(params).await {
            Ok(response_content) => {
                if let Some(e) = response_content.entity {
                    Ok(e)
                } else {
                    Err(datadog::Error::Serde(serde::de::Error::custom(
                        "response content was None",
                    )))
                }
            }
            Err(err) => Err(err),
        }
    }

    /// Gets a list of data deletion requests based on several filter parameters.
    pub async fn get_data_deletion_requests_with_http_info(
        &self,
        params: GetDataDeletionRequestsOptionalParams,
    ) -> Result<
        datadog::ResponseContent<crate::datadogV2::model::GetDataDeletionsResponseBody>,
        datadog::Error<GetDataDeletionRequestsError>,
    > {
        let local_configuration = &self.config;
        let operation_id = "v2.get_data_deletion_requests";

        // unbox and build optional parameters
        let next_page = params.next_page;
        let product = params.product;
        let query = params.query;
        let status = params.status;
        let page_size = params.page_size;

        let local_client = &self.client;

        let local_uri_str = format!(
            "{}/api/v2/deletion/requests",
            local_configuration.get_operation_host(operation_id)
        );
        let mut local_req_builder =
            local_client.request(reqwest::Method::GET, local_uri_str.as_str());

        if let Some(ref local_query_param) = next_page {
            local_req_builder =
                local_req_builder.query(&[("next_page", &local_query_param.to_string())]);
        };
        if let Some(ref local_query_param) = product {
            local_req_builder =
                local_req_builder.query(&[("product", &local_query_param.to_string())]);
        };
        if let Some(ref local_query_param) = query {
            local_req_builder =
                local_req_builder.query(&[("query", &local_query_param.to_string())]);
        };
        if let Some(ref local_query_param) = status {
            local_req_builder =
                local_req_builder.query(&[("status", &local_query_param.to_string())]);
        };
        if let Some(ref local_query_param) = page_size {
            local_req_builder =
                local_req_builder.query(&[("page_size", &local_query_param.to_string())]);
        };

        // build headers
        let mut headers = HeaderMap::new();
        headers.insert("Accept", HeaderValue::from_static("application/json"));

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
            headers.insert(
                "DD-API-KEY",
                HeaderValue::from_str(local_key.key.as_str())
                    .expect("failed to parse DD-API-KEY header"),
            );
        };
        if let Some(local_key) = local_configuration.auth_keys.get("appKeyAuth") {
            headers.insert(
                "DD-APPLICATION-KEY",
                HeaderValue::from_str(local_key.key.as_str())
                    .expect("failed to parse DD-APPLICATION-KEY header"),
            );
        };

        local_req_builder = local_req_builder.headers(headers);
        let local_req = local_req_builder.build()?;
        log::debug!("request content: {:?}", local_req.body());
        let local_resp = local_client.execute(local_req).await?;

        let local_status = local_resp.status();
        let local_content = local_resp.text().await?;
        log::debug!("response content: {}", local_content);

        if !local_status.is_client_error() && !local_status.is_server_error() {
            match serde_json::from_str::<crate::datadogV2::model::GetDataDeletionsResponseBody>(
                &local_content,
            ) {
                Ok(e) => {
                    return Ok(datadog::ResponseContent {
                        status: local_status,
                        content: local_content,
                        entity: Some(e),
                    })
                }
                Err(e) => return Err(datadog::Error::Serde(e)),
            };
        } else {
            let local_entity: Option<GetDataDeletionRequestsError> =
                serde_json::from_str(&local_content).ok();
            let local_error = datadog::ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(datadog::Error::ResponseError(local_error))
        }
    }
}
