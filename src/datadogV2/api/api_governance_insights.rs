// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use crate::datadog;
use log::warn;
use reqwest::header::{HeaderMap, HeaderValue};
use serde::{Deserialize, Serialize};

/// ListGovernanceInsightsOptionalParams is a struct for passing parameters to the method [`GovernanceInsightsAPI::list_governance_insights`]
#[non_exhaustive]
#[derive(Clone, Default, Debug)]
pub struct ListGovernanceInsightsOptionalParams {
    /// Whether to compute and include the current and previous value of each insight.
    /// Defaults to `false`, in which case only insight metadata is returned.
    pub with_values: Option<bool>,
    /// The UUID of the organization to compute insights for. Defaults to the organization of
    /// the authenticated user. Used to retrieve insights for a child organization from a
    /// parent organization.
    pub org_uuid: Option<String>,
    /// Restrict the results to insights belonging to the given products. May be repeated to
    /// filter by multiple products. Matching is case-insensitive.
    pub filter_product: Option<Vec<String>>,
}

impl ListGovernanceInsightsOptionalParams {
    /// Whether to compute and include the current and previous value of each insight.
    /// Defaults to `false`, in which case only insight metadata is returned.
    pub fn with_values(mut self, value: bool) -> Self {
        self.with_values = Some(value);
        self
    }
    /// The UUID of the organization to compute insights for. Defaults to the organization of
    /// the authenticated user. Used to retrieve insights for a child organization from a
    /// parent organization.
    pub fn org_uuid(mut self, value: String) -> Self {
        self.org_uuid = Some(value);
        self
    }
    /// Restrict the results to insights belonging to the given products. May be repeated to
    /// filter by multiple products. Matching is case-insensitive.
    pub fn filter_product(mut self, value: Vec<String>) -> Self {
        self.filter_product = Some(value);
        self
    }
}

/// ListGovernanceInsightsError is a struct for typed errors of method [`GovernanceInsightsAPI::list_governance_insights`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ListGovernanceInsightsError {
    JSONAPIErrorResponse(crate::datadogV2::model::JSONAPIErrorResponse),
    APIErrorResponse(crate::datadogV2::model::APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// Governance Insights surface key usage, configuration, and best-practice signals for an
/// organization within the Governance Console. Each insight reports a current value (and,
/// optionally, a previous value for comparison) along with the query used to compute it, so
/// that the Console can render trends and highlight areas that need attention.
#[derive(Debug, Clone)]
pub struct GovernanceInsightsAPI {
    config: datadog::Configuration,
    client: reqwest_middleware::ClientWithMiddleware,
}

impl Default for GovernanceInsightsAPI {
    fn default() -> Self {
        Self::with_config(datadog::Configuration::default())
    }
}

impl GovernanceInsightsAPI {
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

    /// Retrieve the list of governance insights available to the organization. By default, only
    /// insight metadata is returned; pass `withValues=true` to also compute and include each
    /// insight's current and previous values. Insights can be filtered by product.
    pub async fn list_governance_insights(
        &self,
        params: ListGovernanceInsightsOptionalParams,
    ) -> Result<
        crate::datadogV2::model::GovernanceInsightsResponse,
        datadog::Error<ListGovernanceInsightsError>,
    > {
        match self.list_governance_insights_with_http_info(params).await {
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

    /// Retrieve the list of governance insights available to the organization. By default, only
    /// insight metadata is returned; pass `withValues=true` to also compute and include each
    /// insight's current and previous values. Insights can be filtered by product.
    pub async fn list_governance_insights_with_http_info(
        &self,
        params: ListGovernanceInsightsOptionalParams,
    ) -> Result<
        datadog::ResponseContent<crate::datadogV2::model::GovernanceInsightsResponse>,
        datadog::Error<ListGovernanceInsightsError>,
    > {
        let local_configuration = &self.config;
        let operation_id = "v2.list_governance_insights";
        if local_configuration.is_unstable_operation_enabled(operation_id) {
            warn!("Using unstable operation {operation_id}");
        } else {
            let local_error = datadog::UnstableOperationDisabledError {
                msg: "Operation 'v2.list_governance_insights' is not enabled".to_string(),
            };
            return Err(datadog::Error::UnstableOperationDisabledError(local_error));
        }

        // unbox and build optional parameters
        let with_values = params.with_values;
        let org_uuid = params.org_uuid;
        let filter_product = params.filter_product;

        let local_client = &self.client;

        let local_uri_str = format!(
            "{}/api/v2/governance/insights",
            local_configuration.get_operation_host(operation_id)
        );
        let mut local_req_builder =
            local_client.request(reqwest::Method::GET, local_uri_str.as_str());

        if let Some(ref local_query_param) = with_values {
            local_req_builder =
                local_req_builder.query(&[("withValues", &local_query_param.to_string())]);
        };
        if let Some(ref local_query_param) = org_uuid {
            local_req_builder =
                local_req_builder.query(&[("orgUuid", &local_query_param.to_string())]);
        };
        if let Some(ref local) = filter_product {
            for param in local {
                local_req_builder =
                    local_req_builder.query(&[("filter[product]", &param.to_string())]);
            }
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
            match serde_json::from_str::<crate::datadogV2::model::GovernanceInsightsResponse>(
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
            let local_entity: Option<ListGovernanceInsightsError> =
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
