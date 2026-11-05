// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use crate::datadog;
use log::warn;
use reqwest::header::{HeaderMap, HeaderValue};
use serde::{Deserialize, Serialize};

/// CountSecurityAgentInvestigationsOptionalParams is a struct for passing parameters to the method [`BitsSecurityAnalystAPI::count_security_agent_investigations`]
#[non_exhaustive]
#[derive(Clone, Default, Debug)]
pub struct CountSecurityAgentInvestigationsOptionalParams {
    /// Only investigations whose signal title contains this text, case-insensitive.
    pub filter_title: Option<String>,
    /// Only investigations last updated at or after this time, as RFC 3339 or milliseconds since the epoch.
    pub filter_from: Option<String>,
    /// Only investigations last updated at or before this time, as RFC 3339 or milliseconds since the epoch.
    pub filter_to: Option<String>,
    /// Only investigations with one of these verdicts: `benign`, `suspicious`, or `malicious`.
    /// Comma-separated values; an investigation matches any of them.
    pub filter_verdict: Option<String>,
    /// Only investigations of signals from one of these detection rule IDs. Comma-separated values; an
    /// investigation matches any of them.
    pub filter_rule_id: Option<String>,
    /// Only investigations of signals from detection rules of one of these types, as returned by the count with
    /// `field=rule_type`. Comma-separated values; an investigation matches any of them.
    pub filter_rule_type: Option<String>,
    /// Only investigations of signals with one of these severities: `info`, `low`, `medium`, `high`, or `critical`.
    /// Comma-separated values; an investigation matches any of them.
    pub filter_signal_severity: Option<String>,
    /// Only investigations of signals from one of these sources, as returned by the count with
    /// `field=signal_source`. Comma-separated values; an investigation matches any of them.
    pub filter_signal_source: Option<String>,
    /// Only investigations whose signal has all of these tags, written `key:value`. Comma-separated values; an
    /// investigation must have every one of them.
    pub filter_tags: Option<String>,
    /// Only investigations whose signal has all of these entities. Write each entity as `attribute:value`, matching the
    /// values returned by the count endpoint with `field=entity`. Comma-separated values; an investigation must have
    /// every one of them.
    pub filter_entity: Option<String>,
}

impl CountSecurityAgentInvestigationsOptionalParams {
    /// Only investigations whose signal title contains this text, case-insensitive.
    pub fn filter_title(mut self, value: String) -> Self {
        self.filter_title = Some(value);
        self
    }
    /// Only investigations last updated at or after this time, as RFC 3339 or milliseconds since the epoch.
    pub fn filter_from(mut self, value: String) -> Self {
        self.filter_from = Some(value);
        self
    }
    /// Only investigations last updated at or before this time, as RFC 3339 or milliseconds since the epoch.
    pub fn filter_to(mut self, value: String) -> Self {
        self.filter_to = Some(value);
        self
    }
    /// Only investigations with one of these verdicts: `benign`, `suspicious`, or `malicious`.
    /// Comma-separated values; an investigation matches any of them.
    pub fn filter_verdict(mut self, value: String) -> Self {
        self.filter_verdict = Some(value);
        self
    }
    /// Only investigations of signals from one of these detection rule IDs. Comma-separated values; an
    /// investigation matches any of them.
    pub fn filter_rule_id(mut self, value: String) -> Self {
        self.filter_rule_id = Some(value);
        self
    }
    /// Only investigations of signals from detection rules of one of these types, as returned by the count with
    /// `field=rule_type`. Comma-separated values; an investigation matches any of them.
    pub fn filter_rule_type(mut self, value: String) -> Self {
        self.filter_rule_type = Some(value);
        self
    }
    /// Only investigations of signals with one of these severities: `info`, `low`, `medium`, `high`, or `critical`.
    /// Comma-separated values; an investigation matches any of them.
    pub fn filter_signal_severity(mut self, value: String) -> Self {
        self.filter_signal_severity = Some(value);
        self
    }
    /// Only investigations of signals from one of these sources, as returned by the count with
    /// `field=signal_source`. Comma-separated values; an investigation matches any of them.
    pub fn filter_signal_source(mut self, value: String) -> Self {
        self.filter_signal_source = Some(value);
        self
    }
    /// Only investigations whose signal has all of these tags, written `key:value`. Comma-separated values; an
    /// investigation must have every one of them.
    pub fn filter_tags(mut self, value: String) -> Self {
        self.filter_tags = Some(value);
        self
    }
    /// Only investigations whose signal has all of these entities. Write each entity as `attribute:value`, matching the
    /// values returned by the count endpoint with `field=entity`. Comma-separated values; an investigation must have
    /// every one of them.
    pub fn filter_entity(mut self, value: String) -> Self {
        self.filter_entity = Some(value);
        self
    }
}

/// GetSecurityAgentInvestigationOptionalParams is a struct for passing parameters to the method [`BitsSecurityAnalystAPI::get_security_agent_investigation`]
#[non_exhaustive]
#[derive(Clone, Default, Debug)]
pub struct GetSecurityAgentInvestigationOptionalParams {
    /// Attributes to return, comma-separated. Defaults to all attributes.
    pub fields_investigations:
        Option<Vec<crate::datadogV2::model::SecurityAgentInvestigationField>>,
}

impl GetSecurityAgentInvestigationOptionalParams {
    /// Attributes to return, comma-separated. Defaults to all attributes.
    pub fn fields_investigations(
        mut self,
        value: Vec<crate::datadogV2::model::SecurityAgentInvestigationField>,
    ) -> Self {
        self.fields_investigations = Some(value);
        self
    }
}

/// ListSecurityAgentInvestigationsOptionalParams is a struct for passing parameters to the method [`BitsSecurityAnalystAPI::list_security_agent_investigations`]
#[non_exhaustive]
#[derive(Clone, Default, Debug)]
pub struct ListSecurityAgentInvestigationsOptionalParams {
    /// List every investigation of this signal, newest first. Accepts the signal ID in its V1 form, which starts
    /// with `AQ`, or in its longer V3 form, which starts with `Aw`. Investigations carry the V1 form, so their
    /// `signal_id` can differ from the ID you pass. Can't be combined with other filters or with `sort`.
    pub filter_signal_id: Option<String>,
    /// Only investigations whose signal title contains this text, case-insensitive.
    pub filter_title: Option<String>,
    /// Only investigations last updated at or after this time, as RFC 3339 or milliseconds since the epoch.
    pub filter_from: Option<String>,
    /// Only investigations last updated at or before this time, as RFC 3339 or milliseconds since the epoch.
    pub filter_to: Option<String>,
    /// Only investigations with one of these verdicts: `benign`, `suspicious`, or `malicious`.
    /// Comma-separated values; an investigation matches any of them.
    pub filter_verdict: Option<String>,
    /// Only investigations in one of these states: `investigating` or `completed`. Investigations reported as
    /// `failed` match `investigating`. Comma-separated values; an investigation matches any of them.
    pub filter_state: Option<String>,
    /// Only investigations of signals from one of these detection rule IDs. Comma-separated values; an
    /// investigation matches any of them.
    pub filter_rule_id: Option<String>,
    /// Only investigations of signals from detection rules of one of these types, as returned by the count with
    /// `field=rule_type`. Comma-separated values; an investigation matches any of them.
    pub filter_rule_type: Option<String>,
    /// Only investigations of signals with one of these severities: `info`, `low`, `medium`, `high`, or `critical`.
    /// Comma-separated values; an investigation matches any of them.
    pub filter_signal_severity: Option<String>,
    /// Only investigations of signals from one of these sources, as returned by the count with
    /// `field=signal_source`. Comma-separated values; an investigation matches any of them.
    pub filter_signal_source: Option<String>,
    /// Only investigations whose signal has all of these tags, written `key:value`. Comma-separated values; an
    /// investigation must have every one of them.
    pub filter_tags: Option<String>,
    /// Only investigations whose signal has all of these entities. Write each entity as `attribute:value`, matching the
    /// values returned by the count endpoint with `field=entity`. Comma-separated values; an investigation must have
    /// every one of them.
    pub filter_entity: Option<String>,
    /// Sort field, prefixed with `-` for descending: `updated_at`, `severity`, `verdict`, `rule_id`, or `signal_title`.
    /// Defaults to `-updated_at`. `verdict` sorts by severity (`benign`, `suspicious`, `malicious`), so `-verdict` puts
    /// `malicious` first.
    pub sort: Option<String>,
    /// Page number, starting at 1.
    pub page_number: Option<i64>,
    /// Number of investigations per page.
    pub page_size: Option<i64>,
}

impl ListSecurityAgentInvestigationsOptionalParams {
    /// List every investigation of this signal, newest first. Accepts the signal ID in its V1 form, which starts
    /// with `AQ`, or in its longer V3 form, which starts with `Aw`. Investigations carry the V1 form, so their
    /// `signal_id` can differ from the ID you pass. Can't be combined with other filters or with `sort`.
    pub fn filter_signal_id(mut self, value: String) -> Self {
        self.filter_signal_id = Some(value);
        self
    }
    /// Only investigations whose signal title contains this text, case-insensitive.
    pub fn filter_title(mut self, value: String) -> Self {
        self.filter_title = Some(value);
        self
    }
    /// Only investigations last updated at or after this time, as RFC 3339 or milliseconds since the epoch.
    pub fn filter_from(mut self, value: String) -> Self {
        self.filter_from = Some(value);
        self
    }
    /// Only investigations last updated at or before this time, as RFC 3339 or milliseconds since the epoch.
    pub fn filter_to(mut self, value: String) -> Self {
        self.filter_to = Some(value);
        self
    }
    /// Only investigations with one of these verdicts: `benign`, `suspicious`, or `malicious`.
    /// Comma-separated values; an investigation matches any of them.
    pub fn filter_verdict(mut self, value: String) -> Self {
        self.filter_verdict = Some(value);
        self
    }
    /// Only investigations in one of these states: `investigating` or `completed`. Investigations reported as
    /// `failed` match `investigating`. Comma-separated values; an investigation matches any of them.
    pub fn filter_state(mut self, value: String) -> Self {
        self.filter_state = Some(value);
        self
    }
    /// Only investigations of signals from one of these detection rule IDs. Comma-separated values; an
    /// investigation matches any of them.
    pub fn filter_rule_id(mut self, value: String) -> Self {
        self.filter_rule_id = Some(value);
        self
    }
    /// Only investigations of signals from detection rules of one of these types, as returned by the count with
    /// `field=rule_type`. Comma-separated values; an investigation matches any of them.
    pub fn filter_rule_type(mut self, value: String) -> Self {
        self.filter_rule_type = Some(value);
        self
    }
    /// Only investigations of signals with one of these severities: `info`, `low`, `medium`, `high`, or `critical`.
    /// Comma-separated values; an investigation matches any of them.
    pub fn filter_signal_severity(mut self, value: String) -> Self {
        self.filter_signal_severity = Some(value);
        self
    }
    /// Only investigations of signals from one of these sources, as returned by the count with
    /// `field=signal_source`. Comma-separated values; an investigation matches any of them.
    pub fn filter_signal_source(mut self, value: String) -> Self {
        self.filter_signal_source = Some(value);
        self
    }
    /// Only investigations whose signal has all of these tags, written `key:value`. Comma-separated values; an
    /// investigation must have every one of them.
    pub fn filter_tags(mut self, value: String) -> Self {
        self.filter_tags = Some(value);
        self
    }
    /// Only investigations whose signal has all of these entities. Write each entity as `attribute:value`, matching the
    /// values returned by the count endpoint with `field=entity`. Comma-separated values; an investigation must have
    /// every one of them.
    pub fn filter_entity(mut self, value: String) -> Self {
        self.filter_entity = Some(value);
        self
    }
    /// Sort field, prefixed with `-` for descending: `updated_at`, `severity`, `verdict`, `rule_id`, or `signal_title`.
    /// Defaults to `-updated_at`. `verdict` sorts by severity (`benign`, `suspicious`, `malicious`), so `-verdict` puts
    /// `malicious` first.
    pub fn sort(mut self, value: String) -> Self {
        self.sort = Some(value);
        self
    }
    /// Page number, starting at 1.
    pub fn page_number(mut self, value: i64) -> Self {
        self.page_number = Some(value);
        self
    }
    /// Number of investigations per page.
    pub fn page_size(mut self, value: i64) -> Self {
        self.page_size = Some(value);
        self
    }
}

/// CountSecurityAgentInvestigationsError is a struct for typed errors of method [`BitsSecurityAnalystAPI::count_security_agent_investigations`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CountSecurityAgentInvestigationsError {
    JSONAPIErrorResponse(crate::datadogV2::model::JSONAPIErrorResponse),
    APIErrorResponse(crate::datadogV2::model::APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// GetSecurityAgentInvestigationError is a struct for typed errors of method [`BitsSecurityAnalystAPI::get_security_agent_investigation`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetSecurityAgentInvestigationError {
    JSONAPIErrorResponse(crate::datadogV2::model::JSONAPIErrorResponse),
    APIErrorResponse(crate::datadogV2::model::APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// ListSecurityAgentInvestigationsError is a struct for typed errors of method [`BitsSecurityAnalystAPI::list_security_agent_investigations`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ListSecurityAgentInvestigationsError {
    JSONAPIErrorResponse(crate::datadogV2::model::JSONAPIErrorResponse),
    APIErrorResponse(crate::datadogV2::model::APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// Use the Bits Security Analyst endpoints to find the investigations of your Cloud SIEM signals, read an
/// investigation's verdict and findings, and count investigations.
#[derive(Debug, Clone)]
pub struct BitsSecurityAnalystAPI {
    config: datadog::Configuration,
    client: reqwest_middleware::ClientWithMiddleware,
}

impl Default for BitsSecurityAnalystAPI {
    fn default() -> Self {
        Self::with_config(datadog::Configuration::default())
    }
}

impl BitsSecurityAnalystAPI {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn with_config(config: datadog::Configuration) -> Self {
        let reqwest_client_builder = {
            let builder = config.apply_headers(reqwest::Client::builder());
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

    /// Count completed Bits Security Analyst investigations by the values of one field. Buckets are sorted by
    /// ascending count. With `field=entity` or `field=signal_source`, an investigation counts in the bucket of
    /// each of its entities or sources.
    pub async fn count_security_agent_investigations(
        &self,
        field: crate::datadogV2::model::SecurityAgentInvestigationCountField,
        params: CountSecurityAgentInvestigationsOptionalParams,
    ) -> Result<
        crate::datadogV2::model::SecurityAgentInvestigationCountsResponse,
        datadog::Error<CountSecurityAgentInvestigationsError>,
    > {
        match self
            .count_security_agent_investigations_with_http_info(field, params)
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

    /// Count completed Bits Security Analyst investigations by the values of one field. Buckets are sorted by
    /// ascending count. With `field=entity` or `field=signal_source`, an investigation counts in the bucket of
    /// each of its entities or sources.
    pub async fn count_security_agent_investigations_with_http_info(
        &self,
        field: crate::datadogV2::model::SecurityAgentInvestigationCountField,
        params: CountSecurityAgentInvestigationsOptionalParams,
    ) -> Result<
        datadog::ResponseContent<crate::datadogV2::model::SecurityAgentInvestigationCountsResponse>,
        datadog::Error<CountSecurityAgentInvestigationsError>,
    > {
        let local_configuration = &self.config;
        let local_operation_id = "v2.count_security_agent_investigations";
        if local_configuration.is_unstable_operation_enabled(local_operation_id) {
            warn!("Using unstable operation {local_operation_id}");
        } else {
            let local_error = datadog::UnstableOperationDisabledError {
                msg: "Operation 'v2.count_security_agent_investigations' is not enabled"
                    .to_string(),
            };
            return Err(datadog::Error::UnstableOperationDisabledError(local_error));
        }

        // unbox and build optional parameters
        let filter_title = params.filter_title;
        let filter_from = params.filter_from;
        let filter_to = params.filter_to;
        let filter_verdict = params.filter_verdict;
        let filter_rule_id = params.filter_rule_id;
        let filter_rule_type = params.filter_rule_type;
        let filter_signal_severity = params.filter_signal_severity;
        let filter_signal_source = params.filter_signal_source;
        let filter_tags = params.filter_tags;
        let filter_entity = params.filter_entity;

        let local_client = &self.client;

        let local_uri_str = format!(
            "{}/api/v2/security_monitoring/security_agent/investigations/count",
            local_configuration.get_operation_host(local_operation_id)
        );
        let mut local_req_builder =
            local_client.request(reqwest::Method::GET, local_uri_str.as_str());

        local_req_builder = local_req_builder.query(&[("field", &field.to_string())]);
        if let Some(ref local_query_param) = filter_title {
            local_req_builder =
                local_req_builder.query(&[("filter[title]", &local_query_param.to_string())]);
        };
        if let Some(ref local_query_param) = filter_from {
            local_req_builder =
                local_req_builder.query(&[("filter[from]", &local_query_param.to_string())]);
        };
        if let Some(ref local_query_param) = filter_to {
            local_req_builder =
                local_req_builder.query(&[("filter[to]", &local_query_param.to_string())]);
        };
        if let Some(ref local_query_param) = filter_verdict {
            local_req_builder =
                local_req_builder.query(&[("filter[verdict]", &local_query_param.to_string())]);
        };
        if let Some(ref local_query_param) = filter_rule_id {
            local_req_builder =
                local_req_builder.query(&[("filter[rule_id]", &local_query_param.to_string())]);
        };
        if let Some(ref local_query_param) = filter_rule_type {
            local_req_builder =
                local_req_builder.query(&[("filter[rule_type]", &local_query_param.to_string())]);
        };
        if let Some(ref local_query_param) = filter_signal_severity {
            local_req_builder = local_req_builder
                .query(&[("filter[signal_severity]", &local_query_param.to_string())]);
        };
        if let Some(ref local_query_param) = filter_signal_source {
            local_req_builder = local_req_builder
                .query(&[("filter[signal_source]", &local_query_param.to_string())]);
        };
        if let Some(ref local_query_param) = filter_tags {
            local_req_builder =
                local_req_builder.query(&[("filter[tags]", &local_query_param.to_string())]);
        };
        if let Some(ref local_query_param) = filter_entity {
            local_req_builder =
                local_req_builder.query(&[("filter[entity]", &local_query_param.to_string())]);
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
                crate::datadogV2::model::SecurityAgentInvestigationCountsResponse,
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
            let local_entity: Option<CountSecurityAgentInvestigationsError> =
                serde_json::from_str(&local_content).ok();
            let local_error = datadog::ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(datadog::Error::ResponseError(local_error))
        }
    }

    /// Get a Bits Security Analyst investigation by its ID, with its verdict, conclusion, evidence, steps, and
    /// response suggestions. While the investigation is running, the response has a `Retry-After` header.
    pub async fn get_security_agent_investigation(
        &self,
        investigation_id: String,
        params: GetSecurityAgentInvestigationOptionalParams,
    ) -> Result<
        crate::datadogV2::model::SecurityAgentInvestigationResponse,
        datadog::Error<GetSecurityAgentInvestigationError>,
    > {
        match self
            .get_security_agent_investigation_with_http_info(investigation_id, params)
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

    /// Get a Bits Security Analyst investigation by its ID, with its verdict, conclusion, evidence, steps, and
    /// response suggestions. While the investigation is running, the response has a `Retry-After` header.
    pub async fn get_security_agent_investigation_with_http_info(
        &self,
        investigation_id: String,
        params: GetSecurityAgentInvestigationOptionalParams,
    ) -> Result<
        datadog::ResponseContent<crate::datadogV2::model::SecurityAgentInvestigationResponse>,
        datadog::Error<GetSecurityAgentInvestigationError>,
    > {
        let local_configuration = &self.config;
        let local_operation_id = "v2.get_security_agent_investigation";
        if local_configuration.is_unstable_operation_enabled(local_operation_id) {
            warn!("Using unstable operation {local_operation_id}");
        } else {
            let local_error = datadog::UnstableOperationDisabledError {
                msg: "Operation 'v2.get_security_agent_investigation' is not enabled".to_string(),
            };
            return Err(datadog::Error::UnstableOperationDisabledError(local_error));
        }

        // unbox and build optional parameters
        let fields_investigations = params.fields_investigations;

        let local_client = &self.client;

        let local_uri_str = format!(
            "{}/api/v2/security_monitoring/security_agent/investigations/{investigation_id}",
            local_configuration.get_operation_host(local_operation_id),
            investigation_id = datadog::urlencode(investigation_id)
        );
        let mut local_req_builder =
            local_client.request(reqwest::Method::GET, local_uri_str.as_str());

        if let Some(ref local) = fields_investigations {
            local_req_builder = local_req_builder.query(&[(
                "fields[investigations]",
                &local
                    .iter()
                    .map(|p| p.to_string())
                    .collect::<Vec<String>>()
                    .join(",")
                    .to_string(),
            )]);
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
            match serde_json::from_str::<crate::datadogV2::model::SecurityAgentInvestigationResponse>(
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
            let local_entity: Option<GetSecurityAgentInvestigationError> =
                serde_json::from_str(&local_content).ok();
            let local_error = datadog::ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(datadog::Error::ResponseError(local_error))
        }
    }

    /// List Bits Security Analyst investigations.
    ///
    /// Without `filter[signal_id]`, returns the latest investigation of each signal, filtered, sorted, and paginated.
    /// With `filter[signal_id]`, returns every investigation of that signal, newest first. When the signal has no
    /// investigation, the first page carries `meta.reason`, which says why.
    pub async fn list_security_agent_investigations(
        &self,
        params: ListSecurityAgentInvestigationsOptionalParams,
    ) -> Result<
        crate::datadogV2::model::SecurityAgentInvestigationsListResponse,
        datadog::Error<ListSecurityAgentInvestigationsError>,
    > {
        match self
            .list_security_agent_investigations_with_http_info(params)
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

    /// List Bits Security Analyst investigations.
    ///
    /// Without `filter[signal_id]`, returns the latest investigation of each signal, filtered, sorted, and paginated.
    /// With `filter[signal_id]`, returns every investigation of that signal, newest first. When the signal has no
    /// investigation, the first page carries `meta.reason`, which says why.
    pub async fn list_security_agent_investigations_with_http_info(
        &self,
        params: ListSecurityAgentInvestigationsOptionalParams,
    ) -> Result<
        datadog::ResponseContent<crate::datadogV2::model::SecurityAgentInvestigationsListResponse>,
        datadog::Error<ListSecurityAgentInvestigationsError>,
    > {
        let local_configuration = &self.config;
        let local_operation_id = "v2.list_security_agent_investigations";
        if local_configuration.is_unstable_operation_enabled(local_operation_id) {
            warn!("Using unstable operation {local_operation_id}");
        } else {
            let local_error = datadog::UnstableOperationDisabledError {
                msg: "Operation 'v2.list_security_agent_investigations' is not enabled".to_string(),
            };
            return Err(datadog::Error::UnstableOperationDisabledError(local_error));
        }

        // unbox and build optional parameters
        let filter_signal_id = params.filter_signal_id;
        let filter_title = params.filter_title;
        let filter_from = params.filter_from;
        let filter_to = params.filter_to;
        let filter_verdict = params.filter_verdict;
        let filter_state = params.filter_state;
        let filter_rule_id = params.filter_rule_id;
        let filter_rule_type = params.filter_rule_type;
        let filter_signal_severity = params.filter_signal_severity;
        let filter_signal_source = params.filter_signal_source;
        let filter_tags = params.filter_tags;
        let filter_entity = params.filter_entity;
        let sort = params.sort;
        let page_number = params.page_number;
        let page_size = params.page_size;

        let local_client = &self.client;

        let local_uri_str = format!(
            "{}/api/v2/security_monitoring/security_agent/investigations",
            local_configuration.get_operation_host(local_operation_id)
        );
        let mut local_req_builder =
            local_client.request(reqwest::Method::GET, local_uri_str.as_str());

        if let Some(ref local_query_param) = filter_signal_id {
            local_req_builder =
                local_req_builder.query(&[("filter[signal_id]", &local_query_param.to_string())]);
        };
        if let Some(ref local_query_param) = filter_title {
            local_req_builder =
                local_req_builder.query(&[("filter[title]", &local_query_param.to_string())]);
        };
        if let Some(ref local_query_param) = filter_from {
            local_req_builder =
                local_req_builder.query(&[("filter[from]", &local_query_param.to_string())]);
        };
        if let Some(ref local_query_param) = filter_to {
            local_req_builder =
                local_req_builder.query(&[("filter[to]", &local_query_param.to_string())]);
        };
        if let Some(ref local_query_param) = filter_verdict {
            local_req_builder =
                local_req_builder.query(&[("filter[verdict]", &local_query_param.to_string())]);
        };
        if let Some(ref local_query_param) = filter_state {
            local_req_builder =
                local_req_builder.query(&[("filter[state]", &local_query_param.to_string())]);
        };
        if let Some(ref local_query_param) = filter_rule_id {
            local_req_builder =
                local_req_builder.query(&[("filter[rule_id]", &local_query_param.to_string())]);
        };
        if let Some(ref local_query_param) = filter_rule_type {
            local_req_builder =
                local_req_builder.query(&[("filter[rule_type]", &local_query_param.to_string())]);
        };
        if let Some(ref local_query_param) = filter_signal_severity {
            local_req_builder = local_req_builder
                .query(&[("filter[signal_severity]", &local_query_param.to_string())]);
        };
        if let Some(ref local_query_param) = filter_signal_source {
            local_req_builder = local_req_builder
                .query(&[("filter[signal_source]", &local_query_param.to_string())]);
        };
        if let Some(ref local_query_param) = filter_tags {
            local_req_builder =
                local_req_builder.query(&[("filter[tags]", &local_query_param.to_string())]);
        };
        if let Some(ref local_query_param) = filter_entity {
            local_req_builder =
                local_req_builder.query(&[("filter[entity]", &local_query_param.to_string())]);
        };
        if let Some(ref local_query_param) = sort {
            local_req_builder =
                local_req_builder.query(&[("sort", &local_query_param.to_string())]);
        };
        if let Some(ref local_query_param) = page_number {
            local_req_builder =
                local_req_builder.query(&[("page[number]", &local_query_param.to_string())]);
        };
        if let Some(ref local_query_param) = page_size {
            local_req_builder =
                local_req_builder.query(&[("page[size]", &local_query_param.to_string())]);
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
                crate::datadogV2::model::SecurityAgentInvestigationsListResponse,
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
            let local_entity: Option<ListSecurityAgentInvestigationsError> =
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
