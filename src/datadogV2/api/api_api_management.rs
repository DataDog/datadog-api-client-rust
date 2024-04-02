// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use crate::datadog;
use log::warn;
use reqwest;
use reqwest::header::{HeaderMap, HeaderValue};
use serde::{Deserialize, Serialize};

/// CreateOpenAPIOptionalParams is a struct for passing parameters to the method [`APIManagementAPI::create_open_api`]
#[non_exhaustive]
#[derive(Clone, Default, Debug)]
pub struct CreateOpenAPIOptionalParams {
    /// Binary `OpenAPI` spec file
    pub openapi_spec_file: Option<Vec<u8>>,
}

impl CreateOpenAPIOptionalParams {
    /// Binary `OpenAPI` spec file
    pub fn openapi_spec_file(mut self, value: Vec<u8>) -> Self {
        self.openapi_spec_file = Some(value);
        self
    }
}

/// UpdateOpenAPIOptionalParams is a struct for passing parameters to the method [`APIManagementAPI::update_open_api`]
#[non_exhaustive]
#[derive(Clone, Default, Debug)]
pub struct UpdateOpenAPIOptionalParams {
    /// Binary `OpenAPI` spec file
    pub openapi_spec_file: Option<Vec<u8>>,
}

impl UpdateOpenAPIOptionalParams {
    /// Binary `OpenAPI` spec file
    pub fn openapi_spec_file(mut self, value: Vec<u8>) -> Self {
        self.openapi_spec_file = Some(value);
        self
    }
}

/// CreateOpenAPIError is a struct for typed errors of method [`APIManagementAPI::create_open_api`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CreateOpenAPIError {
    Status400(Option<crate::datadogV2::model::JSONAPIErrorResponse>),
    Status403(Option<crate::datadogV2::model::JSONAPIErrorResponse>),
    Status429(Option<crate::datadogV2::model::APIErrorResponse>),
    UnknownValue(serde_json::Value),
}

/// DeleteOpenAPIError is a struct for typed errors of method [`APIManagementAPI::delete_open_api`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DeleteOpenAPIError {
    Status400(Option<crate::datadogV2::model::JSONAPIErrorResponse>),
    Status403(Option<crate::datadogV2::model::JSONAPIErrorResponse>),
    Status404(Option<crate::datadogV2::model::JSONAPIErrorResponse>),
    Status429(Option<crate::datadogV2::model::APIErrorResponse>),
    UnknownValue(serde_json::Value),
}

/// GetOpenAPIError is a struct for typed errors of method [`APIManagementAPI::get_open_api`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetOpenAPIError {
    Status400(Option<crate::datadogV2::model::JSONAPIErrorResponse>),
    Status403(Option<crate::datadogV2::model::JSONAPIErrorResponse>),
    Status404(Option<crate::datadogV2::model::JSONAPIErrorResponse>),
    Status429(Option<crate::datadogV2::model::APIErrorResponse>),
    UnknownValue(serde_json::Value),
}

/// UpdateOpenAPIError is a struct for typed errors of method [`APIManagementAPI::update_open_api`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum UpdateOpenAPIError {
    Status400(Option<crate::datadogV2::model::JSONAPIErrorResponse>),
    Status403(Option<crate::datadogV2::model::JSONAPIErrorResponse>),
    Status404(Option<crate::datadogV2::model::JSONAPIErrorResponse>),
    Status429(Option<crate::datadogV2::model::APIErrorResponse>),
    UnknownValue(serde_json::Value),
}

#[derive(Debug, Clone)]
pub struct APIManagementAPI {
    config: datadog::Configuration,
    client: reqwest_middleware::ClientWithMiddleware,
}

impl Default for APIManagementAPI {
    fn default() -> Self {
        Self::with_config(datadog::Configuration::default())
    }
}

impl APIManagementAPI {
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

    /// Create a new API from the [OpenAPI](<https://spec.openapis.org/oas/latest.html>) specification given.
    /// It supports version `2.0`, `3.0` and `3.1` of the specification. A specific extension section, `x-datadog`,
    /// let you specify the `teamHandle` for your team responsible for the API in Datadog.
    /// It returns the created API ID.
    ///
    pub async fn create_open_api(
        &self,
        params: CreateOpenAPIOptionalParams,
    ) -> Result<crate::datadogV2::model::CreateOpenAPIResponse, datadog::Error<CreateOpenAPIError>>
    {
        match self.create_open_api_with_http_info(params).await {
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

    /// Create a new API from the [OpenAPI](<https://spec.openapis.org/oas/latest.html>) specification given.
    /// It supports version `2.0`, `3.0` and `3.1` of the specification. A specific extension section, `x-datadog`,
    /// let you specify the `teamHandle` for your team responsible for the API in Datadog.
    /// It returns the created API ID.
    ///
    pub async fn create_open_api_with_http_info(
        &self,
        params: CreateOpenAPIOptionalParams,
    ) -> Result<
        datadog::ResponseContent<crate::datadogV2::model::CreateOpenAPIResponse>,
        datadog::Error<CreateOpenAPIError>,
    > {
        let local_configuration = &self.config;
        let operation_id = "v2.create_open_api";
        if local_configuration.is_unstable_operation_enabled(operation_id) {
            warn!("Using unstable operation {operation_id}");
        } else {
            let local_error = datadog::UnstableOperationDisabledError {
                msg: "Operation 'v2.create_open_api' is not enabled".to_string(),
            };
            return Err(datadog::Error::UnstableOperationDisabledError(local_error));
        }

        // unbox and build optional parameters
        let openapi_spec_file = params.openapi_spec_file;

        let local_client = &self.client;

        let local_uri_str = format!(
            "{}/api/v2/apicatalog/openapi",
            local_configuration.get_operation_host(operation_id)
        );
        let mut local_req_builder =
            local_client.request(reqwest::Method::POST, local_uri_str.as_str());

        // build headers
        let mut headers = HeaderMap::new();
        headers.insert(
            "Content-Type",
            HeaderValue::from_static("multipart/form-data"),
        );
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

        // build form parameters
        if let Some(openapi_spec_file) = openapi_spec_file {
            let mut local_form = reqwest::multipart::Form::new();
            local_form = local_form.part(
                "openapi_spec_file",
                reqwest::multipart::Part::bytes(openapi_spec_file).file_name("openapi_spec_file"),
            );
            headers.insert(
                "Content-Type",
                format!("multipart/form-data; boundary={}", local_form.boundary())
                    .parse()
                    .unwrap(),
            );
            local_req_builder = local_req_builder.multipart(local_form);
        };

        local_req_builder = local_req_builder.headers(headers);
        let local_req = local_req_builder.build()?;
        let local_resp = local_client.execute(local_req).await?;

        let local_status = local_resp.status();
        let local_content = local_resp.text().await?;

        if !local_status.is_client_error() && !local_status.is_server_error() {
            match serde_json::from_str::<crate::datadogV2::model::CreateOpenAPIResponse>(
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
            let local_entity: Option<CreateOpenAPIError> =
                serde_json::from_str(&local_content).ok();
            let local_error = datadog::ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(datadog::Error::ResponseError(local_error))
        }
    }

    /// Delete a specific API by ID.
    pub async fn delete_open_api(
        &self,
        id: String,
    ) -> Result<(), datadog::Error<DeleteOpenAPIError>> {
        match self.delete_open_api_with_http_info(id).await {
            Ok(_) => Ok(()),
            Err(err) => Err(err),
        }
    }

    /// Delete a specific API by ID.
    pub async fn delete_open_api_with_http_info(
        &self,
        id: String,
    ) -> Result<datadog::ResponseContent<()>, datadog::Error<DeleteOpenAPIError>> {
        let local_configuration = &self.config;
        let operation_id = "v2.delete_open_api";
        if local_configuration.is_unstable_operation_enabled(operation_id) {
            warn!("Using unstable operation {operation_id}");
        } else {
            let local_error = datadog::UnstableOperationDisabledError {
                msg: "Operation 'v2.delete_open_api' is not enabled".to_string(),
            };
            return Err(datadog::Error::UnstableOperationDisabledError(local_error));
        }

        let local_client = &self.client;

        let local_uri_str = format!(
            "{}/api/v2/apicatalog/api/{id}",
            local_configuration.get_operation_host(operation_id),
            id = datadog::urlencode(id)
        );
        let mut local_req_builder =
            local_client.request(reqwest::Method::DELETE, local_uri_str.as_str());

        // build headers
        let mut headers = HeaderMap::new();
        headers.insert("Accept", HeaderValue::from_static("*/*"));

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
        let local_resp = local_client.execute(local_req).await?;

        let local_status = local_resp.status();
        let local_content = local_resp.text().await?;

        if !local_status.is_client_error() && !local_status.is_server_error() {
            Ok(datadog::ResponseContent {
                status: local_status,
                content: local_content,
                entity: None,
            })
        } else {
            let local_entity: Option<DeleteOpenAPIError> =
                serde_json::from_str(&local_content).ok();
            let local_error = datadog::ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(datadog::Error::ResponseError(local_error))
        }
    }

    /// Retrieve information about a specific API in [OpenAPI](<https://spec.openapis.org/oas/latest.html>) format file.
    pub async fn get_open_api(
        &self,
        id: String,
    ) -> Result<Vec<u8>, datadog::Error<GetOpenAPIError>> {
        match self.get_open_api_with_http_info(id).await {
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

    /// Retrieve information about a specific API in [OpenAPI](<https://spec.openapis.org/oas/latest.html>) format file.
    pub async fn get_open_api_with_http_info(
        &self,
        id: String,
    ) -> Result<datadog::ResponseContent<Vec<u8>>, datadog::Error<GetOpenAPIError>> {
        let local_configuration = &self.config;
        let operation_id = "v2.get_open_api";
        if local_configuration.is_unstable_operation_enabled(operation_id) {
            warn!("Using unstable operation {operation_id}");
        } else {
            let local_error = datadog::UnstableOperationDisabledError {
                msg: "Operation 'v2.get_open_api' is not enabled".to_string(),
            };
            return Err(datadog::Error::UnstableOperationDisabledError(local_error));
        }

        let local_client = &self.client;

        let local_uri_str = format!(
            "{}/api/v2/apicatalog/api/{id}/openapi",
            local_configuration.get_operation_host(operation_id),
            id = datadog::urlencode(id)
        );
        let mut local_req_builder =
            local_client.request(reqwest::Method::GET, local_uri_str.as_str());

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
        let local_resp = local_client.execute(local_req).await?;

        let local_status = local_resp.status();
        let local_content = local_resp.text().await?;

        if !local_status.is_client_error() && !local_status.is_server_error() {
            Ok(datadog::ResponseContent {
                status: local_status,
                content: local_content.clone(),
                entity: Some(local_content.into_bytes()),
            })
        } else {
            let local_entity: Option<GetOpenAPIError> = serde_json::from_str(&local_content).ok();
            let local_error = datadog::ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(datadog::Error::ResponseError(local_error))
        }
    }

    /// Update information about a specific API. The given content will replace all API content of the given ID.
    /// The ID is returned by the create API, or can be found in the URL in the API catalog UI.
    ///
    pub async fn update_open_api(
        &self,
        id: String,
        params: UpdateOpenAPIOptionalParams,
    ) -> Result<crate::datadogV2::model::UpdateOpenAPIResponse, datadog::Error<UpdateOpenAPIError>>
    {
        match self.update_open_api_with_http_info(id, params).await {
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

    /// Update information about a specific API. The given content will replace all API content of the given ID.
    /// The ID is returned by the create API, or can be found in the URL in the API catalog UI.
    ///
    pub async fn update_open_api_with_http_info(
        &self,
        id: String,
        params: UpdateOpenAPIOptionalParams,
    ) -> Result<
        datadog::ResponseContent<crate::datadogV2::model::UpdateOpenAPIResponse>,
        datadog::Error<UpdateOpenAPIError>,
    > {
        let local_configuration = &self.config;
        let operation_id = "v2.update_open_api";
        if local_configuration.is_unstable_operation_enabled(operation_id) {
            warn!("Using unstable operation {operation_id}");
        } else {
            let local_error = datadog::UnstableOperationDisabledError {
                msg: "Operation 'v2.update_open_api' is not enabled".to_string(),
            };
            return Err(datadog::Error::UnstableOperationDisabledError(local_error));
        }

        // unbox and build optional parameters
        let openapi_spec_file = params.openapi_spec_file;

        let local_client = &self.client;

        let local_uri_str = format!(
            "{}/api/v2/apicatalog/api/{id}/openapi",
            local_configuration.get_operation_host(operation_id),
            id = datadog::urlencode(id)
        );
        let mut local_req_builder =
            local_client.request(reqwest::Method::PUT, local_uri_str.as_str());

        // build headers
        let mut headers = HeaderMap::new();
        headers.insert(
            "Content-Type",
            HeaderValue::from_static("multipart/form-data"),
        );
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

        // build form parameters
        if let Some(openapi_spec_file) = openapi_spec_file {
            let mut local_form = reqwest::multipart::Form::new();
            local_form = local_form.part(
                "openapi_spec_file",
                reqwest::multipart::Part::bytes(openapi_spec_file).file_name("openapi_spec_file"),
            );
            headers.insert(
                "Content-Type",
                format!("multipart/form-data; boundary={}", local_form.boundary())
                    .parse()
                    .unwrap(),
            );
            local_req_builder = local_req_builder.multipart(local_form);
        };

        local_req_builder = local_req_builder.headers(headers);
        let local_req = local_req_builder.build()?;
        let local_resp = local_client.execute(local_req).await?;

        let local_status = local_resp.status();
        let local_content = local_resp.text().await?;

        if !local_status.is_client_error() && !local_status.is_server_error() {
            match serde_json::from_str::<crate::datadogV2::model::UpdateOpenAPIResponse>(
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
            let local_entity: Option<UpdateOpenAPIError> =
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
