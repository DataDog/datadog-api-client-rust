// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use crate::datadog;
use log::warn;
use reqwest::header::{HeaderMap, HeaderValue};
use serde::{Deserialize, Serialize};

/// GetRuleBasedViewOptionalParams is a struct for passing parameters to the method [`ComplianceAPI::get_rule_based_view`]
#[non_exhaustive]
#[derive(Clone, Default, Debug)]
pub struct GetRuleBasedViewOptionalParams {
    /// Compliance framework handle to filter rules and findings by.
    pub framework: Option<String>,
    /// Version of the compliance framework to filter rules and findings by.
    pub version: Option<String>,
    /// When `true`, returns findings without a `framework_version` tag. Used for findings from custom frameworks or those created before framework versioning was introduced.
    pub query_findings_without_framework_version: Option<bool>,
    /// When `true`, includes rules in the response that have no associated findings.
    pub include_rules_without_findings: Option<bool>,
    /// Set to `true` when the requested `framework` is a custom framework.
    pub is_custom: Option<bool>,
    /// Additional event-platform filters applied to the underlying findings query. For example, `scored:true project_id:datadog-prod-us5`.
    pub query: Option<String>,
}

impl GetRuleBasedViewOptionalParams {
    /// Compliance framework handle to filter rules and findings by.
    pub fn framework(mut self, value: String) -> Self {
        self.framework = Some(value);
        self
    }
    /// Version of the compliance framework to filter rules and findings by.
    pub fn version(mut self, value: String) -> Self {
        self.version = Some(value);
        self
    }
    /// When `true`, returns findings without a `framework_version` tag. Used for findings from custom frameworks or those created before framework versioning was introduced.
    pub fn query_findings_without_framework_version(mut self, value: bool) -> Self {
        self.query_findings_without_framework_version = Some(value);
        self
    }
    /// When `true`, includes rules in the response that have no associated findings.
    pub fn include_rules_without_findings(mut self, value: bool) -> Self {
        self.include_rules_without_findings = Some(value);
        self
    }
    /// Set to `true` when the requested `framework` is a custom framework.
    pub fn is_custom(mut self, value: bool) -> Self {
        self.is_custom = Some(value);
        self
    }
    /// Additional event-platform filters applied to the underlying findings query. For example, `scored:true project_id:datadog-prod-us5`.
    pub fn query(mut self, value: String) -> Self {
        self.query = Some(value);
        self
    }
}

/// GetRuleBasedViewError is a struct for typed errors of method [`ComplianceAPI::get_rule_based_view`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetRuleBasedViewError {
    JSONAPIErrorResponse(crate::datadogV2::model::JSONAPIErrorResponse),
    APIErrorResponse(crate::datadogV2::model::APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// Datadog Cloud Security Misconfigurations provides aggregated views of
/// compliance rules and findings across your cloud resources, helping you assess
/// posture against industry frameworks (such as HIPAA, SOC 2, ISO 27001) and custom
/// frameworks. Learn more at <https://docs.datadoghq.com/security/cloud_security_management/misconfigurations/#maintain-compliance-with-industry-frameworks-and-benchmarks.>
#[derive(Debug, Clone)]
pub struct ComplianceAPI {
    config: datadog::Configuration,
    client: reqwest_middleware::ClientWithMiddleware,
}

impl Default for ComplianceAPI {
    fn default() -> Self {
        Self::with_config(datadog::Configuration::default())
    }
}

impl ComplianceAPI {
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

    /// Get an aggregated view of compliance rules with their pass, fail, and muted finding counts.
    /// Supports filtering by compliance framework, framework version, and additional query filters.
    pub async fn get_rule_based_view(
        &self,
        to: i64,
        params: GetRuleBasedViewOptionalParams,
    ) -> Result<crate::datadogV2::model::RuleBasedViewResponse, datadog::Error<GetRuleBasedViewError>>
    {
        match self.get_rule_based_view_with_http_info(to, params).await {
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

    /// Get an aggregated view of compliance rules with their pass, fail, and muted finding counts.
    /// Supports filtering by compliance framework, framework version, and additional query filters.
    pub async fn get_rule_based_view_with_http_info(
        &self,
        to: i64,
        params: GetRuleBasedViewOptionalParams,
    ) -> Result<
        datadog::ResponseContent<crate::datadogV2::model::RuleBasedViewResponse>,
        datadog::Error<GetRuleBasedViewError>,
    > {
        let local_configuration = &self.config;
        let operation_id = "v2.get_rule_based_view";
        if local_configuration.is_unstable_operation_enabled(operation_id) {
            warn!("Using unstable operation {operation_id}");
        } else {
            let local_error = datadog::UnstableOperationDisabledError {
                msg: "Operation 'v2.get_rule_based_view' is not enabled".to_string(),
            };
            return Err(datadog::Error::UnstableOperationDisabledError(local_error));
        }

        // unbox and build optional parameters
        let framework = params.framework;
        let version = params.version;
        let query_findings_without_framework_version =
            params.query_findings_without_framework_version;
        let include_rules_without_findings = params.include_rules_without_findings;
        let is_custom = params.is_custom;
        let query = params.query;

        let local_client = &self.client;

        let local_uri_str = format!(
            "{}/api/v2/compliance_findings/rule_based_view",
            local_configuration.get_operation_host(operation_id)
        );
        let mut local_req_builder =
            local_client.request(reqwest::Method::GET, local_uri_str.as_str());

        local_req_builder = local_req_builder.query(&[("to", &to.to_string())]);
        if let Some(ref local_query_param) = framework {
            local_req_builder =
                local_req_builder.query(&[("framework", &local_query_param.to_string())]);
        };
        if let Some(ref local_query_param) = version {
            local_req_builder =
                local_req_builder.query(&[("version", &local_query_param.to_string())]);
        };
        if let Some(ref local_query_param) = query_findings_without_framework_version {
            local_req_builder = local_req_builder.query(&[(
                "query_findings_without_framework_version",
                &local_query_param.to_string(),
            )]);
        };
        if let Some(ref local_query_param) = include_rules_without_findings {
            local_req_builder = local_req_builder.query(&[(
                "include_rules_without_findings",
                &local_query_param.to_string(),
            )]);
        };
        if let Some(ref local_query_param) = is_custom {
            local_req_builder =
                local_req_builder.query(&[("is_custom", &local_query_param.to_string())]);
        };
        if let Some(ref local_query_param) = query {
            local_req_builder =
                local_req_builder.query(&[("query", &local_query_param.to_string())]);
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
            match serde_json::from_str::<crate::datadogV2::model::RuleBasedViewResponse>(
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
            let local_entity: Option<GetRuleBasedViewError> =
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
