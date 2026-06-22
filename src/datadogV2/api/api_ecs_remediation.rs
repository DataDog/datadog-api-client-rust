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

/// ListEcsRemediationInvestigationsOptionalParams is a struct for passing parameters to the method [`ECSRemediationAPI::list_ecs_remediation_investigations`]
#[non_exhaustive]
#[derive(Clone, Default, Debug)]
pub struct ListEcsRemediationInvestigationsOptionalParams {
    /// Filter by ECS cluster ARN.
    pub cluster_arn: Option<String>,
    /// Filter by ECS cluster name.
    pub cluster_name: Option<String>,
    /// Filter by ECS service ARN.
    pub service_arn: Option<String>,
    /// Filter by ECS task ARN.
    pub task_arn: Option<String>,
    /// Filter by resource ARN. Matches services, daemons, and standalone tasks.
    pub resource_arn: Option<String>,
    /// Filter by investigation status. Repeatable. Accepted values: open, approval_required, executing, succeeded, failed. Unknown values are ignored.
    pub status: Option<Vec<String>>,
    /// Filter by issue type.
    pub issue_type: Option<String>,
    /// Start of the created-at time range, in epoch milliseconds.
    pub since_ms: Option<i64>,
    /// End of the created-at time range, in epoch milliseconds.
    pub until_ms: Option<i64>,
    /// Maximum number of investigations to return. Clamped to 200.
    pub page_size: Option<i32>,
    /// Pagination token returned by a previous call.
    pub page_token: Option<String>,
}

impl ListEcsRemediationInvestigationsOptionalParams {
    /// Filter by ECS cluster ARN.
    pub fn cluster_arn(mut self, value: String) -> Self {
        self.cluster_arn = Some(value);
        self
    }
    /// Filter by ECS cluster name.
    pub fn cluster_name(mut self, value: String) -> Self {
        self.cluster_name = Some(value);
        self
    }
    /// Filter by ECS service ARN.
    pub fn service_arn(mut self, value: String) -> Self {
        self.service_arn = Some(value);
        self
    }
    /// Filter by ECS task ARN.
    pub fn task_arn(mut self, value: String) -> Self {
        self.task_arn = Some(value);
        self
    }
    /// Filter by resource ARN. Matches services, daemons, and standalone tasks.
    pub fn resource_arn(mut self, value: String) -> Self {
        self.resource_arn = Some(value);
        self
    }
    /// Filter by investigation status. Repeatable. Accepted values: open, approval_required, executing, succeeded, failed. Unknown values are ignored.
    pub fn status(mut self, value: Vec<String>) -> Self {
        self.status = Some(value);
        self
    }
    /// Filter by issue type.
    pub fn issue_type(mut self, value: String) -> Self {
        self.issue_type = Some(value);
        self
    }
    /// Start of the created-at time range, in epoch milliseconds.
    pub fn since_ms(mut self, value: i64) -> Self {
        self.since_ms = Some(value);
        self
    }
    /// End of the created-at time range, in epoch milliseconds.
    pub fn until_ms(mut self, value: i64) -> Self {
        self.until_ms = Some(value);
        self
    }
    /// Maximum number of investigations to return. Clamped to 200.
    pub fn page_size(mut self, value: i32) -> Self {
        self.page_size = Some(value);
        self
    }
    /// Pagination token returned by a previous call.
    pub fn page_token(mut self, value: String) -> Self {
        self.page_token = Some(value);
        self
    }
}

/// ExecuteRemediationError is a struct for typed errors of method [`ECSRemediationAPI::execute_remediation`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ExecuteRemediationError {
    APIErrorResponse(crate::datadogV2::model::APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// GetEcsRemediationInvestigationError is a struct for typed errors of method [`ECSRemediationAPI::get_ecs_remediation_investigation`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetEcsRemediationInvestigationError {
    APIErrorResponse(crate::datadogV2::model::APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// ListEcsRemediationInvestigationsError is a struct for typed errors of method [`ECSRemediationAPI::list_ecs_remediation_investigations`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ListEcsRemediationInvestigationsError {
    APIErrorResponse(crate::datadogV2::model::APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// Manage ECS remediation investigations and execute proposed remediation plans.
#[derive(Debug, Clone)]
pub struct ECSRemediationAPI {
    config: datadog::Configuration,
    client: reqwest_middleware::ClientWithMiddleware,
}

impl Default for ECSRemediationAPI {
    fn default() -> Self {
        Self::with_config(datadog::Configuration::default())
    }
}

impl ECSRemediationAPI {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn with_config(config: datadog::Configuration) -> Self {
        let reqwest_client_builder = {
            let builder = reqwest::Client::builder();
            #[cfg(not(target_arch = "wasm32"))]
            let builder = if let Some(proxy_url) = &config.proxy_url {
                builder.proxy(reqwest::Proxy::all(proxy_url).expect("Failed to parse proxy URL"))
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

    pub fn with_client_and_config(
        config: datadog::Configuration,
        client: reqwest_middleware::ClientWithMiddleware,
    ) -> Self {
        Self { config, client }
    }

    /// Trigger execution of the proposed remediation for an investigation through the configured AWS connection. The investigation must belong to the caller's organization.
    pub async fn execute_remediation(
        &self,
        body: crate::datadogV2::model::RemediationExecuteRequest,
    ) -> Result<
        crate::datadogV2::model::RemediationExecuteResponse,
        datadog::Error<ExecuteRemediationError>,
    > {
        match self.execute_remediation_with_http_info(body).await {
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

    /// Trigger execution of the proposed remediation for an investigation through the configured AWS connection. The investigation must belong to the caller's organization.
    pub async fn execute_remediation_with_http_info(
        &self,
        body: crate::datadogV2::model::RemediationExecuteRequest,
    ) -> Result<
        datadog::ResponseContent<crate::datadogV2::model::RemediationExecuteResponse>,
        datadog::Error<ExecuteRemediationError>,
    > {
        let local_configuration = &self.config;
        let operation_id = "v2.execute_remediation";

        let local_client = &self.client;

        let local_uri_str = format!(
            "{}/api/ui/orchestration/ecs_remediation/execute",
            local_configuration.get_operation_host(operation_id)
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
            match serde_json::from_str::<crate::datadogV2::model::RemediationExecuteResponse>(
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
            let local_entity: Option<ExecuteRemediationError> =
                serde_json::from_str(&local_content).ok();
            let local_error = datadog::ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(datadog::Error::ResponseError(local_error))
        }
    }

    /// Get a single ECS remediation investigation by ID, including its proposed plan, history, and ECS workload metadata.
    pub async fn get_ecs_remediation_investigation(
        &self,
        id: String,
    ) -> Result<
        crate::datadogV2::model::RemediationGetInvestigationResponse,
        datadog::Error<GetEcsRemediationInvestigationError>,
    > {
        match self
            .get_ecs_remediation_investigation_with_http_info(id)
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

    /// Get a single ECS remediation investigation by ID, including its proposed plan, history, and ECS workload metadata.
    pub async fn get_ecs_remediation_investigation_with_http_info(
        &self,
        id: String,
    ) -> Result<
        datadog::ResponseContent<crate::datadogV2::model::RemediationGetInvestigationResponse>,
        datadog::Error<GetEcsRemediationInvestigationError>,
    > {
        let local_configuration = &self.config;
        let operation_id = "v2.get_ecs_remediation_investigation";

        let local_client = &self.client;

        let local_uri_str = format!(
            "{}/api/ui/orchestration/ecs_remediation/investigation",
            local_configuration.get_operation_host(operation_id)
        );
        let mut local_req_builder =
            local_client.request(reqwest::Method::GET, local_uri_str.as_str());

        local_req_builder = local_req_builder.query(&[("id", &id.to_string())]);

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
            match serde_json::from_str::<crate::datadogV2::model::RemediationGetInvestigationResponse>(
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
            let local_entity: Option<GetEcsRemediationInvestigationError> =
                serde_json::from_str(&local_content).ok();
            let local_error = datadog::ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(datadog::Error::ResponseError(local_error))
        }
    }

    /// List ECS remediation investigations for the caller's organization. All filters are optional and applied together. Results are returned in pages.
    pub async fn list_ecs_remediation_investigations(
        &self,
        params: ListEcsRemediationInvestigationsOptionalParams,
    ) -> Result<
        crate::datadogV2::model::RemediationListInvestigationsResponse,
        datadog::Error<ListEcsRemediationInvestigationsError>,
    > {
        match self
            .list_ecs_remediation_investigations_with_http_info(params)
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

    /// List ECS remediation investigations for the caller's organization. All filters are optional and applied together. Results are returned in pages.
    pub async fn list_ecs_remediation_investigations_with_http_info(
        &self,
        params: ListEcsRemediationInvestigationsOptionalParams,
    ) -> Result<
        datadog::ResponseContent<crate::datadogV2::model::RemediationListInvestigationsResponse>,
        datadog::Error<ListEcsRemediationInvestigationsError>,
    > {
        let local_configuration = &self.config;
        let operation_id = "v2.list_ecs_remediation_investigations";

        // unbox and build optional parameters
        let cluster_arn = params.cluster_arn;
        let cluster_name = params.cluster_name;
        let service_arn = params.service_arn;
        let task_arn = params.task_arn;
        let resource_arn = params.resource_arn;
        let status = params.status;
        let issue_type = params.issue_type;
        let since_ms = params.since_ms;
        let until_ms = params.until_ms;
        let page_size = params.page_size;
        let page_token = params.page_token;

        let local_client = &self.client;

        let local_uri_str = format!(
            "{}/api/ui/orchestration/ecs_remediation/investigations",
            local_configuration.get_operation_host(operation_id)
        );
        let mut local_req_builder =
            local_client.request(reqwest::Method::GET, local_uri_str.as_str());

        if let Some(ref local_query_param) = cluster_arn {
            local_req_builder =
                local_req_builder.query(&[("cluster_arn", &local_query_param.to_string())]);
        };
        if let Some(ref local_query_param) = cluster_name {
            local_req_builder =
                local_req_builder.query(&[("cluster_name", &local_query_param.to_string())]);
        };
        if let Some(ref local_query_param) = service_arn {
            local_req_builder =
                local_req_builder.query(&[("service_arn", &local_query_param.to_string())]);
        };
        if let Some(ref local_query_param) = task_arn {
            local_req_builder =
                local_req_builder.query(&[("task_arn", &local_query_param.to_string())]);
        };
        if let Some(ref local_query_param) = resource_arn {
            local_req_builder =
                local_req_builder.query(&[("resource_arn", &local_query_param.to_string())]);
        };
        if let Some(ref local) = status {
            for param in local {
                local_req_builder = local_req_builder.query(&[("status", &param.to_string())]);
            }
        };
        if let Some(ref local_query_param) = issue_type {
            local_req_builder =
                local_req_builder.query(&[("issue_type", &local_query_param.to_string())]);
        };
        if let Some(ref local_query_param) = since_ms {
            local_req_builder =
                local_req_builder.query(&[("since_ms", &local_query_param.to_string())]);
        };
        if let Some(ref local_query_param) = until_ms {
            local_req_builder =
                local_req_builder.query(&[("until_ms", &local_query_param.to_string())]);
        };
        if let Some(ref local_query_param) = page_size {
            local_req_builder =
                local_req_builder.query(&[("page_size", &local_query_param.to_string())]);
        };
        if let Some(ref local_query_param) = page_token {
            local_req_builder =
                local_req_builder.query(&[("page_token", &local_query_param.to_string())]);
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
            match serde_json::from_str::<
                crate::datadogV2::model::RemediationListInvestigationsResponse,
            >(&local_content)
            {
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
            let local_entity: Option<ListEcsRemediationInvestigationsError> =
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
