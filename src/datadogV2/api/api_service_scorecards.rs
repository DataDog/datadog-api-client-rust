// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use crate::datadog;
use async_stream::try_stream;
use flate2::{
    write::{GzEncoder, ZlibEncoder},
    Compression,
};
use futures_core::stream::Stream;
use log::warn;
use reqwest::header::{HeaderMap, HeaderValue};
use serde::{Deserialize, Serialize};
use std::io::Write;

/// ListScorecardOutcomesOptionalParams is a struct for passing parameters to the method [`ServiceScorecardsAPI::list_scorecard_outcomes`]
#[non_exhaustive]
#[derive(Clone, Default, Debug)]
pub struct ListScorecardOutcomesOptionalParams {
    /// Size for a given page. The maximum allowed value is 100.
    pub page_size: Option<i64>,
    /// Specific offset to use as the beginning of the returned page.
    pub page_offset: Option<i64>,
    /// Include related rule details in the response.
    pub include: Option<String>,
    /// Return only specified values in the outcome attributes.
    pub fields_outcome: Option<String>,
    /// Return only specified values in the included rule details.
    pub fields_rule: Option<String>,
    /// Filter the outcomes on a specific service name.
    pub filter_outcome_service_name: Option<String>,
    /// Filter the outcomes by a specific state.
    pub filter_outcome_state: Option<String>,
    /// Filter outcomes on whether a rule is enabled/disabled.
    pub filter_rule_enabled: Option<bool>,
    /// Filter outcomes based on rule ID.
    pub filter_rule_id: Option<String>,
    /// Filter outcomes based on rule name.
    pub filter_rule_name: Option<String>,
}

impl ListScorecardOutcomesOptionalParams {
    /// Size for a given page. The maximum allowed value is 100.
    pub fn page_size(mut self, value: i64) -> Self {
        self.page_size = Some(value);
        self
    }
    /// Specific offset to use as the beginning of the returned page.
    pub fn page_offset(mut self, value: i64) -> Self {
        self.page_offset = Some(value);
        self
    }
    /// Include related rule details in the response.
    pub fn include(mut self, value: String) -> Self {
        self.include = Some(value);
        self
    }
    /// Return only specified values in the outcome attributes.
    pub fn fields_outcome(mut self, value: String) -> Self {
        self.fields_outcome = Some(value);
        self
    }
    /// Return only specified values in the included rule details.
    pub fn fields_rule(mut self, value: String) -> Self {
        self.fields_rule = Some(value);
        self
    }
    /// Filter the outcomes on a specific service name.
    pub fn filter_outcome_service_name(mut self, value: String) -> Self {
        self.filter_outcome_service_name = Some(value);
        self
    }
    /// Filter the outcomes by a specific state.
    pub fn filter_outcome_state(mut self, value: String) -> Self {
        self.filter_outcome_state = Some(value);
        self
    }
    /// Filter outcomes on whether a rule is enabled/disabled.
    pub fn filter_rule_enabled(mut self, value: bool) -> Self {
        self.filter_rule_enabled = Some(value);
        self
    }
    /// Filter outcomes based on rule ID.
    pub fn filter_rule_id(mut self, value: String) -> Self {
        self.filter_rule_id = Some(value);
        self
    }
    /// Filter outcomes based on rule name.
    pub fn filter_rule_name(mut self, value: String) -> Self {
        self.filter_rule_name = Some(value);
        self
    }
}

/// ListScorecardRulesOptionalParams is a struct for passing parameters to the method [`ServiceScorecardsAPI::list_scorecard_rules`]
#[non_exhaustive]
#[derive(Clone, Default, Debug)]
pub struct ListScorecardRulesOptionalParams {
    /// Size for a given page. The maximum allowed value is 100.
    pub page_size: Option<i64>,
    /// Specific offset to use as the beginning of the returned page.
    pub page_offset: Option<i64>,
    /// Include related scorecard details in the response.
    pub include: Option<String>,
    /// Filter the rules on a rule ID.
    pub filter_rule_id: Option<String>,
    /// Filter for enabled rules only.
    pub filter_rule_enabled: Option<bool>,
    /// Filter for custom rules only.
    pub filter_rule_custom: Option<bool>,
    /// Filter rules on the rule name.
    pub filter_rule_name: Option<String>,
    /// Filter rules on the rule description.
    pub filter_rule_description: Option<String>,
    /// Return only specific fields in the response for rule attributes.
    pub fields_rule: Option<String>,
    /// Return only specific fields in the included response for scorecard attributes.
    pub fields_scorecard: Option<String>,
}

impl ListScorecardRulesOptionalParams {
    /// Size for a given page. The maximum allowed value is 100.
    pub fn page_size(mut self, value: i64) -> Self {
        self.page_size = Some(value);
        self
    }
    /// Specific offset to use as the beginning of the returned page.
    pub fn page_offset(mut self, value: i64) -> Self {
        self.page_offset = Some(value);
        self
    }
    /// Include related scorecard details in the response.
    pub fn include(mut self, value: String) -> Self {
        self.include = Some(value);
        self
    }
    /// Filter the rules on a rule ID.
    pub fn filter_rule_id(mut self, value: String) -> Self {
        self.filter_rule_id = Some(value);
        self
    }
    /// Filter for enabled rules only.
    pub fn filter_rule_enabled(mut self, value: bool) -> Self {
        self.filter_rule_enabled = Some(value);
        self
    }
    /// Filter for custom rules only.
    pub fn filter_rule_custom(mut self, value: bool) -> Self {
        self.filter_rule_custom = Some(value);
        self
    }
    /// Filter rules on the rule name.
    pub fn filter_rule_name(mut self, value: String) -> Self {
        self.filter_rule_name = Some(value);
        self
    }
    /// Filter rules on the rule description.
    pub fn filter_rule_description(mut self, value: String) -> Self {
        self.filter_rule_description = Some(value);
        self
    }
    /// Return only specific fields in the response for rule attributes.
    pub fn fields_rule(mut self, value: String) -> Self {
        self.fields_rule = Some(value);
        self
    }
    /// Return only specific fields in the included response for scorecard attributes.
    pub fn fields_scorecard(mut self, value: String) -> Self {
        self.fields_scorecard = Some(value);
        self
    }
}

/// CreateScorecardOutcomesBatchError is a struct for typed errors of method [`ServiceScorecardsAPI::create_scorecard_outcomes_batch`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CreateScorecardOutcomesBatchError {
    APIErrorResponse(crate::datadogV2::model::APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// CreateScorecardRuleError is a struct for typed errors of method [`ServiceScorecardsAPI::create_scorecard_rule`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CreateScorecardRuleError {
    APIErrorResponse(crate::datadogV2::model::APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// DeleteScorecardRuleError is a struct for typed errors of method [`ServiceScorecardsAPI::delete_scorecard_rule`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DeleteScorecardRuleError {
    APIErrorResponse(crate::datadogV2::model::APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// ListScorecardOutcomesError is a struct for typed errors of method [`ServiceScorecardsAPI::list_scorecard_outcomes`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ListScorecardOutcomesError {
    APIErrorResponse(crate::datadogV2::model::APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// ListScorecardRulesError is a struct for typed errors of method [`ServiceScorecardsAPI::list_scorecard_rules`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ListScorecardRulesError {
    APIErrorResponse(crate::datadogV2::model::APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// UpdateScorecardRuleError is a struct for typed errors of method [`ServiceScorecardsAPI::update_scorecard_rule`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum UpdateScorecardRuleError {
    APIErrorResponse(crate::datadogV2::model::APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// API to create and update scorecard rules and outcomes. See [Service Scorecards](<https://docs.datadoghq.com/service_catalog/scorecards>) for more information.
///
/// This feature is currently in BETA. If you have any feedback, contact [Datadog support](<https://docs.datadoghq.com/help/>).
#[derive(Debug, Clone)]
pub struct ServiceScorecardsAPI {
    config: datadog::Configuration,
    client: reqwest_middleware::ClientWithMiddleware,
}

impl Default for ServiceScorecardsAPI {
    fn default() -> Self {
        Self::with_config(datadog::Configuration::default())
    }
}

impl ServiceScorecardsAPI {
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

    /// Sets multiple service-rule outcomes in a single batched request.
    pub async fn create_scorecard_outcomes_batch(
        &self,
        body: crate::datadogV2::model::OutcomesBatchRequest,
    ) -> Result<
        crate::datadogV2::model::OutcomesBatchResponse,
        datadog::Error<CreateScorecardOutcomesBatchError>,
    > {
        match self
            .create_scorecard_outcomes_batch_with_http_info(body)
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

    /// Sets multiple service-rule outcomes in a single batched request.
    pub async fn create_scorecard_outcomes_batch_with_http_info(
        &self,
        body: crate::datadogV2::model::OutcomesBatchRequest,
    ) -> Result<
        datadog::ResponseContent<crate::datadogV2::model::OutcomesBatchResponse>,
        datadog::Error<CreateScorecardOutcomesBatchError>,
    > {
        let local_configuration = &self.config;
        let operation_id = "v2.create_scorecard_outcomes_batch";
        if local_configuration.is_unstable_operation_enabled(operation_id) {
            warn!("Using unstable operation {operation_id}");
        } else {
            let local_error = datadog::UnstableOperationDisabledError {
                msg: "Operation 'v2.create_scorecard_outcomes_batch' is not enabled".to_string(),
            };
            return Err(datadog::Error::UnstableOperationDisabledError(local_error));
        }

        let local_client = &self.client;

        let local_uri_str = format!(
            "{}/api/v2/scorecard/outcomes/batch",
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
            match serde_json::from_str::<crate::datadogV2::model::OutcomesBatchResponse>(
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
            let local_entity: Option<CreateScorecardOutcomesBatchError> =
                serde_json::from_str(&local_content).ok();
            let local_error = datadog::ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(datadog::Error::ResponseError(local_error))
        }
    }

    /// Creates a new rule.
    pub async fn create_scorecard_rule(
        &self,
        body: crate::datadogV2::model::CreateRuleRequest,
    ) -> Result<crate::datadogV2::model::CreateRuleResponse, datadog::Error<CreateScorecardRuleError>>
    {
        match self.create_scorecard_rule_with_http_info(body).await {
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

    /// Creates a new rule.
    pub async fn create_scorecard_rule_with_http_info(
        &self,
        body: crate::datadogV2::model::CreateRuleRequest,
    ) -> Result<
        datadog::ResponseContent<crate::datadogV2::model::CreateRuleResponse>,
        datadog::Error<CreateScorecardRuleError>,
    > {
        let local_configuration = &self.config;
        let operation_id = "v2.create_scorecard_rule";
        if local_configuration.is_unstable_operation_enabled(operation_id) {
            warn!("Using unstable operation {operation_id}");
        } else {
            let local_error = datadog::UnstableOperationDisabledError {
                msg: "Operation 'v2.create_scorecard_rule' is not enabled".to_string(),
            };
            return Err(datadog::Error::UnstableOperationDisabledError(local_error));
        }

        let local_client = &self.client;

        let local_uri_str = format!(
            "{}/api/v2/scorecard/rules",
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
            match serde_json::from_str::<crate::datadogV2::model::CreateRuleResponse>(
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
            let local_entity: Option<CreateScorecardRuleError> =
                serde_json::from_str(&local_content).ok();
            let local_error = datadog::ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(datadog::Error::ResponseError(local_error))
        }
    }

    /// Deletes a single rule.
    pub async fn delete_scorecard_rule(
        &self,
        rule_id: String,
    ) -> Result<(), datadog::Error<DeleteScorecardRuleError>> {
        match self.delete_scorecard_rule_with_http_info(rule_id).await {
            Ok(_) => Ok(()),
            Err(err) => Err(err),
        }
    }

    /// Deletes a single rule.
    pub async fn delete_scorecard_rule_with_http_info(
        &self,
        rule_id: String,
    ) -> Result<datadog::ResponseContent<()>, datadog::Error<DeleteScorecardRuleError>> {
        let local_configuration = &self.config;
        let operation_id = "v2.delete_scorecard_rule";
        if local_configuration.is_unstable_operation_enabled(operation_id) {
            warn!("Using unstable operation {operation_id}");
        } else {
            let local_error = datadog::UnstableOperationDisabledError {
                msg: "Operation 'v2.delete_scorecard_rule' is not enabled".to_string(),
            };
            return Err(datadog::Error::UnstableOperationDisabledError(local_error));
        }

        let local_client = &self.client;

        let local_uri_str = format!(
            "{}/api/v2/scorecard/rules/{rule_id}",
            local_configuration.get_operation_host(operation_id),
            rule_id = datadog::urlencode(rule_id)
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
            let local_entity: Option<DeleteScorecardRuleError> =
                serde_json::from_str(&local_content).ok();
            let local_error = datadog::ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(datadog::Error::ResponseError(local_error))
        }
    }

    /// Fetches all rule outcomes.
    pub async fn list_scorecard_outcomes(
        &self,
        params: ListScorecardOutcomesOptionalParams,
    ) -> Result<crate::datadogV2::model::OutcomesResponse, datadog::Error<ListScorecardOutcomesError>>
    {
        match self.list_scorecard_outcomes_with_http_info(params).await {
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

    pub fn list_scorecard_outcomes_with_pagination(
        &self,
        mut params: ListScorecardOutcomesOptionalParams,
    ) -> impl Stream<
        Item = Result<
            crate::datadogV2::model::OutcomesResponseDataItem,
            datadog::Error<ListScorecardOutcomesError>,
        >,
    > + '_ {
        try_stream! {
            let mut page_size: i64 = 10;
            if params.page_size.is_none() {
                params.page_size = Some(page_size);
            } else {
                page_size = params.page_size.unwrap().clone();
            }
            loop {
                let resp = self.list_scorecard_outcomes(params.clone()).await?;
                let Some(data) = resp.data else { break };

                let r = data;
                let count = r.len();
                for team in r {
                    yield team;
                }

                if count < page_size as usize {
                    break;
                }
                if params.page_offset.is_none() {
                    params.page_offset = Some(page_size.clone());
                } else {
                    params.page_offset = Some(params.page_offset.unwrap() + page_size.clone());
                }
            }
        }
    }

    /// Fetches all rule outcomes.
    pub async fn list_scorecard_outcomes_with_http_info(
        &self,
        params: ListScorecardOutcomesOptionalParams,
    ) -> Result<
        datadog::ResponseContent<crate::datadogV2::model::OutcomesResponse>,
        datadog::Error<ListScorecardOutcomesError>,
    > {
        let local_configuration = &self.config;
        let operation_id = "v2.list_scorecard_outcomes";
        if local_configuration.is_unstable_operation_enabled(operation_id) {
            warn!("Using unstable operation {operation_id}");
        } else {
            let local_error = datadog::UnstableOperationDisabledError {
                msg: "Operation 'v2.list_scorecard_outcomes' is not enabled".to_string(),
            };
            return Err(datadog::Error::UnstableOperationDisabledError(local_error));
        }

        // unbox and build optional parameters
        let page_size = params.page_size;
        let page_offset = params.page_offset;
        let include = params.include;
        let fields_outcome = params.fields_outcome;
        let fields_rule = params.fields_rule;
        let filter_outcome_service_name = params.filter_outcome_service_name;
        let filter_outcome_state = params.filter_outcome_state;
        let filter_rule_enabled = params.filter_rule_enabled;
        let filter_rule_id = params.filter_rule_id;
        let filter_rule_name = params.filter_rule_name;

        let local_client = &self.client;

        let local_uri_str = format!(
            "{}/api/v2/scorecard/outcomes",
            local_configuration.get_operation_host(operation_id)
        );
        let mut local_req_builder =
            local_client.request(reqwest::Method::GET, local_uri_str.as_str());

        if let Some(ref local_query_param) = page_size {
            local_req_builder =
                local_req_builder.query(&[("page[size]", &local_query_param.to_string())]);
        };
        if let Some(ref local_query_param) = page_offset {
            local_req_builder =
                local_req_builder.query(&[("page[offset]", &local_query_param.to_string())]);
        };
        if let Some(ref local_query_param) = include {
            local_req_builder =
                local_req_builder.query(&[("include", &local_query_param.to_string())]);
        };
        if let Some(ref local_query_param) = fields_outcome {
            local_req_builder =
                local_req_builder.query(&[("fields[outcome]", &local_query_param.to_string())]);
        };
        if let Some(ref local_query_param) = fields_rule {
            local_req_builder =
                local_req_builder.query(&[("fields[rule]", &local_query_param.to_string())]);
        };
        if let Some(ref local_query_param) = filter_outcome_service_name {
            local_req_builder = local_req_builder.query(&[(
                "filter[outcome][service_name]",
                &local_query_param.to_string(),
            )]);
        };
        if let Some(ref local_query_param) = filter_outcome_state {
            local_req_builder = local_req_builder
                .query(&[("filter[outcome][state]", &local_query_param.to_string())]);
        };
        if let Some(ref local_query_param) = filter_rule_enabled {
            local_req_builder = local_req_builder
                .query(&[("filter[rule][enabled]", &local_query_param.to_string())]);
        };
        if let Some(ref local_query_param) = filter_rule_id {
            local_req_builder =
                local_req_builder.query(&[("filter[rule][id]", &local_query_param.to_string())]);
        };
        if let Some(ref local_query_param) = filter_rule_name {
            local_req_builder =
                local_req_builder.query(&[("filter[rule][name]", &local_query_param.to_string())]);
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
            match serde_json::from_str::<crate::datadogV2::model::OutcomesResponse>(&local_content)
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
            let local_entity: Option<ListScorecardOutcomesError> =
                serde_json::from_str(&local_content).ok();
            let local_error = datadog::ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(datadog::Error::ResponseError(local_error))
        }
    }

    /// Fetch all rules.
    pub async fn list_scorecard_rules(
        &self,
        params: ListScorecardRulesOptionalParams,
    ) -> Result<crate::datadogV2::model::ListRulesResponse, datadog::Error<ListScorecardRulesError>>
    {
        match self.list_scorecard_rules_with_http_info(params).await {
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

    pub fn list_scorecard_rules_with_pagination(
        &self,
        mut params: ListScorecardRulesOptionalParams,
    ) -> impl Stream<
        Item = Result<
            crate::datadogV2::model::ListRulesResponseDataItem,
            datadog::Error<ListScorecardRulesError>,
        >,
    > + '_ {
        try_stream! {
            let mut page_size: i64 = 10;
            if params.page_size.is_none() {
                params.page_size = Some(page_size);
            } else {
                page_size = params.page_size.unwrap().clone();
            }
            loop {
                let resp = self.list_scorecard_rules(params.clone()).await?;
                let Some(data) = resp.data else { break };

                let r = data;
                let count = r.len();
                for team in r {
                    yield team;
                }

                if count < page_size as usize {
                    break;
                }
                if params.page_offset.is_none() {
                    params.page_offset = Some(page_size.clone());
                } else {
                    params.page_offset = Some(params.page_offset.unwrap() + page_size.clone());
                }
            }
        }
    }

    /// Fetch all rules.
    pub async fn list_scorecard_rules_with_http_info(
        &self,
        params: ListScorecardRulesOptionalParams,
    ) -> Result<
        datadog::ResponseContent<crate::datadogV2::model::ListRulesResponse>,
        datadog::Error<ListScorecardRulesError>,
    > {
        let local_configuration = &self.config;
        let operation_id = "v2.list_scorecard_rules";
        if local_configuration.is_unstable_operation_enabled(operation_id) {
            warn!("Using unstable operation {operation_id}");
        } else {
            let local_error = datadog::UnstableOperationDisabledError {
                msg: "Operation 'v2.list_scorecard_rules' is not enabled".to_string(),
            };
            return Err(datadog::Error::UnstableOperationDisabledError(local_error));
        }

        // unbox and build optional parameters
        let page_size = params.page_size;
        let page_offset = params.page_offset;
        let include = params.include;
        let filter_rule_id = params.filter_rule_id;
        let filter_rule_enabled = params.filter_rule_enabled;
        let filter_rule_custom = params.filter_rule_custom;
        let filter_rule_name = params.filter_rule_name;
        let filter_rule_description = params.filter_rule_description;
        let fields_rule = params.fields_rule;
        let fields_scorecard = params.fields_scorecard;

        let local_client = &self.client;

        let local_uri_str = format!(
            "{}/api/v2/scorecard/rules",
            local_configuration.get_operation_host(operation_id)
        );
        let mut local_req_builder =
            local_client.request(reqwest::Method::GET, local_uri_str.as_str());

        if let Some(ref local_query_param) = page_size {
            local_req_builder =
                local_req_builder.query(&[("page[size]", &local_query_param.to_string())]);
        };
        if let Some(ref local_query_param) = page_offset {
            local_req_builder =
                local_req_builder.query(&[("page[offset]", &local_query_param.to_string())]);
        };
        if let Some(ref local_query_param) = include {
            local_req_builder =
                local_req_builder.query(&[("include", &local_query_param.to_string())]);
        };
        if let Some(ref local_query_param) = filter_rule_id {
            local_req_builder =
                local_req_builder.query(&[("filter[rule][id]", &local_query_param.to_string())]);
        };
        if let Some(ref local_query_param) = filter_rule_enabled {
            local_req_builder = local_req_builder
                .query(&[("filter[rule][enabled]", &local_query_param.to_string())]);
        };
        if let Some(ref local_query_param) = filter_rule_custom {
            local_req_builder = local_req_builder
                .query(&[("filter[rule][custom]", &local_query_param.to_string())]);
        };
        if let Some(ref local_query_param) = filter_rule_name {
            local_req_builder =
                local_req_builder.query(&[("filter[rule][name]", &local_query_param.to_string())]);
        };
        if let Some(ref local_query_param) = filter_rule_description {
            local_req_builder = local_req_builder
                .query(&[("filter[rule][description]", &local_query_param.to_string())]);
        };
        if let Some(ref local_query_param) = fields_rule {
            local_req_builder =
                local_req_builder.query(&[("fields[rule]", &local_query_param.to_string())]);
        };
        if let Some(ref local_query_param) = fields_scorecard {
            local_req_builder =
                local_req_builder.query(&[("fields[scorecard]", &local_query_param.to_string())]);
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
            match serde_json::from_str::<crate::datadogV2::model::ListRulesResponse>(&local_content)
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
            let local_entity: Option<ListScorecardRulesError> =
                serde_json::from_str(&local_content).ok();
            let local_error = datadog::ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(datadog::Error::ResponseError(local_error))
        }
    }

    /// Updates an existing rule.
    pub async fn update_scorecard_rule(
        &self,
        rule_id: String,
        body: crate::datadogV2::model::UpdateRuleRequest,
    ) -> Result<crate::datadogV2::model::UpdateRuleResponse, datadog::Error<UpdateScorecardRuleError>>
    {
        match self
            .update_scorecard_rule_with_http_info(rule_id, body)
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

    /// Updates an existing rule.
    pub async fn update_scorecard_rule_with_http_info(
        &self,
        rule_id: String,
        body: crate::datadogV2::model::UpdateRuleRequest,
    ) -> Result<
        datadog::ResponseContent<crate::datadogV2::model::UpdateRuleResponse>,
        datadog::Error<UpdateScorecardRuleError>,
    > {
        let local_configuration = &self.config;
        let operation_id = "v2.update_scorecard_rule";
        if local_configuration.is_unstable_operation_enabled(operation_id) {
            warn!("Using unstable operation {operation_id}");
        } else {
            let local_error = datadog::UnstableOperationDisabledError {
                msg: "Operation 'v2.update_scorecard_rule' is not enabled".to_string(),
            };
            return Err(datadog::Error::UnstableOperationDisabledError(local_error));
        }

        let local_client = &self.client;

        let local_uri_str = format!(
            "{}/api/v2/scorecard/rules/{rule_id}",
            local_configuration.get_operation_host(operation_id),
            rule_id = datadog::urlencode(rule_id)
        );
        let mut local_req_builder =
            local_client.request(reqwest::Method::PUT, local_uri_str.as_str());

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
            match serde_json::from_str::<crate::datadogV2::model::UpdateRuleResponse>(
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
            let local_entity: Option<UpdateScorecardRuleError> =
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
