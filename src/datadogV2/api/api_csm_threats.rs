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

/// DeleteCSMThreatsAgentRuleOptionalParams is a struct for passing parameters to the method [`CSMThreatsAPI::delete_csm_threats_agent_rule`]
#[non_exhaustive]
#[derive(Clone, Default, Debug)]
pub struct DeleteCSMThreatsAgentRuleOptionalParams {
    /// The ID of the Agent policy
    pub policy_id: Option<String>,
}

impl DeleteCSMThreatsAgentRuleOptionalParams {
    /// The ID of the Agent policy
    pub fn policy_id(mut self, value: String) -> Self {
        self.policy_id = Some(value);
        self
    }
}

/// GetCSMThreatsAgentRuleOptionalParams is a struct for passing parameters to the method [`CSMThreatsAPI::get_csm_threats_agent_rule`]
#[non_exhaustive]
#[derive(Clone, Default, Debug)]
pub struct GetCSMThreatsAgentRuleOptionalParams {
    /// The ID of the Agent policy
    pub policy_id: Option<String>,
}

impl GetCSMThreatsAgentRuleOptionalParams {
    /// The ID of the Agent policy
    pub fn policy_id(mut self, value: String) -> Self {
        self.policy_id = Some(value);
        self
    }
}

/// ListCSMThreatsAgentRulesOptionalParams is a struct for passing parameters to the method [`CSMThreatsAPI::list_csm_threats_agent_rules`]
#[non_exhaustive]
#[derive(Clone, Default, Debug)]
pub struct ListCSMThreatsAgentRulesOptionalParams {
    /// The ID of the Agent policy
    pub policy_id: Option<String>,
}

impl ListCSMThreatsAgentRulesOptionalParams {
    /// The ID of the Agent policy
    pub fn policy_id(mut self, value: String) -> Self {
        self.policy_id = Some(value);
        self
    }
}

/// UpdateCSMThreatsAgentRuleOptionalParams is a struct for passing parameters to the method [`CSMThreatsAPI::update_csm_threats_agent_rule`]
#[non_exhaustive]
#[derive(Clone, Default, Debug)]
pub struct UpdateCSMThreatsAgentRuleOptionalParams {
    /// The ID of the Agent policy
    pub policy_id: Option<String>,
}

impl UpdateCSMThreatsAgentRuleOptionalParams {
    /// The ID of the Agent policy
    pub fn policy_id(mut self, value: String) -> Self {
        self.policy_id = Some(value);
        self
    }
}

/// CreateCSMThreatsAgentPolicyError is a struct for typed errors of method [`CSMThreatsAPI::create_csm_threats_agent_policy`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CreateCSMThreatsAgentPolicyError {
    APIErrorResponse(crate::datadogV2::model::APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// CreateCSMThreatsAgentRuleError is a struct for typed errors of method [`CSMThreatsAPI::create_csm_threats_agent_rule`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CreateCSMThreatsAgentRuleError {
    APIErrorResponse(crate::datadogV2::model::APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// CreateCloudWorkloadSecurityAgentRuleError is a struct for typed errors of method [`CSMThreatsAPI::create_cloud_workload_security_agent_rule`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CreateCloudWorkloadSecurityAgentRuleError {
    APIErrorResponse(crate::datadogV2::model::APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// DeleteCSMThreatsAgentPolicyError is a struct for typed errors of method [`CSMThreatsAPI::delete_csm_threats_agent_policy`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DeleteCSMThreatsAgentPolicyError {
    APIErrorResponse(crate::datadogV2::model::APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// DeleteCSMThreatsAgentRuleError is a struct for typed errors of method [`CSMThreatsAPI::delete_csm_threats_agent_rule`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DeleteCSMThreatsAgentRuleError {
    APIErrorResponse(crate::datadogV2::model::APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// DeleteCloudWorkloadSecurityAgentRuleError is a struct for typed errors of method [`CSMThreatsAPI::delete_cloud_workload_security_agent_rule`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DeleteCloudWorkloadSecurityAgentRuleError {
    APIErrorResponse(crate::datadogV2::model::APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// DownloadCSMThreatsPolicyError is a struct for typed errors of method [`CSMThreatsAPI::download_csm_threats_policy`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DownloadCSMThreatsPolicyError {
    APIErrorResponse(crate::datadogV2::model::APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// DownloadCloudWorkloadPolicyFileError is a struct for typed errors of method [`CSMThreatsAPI::download_cloud_workload_policy_file`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DownloadCloudWorkloadPolicyFileError {
    APIErrorResponse(crate::datadogV2::model::APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// GetCSMThreatsAgentPolicyError is a struct for typed errors of method [`CSMThreatsAPI::get_csm_threats_agent_policy`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetCSMThreatsAgentPolicyError {
    APIErrorResponse(crate::datadogV2::model::APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// GetCSMThreatsAgentRuleError is a struct for typed errors of method [`CSMThreatsAPI::get_csm_threats_agent_rule`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetCSMThreatsAgentRuleError {
    APIErrorResponse(crate::datadogV2::model::APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// GetCloudWorkloadSecurityAgentRuleError is a struct for typed errors of method [`CSMThreatsAPI::get_cloud_workload_security_agent_rule`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetCloudWorkloadSecurityAgentRuleError {
    APIErrorResponse(crate::datadogV2::model::APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// ListCSMThreatsAgentPoliciesError is a struct for typed errors of method [`CSMThreatsAPI::list_csm_threats_agent_policies`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ListCSMThreatsAgentPoliciesError {
    APIErrorResponse(crate::datadogV2::model::APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// ListCSMThreatsAgentRulesError is a struct for typed errors of method [`CSMThreatsAPI::list_csm_threats_agent_rules`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ListCSMThreatsAgentRulesError {
    APIErrorResponse(crate::datadogV2::model::APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// ListCloudWorkloadSecurityAgentRulesError is a struct for typed errors of method [`CSMThreatsAPI::list_cloud_workload_security_agent_rules`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ListCloudWorkloadSecurityAgentRulesError {
    APIErrorResponse(crate::datadogV2::model::APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// UpdateCSMThreatsAgentPolicyError is a struct for typed errors of method [`CSMThreatsAPI::update_csm_threats_agent_policy`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum UpdateCSMThreatsAgentPolicyError {
    APIErrorResponse(crate::datadogV2::model::APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// UpdateCSMThreatsAgentRuleError is a struct for typed errors of method [`CSMThreatsAPI::update_csm_threats_agent_rule`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum UpdateCSMThreatsAgentRuleError {
    APIErrorResponse(crate::datadogV2::model::APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// UpdateCloudWorkloadSecurityAgentRuleError is a struct for typed errors of method [`CSMThreatsAPI::update_cloud_workload_security_agent_rule`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum UpdateCloudWorkloadSecurityAgentRuleError {
    APIErrorResponse(crate::datadogV2::model::APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// Workload Protection monitors file, network, and process activity across your environment to detect real-time threats to your infrastructure. See [Workload Protection](<https://docs.datadoghq.com/security/workload_protection/>) for more information on setting up Workload Protection.
///
/// **Note**: These endpoints are split based on whether you are using the US1-FED site or not. Please reference the specific resource for the site you are using.
#[derive(Debug, Clone)]
pub struct CSMThreatsAPI {
    config: datadog::Configuration,
    client: reqwest_middleware::ClientWithMiddleware,
}

impl Default for CSMThreatsAPI {
    fn default() -> Self {
        Self::with_config(datadog::Configuration::default())
    }
}

impl CSMThreatsAPI {
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

    /// Create a new Workload Protection policy with the given parameters.
    ///
    /// **Note**: This endpoint is not available for the Government (US1-FED) site. Please reference the (US1-FED) specific resource below.
    pub async fn create_csm_threats_agent_policy(
        &self,
        body: crate::datadogV2::model::CloudWorkloadSecurityAgentPolicyCreateRequest,
    ) -> Result<
        crate::datadogV2::model::CloudWorkloadSecurityAgentPolicyResponse,
        datadog::Error<CreateCSMThreatsAgentPolicyError>,
    > {
        match self
            .create_csm_threats_agent_policy_with_http_info(body)
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

    /// Create a new Workload Protection policy with the given parameters.
    ///
    /// **Note**: This endpoint is not available for the Government (US1-FED) site. Please reference the (US1-FED) specific resource below.
    pub async fn create_csm_threats_agent_policy_with_http_info(
        &self,
        body: crate::datadogV2::model::CloudWorkloadSecurityAgentPolicyCreateRequest,
    ) -> Result<
        datadog::ResponseContent<crate::datadogV2::model::CloudWorkloadSecurityAgentPolicyResponse>,
        datadog::Error<CreateCSMThreatsAgentPolicyError>,
    > {
        let local_configuration = &self.config;
        let operation_id = "v2.create_csm_threats_agent_policy";

        let local_client = &self.client;

        let local_uri_str = format!(
            "{}/api/v2/remote_config/products/cws/policy",
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
            match serde_json::from_str::<
                crate::datadogV2::model::CloudWorkloadSecurityAgentPolicyResponse,
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
            let local_entity: Option<CreateCSMThreatsAgentPolicyError> =
                serde_json::from_str(&local_content).ok();
            let local_error = datadog::ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(datadog::Error::ResponseError(local_error))
        }
    }

    /// Create a new Workload Protection agent rule with the given parameters.
    ///
    /// **Note**: This endpoint is not available for the Government (US1-FED) site. Please reference the (US1-FED) specific resource below.
    pub async fn create_csm_threats_agent_rule(
        &self,
        body: crate::datadogV2::model::CloudWorkloadSecurityAgentRuleCreateRequest,
    ) -> Result<
        crate::datadogV2::model::CloudWorkloadSecurityAgentRuleResponse,
        datadog::Error<CreateCSMThreatsAgentRuleError>,
    > {
        match self
            .create_csm_threats_agent_rule_with_http_info(body)
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

    /// Create a new Workload Protection agent rule with the given parameters.
    ///
    /// **Note**: This endpoint is not available for the Government (US1-FED) site. Please reference the (US1-FED) specific resource below.
    pub async fn create_csm_threats_agent_rule_with_http_info(
        &self,
        body: crate::datadogV2::model::CloudWorkloadSecurityAgentRuleCreateRequest,
    ) -> Result<
        datadog::ResponseContent<crate::datadogV2::model::CloudWorkloadSecurityAgentRuleResponse>,
        datadog::Error<CreateCSMThreatsAgentRuleError>,
    > {
        let local_configuration = &self.config;
        let operation_id = "v2.create_csm_threats_agent_rule";

        let local_client = &self.client;

        let local_uri_str = format!(
            "{}/api/v2/remote_config/products/cws/agent_rules",
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
            match serde_json::from_str::<
                crate::datadogV2::model::CloudWorkloadSecurityAgentRuleResponse,
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
            let local_entity: Option<CreateCSMThreatsAgentRuleError> =
                serde_json::from_str(&local_content).ok();
            let local_error = datadog::ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(datadog::Error::ResponseError(local_error))
        }
    }

    /// Create a new agent rule with the given parameters.
    ///
    /// **Note**: This endpoint should only be used for the Government (US1-FED) site.
    pub async fn create_cloud_workload_security_agent_rule(
        &self,
        body: crate::datadogV2::model::CloudWorkloadSecurityAgentRuleCreateRequest,
    ) -> Result<
        crate::datadogV2::model::CloudWorkloadSecurityAgentRuleResponse,
        datadog::Error<CreateCloudWorkloadSecurityAgentRuleError>,
    > {
        match self
            .create_cloud_workload_security_agent_rule_with_http_info(body)
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

    /// Create a new agent rule with the given parameters.
    ///
    /// **Note**: This endpoint should only be used for the Government (US1-FED) site.
    pub async fn create_cloud_workload_security_agent_rule_with_http_info(
        &self,
        body: crate::datadogV2::model::CloudWorkloadSecurityAgentRuleCreateRequest,
    ) -> Result<
        datadog::ResponseContent<crate::datadogV2::model::CloudWorkloadSecurityAgentRuleResponse>,
        datadog::Error<CreateCloudWorkloadSecurityAgentRuleError>,
    > {
        let local_configuration = &self.config;
        let operation_id = "v2.create_cloud_workload_security_agent_rule";

        let local_client = &self.client;

        let local_uri_str = format!(
            "{}/api/v2/security_monitoring/cloud_workload_security/agent_rules",
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
            match serde_json::from_str::<
                crate::datadogV2::model::CloudWorkloadSecurityAgentRuleResponse,
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
            let local_entity: Option<CreateCloudWorkloadSecurityAgentRuleError> =
                serde_json::from_str(&local_content).ok();
            let local_error = datadog::ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(datadog::Error::ResponseError(local_error))
        }
    }

    /// Delete a specific Workload Protection policy.
    ///
    /// **Note**: This endpoint is not available for the Government (US1-FED) site. Please reference the (US1-FED) specific resource below.
    pub async fn delete_csm_threats_agent_policy(
        &self,
        policy_id: String,
    ) -> Result<(), datadog::Error<DeleteCSMThreatsAgentPolicyError>> {
        match self
            .delete_csm_threats_agent_policy_with_http_info(policy_id)
            .await
        {
            Ok(_) => Ok(()),
            Err(err) => Err(err),
        }
    }

    /// Delete a specific Workload Protection policy.
    ///
    /// **Note**: This endpoint is not available for the Government (US1-FED) site. Please reference the (US1-FED) specific resource below.
    pub async fn delete_csm_threats_agent_policy_with_http_info(
        &self,
        policy_id: String,
    ) -> Result<datadog::ResponseContent<()>, datadog::Error<DeleteCSMThreatsAgentPolicyError>>
    {
        let local_configuration = &self.config;
        let operation_id = "v2.delete_csm_threats_agent_policy";

        let local_client = &self.client;

        let local_uri_str = format!(
            "{}/api/v2/remote_config/products/cws/policy/{policy_id}",
            local_configuration.get_operation_host(operation_id),
            policy_id = datadog::urlencode(policy_id)
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
        log::debug!("request content: {:?}", local_req.body());
        let local_resp = local_client.execute(local_req).await?;

        let local_status = local_resp.status();
        let local_content = local_resp.text().await?;
        log::debug!("response content: {}", local_content);

        if !local_status.is_client_error() && !local_status.is_server_error() {
            Ok(datadog::ResponseContent {
                status: local_status,
                content: local_content,
                entity: None,
            })
        } else {
            let local_entity: Option<DeleteCSMThreatsAgentPolicyError> =
                serde_json::from_str(&local_content).ok();
            let local_error = datadog::ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(datadog::Error::ResponseError(local_error))
        }
    }

    /// Delete a specific Workload Protection agent rule.
    ///
    /// **Note**: This endpoint is not available for the Government (US1-FED) site. Please reference the (US1-FED) specific resource below.
    pub async fn delete_csm_threats_agent_rule(
        &self,
        agent_rule_id: String,
        params: DeleteCSMThreatsAgentRuleOptionalParams,
    ) -> Result<(), datadog::Error<DeleteCSMThreatsAgentRuleError>> {
        match self
            .delete_csm_threats_agent_rule_with_http_info(agent_rule_id, params)
            .await
        {
            Ok(_) => Ok(()),
            Err(err) => Err(err),
        }
    }

    /// Delete a specific Workload Protection agent rule.
    ///
    /// **Note**: This endpoint is not available for the Government (US1-FED) site. Please reference the (US1-FED) specific resource below.
    pub async fn delete_csm_threats_agent_rule_with_http_info(
        &self,
        agent_rule_id: String,
        params: DeleteCSMThreatsAgentRuleOptionalParams,
    ) -> Result<datadog::ResponseContent<()>, datadog::Error<DeleteCSMThreatsAgentRuleError>> {
        let local_configuration = &self.config;
        let operation_id = "v2.delete_csm_threats_agent_rule";

        // unbox and build optional parameters
        let policy_id = params.policy_id;

        let local_client = &self.client;

        let local_uri_str = format!(
            "{}/api/v2/remote_config/products/cws/agent_rules/{agent_rule_id}",
            local_configuration.get_operation_host(operation_id),
            agent_rule_id = datadog::urlencode(agent_rule_id)
        );
        let mut local_req_builder =
            local_client.request(reqwest::Method::DELETE, local_uri_str.as_str());

        if let Some(ref local_query_param) = policy_id {
            local_req_builder =
                local_req_builder.query(&[("policy_id", &local_query_param.to_string())]);
        };

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
        log::debug!("request content: {:?}", local_req.body());
        let local_resp = local_client.execute(local_req).await?;

        let local_status = local_resp.status();
        let local_content = local_resp.text().await?;
        log::debug!("response content: {}", local_content);

        if !local_status.is_client_error() && !local_status.is_server_error() {
            Ok(datadog::ResponseContent {
                status: local_status,
                content: local_content,
                entity: None,
            })
        } else {
            let local_entity: Option<DeleteCSMThreatsAgentRuleError> =
                serde_json::from_str(&local_content).ok();
            let local_error = datadog::ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(datadog::Error::ResponseError(local_error))
        }
    }

    /// Delete a specific agent rule.
    ///
    /// **Note**: This endpoint should only be used for the Government (US1-FED) site.
    pub async fn delete_cloud_workload_security_agent_rule(
        &self,
        agent_rule_id: String,
    ) -> Result<(), datadog::Error<DeleteCloudWorkloadSecurityAgentRuleError>> {
        match self
            .delete_cloud_workload_security_agent_rule_with_http_info(agent_rule_id)
            .await
        {
            Ok(_) => Ok(()),
            Err(err) => Err(err),
        }
    }

    /// Delete a specific agent rule.
    ///
    /// **Note**: This endpoint should only be used for the Government (US1-FED) site.
    pub async fn delete_cloud_workload_security_agent_rule_with_http_info(
        &self,
        agent_rule_id: String,
    ) -> Result<
        datadog::ResponseContent<()>,
        datadog::Error<DeleteCloudWorkloadSecurityAgentRuleError>,
    > {
        let local_configuration = &self.config;
        let operation_id = "v2.delete_cloud_workload_security_agent_rule";

        let local_client = &self.client;

        let local_uri_str = format!(
            "{}/api/v2/security_monitoring/cloud_workload_security/agent_rules/{agent_rule_id}",
            local_configuration.get_operation_host(operation_id),
            agent_rule_id = datadog::urlencode(agent_rule_id)
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
        log::debug!("request content: {:?}", local_req.body());
        let local_resp = local_client.execute(local_req).await?;

        let local_status = local_resp.status();
        let local_content = local_resp.text().await?;
        log::debug!("response content: {}", local_content);

        if !local_status.is_client_error() && !local_status.is_server_error() {
            Ok(datadog::ResponseContent {
                status: local_status,
                content: local_content,
                entity: None,
            })
        } else {
            let local_entity: Option<DeleteCloudWorkloadSecurityAgentRuleError> =
                serde_json::from_str(&local_content).ok();
            let local_error = datadog::ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(datadog::Error::ResponseError(local_error))
        }
    }

    /// The download endpoint generates a Workload Protection policy file from your currently active
    /// Workload Protection agent rules, and downloads them as a `.policy` file. This file can then be deployed to
    /// your agents to update the policy running in your environment.
    ///
    /// **Note**: This endpoint is not available for the Government (US1-FED) site. Please reference the (US1-FED) specific resource below.
    pub async fn download_csm_threats_policy(
        &self,
    ) -> Result<Vec<u8>, datadog::Error<DownloadCSMThreatsPolicyError>> {
        match self.download_csm_threats_policy_with_http_info().await {
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

    /// The download endpoint generates a Workload Protection policy file from your currently active
    /// Workload Protection agent rules, and downloads them as a `.policy` file. This file can then be deployed to
    /// your agents to update the policy running in your environment.
    ///
    /// **Note**: This endpoint is not available for the Government (US1-FED) site. Please reference the (US1-FED) specific resource below.
    pub async fn download_csm_threats_policy_with_http_info(
        &self,
    ) -> Result<datadog::ResponseContent<Vec<u8>>, datadog::Error<DownloadCSMThreatsPolicyError>>
    {
        let local_configuration = &self.config;
        let operation_id = "v2.download_csm_threats_policy";

        let local_client = &self.client;

        let local_uri_str = format!(
            "{}/api/v2/remote_config/products/cws/policy/download",
            local_configuration.get_operation_host(operation_id)
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
        log::debug!("request content: {:?}", local_req.body());
        let local_resp = local_client.execute(local_req).await?;

        let local_status = local_resp.status();
        let local_content = local_resp.text().await?;
        log::debug!("response content: {}", local_content);

        if !local_status.is_client_error() && !local_status.is_server_error() {
            Ok(datadog::ResponseContent {
                status: local_status,
                content: local_content.clone(),
                entity: Some(local_content.into_bytes()),
            })
        } else {
            let local_entity: Option<DownloadCSMThreatsPolicyError> =
                serde_json::from_str(&local_content).ok();
            let local_error = datadog::ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(datadog::Error::ResponseError(local_error))
        }
    }

    /// The download endpoint generates a Workload Protection policy file from your currently active
    /// Workload Protection agent rules, and downloads them as a `.policy` file. This file can then be deployed to
    /// your agents to update the policy running in your environment.
    ///
    /// **Note**: This endpoint should only be used for the Government (US1-FED) site.
    pub async fn download_cloud_workload_policy_file(
        &self,
    ) -> Result<Vec<u8>, datadog::Error<DownloadCloudWorkloadPolicyFileError>> {
        match self
            .download_cloud_workload_policy_file_with_http_info()
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

    /// The download endpoint generates a Workload Protection policy file from your currently active
    /// Workload Protection agent rules, and downloads them as a `.policy` file. This file can then be deployed to
    /// your agents to update the policy running in your environment.
    ///
    /// **Note**: This endpoint should only be used for the Government (US1-FED) site.
    pub async fn download_cloud_workload_policy_file_with_http_info(
        &self,
    ) -> Result<
        datadog::ResponseContent<Vec<u8>>,
        datadog::Error<DownloadCloudWorkloadPolicyFileError>,
    > {
        let local_configuration = &self.config;
        let operation_id = "v2.download_cloud_workload_policy_file";

        let local_client = &self.client;

        let local_uri_str = format!(
            "{}/api/v2/security/cloud_workload/policy/download",
            local_configuration.get_operation_host(operation_id)
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
        log::debug!("request content: {:?}", local_req.body());
        let local_resp = local_client.execute(local_req).await?;

        let local_status = local_resp.status();
        let local_content = local_resp.text().await?;
        log::debug!("response content: {}", local_content);

        if !local_status.is_client_error() && !local_status.is_server_error() {
            Ok(datadog::ResponseContent {
                status: local_status,
                content: local_content.clone(),
                entity: Some(local_content.into_bytes()),
            })
        } else {
            let local_entity: Option<DownloadCloudWorkloadPolicyFileError> =
                serde_json::from_str(&local_content).ok();
            let local_error = datadog::ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(datadog::Error::ResponseError(local_error))
        }
    }

    /// Get the details of a specific Workload Protection policy.
    ///
    /// **Note**: This endpoint is not available for the Government (US1-FED) site. Please reference the (US1-FED) specific resource below.
    pub async fn get_csm_threats_agent_policy(
        &self,
        policy_id: String,
    ) -> Result<
        crate::datadogV2::model::CloudWorkloadSecurityAgentPolicyResponse,
        datadog::Error<GetCSMThreatsAgentPolicyError>,
    > {
        match self
            .get_csm_threats_agent_policy_with_http_info(policy_id)
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

    /// Get the details of a specific Workload Protection policy.
    ///
    /// **Note**: This endpoint is not available for the Government (US1-FED) site. Please reference the (US1-FED) specific resource below.
    pub async fn get_csm_threats_agent_policy_with_http_info(
        &self,
        policy_id: String,
    ) -> Result<
        datadog::ResponseContent<crate::datadogV2::model::CloudWorkloadSecurityAgentPolicyResponse>,
        datadog::Error<GetCSMThreatsAgentPolicyError>,
    > {
        let local_configuration = &self.config;
        let operation_id = "v2.get_csm_threats_agent_policy";

        let local_client = &self.client;

        let local_uri_str = format!(
            "{}/api/v2/remote_config/products/cws/policy/{policy_id}",
            local_configuration.get_operation_host(operation_id),
            policy_id = datadog::urlencode(policy_id)
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
        log::debug!("request content: {:?}", local_req.body());
        let local_resp = local_client.execute(local_req).await?;

        let local_status = local_resp.status();
        let local_content = local_resp.text().await?;
        log::debug!("response content: {}", local_content);

        if !local_status.is_client_error() && !local_status.is_server_error() {
            match serde_json::from_str::<
                crate::datadogV2::model::CloudWorkloadSecurityAgentPolicyResponse,
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
            let local_entity: Option<GetCSMThreatsAgentPolicyError> =
                serde_json::from_str(&local_content).ok();
            let local_error = datadog::ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(datadog::Error::ResponseError(local_error))
        }
    }

    /// Get the details of a specific Workload Protection agent rule.
    ///
    /// **Note**: This endpoint is not available for the Government (US1-FED) site. Please reference the (US1-FED) specific resource below.
    pub async fn get_csm_threats_agent_rule(
        &self,
        agent_rule_id: String,
        params: GetCSMThreatsAgentRuleOptionalParams,
    ) -> Result<
        crate::datadogV2::model::CloudWorkloadSecurityAgentRuleResponse,
        datadog::Error<GetCSMThreatsAgentRuleError>,
    > {
        match self
            .get_csm_threats_agent_rule_with_http_info(agent_rule_id, params)
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

    /// Get the details of a specific Workload Protection agent rule.
    ///
    /// **Note**: This endpoint is not available for the Government (US1-FED) site. Please reference the (US1-FED) specific resource below.
    pub async fn get_csm_threats_agent_rule_with_http_info(
        &self,
        agent_rule_id: String,
        params: GetCSMThreatsAgentRuleOptionalParams,
    ) -> Result<
        datadog::ResponseContent<crate::datadogV2::model::CloudWorkloadSecurityAgentRuleResponse>,
        datadog::Error<GetCSMThreatsAgentRuleError>,
    > {
        let local_configuration = &self.config;
        let operation_id = "v2.get_csm_threats_agent_rule";

        // unbox and build optional parameters
        let policy_id = params.policy_id;

        let local_client = &self.client;

        let local_uri_str = format!(
            "{}/api/v2/remote_config/products/cws/agent_rules/{agent_rule_id}",
            local_configuration.get_operation_host(operation_id),
            agent_rule_id = datadog::urlencode(agent_rule_id)
        );
        let mut local_req_builder =
            local_client.request(reqwest::Method::GET, local_uri_str.as_str());

        if let Some(ref local_query_param) = policy_id {
            local_req_builder =
                local_req_builder.query(&[("policy_id", &local_query_param.to_string())]);
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
                crate::datadogV2::model::CloudWorkloadSecurityAgentRuleResponse,
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
            let local_entity: Option<GetCSMThreatsAgentRuleError> =
                serde_json::from_str(&local_content).ok();
            let local_error = datadog::ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(datadog::Error::ResponseError(local_error))
        }
    }

    /// Get the details of a specific agent rule.
    ///
    /// **Note**: This endpoint should only be used for the Government (US1-FED) site.
    pub async fn get_cloud_workload_security_agent_rule(
        &self,
        agent_rule_id: String,
    ) -> Result<
        crate::datadogV2::model::CloudWorkloadSecurityAgentRuleResponse,
        datadog::Error<GetCloudWorkloadSecurityAgentRuleError>,
    > {
        match self
            .get_cloud_workload_security_agent_rule_with_http_info(agent_rule_id)
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

    /// Get the details of a specific agent rule.
    ///
    /// **Note**: This endpoint should only be used for the Government (US1-FED) site.
    pub async fn get_cloud_workload_security_agent_rule_with_http_info(
        &self,
        agent_rule_id: String,
    ) -> Result<
        datadog::ResponseContent<crate::datadogV2::model::CloudWorkloadSecurityAgentRuleResponse>,
        datadog::Error<GetCloudWorkloadSecurityAgentRuleError>,
    > {
        let local_configuration = &self.config;
        let operation_id = "v2.get_cloud_workload_security_agent_rule";

        let local_client = &self.client;

        let local_uri_str = format!(
            "{}/api/v2/security_monitoring/cloud_workload_security/agent_rules/{agent_rule_id}",
            local_configuration.get_operation_host(operation_id),
            agent_rule_id = datadog::urlencode(agent_rule_id)
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
        log::debug!("request content: {:?}", local_req.body());
        let local_resp = local_client.execute(local_req).await?;

        let local_status = local_resp.status();
        let local_content = local_resp.text().await?;
        log::debug!("response content: {}", local_content);

        if !local_status.is_client_error() && !local_status.is_server_error() {
            match serde_json::from_str::<
                crate::datadogV2::model::CloudWorkloadSecurityAgentRuleResponse,
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
            let local_entity: Option<GetCloudWorkloadSecurityAgentRuleError> =
                serde_json::from_str(&local_content).ok();
            let local_error = datadog::ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(datadog::Error::ResponseError(local_error))
        }
    }

    /// Get the list of Workload Protection policies.
    ///
    /// **Note**: This endpoint is not available for the Government (US1-FED) site. Please reference the (US1-FED) specific resource below.
    pub async fn list_csm_threats_agent_policies(
        &self,
    ) -> Result<
        crate::datadogV2::model::CloudWorkloadSecurityAgentPoliciesListResponse,
        datadog::Error<ListCSMThreatsAgentPoliciesError>,
    > {
        match self.list_csm_threats_agent_policies_with_http_info().await {
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

    /// Get the list of Workload Protection policies.
    ///
    /// **Note**: This endpoint is not available for the Government (US1-FED) site. Please reference the (US1-FED) specific resource below.
    pub async fn list_csm_threats_agent_policies_with_http_info(
        &self,
    ) -> Result<
        datadog::ResponseContent<
            crate::datadogV2::model::CloudWorkloadSecurityAgentPoliciesListResponse,
        >,
        datadog::Error<ListCSMThreatsAgentPoliciesError>,
    > {
        let local_configuration = &self.config;
        let operation_id = "v2.list_csm_threats_agent_policies";

        let local_client = &self.client;

        let local_uri_str = format!(
            "{}/api/v2/remote_config/products/cws/policy",
            local_configuration.get_operation_host(operation_id)
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
        log::debug!("request content: {:?}", local_req.body());
        let local_resp = local_client.execute(local_req).await?;

        let local_status = local_resp.status();
        let local_content = local_resp.text().await?;
        log::debug!("response content: {}", local_content);

        if !local_status.is_client_error() && !local_status.is_server_error() {
            match serde_json::from_str::<
                crate::datadogV2::model::CloudWorkloadSecurityAgentPoliciesListResponse,
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
            let local_entity: Option<ListCSMThreatsAgentPoliciesError> =
                serde_json::from_str(&local_content).ok();
            let local_error = datadog::ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(datadog::Error::ResponseError(local_error))
        }
    }

    /// Get the list of Workload Protection agent rules.
    ///
    /// **Note**: This endpoint is not available for the Government (US1-FED) site. Please reference the (US1-FED) specific resource below.
    pub async fn list_csm_threats_agent_rules(
        &self,
        params: ListCSMThreatsAgentRulesOptionalParams,
    ) -> Result<
        crate::datadogV2::model::CloudWorkloadSecurityAgentRulesListResponse,
        datadog::Error<ListCSMThreatsAgentRulesError>,
    > {
        match self
            .list_csm_threats_agent_rules_with_http_info(params)
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

    /// Get the list of Workload Protection agent rules.
    ///
    /// **Note**: This endpoint is not available for the Government (US1-FED) site. Please reference the (US1-FED) specific resource below.
    pub async fn list_csm_threats_agent_rules_with_http_info(
        &self,
        params: ListCSMThreatsAgentRulesOptionalParams,
    ) -> Result<
        datadog::ResponseContent<
            crate::datadogV2::model::CloudWorkloadSecurityAgentRulesListResponse,
        >,
        datadog::Error<ListCSMThreatsAgentRulesError>,
    > {
        let local_configuration = &self.config;
        let operation_id = "v2.list_csm_threats_agent_rules";

        // unbox and build optional parameters
        let policy_id = params.policy_id;

        let local_client = &self.client;

        let local_uri_str = format!(
            "{}/api/v2/remote_config/products/cws/agent_rules",
            local_configuration.get_operation_host(operation_id)
        );
        let mut local_req_builder =
            local_client.request(reqwest::Method::GET, local_uri_str.as_str());

        if let Some(ref local_query_param) = policy_id {
            local_req_builder =
                local_req_builder.query(&[("policy_id", &local_query_param.to_string())]);
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
                crate::datadogV2::model::CloudWorkloadSecurityAgentRulesListResponse,
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
            let local_entity: Option<ListCSMThreatsAgentRulesError> =
                serde_json::from_str(&local_content).ok();
            let local_error = datadog::ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(datadog::Error::ResponseError(local_error))
        }
    }

    /// Get the list of agent rules.
    ///
    /// **Note**: This endpoint should only be used for the Government (US1-FED) site.
    pub async fn list_cloud_workload_security_agent_rules(
        &self,
    ) -> Result<
        crate::datadogV2::model::CloudWorkloadSecurityAgentRulesListResponse,
        datadog::Error<ListCloudWorkloadSecurityAgentRulesError>,
    > {
        match self
            .list_cloud_workload_security_agent_rules_with_http_info()
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

    /// Get the list of agent rules.
    ///
    /// **Note**: This endpoint should only be used for the Government (US1-FED) site.
    pub async fn list_cloud_workload_security_agent_rules_with_http_info(
        &self,
    ) -> Result<
        datadog::ResponseContent<
            crate::datadogV2::model::CloudWorkloadSecurityAgentRulesListResponse,
        >,
        datadog::Error<ListCloudWorkloadSecurityAgentRulesError>,
    > {
        let local_configuration = &self.config;
        let operation_id = "v2.list_cloud_workload_security_agent_rules";

        let local_client = &self.client;

        let local_uri_str = format!(
            "{}/api/v2/security_monitoring/cloud_workload_security/agent_rules",
            local_configuration.get_operation_host(operation_id)
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
        log::debug!("request content: {:?}", local_req.body());
        let local_resp = local_client.execute(local_req).await?;

        let local_status = local_resp.status();
        let local_content = local_resp.text().await?;
        log::debug!("response content: {}", local_content);

        if !local_status.is_client_error() && !local_status.is_server_error() {
            match serde_json::from_str::<
                crate::datadogV2::model::CloudWorkloadSecurityAgentRulesListResponse,
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
            let local_entity: Option<ListCloudWorkloadSecurityAgentRulesError> =
                serde_json::from_str(&local_content).ok();
            let local_error = datadog::ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(datadog::Error::ResponseError(local_error))
        }
    }

    /// Update a specific Workload Protection policy.
    /// Returns the policy object when the request is successful.
    ///
    /// **Note**: This endpoint is not available for the Government (US1-FED) site. Please reference the (US1-FED) specific resource below.
    pub async fn update_csm_threats_agent_policy(
        &self,
        policy_id: String,
        body: crate::datadogV2::model::CloudWorkloadSecurityAgentPolicyUpdateRequest,
    ) -> Result<
        crate::datadogV2::model::CloudWorkloadSecurityAgentPolicyResponse,
        datadog::Error<UpdateCSMThreatsAgentPolicyError>,
    > {
        match self
            .update_csm_threats_agent_policy_with_http_info(policy_id, body)
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

    /// Update a specific Workload Protection policy.
    /// Returns the policy object when the request is successful.
    ///
    /// **Note**: This endpoint is not available for the Government (US1-FED) site. Please reference the (US1-FED) specific resource below.
    pub async fn update_csm_threats_agent_policy_with_http_info(
        &self,
        policy_id: String,
        body: crate::datadogV2::model::CloudWorkloadSecurityAgentPolicyUpdateRequest,
    ) -> Result<
        datadog::ResponseContent<crate::datadogV2::model::CloudWorkloadSecurityAgentPolicyResponse>,
        datadog::Error<UpdateCSMThreatsAgentPolicyError>,
    > {
        let local_configuration = &self.config;
        let operation_id = "v2.update_csm_threats_agent_policy";

        let local_client = &self.client;

        let local_uri_str = format!(
            "{}/api/v2/remote_config/products/cws/policy/{policy_id}",
            local_configuration.get_operation_host(operation_id),
            policy_id = datadog::urlencode(policy_id)
        );
        let mut local_req_builder =
            local_client.request(reqwest::Method::PATCH, local_uri_str.as_str());

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
            match serde_json::from_str::<
                crate::datadogV2::model::CloudWorkloadSecurityAgentPolicyResponse,
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
            let local_entity: Option<UpdateCSMThreatsAgentPolicyError> =
                serde_json::from_str(&local_content).ok();
            let local_error = datadog::ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(datadog::Error::ResponseError(local_error))
        }
    }

    /// Update a specific Workload Protection Agent rule.
    /// Returns the agent rule object when the request is successful.
    ///
    /// **Note**: This endpoint is not available for the Government (US1-FED) site. Please reference the (US1-FED) specific resource below.
    pub async fn update_csm_threats_agent_rule(
        &self,
        agent_rule_id: String,
        body: crate::datadogV2::model::CloudWorkloadSecurityAgentRuleUpdateRequest,
        params: UpdateCSMThreatsAgentRuleOptionalParams,
    ) -> Result<
        crate::datadogV2::model::CloudWorkloadSecurityAgentRuleResponse,
        datadog::Error<UpdateCSMThreatsAgentRuleError>,
    > {
        match self
            .update_csm_threats_agent_rule_with_http_info(agent_rule_id, body, params)
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

    /// Update a specific Workload Protection Agent rule.
    /// Returns the agent rule object when the request is successful.
    ///
    /// **Note**: This endpoint is not available for the Government (US1-FED) site. Please reference the (US1-FED) specific resource below.
    pub async fn update_csm_threats_agent_rule_with_http_info(
        &self,
        agent_rule_id: String,
        body: crate::datadogV2::model::CloudWorkloadSecurityAgentRuleUpdateRequest,
        params: UpdateCSMThreatsAgentRuleOptionalParams,
    ) -> Result<
        datadog::ResponseContent<crate::datadogV2::model::CloudWorkloadSecurityAgentRuleResponse>,
        datadog::Error<UpdateCSMThreatsAgentRuleError>,
    > {
        let local_configuration = &self.config;
        let operation_id = "v2.update_csm_threats_agent_rule";

        // unbox and build optional parameters
        let policy_id = params.policy_id;

        let local_client = &self.client;

        let local_uri_str = format!(
            "{}/api/v2/remote_config/products/cws/agent_rules/{agent_rule_id}",
            local_configuration.get_operation_host(operation_id),
            agent_rule_id = datadog::urlencode(agent_rule_id)
        );
        let mut local_req_builder =
            local_client.request(reqwest::Method::PATCH, local_uri_str.as_str());

        if let Some(ref local_query_param) = policy_id {
            local_req_builder =
                local_req_builder.query(&[("policy_id", &local_query_param.to_string())]);
        };

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
            match serde_json::from_str::<
                crate::datadogV2::model::CloudWorkloadSecurityAgentRuleResponse,
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
            let local_entity: Option<UpdateCSMThreatsAgentRuleError> =
                serde_json::from_str(&local_content).ok();
            let local_error = datadog::ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(datadog::Error::ResponseError(local_error))
        }
    }

    /// Update a specific agent rule.
    /// Returns the agent rule object when the request is successful.
    ///
    /// **Note**: This endpoint should only be used for the Government (US1-FED) site.
    pub async fn update_cloud_workload_security_agent_rule(
        &self,
        agent_rule_id: String,
        body: crate::datadogV2::model::CloudWorkloadSecurityAgentRuleUpdateRequest,
    ) -> Result<
        crate::datadogV2::model::CloudWorkloadSecurityAgentRuleResponse,
        datadog::Error<UpdateCloudWorkloadSecurityAgentRuleError>,
    > {
        match self
            .update_cloud_workload_security_agent_rule_with_http_info(agent_rule_id, body)
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

    /// Update a specific agent rule.
    /// Returns the agent rule object when the request is successful.
    ///
    /// **Note**: This endpoint should only be used for the Government (US1-FED) site.
    pub async fn update_cloud_workload_security_agent_rule_with_http_info(
        &self,
        agent_rule_id: String,
        body: crate::datadogV2::model::CloudWorkloadSecurityAgentRuleUpdateRequest,
    ) -> Result<
        datadog::ResponseContent<crate::datadogV2::model::CloudWorkloadSecurityAgentRuleResponse>,
        datadog::Error<UpdateCloudWorkloadSecurityAgentRuleError>,
    > {
        let local_configuration = &self.config;
        let operation_id = "v2.update_cloud_workload_security_agent_rule";

        let local_client = &self.client;

        let local_uri_str = format!(
            "{}/api/v2/security_monitoring/cloud_workload_security/agent_rules/{agent_rule_id}",
            local_configuration.get_operation_host(operation_id),
            agent_rule_id = datadog::urlencode(agent_rule_id)
        );
        let mut local_req_builder =
            local_client.request(reqwest::Method::PATCH, local_uri_str.as_str());

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
            match serde_json::from_str::<
                crate::datadogV2::model::CloudWorkloadSecurityAgentRuleResponse,
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
            let local_entity: Option<UpdateCloudWorkloadSecurityAgentRuleError> =
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
