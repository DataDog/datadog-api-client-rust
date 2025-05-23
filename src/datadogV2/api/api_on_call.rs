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

/// CreateOnCallEscalationPolicyOptionalParams is a struct for passing parameters to the method [`OnCallAPI::create_on_call_escalation_policy`]
#[non_exhaustive]
#[derive(Clone, Default, Debug)]
pub struct CreateOnCallEscalationPolicyOptionalParams {
    /// Comma-separated list of included relationships to be returned. Allowed values: `teams`, `steps`, `steps.targets`.
    pub include: Option<String>,
}

impl CreateOnCallEscalationPolicyOptionalParams {
    /// Comma-separated list of included relationships to be returned. Allowed values: `teams`, `steps`, `steps.targets`.
    pub fn include(mut self, value: String) -> Self {
        self.include = Some(value);
        self
    }
}

/// CreateOnCallScheduleOptionalParams is a struct for passing parameters to the method [`OnCallAPI::create_on_call_schedule`]
#[non_exhaustive]
#[derive(Clone, Default, Debug)]
pub struct CreateOnCallScheduleOptionalParams {
    /// Comma-separated list of included relationships to be returned. Allowed values: `teams`, `layers`, `layers.members`, `layers.members.user`.
    pub include: Option<String>,
}

impl CreateOnCallScheduleOptionalParams {
    /// Comma-separated list of included relationships to be returned. Allowed values: `teams`, `layers`, `layers.members`, `layers.members.user`.
    pub fn include(mut self, value: String) -> Self {
        self.include = Some(value);
        self
    }
}

/// GetOnCallEscalationPolicyOptionalParams is a struct for passing parameters to the method [`OnCallAPI::get_on_call_escalation_policy`]
#[non_exhaustive]
#[derive(Clone, Default, Debug)]
pub struct GetOnCallEscalationPolicyOptionalParams {
    /// Comma-separated list of included relationships to be returned. Allowed values: `teams`, `steps`, `steps.targets`.
    pub include: Option<String>,
}

impl GetOnCallEscalationPolicyOptionalParams {
    /// Comma-separated list of included relationships to be returned. Allowed values: `teams`, `steps`, `steps.targets`.
    pub fn include(mut self, value: String) -> Self {
        self.include = Some(value);
        self
    }
}

/// GetOnCallScheduleOptionalParams is a struct for passing parameters to the method [`OnCallAPI::get_on_call_schedule`]
#[non_exhaustive]
#[derive(Clone, Default, Debug)]
pub struct GetOnCallScheduleOptionalParams {
    /// Comma-separated list of included relationships to be returned. Allowed values: `teams`, `layers`, `layers.members`, `layers.members.user`.
    pub include: Option<String>,
}

impl GetOnCallScheduleOptionalParams {
    /// Comma-separated list of included relationships to be returned. Allowed values: `teams`, `layers`, `layers.members`, `layers.members.user`.
    pub fn include(mut self, value: String) -> Self {
        self.include = Some(value);
        self
    }
}

/// GetOnCallTeamRoutingRulesOptionalParams is a struct for passing parameters to the method [`OnCallAPI::get_on_call_team_routing_rules`]
#[non_exhaustive]
#[derive(Clone, Default, Debug)]
pub struct GetOnCallTeamRoutingRulesOptionalParams {
    /// Comma-separated list of included relationships to be returned. Allowed values: `rules`, `rules.policy`.
    pub include: Option<String>,
}

impl GetOnCallTeamRoutingRulesOptionalParams {
    /// Comma-separated list of included relationships to be returned. Allowed values: `rules`, `rules.policy`.
    pub fn include(mut self, value: String) -> Self {
        self.include = Some(value);
        self
    }
}

/// GetScheduleOnCallUserOptionalParams is a struct for passing parameters to the method [`OnCallAPI::get_schedule_on_call_user`]
#[non_exhaustive]
#[derive(Clone, Default, Debug)]
pub struct GetScheduleOnCallUserOptionalParams {
    /// Specifies related resources to include in the response as a comma-separated list. Allowed value: `user`.
    pub include: Option<String>,
    /// Retrieves the on-call user at the given timestamp (ISO-8601). Defaults to the current time if omitted."
    pub filter_at_ts: Option<String>,
}

impl GetScheduleOnCallUserOptionalParams {
    /// Specifies related resources to include in the response as a comma-separated list. Allowed value: `user`.
    pub fn include(mut self, value: String) -> Self {
        self.include = Some(value);
        self
    }
    /// Retrieves the on-call user at the given timestamp (ISO-8601). Defaults to the current time if omitted."
    pub fn filter_at_ts(mut self, value: String) -> Self {
        self.filter_at_ts = Some(value);
        self
    }
}

/// SetOnCallTeamRoutingRulesOptionalParams is a struct for passing parameters to the method [`OnCallAPI::set_on_call_team_routing_rules`]
#[non_exhaustive]
#[derive(Clone, Default, Debug)]
pub struct SetOnCallTeamRoutingRulesOptionalParams {
    /// Comma-separated list of included relationships to be returned. Allowed values: `rules`, `rules.policy`.
    pub include: Option<String>,
}

impl SetOnCallTeamRoutingRulesOptionalParams {
    /// Comma-separated list of included relationships to be returned. Allowed values: `rules`, `rules.policy`.
    pub fn include(mut self, value: String) -> Self {
        self.include = Some(value);
        self
    }
}

/// UpdateOnCallEscalationPolicyOptionalParams is a struct for passing parameters to the method [`OnCallAPI::update_on_call_escalation_policy`]
#[non_exhaustive]
#[derive(Clone, Default, Debug)]
pub struct UpdateOnCallEscalationPolicyOptionalParams {
    /// Comma-separated list of included relationships to be returned. Allowed values: `teams`, `steps`, `steps.targets`.
    pub include: Option<String>,
}

impl UpdateOnCallEscalationPolicyOptionalParams {
    /// Comma-separated list of included relationships to be returned. Allowed values: `teams`, `steps`, `steps.targets`.
    pub fn include(mut self, value: String) -> Self {
        self.include = Some(value);
        self
    }
}

/// UpdateOnCallScheduleOptionalParams is a struct for passing parameters to the method [`OnCallAPI::update_on_call_schedule`]
#[non_exhaustive]
#[derive(Clone, Default, Debug)]
pub struct UpdateOnCallScheduleOptionalParams {
    /// Comma-separated list of included relationships to be returned. Allowed values: `teams`, `layers`, `layers.members`, `layers.members.user`.
    pub include: Option<String>,
}

impl UpdateOnCallScheduleOptionalParams {
    /// Comma-separated list of included relationships to be returned. Allowed values: `teams`, `layers`, `layers.members`, `layers.members.user`.
    pub fn include(mut self, value: String) -> Self {
        self.include = Some(value);
        self
    }
}

/// CreateOnCallEscalationPolicyError is a struct for typed errors of method [`OnCallAPI::create_on_call_escalation_policy`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CreateOnCallEscalationPolicyError {
    APIErrorResponse(crate::datadogV2::model::APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// CreateOnCallScheduleError is a struct for typed errors of method [`OnCallAPI::create_on_call_schedule`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CreateOnCallScheduleError {
    APIErrorResponse(crate::datadogV2::model::APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// DeleteOnCallEscalationPolicyError is a struct for typed errors of method [`OnCallAPI::delete_on_call_escalation_policy`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DeleteOnCallEscalationPolicyError {
    APIErrorResponse(crate::datadogV2::model::APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// DeleteOnCallScheduleError is a struct for typed errors of method [`OnCallAPI::delete_on_call_schedule`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DeleteOnCallScheduleError {
    APIErrorResponse(crate::datadogV2::model::APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// GetOnCallEscalationPolicyError is a struct for typed errors of method [`OnCallAPI::get_on_call_escalation_policy`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetOnCallEscalationPolicyError {
    APIErrorResponse(crate::datadogV2::model::APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// GetOnCallScheduleError is a struct for typed errors of method [`OnCallAPI::get_on_call_schedule`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetOnCallScheduleError {
    APIErrorResponse(crate::datadogV2::model::APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// GetOnCallTeamRoutingRulesError is a struct for typed errors of method [`OnCallAPI::get_on_call_team_routing_rules`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetOnCallTeamRoutingRulesError {
    APIErrorResponse(crate::datadogV2::model::APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// GetScheduleOnCallUserError is a struct for typed errors of method [`OnCallAPI::get_schedule_on_call_user`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetScheduleOnCallUserError {
    APIErrorResponse(crate::datadogV2::model::APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// SetOnCallTeamRoutingRulesError is a struct for typed errors of method [`OnCallAPI::set_on_call_team_routing_rules`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum SetOnCallTeamRoutingRulesError {
    APIErrorResponse(crate::datadogV2::model::APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// UpdateOnCallEscalationPolicyError is a struct for typed errors of method [`OnCallAPI::update_on_call_escalation_policy`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum UpdateOnCallEscalationPolicyError {
    APIErrorResponse(crate::datadogV2::model::APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// UpdateOnCallScheduleError is a struct for typed errors of method [`OnCallAPI::update_on_call_schedule`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum UpdateOnCallScheduleError {
    APIErrorResponse(crate::datadogV2::model::APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// Configure your [Datadog On-Call](<https://docs.datadoghq.com/service_management/on-call/>)
/// directly through the Datadog API.
#[derive(Debug, Clone)]
pub struct OnCallAPI {
    config: datadog::Configuration,
    client: reqwest_middleware::ClientWithMiddleware,
}

impl Default for OnCallAPI {
    fn default() -> Self {
        Self::with_config(datadog::Configuration::default())
    }
}

impl OnCallAPI {
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

    /// Create a new On-Call escalation policy
    pub async fn create_on_call_escalation_policy(
        &self,
        body: crate::datadogV2::model::EscalationPolicyCreateRequest,
        params: CreateOnCallEscalationPolicyOptionalParams,
    ) -> Result<
        crate::datadogV2::model::EscalationPolicy,
        datadog::Error<CreateOnCallEscalationPolicyError>,
    > {
        match self
            .create_on_call_escalation_policy_with_http_info(body, params)
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

    /// Create a new On-Call escalation policy
    pub async fn create_on_call_escalation_policy_with_http_info(
        &self,
        body: crate::datadogV2::model::EscalationPolicyCreateRequest,
        params: CreateOnCallEscalationPolicyOptionalParams,
    ) -> Result<
        datadog::ResponseContent<crate::datadogV2::model::EscalationPolicy>,
        datadog::Error<CreateOnCallEscalationPolicyError>,
    > {
        let local_configuration = &self.config;
        let operation_id = "v2.create_on_call_escalation_policy";

        // unbox and build optional parameters
        let include = params.include;

        let local_client = &self.client;

        let local_uri_str = format!(
            "{}/api/v2/on-call/escalation-policies",
            local_configuration.get_operation_host(operation_id)
        );
        let mut local_req_builder =
            local_client.request(reqwest::Method::POST, local_uri_str.as_str());

        if let Some(ref local_query_param) = include {
            local_req_builder =
                local_req_builder.query(&[("include", &local_query_param.to_string())]);
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
            match serde_json::from_str::<crate::datadogV2::model::EscalationPolicy>(&local_content)
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
            let local_entity: Option<CreateOnCallEscalationPolicyError> =
                serde_json::from_str(&local_content).ok();
            let local_error = datadog::ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(datadog::Error::ResponseError(local_error))
        }
    }

    /// Create a new On-Call schedule
    pub async fn create_on_call_schedule(
        &self,
        body: crate::datadogV2::model::ScheduleCreateRequest,
        params: CreateOnCallScheduleOptionalParams,
    ) -> Result<crate::datadogV2::model::Schedule, datadog::Error<CreateOnCallScheduleError>> {
        match self
            .create_on_call_schedule_with_http_info(body, params)
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

    /// Create a new On-Call schedule
    pub async fn create_on_call_schedule_with_http_info(
        &self,
        body: crate::datadogV2::model::ScheduleCreateRequest,
        params: CreateOnCallScheduleOptionalParams,
    ) -> Result<
        datadog::ResponseContent<crate::datadogV2::model::Schedule>,
        datadog::Error<CreateOnCallScheduleError>,
    > {
        let local_configuration = &self.config;
        let operation_id = "v2.create_on_call_schedule";

        // unbox and build optional parameters
        let include = params.include;

        let local_client = &self.client;

        let local_uri_str = format!(
            "{}/api/v2/on-call/schedules",
            local_configuration.get_operation_host(operation_id)
        );
        let mut local_req_builder =
            local_client.request(reqwest::Method::POST, local_uri_str.as_str());

        if let Some(ref local_query_param) = include {
            local_req_builder =
                local_req_builder.query(&[("include", &local_query_param.to_string())]);
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
            match serde_json::from_str::<crate::datadogV2::model::Schedule>(&local_content) {
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
            let local_entity: Option<CreateOnCallScheduleError> =
                serde_json::from_str(&local_content).ok();
            let local_error = datadog::ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(datadog::Error::ResponseError(local_error))
        }
    }

    /// Delete an On-Call escalation policy
    pub async fn delete_on_call_escalation_policy(
        &self,
        policy_id: String,
    ) -> Result<(), datadog::Error<DeleteOnCallEscalationPolicyError>> {
        match self
            .delete_on_call_escalation_policy_with_http_info(policy_id)
            .await
        {
            Ok(_) => Ok(()),
            Err(err) => Err(err),
        }
    }

    /// Delete an On-Call escalation policy
    pub async fn delete_on_call_escalation_policy_with_http_info(
        &self,
        policy_id: String,
    ) -> Result<datadog::ResponseContent<()>, datadog::Error<DeleteOnCallEscalationPolicyError>>
    {
        let local_configuration = &self.config;
        let operation_id = "v2.delete_on_call_escalation_policy";

        let local_client = &self.client;

        let local_uri_str = format!(
            "{}/api/v2/on-call/escalation-policies/{policy_id}",
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
            let local_entity: Option<DeleteOnCallEscalationPolicyError> =
                serde_json::from_str(&local_content).ok();
            let local_error = datadog::ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(datadog::Error::ResponseError(local_error))
        }
    }

    /// Delete an On-Call schedule
    pub async fn delete_on_call_schedule(
        &self,
        schedule_id: String,
    ) -> Result<(), datadog::Error<DeleteOnCallScheduleError>> {
        match self
            .delete_on_call_schedule_with_http_info(schedule_id)
            .await
        {
            Ok(_) => Ok(()),
            Err(err) => Err(err),
        }
    }

    /// Delete an On-Call schedule
    pub async fn delete_on_call_schedule_with_http_info(
        &self,
        schedule_id: String,
    ) -> Result<datadog::ResponseContent<()>, datadog::Error<DeleteOnCallScheduleError>> {
        let local_configuration = &self.config;
        let operation_id = "v2.delete_on_call_schedule";

        let local_client = &self.client;

        let local_uri_str = format!(
            "{}/api/v2/on-call/schedules/{schedule_id}",
            local_configuration.get_operation_host(operation_id),
            schedule_id = datadog::urlencode(schedule_id)
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
            let local_entity: Option<DeleteOnCallScheduleError> =
                serde_json::from_str(&local_content).ok();
            let local_error = datadog::ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(datadog::Error::ResponseError(local_error))
        }
    }

    /// Get an On-Call escalation policy
    pub async fn get_on_call_escalation_policy(
        &self,
        policy_id: String,
        params: GetOnCallEscalationPolicyOptionalParams,
    ) -> Result<
        crate::datadogV2::model::EscalationPolicy,
        datadog::Error<GetOnCallEscalationPolicyError>,
    > {
        match self
            .get_on_call_escalation_policy_with_http_info(policy_id, params)
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

    /// Get an On-Call escalation policy
    pub async fn get_on_call_escalation_policy_with_http_info(
        &self,
        policy_id: String,
        params: GetOnCallEscalationPolicyOptionalParams,
    ) -> Result<
        datadog::ResponseContent<crate::datadogV2::model::EscalationPolicy>,
        datadog::Error<GetOnCallEscalationPolicyError>,
    > {
        let local_configuration = &self.config;
        let operation_id = "v2.get_on_call_escalation_policy";

        // unbox and build optional parameters
        let include = params.include;

        let local_client = &self.client;

        let local_uri_str = format!(
            "{}/api/v2/on-call/escalation-policies/{policy_id}",
            local_configuration.get_operation_host(operation_id),
            policy_id = datadog::urlencode(policy_id)
        );
        let mut local_req_builder =
            local_client.request(reqwest::Method::GET, local_uri_str.as_str());

        if let Some(ref local_query_param) = include {
            local_req_builder =
                local_req_builder.query(&[("include", &local_query_param.to_string())]);
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
            match serde_json::from_str::<crate::datadogV2::model::EscalationPolicy>(&local_content)
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
            let local_entity: Option<GetOnCallEscalationPolicyError> =
                serde_json::from_str(&local_content).ok();
            let local_error = datadog::ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(datadog::Error::ResponseError(local_error))
        }
    }

    /// Get an On-Call schedule
    pub async fn get_on_call_schedule(
        &self,
        schedule_id: String,
        params: GetOnCallScheduleOptionalParams,
    ) -> Result<crate::datadogV2::model::Schedule, datadog::Error<GetOnCallScheduleError>> {
        match self
            .get_on_call_schedule_with_http_info(schedule_id, params)
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

    /// Get an On-Call schedule
    pub async fn get_on_call_schedule_with_http_info(
        &self,
        schedule_id: String,
        params: GetOnCallScheduleOptionalParams,
    ) -> Result<
        datadog::ResponseContent<crate::datadogV2::model::Schedule>,
        datadog::Error<GetOnCallScheduleError>,
    > {
        let local_configuration = &self.config;
        let operation_id = "v2.get_on_call_schedule";

        // unbox and build optional parameters
        let include = params.include;

        let local_client = &self.client;

        let local_uri_str = format!(
            "{}/api/v2/on-call/schedules/{schedule_id}",
            local_configuration.get_operation_host(operation_id),
            schedule_id = datadog::urlencode(schedule_id)
        );
        let mut local_req_builder =
            local_client.request(reqwest::Method::GET, local_uri_str.as_str());

        if let Some(ref local_query_param) = include {
            local_req_builder =
                local_req_builder.query(&[("include", &local_query_param.to_string())]);
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
            match serde_json::from_str::<crate::datadogV2::model::Schedule>(&local_content) {
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
            let local_entity: Option<GetOnCallScheduleError> =
                serde_json::from_str(&local_content).ok();
            let local_error = datadog::ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(datadog::Error::ResponseError(local_error))
        }
    }

    /// Get a team's On-Call routing rules
    pub async fn get_on_call_team_routing_rules(
        &self,
        team_id: String,
        params: GetOnCallTeamRoutingRulesOptionalParams,
    ) -> Result<
        crate::datadogV2::model::TeamRoutingRules,
        datadog::Error<GetOnCallTeamRoutingRulesError>,
    > {
        match self
            .get_on_call_team_routing_rules_with_http_info(team_id, params)
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

    /// Get a team's On-Call routing rules
    pub async fn get_on_call_team_routing_rules_with_http_info(
        &self,
        team_id: String,
        params: GetOnCallTeamRoutingRulesOptionalParams,
    ) -> Result<
        datadog::ResponseContent<crate::datadogV2::model::TeamRoutingRules>,
        datadog::Error<GetOnCallTeamRoutingRulesError>,
    > {
        let local_configuration = &self.config;
        let operation_id = "v2.get_on_call_team_routing_rules";

        // unbox and build optional parameters
        let include = params.include;

        let local_client = &self.client;

        let local_uri_str = format!(
            "{}/api/v2/on-call/teams/{team_id}/routing-rules",
            local_configuration.get_operation_host(operation_id),
            team_id = datadog::urlencode(team_id)
        );
        let mut local_req_builder =
            local_client.request(reqwest::Method::GET, local_uri_str.as_str());

        if let Some(ref local_query_param) = include {
            local_req_builder =
                local_req_builder.query(&[("include", &local_query_param.to_string())]);
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
            match serde_json::from_str::<crate::datadogV2::model::TeamRoutingRules>(&local_content)
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
            let local_entity: Option<GetOnCallTeamRoutingRulesError> =
                serde_json::from_str(&local_content).ok();
            let local_error = datadog::ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(datadog::Error::ResponseError(local_error))
        }
    }

    /// Retrieves the user who is on-call for the specified schedule at a given time.
    pub async fn get_schedule_on_call_user(
        &self,
        schedule_id: String,
        params: GetScheduleOnCallUserOptionalParams,
    ) -> Result<crate::datadogV2::model::Shift, datadog::Error<GetScheduleOnCallUserError>> {
        match self
            .get_schedule_on_call_user_with_http_info(schedule_id, params)
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

    /// Retrieves the user who is on-call for the specified schedule at a given time.
    pub async fn get_schedule_on_call_user_with_http_info(
        &self,
        schedule_id: String,
        params: GetScheduleOnCallUserOptionalParams,
    ) -> Result<
        datadog::ResponseContent<crate::datadogV2::model::Shift>,
        datadog::Error<GetScheduleOnCallUserError>,
    > {
        let local_configuration = &self.config;
        let operation_id = "v2.get_schedule_on_call_user";

        // unbox and build optional parameters
        let include = params.include;
        let filter_at_ts = params.filter_at_ts;

        let local_client = &self.client;

        let local_uri_str = format!(
            "{}/api/v2/on-call/schedules/{schedule_id}/on-call",
            local_configuration.get_operation_host(operation_id),
            schedule_id = datadog::urlencode(schedule_id)
        );
        let mut local_req_builder =
            local_client.request(reqwest::Method::GET, local_uri_str.as_str());

        if let Some(ref local_query_param) = include {
            local_req_builder =
                local_req_builder.query(&[("include", &local_query_param.to_string())]);
        };
        if let Some(ref local_query_param) = filter_at_ts {
            local_req_builder =
                local_req_builder.query(&[("filter[at_ts]", &local_query_param.to_string())]);
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
            match serde_json::from_str::<crate::datadogV2::model::Shift>(&local_content) {
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
            let local_entity: Option<GetScheduleOnCallUserError> =
                serde_json::from_str(&local_content).ok();
            let local_error = datadog::ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(datadog::Error::ResponseError(local_error))
        }
    }

    /// Set a team's On-Call routing rules
    pub async fn set_on_call_team_routing_rules(
        &self,
        team_id: String,
        body: crate::datadogV2::model::TeamRoutingRulesRequest,
        params: SetOnCallTeamRoutingRulesOptionalParams,
    ) -> Result<
        crate::datadogV2::model::TeamRoutingRules,
        datadog::Error<SetOnCallTeamRoutingRulesError>,
    > {
        match self
            .set_on_call_team_routing_rules_with_http_info(team_id, body, params)
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

    /// Set a team's On-Call routing rules
    pub async fn set_on_call_team_routing_rules_with_http_info(
        &self,
        team_id: String,
        body: crate::datadogV2::model::TeamRoutingRulesRequest,
        params: SetOnCallTeamRoutingRulesOptionalParams,
    ) -> Result<
        datadog::ResponseContent<crate::datadogV2::model::TeamRoutingRules>,
        datadog::Error<SetOnCallTeamRoutingRulesError>,
    > {
        let local_configuration = &self.config;
        let operation_id = "v2.set_on_call_team_routing_rules";

        // unbox and build optional parameters
        let include = params.include;

        let local_client = &self.client;

        let local_uri_str = format!(
            "{}/api/v2/on-call/teams/{team_id}/routing-rules",
            local_configuration.get_operation_host(operation_id),
            team_id = datadog::urlencode(team_id)
        );
        let mut local_req_builder =
            local_client.request(reqwest::Method::PUT, local_uri_str.as_str());

        if let Some(ref local_query_param) = include {
            local_req_builder =
                local_req_builder.query(&[("include", &local_query_param.to_string())]);
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
            match serde_json::from_str::<crate::datadogV2::model::TeamRoutingRules>(&local_content)
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
            let local_entity: Option<SetOnCallTeamRoutingRulesError> =
                serde_json::from_str(&local_content).ok();
            let local_error = datadog::ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(datadog::Error::ResponseError(local_error))
        }
    }

    /// Update an On-Call escalation policy
    pub async fn update_on_call_escalation_policy(
        &self,
        policy_id: String,
        body: crate::datadogV2::model::EscalationPolicyUpdateRequest,
        params: UpdateOnCallEscalationPolicyOptionalParams,
    ) -> Result<
        crate::datadogV2::model::EscalationPolicy,
        datadog::Error<UpdateOnCallEscalationPolicyError>,
    > {
        match self
            .update_on_call_escalation_policy_with_http_info(policy_id, body, params)
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

    /// Update an On-Call escalation policy
    pub async fn update_on_call_escalation_policy_with_http_info(
        &self,
        policy_id: String,
        body: crate::datadogV2::model::EscalationPolicyUpdateRequest,
        params: UpdateOnCallEscalationPolicyOptionalParams,
    ) -> Result<
        datadog::ResponseContent<crate::datadogV2::model::EscalationPolicy>,
        datadog::Error<UpdateOnCallEscalationPolicyError>,
    > {
        let local_configuration = &self.config;
        let operation_id = "v2.update_on_call_escalation_policy";

        // unbox and build optional parameters
        let include = params.include;

        let local_client = &self.client;

        let local_uri_str = format!(
            "{}/api/v2/on-call/escalation-policies/{policy_id}",
            local_configuration.get_operation_host(operation_id),
            policy_id = datadog::urlencode(policy_id)
        );
        let mut local_req_builder =
            local_client.request(reqwest::Method::PUT, local_uri_str.as_str());

        if let Some(ref local_query_param) = include {
            local_req_builder =
                local_req_builder.query(&[("include", &local_query_param.to_string())]);
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
            match serde_json::from_str::<crate::datadogV2::model::EscalationPolicy>(&local_content)
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
            let local_entity: Option<UpdateOnCallEscalationPolicyError> =
                serde_json::from_str(&local_content).ok();
            let local_error = datadog::ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(datadog::Error::ResponseError(local_error))
        }
    }

    /// Update a new On-Call schedule
    pub async fn update_on_call_schedule(
        &self,
        schedule_id: String,
        body: crate::datadogV2::model::ScheduleUpdateRequest,
        params: UpdateOnCallScheduleOptionalParams,
    ) -> Result<crate::datadogV2::model::Schedule, datadog::Error<UpdateOnCallScheduleError>> {
        match self
            .update_on_call_schedule_with_http_info(schedule_id, body, params)
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

    /// Update a new On-Call schedule
    pub async fn update_on_call_schedule_with_http_info(
        &self,
        schedule_id: String,
        body: crate::datadogV2::model::ScheduleUpdateRequest,
        params: UpdateOnCallScheduleOptionalParams,
    ) -> Result<
        datadog::ResponseContent<crate::datadogV2::model::Schedule>,
        datadog::Error<UpdateOnCallScheduleError>,
    > {
        let local_configuration = &self.config;
        let operation_id = "v2.update_on_call_schedule";

        // unbox and build optional parameters
        let include = params.include;

        let local_client = &self.client;

        let local_uri_str = format!(
            "{}/api/v2/on-call/schedules/{schedule_id}",
            local_configuration.get_operation_host(operation_id),
            schedule_id = datadog::urlencode(schedule_id)
        );
        let mut local_req_builder =
            local_client.request(reqwest::Method::PUT, local_uri_str.as_str());

        if let Some(ref local_query_param) = include {
            local_req_builder =
                local_req_builder.query(&[("include", &local_query_param.to_string())]);
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
            match serde_json::from_str::<crate::datadogV2::model::Schedule>(&local_content) {
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
            let local_entity: Option<UpdateOnCallScheduleError> =
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
