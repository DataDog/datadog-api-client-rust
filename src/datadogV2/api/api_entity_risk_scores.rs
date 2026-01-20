// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use crate::datadog;
use log::warn;
use reqwest::header::{HeaderMap, HeaderValue};
use serde::{Deserialize, Serialize};

/// ListEntityRiskScoresOptionalParams is a struct for passing parameters to the method [`EntityRiskScoresAPI::list_entity_risk_scores`]
#[non_exhaustive]
#[derive(Clone, Default, Debug)]
pub struct ListEntityRiskScoresOptionalParams {
    /// Start time for the query in Unix timestamp (milliseconds). Defaults to 2 weeks ago.
    pub from: Option<i64>,
    /// End time for the query in Unix timestamp (milliseconds). Defaults to now.
    pub to: Option<i64>,
    /// Size of the page to return. Maximum is 1000.
    pub page_size: Option<i32>,
    /// Page number to return (1-indexed).
    pub page_number: Option<i32>,
    /// Query ID for pagination consistency.
    pub page_query_id: Option<String>,
    /// Sort order for results. Format: `field:direction` where direction is `asc` or `desc`.
    /// Supported fields: `riskScore`, `lastDetected`, `firstDetected`, `entityName`, `signalsDetected`.
    pub filter_sort: Option<String>,
    /// Supports filtering by entity attributes, risk scores, severity, and more.
    /// Example: `severity:critical AND entityType:aws_iam_user`
    pub filter_query: Option<String>,
    /// Filter by entity type(s). Can specify multiple values.
    pub entity_type: Option<Vec<String>>,
}

impl ListEntityRiskScoresOptionalParams {
    /// Start time for the query in Unix timestamp (milliseconds). Defaults to 2 weeks ago.
    pub fn from(mut self, value: i64) -> Self {
        self.from = Some(value);
        self
    }
    /// End time for the query in Unix timestamp (milliseconds). Defaults to now.
    pub fn to(mut self, value: i64) -> Self {
        self.to = Some(value);
        self
    }
    /// Size of the page to return. Maximum is 1000.
    pub fn page_size(mut self, value: i32) -> Self {
        self.page_size = Some(value);
        self
    }
    /// Page number to return (1-indexed).
    pub fn page_number(mut self, value: i32) -> Self {
        self.page_number = Some(value);
        self
    }
    /// Query ID for pagination consistency.
    pub fn page_query_id(mut self, value: String) -> Self {
        self.page_query_id = Some(value);
        self
    }
    /// Sort order for results. Format: `field:direction` where direction is `asc` or `desc`.
    /// Supported fields: `riskScore`, `lastDetected`, `firstDetected`, `entityName`, `signalsDetected`.
    pub fn filter_sort(mut self, value: String) -> Self {
        self.filter_sort = Some(value);
        self
    }
    /// Supports filtering by entity attributes, risk scores, severity, and more.
    /// Example: `severity:critical AND entityType:aws_iam_user`
    pub fn filter_query(mut self, value: String) -> Self {
        self.filter_query = Some(value);
        self
    }
    /// Filter by entity type(s). Can specify multiple values.
    pub fn entity_type(mut self, value: Vec<String>) -> Self {
        self.entity_type = Some(value);
        self
    }
}

/// ListEntityRiskScoresError is a struct for typed errors of method [`EntityRiskScoresAPI::list_entity_risk_scores`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ListEntityRiskScoresError {
    JSONAPIErrorResponse(crate::datadogV2::model::JSONAPIErrorResponse),
    APIErrorResponse(crate::datadogV2::model::APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// Retrieves security risk scores for entities in your organization.
#[derive(Debug, Clone)]
pub struct EntityRiskScoresAPI {
    config: datadog::Configuration,
    client: reqwest_middleware::ClientWithMiddleware,
}

impl Default for EntityRiskScoresAPI {
    fn default() -> Self {
        Self::with_config(datadog::Configuration::default())
    }
}

impl EntityRiskScoresAPI {
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

    /// Get a list of entity risk scores for your organization. Entity risk scores provide security risk assessment for entities like cloud resources, identities, or services based on detected signals, misconfigurations, and identity risks.
    pub async fn list_entity_risk_scores(
        &self,
        params: ListEntityRiskScoresOptionalParams,
    ) -> Result<
        crate::datadogV2::model::SecurityEntityRiskScoresResponse,
        datadog::Error<ListEntityRiskScoresError>,
    > {
        match self.list_entity_risk_scores_with_http_info(params).await {
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

    /// Get a list of entity risk scores for your organization. Entity risk scores provide security risk assessment for entities like cloud resources, identities, or services based on detected signals, misconfigurations, and identity risks.
    pub async fn list_entity_risk_scores_with_http_info(
        &self,
        params: ListEntityRiskScoresOptionalParams,
    ) -> Result<
        datadog::ResponseContent<crate::datadogV2::model::SecurityEntityRiskScoresResponse>,
        datadog::Error<ListEntityRiskScoresError>,
    > {
        let local_configuration = &self.config;
        let operation_id = "v2.list_entity_risk_scores";
        if local_configuration.is_unstable_operation_enabled(operation_id) {
            warn!("Using unstable operation {operation_id}");
        } else {
            let local_error = datadog::UnstableOperationDisabledError {
                msg: "Operation 'v2.list_entity_risk_scores' is not enabled".to_string(),
            };
            return Err(datadog::Error::UnstableOperationDisabledError(local_error));
        }

        // unbox and build optional parameters
        let from = params.from;
        let to = params.to;
        let page_size = params.page_size;
        let page_number = params.page_number;
        let page_query_id = params.page_query_id;
        let filter_sort = params.filter_sort;
        let filter_query = params.filter_query;
        let entity_type = params.entity_type;

        let local_client = &self.client;

        let local_uri_str = format!(
            "{}/api/v2/security-entities/risk-scores",
            local_configuration.get_operation_host(operation_id)
        );
        let mut local_req_builder =
            local_client.request(reqwest::Method::GET, local_uri_str.as_str());

        if let Some(ref local_query_param) = from {
            local_req_builder =
                local_req_builder.query(&[("from", &local_query_param.to_string())]);
        };
        if let Some(ref local_query_param) = to {
            local_req_builder = local_req_builder.query(&[("to", &local_query_param.to_string())]);
        };
        if let Some(ref local_query_param) = page_size {
            local_req_builder =
                local_req_builder.query(&[("page[size]", &local_query_param.to_string())]);
        };
        if let Some(ref local_query_param) = page_number {
            local_req_builder =
                local_req_builder.query(&[("page[number]", &local_query_param.to_string())]);
        };
        if let Some(ref local_query_param) = page_query_id {
            local_req_builder =
                local_req_builder.query(&[("page[queryId]", &local_query_param.to_string())]);
        };
        if let Some(ref local_query_param) = filter_sort {
            local_req_builder =
                local_req_builder.query(&[("filter[sort]", &local_query_param.to_string())]);
        };
        if let Some(ref local_query_param) = filter_query {
            local_req_builder =
                local_req_builder.query(&[("filter[query]", &local_query_param.to_string())]);
        };
        if let Some(ref local) = entity_type {
            for param in local {
                local_req_builder = local_req_builder.query(&[("entityType", &param.to_string())]);
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
            match serde_json::from_str::<crate::datadogV2::model::SecurityEntityRiskScoresResponse>(
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
            let local_entity: Option<ListEntityRiskScoresError> =
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
