// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use crate::datadog;
use log::warn;
use reqwest::header::{HeaderMap, HeaderValue};
use serde::{Deserialize, Serialize};

/// ListModelLabProjectsOptionalParams is a struct for passing parameters to the method [`ModelLabAPIAPI::list_model_lab_projects`]
#[non_exhaustive]
#[derive(Clone, Default, Debug)]
pub struct ListModelLabProjectsOptionalParams {
    /// Text search filter for project name or description.
    pub filter: Option<String>,
    /// Filter by owner UUID.
    pub filter_owner_id: Option<uuid::Uuid>,
    /// Filter by tags. Format: key:value,key2:value2.
    pub filter_tags: Option<String>,
    /// Sort field. Valid values: name, created_at, updated_at. Prefix with '-' for descending order (e.g., -updated_at).
    pub sort: Option<String>,
    /// Number of items per page. Maximum is 100.
    pub page_size: Option<i32>,
    /// Page number (1-indexed).
    pub page_number: Option<i32>,
}

impl ListModelLabProjectsOptionalParams {
    /// Text search filter for project name or description.
    pub fn filter(mut self, value: String) -> Self {
        self.filter = Some(value);
        self
    }
    /// Filter by owner UUID.
    pub fn filter_owner_id(mut self, value: uuid::Uuid) -> Self {
        self.filter_owner_id = Some(value);
        self
    }
    /// Filter by tags. Format: key:value,key2:value2.
    pub fn filter_tags(mut self, value: String) -> Self {
        self.filter_tags = Some(value);
        self
    }
    /// Sort field. Valid values: name, created_at, updated_at. Prefix with '-' for descending order (e.g., -updated_at).
    pub fn sort(mut self, value: String) -> Self {
        self.sort = Some(value);
        self
    }
    /// Number of items per page. Maximum is 100.
    pub fn page_size(mut self, value: i32) -> Self {
        self.page_size = Some(value);
        self
    }
    /// Page number (1-indexed).
    pub fn page_number(mut self, value: i32) -> Self {
        self.page_number = Some(value);
        self
    }
}

/// ListModelLabRunArtifactsOptionalParams is a struct for passing parameters to the method [`ModelLabAPIAPI::list_model_lab_run_artifacts`]
#[non_exhaustive]
#[derive(Clone, Default, Debug)]
pub struct ListModelLabRunArtifactsOptionalParams {
    /// Optional subdirectory path within the run's artifacts.
    pub path: Option<String>,
}

impl ListModelLabRunArtifactsOptionalParams {
    /// Optional subdirectory path within the run's artifacts.
    pub fn path(mut self, value: String) -> Self {
        self.path = Some(value);
        self
    }
}

/// ListModelLabRunsOptionalParams is a struct for passing parameters to the method [`ModelLabAPIAPI::list_model_lab_runs`]
#[non_exhaustive]
#[derive(Clone, Default, Debug)]
pub struct ListModelLabRunsOptionalParams {
    /// Filter by run ID(s). Comma-separated list for multiple IDs.
    pub filter_id: Option<String>,
    /// Text search filter for run name or description.
    pub filter: Option<String>,
    /// Filter by owner UUID.
    pub filter_owner_id: Option<String>,
    /// Filter by run status. Valid values: pending, running, completed, failed, killed, unresponsive, paused.
    pub filter_status: Option<crate::datadogV2::model::ModelLabRunStatus>,
    /// Filter by project ID.
    pub filter_project_id: Option<i64>,
    /// Filter by tags. Format: key:value,key2:value2.
    pub filter_tags: Option<String>,
    /// Filter by params. Format: key:value,key2:>0.5,key3:true.
    pub filter_params: Option<String>,
    /// Filter by parent run ID. Use 'null' to return only root runs (runs with no parent).
    pub filter_parent_run_id: Option<String>,
    /// Sort pinned runs before non-pinned runs. Pinned runs are ordered by pin time descending.
    pub pinned_first: Option<bool>,
    /// Include all runs pinned by the current user, regardless of other filters.
    pub include_pinned: Option<bool>,
    /// When true, also return runs whose descendants match the active filters. The descendant_match field in each result indicates whether the run was included via a descendant match.
    pub include_descendant_matches: Option<bool>,
    /// Sort field. Valid values: name, created_at, updated_at, duration. Prefix with '-' for descending order (e.g., -updated_at).
    pub sort: Option<String>,
    /// Number of items per page. Maximum is 100.
    pub page_size: Option<i32>,
    /// Page number (1-indexed).
    pub page_number: Option<i32>,
}

impl ListModelLabRunsOptionalParams {
    /// Filter by run ID(s). Comma-separated list for multiple IDs.
    pub fn filter_id(mut self, value: String) -> Self {
        self.filter_id = Some(value);
        self
    }
    /// Text search filter for run name or description.
    pub fn filter(mut self, value: String) -> Self {
        self.filter = Some(value);
        self
    }
    /// Filter by owner UUID.
    pub fn filter_owner_id(mut self, value: String) -> Self {
        self.filter_owner_id = Some(value);
        self
    }
    /// Filter by run status. Valid values: pending, running, completed, failed, killed, unresponsive, paused.
    pub fn filter_status(mut self, value: crate::datadogV2::model::ModelLabRunStatus) -> Self {
        self.filter_status = Some(value);
        self
    }
    /// Filter by project ID.
    pub fn filter_project_id(mut self, value: i64) -> Self {
        self.filter_project_id = Some(value);
        self
    }
    /// Filter by tags. Format: key:value,key2:value2.
    pub fn filter_tags(mut self, value: String) -> Self {
        self.filter_tags = Some(value);
        self
    }
    /// Filter by params. Format: key:value,key2:>0.5,key3:true.
    pub fn filter_params(mut self, value: String) -> Self {
        self.filter_params = Some(value);
        self
    }
    /// Filter by parent run ID. Use 'null' to return only root runs (runs with no parent).
    pub fn filter_parent_run_id(mut self, value: String) -> Self {
        self.filter_parent_run_id = Some(value);
        self
    }
    /// Sort pinned runs before non-pinned runs. Pinned runs are ordered by pin time descending.
    pub fn pinned_first(mut self, value: bool) -> Self {
        self.pinned_first = Some(value);
        self
    }
    /// Include all runs pinned by the current user, regardless of other filters.
    pub fn include_pinned(mut self, value: bool) -> Self {
        self.include_pinned = Some(value);
        self
    }
    /// When true, also return runs whose descendants match the active filters. The descendant_match field in each result indicates whether the run was included via a descendant match.
    pub fn include_descendant_matches(mut self, value: bool) -> Self {
        self.include_descendant_matches = Some(value);
        self
    }
    /// Sort field. Valid values: name, created_at, updated_at, duration. Prefix with '-' for descending order (e.g., -updated_at).
    pub fn sort(mut self, value: String) -> Self {
        self.sort = Some(value);
        self
    }
    /// Number of items per page. Maximum is 100.
    pub fn page_size(mut self, value: i32) -> Self {
        self.page_size = Some(value);
        self
    }
    /// Page number (1-indexed).
    pub fn page_number(mut self, value: i32) -> Self {
        self.page_number = Some(value);
        self
    }
}

/// DeleteModelLabRunError is a struct for typed errors of method [`ModelLabAPIAPI::delete_model_lab_run`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DeleteModelLabRunError {
    JSONAPIErrorResponse(crate::datadogV2::model::JSONAPIErrorResponse),
    APIErrorResponse(crate::datadogV2::model::APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// GetModelLabArtifactContentError is a struct for typed errors of method [`ModelLabAPIAPI::get_model_lab_artifact_content`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetModelLabArtifactContentError {
    JSONAPIErrorResponse(crate::datadogV2::model::JSONAPIErrorResponse),
    APIErrorResponse(crate::datadogV2::model::APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// GetModelLabProjectError is a struct for typed errors of method [`ModelLabAPIAPI::get_model_lab_project`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetModelLabProjectError {
    JSONAPIErrorResponse(crate::datadogV2::model::JSONAPIErrorResponse),
    APIErrorResponse(crate::datadogV2::model::APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// GetModelLabRunError is a struct for typed errors of method [`ModelLabAPIAPI::get_model_lab_run`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetModelLabRunError {
    JSONAPIErrorResponse(crate::datadogV2::model::JSONAPIErrorResponse),
    APIErrorResponse(crate::datadogV2::model::APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// ListModelLabProjectArtifactsError is a struct for typed errors of method [`ModelLabAPIAPI::list_model_lab_project_artifacts`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ListModelLabProjectArtifactsError {
    JSONAPIErrorResponse(crate::datadogV2::model::JSONAPIErrorResponse),
    APIErrorResponse(crate::datadogV2::model::APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// ListModelLabProjectFacetKeysError is a struct for typed errors of method [`ModelLabAPIAPI::list_model_lab_project_facet_keys`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ListModelLabProjectFacetKeysError {
    APIErrorResponse(crate::datadogV2::model::APIErrorResponse),
    JSONAPIErrorResponse(crate::datadogV2::model::JSONAPIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// ListModelLabProjectFacetValuesError is a struct for typed errors of method [`ModelLabAPIAPI::list_model_lab_project_facet_values`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ListModelLabProjectFacetValuesError {
    JSONAPIErrorResponse(crate::datadogV2::model::JSONAPIErrorResponse),
    APIErrorResponse(crate::datadogV2::model::APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// ListModelLabProjectsError is a struct for typed errors of method [`ModelLabAPIAPI::list_model_lab_projects`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ListModelLabProjectsError {
    JSONAPIErrorResponse(crate::datadogV2::model::JSONAPIErrorResponse),
    APIErrorResponse(crate::datadogV2::model::APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// ListModelLabRunArtifactsError is a struct for typed errors of method [`ModelLabAPIAPI::list_model_lab_run_artifacts`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ListModelLabRunArtifactsError {
    JSONAPIErrorResponse(crate::datadogV2::model::JSONAPIErrorResponse),
    APIErrorResponse(crate::datadogV2::model::APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// ListModelLabRunFacetKeysError is a struct for typed errors of method [`ModelLabAPIAPI::list_model_lab_run_facet_keys`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ListModelLabRunFacetKeysError {
    JSONAPIErrorResponse(crate::datadogV2::model::JSONAPIErrorResponse),
    APIErrorResponse(crate::datadogV2::model::APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// ListModelLabRunFacetValuesError is a struct for typed errors of method [`ModelLabAPIAPI::list_model_lab_run_facet_values`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ListModelLabRunFacetValuesError {
    JSONAPIErrorResponse(crate::datadogV2::model::JSONAPIErrorResponse),
    APIErrorResponse(crate::datadogV2::model::APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// ListModelLabRunsError is a struct for typed errors of method [`ModelLabAPIAPI::list_model_lab_runs`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ListModelLabRunsError {
    JSONAPIErrorResponse(crate::datadogV2::model::JSONAPIErrorResponse),
    APIErrorResponse(crate::datadogV2::model::APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// PinModelLabRunError is a struct for typed errors of method [`ModelLabAPIAPI::pin_model_lab_run`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum PinModelLabRunError {
    JSONAPIErrorResponse(crate::datadogV2::model::JSONAPIErrorResponse),
    APIErrorResponse(crate::datadogV2::model::APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// StarModelLabProjectError is a struct for typed errors of method [`ModelLabAPIAPI::star_model_lab_project`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum StarModelLabProjectError {
    JSONAPIErrorResponse(crate::datadogV2::model::JSONAPIErrorResponse),
    APIErrorResponse(crate::datadogV2::model::APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// UnpinModelLabRunError is a struct for typed errors of method [`ModelLabAPIAPI::unpin_model_lab_run`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum UnpinModelLabRunError {
    JSONAPIErrorResponse(crate::datadogV2::model::JSONAPIErrorResponse),
    APIErrorResponse(crate::datadogV2::model::APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// UnstarModelLabProjectError is a struct for typed errors of method [`ModelLabAPIAPI::unstar_model_lab_project`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum UnstarModelLabProjectError {
    JSONAPIErrorResponse(crate::datadogV2::model::JSONAPIErrorResponse),
    APIErrorResponse(crate::datadogV2::model::APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// Manage Model Lab projects, runs, artifacts, and facets for ML experiment tracking.
#[derive(Debug, Clone)]
pub struct ModelLabAPIAPI {
    config: datadog::Configuration,
    client: reqwest_middleware::ClientWithMiddleware,
}

impl Default for ModelLabAPIAPI {
    fn default() -> Self {
        Self::with_config(datadog::Configuration::default())
    }
}

impl ModelLabAPIAPI {
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

    /// Delete a Model Lab run by its ID.
    pub async fn delete_model_lab_run(
        &self,
        run_id: i64,
    ) -> Result<(), datadog::Error<DeleteModelLabRunError>> {
        match self.delete_model_lab_run_with_http_info(run_id).await {
            Ok(_) => Ok(()),
            Err(err) => Err(err),
        }
    }

    /// Delete a Model Lab run by its ID.
    pub async fn delete_model_lab_run_with_http_info(
        &self,
        run_id: i64,
    ) -> Result<datadog::ResponseContent<()>, datadog::Error<DeleteModelLabRunError>> {
        let local_configuration = &self.config;
        let operation_id = "v2.delete_model_lab_run";
        if local_configuration.is_unstable_operation_enabled(operation_id) {
            warn!("Using unstable operation {operation_id}");
        } else {
            let local_error = datadog::UnstableOperationDisabledError {
                msg: "Operation 'v2.delete_model_lab_run' is not enabled".to_string(),
            };
            return Err(datadog::Error::UnstableOperationDisabledError(local_error));
        }

        let local_client = &self.client;

        let local_uri_str = format!(
            "{}/api/v2/model-lab-api/runs/{run_id}",
            local_configuration.get_operation_host(operation_id),
            run_id = run_id
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
            let local_entity: Option<DeleteModelLabRunError> =
                serde_json::from_str(&local_content).ok();
            let local_error = datadog::ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(datadog::Error::ResponseError(local_error))
        }
    }

    /// Download the raw content of a Model Lab artifact file.
    pub async fn get_model_lab_artifact_content(
        &self,
        project_id: String,
        artifact_path: String,
    ) -> Result<Vec<u8>, datadog::Error<GetModelLabArtifactContentError>> {
        match self
            .get_model_lab_artifact_content_with_http_info(project_id, artifact_path)
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

    /// Download the raw content of a Model Lab artifact file.
    pub async fn get_model_lab_artifact_content_with_http_info(
        &self,
        project_id: String,
        artifact_path: String,
    ) -> Result<datadog::ResponseContent<Vec<u8>>, datadog::Error<GetModelLabArtifactContentError>>
    {
        let local_configuration = &self.config;
        let operation_id = "v2.get_model_lab_artifact_content";
        if local_configuration.is_unstable_operation_enabled(operation_id) {
            warn!("Using unstable operation {operation_id}");
        } else {
            let local_error = datadog::UnstableOperationDisabledError {
                msg: "Operation 'v2.get_model_lab_artifact_content' is not enabled".to_string(),
            };
            return Err(datadog::Error::UnstableOperationDisabledError(local_error));
        }

        let local_client = &self.client;

        let local_uri_str = format!(
            "{}/api/v2/model-lab-api/artifacts/content",
            local_configuration.get_operation_host(operation_id)
        );
        let mut local_req_builder =
            local_client.request(reqwest::Method::GET, local_uri_str.as_str());

        local_req_builder = local_req_builder.query(&[("project_id", &project_id.to_string())]);
        local_req_builder =
            local_req_builder.query(&[("artifact_path", &artifact_path.to_string())]);

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
            let local_entity: Option<GetModelLabArtifactContentError> =
                serde_json::from_str(&local_content).ok();
            let local_error = datadog::ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(datadog::Error::ResponseError(local_error))
        }
    }

    /// Get a single Model Lab project by its ID.
    pub async fn get_model_lab_project(
        &self,
        project_id: i64,
    ) -> Result<
        crate::datadogV2::model::ModelLabProjectResponse,
        datadog::Error<GetModelLabProjectError>,
    > {
        match self.get_model_lab_project_with_http_info(project_id).await {
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

    /// Get a single Model Lab project by its ID.
    pub async fn get_model_lab_project_with_http_info(
        &self,
        project_id: i64,
    ) -> Result<
        datadog::ResponseContent<crate::datadogV2::model::ModelLabProjectResponse>,
        datadog::Error<GetModelLabProjectError>,
    > {
        let local_configuration = &self.config;
        let operation_id = "v2.get_model_lab_project";
        if local_configuration.is_unstable_operation_enabled(operation_id) {
            warn!("Using unstable operation {operation_id}");
        } else {
            let local_error = datadog::UnstableOperationDisabledError {
                msg: "Operation 'v2.get_model_lab_project' is not enabled".to_string(),
            };
            return Err(datadog::Error::UnstableOperationDisabledError(local_error));
        }

        let local_client = &self.client;

        let local_uri_str = format!(
            "{}/api/v2/model-lab-api/projects/{project_id}",
            local_configuration.get_operation_host(operation_id),
            project_id = project_id
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
            match serde_json::from_str::<crate::datadogV2::model::ModelLabProjectResponse>(
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
            let local_entity: Option<GetModelLabProjectError> =
                serde_json::from_str(&local_content).ok();
            let local_error = datadog::ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(datadog::Error::ResponseError(local_error))
        }
    }

    /// Get a single Model Lab run by its ID.
    pub async fn get_model_lab_run(
        &self,
        run_id: i64,
    ) -> Result<crate::datadogV2::model::ModelLabRunResponse, datadog::Error<GetModelLabRunError>>
    {
        match self.get_model_lab_run_with_http_info(run_id).await {
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

    /// Get a single Model Lab run by its ID.
    pub async fn get_model_lab_run_with_http_info(
        &self,
        run_id: i64,
    ) -> Result<
        datadog::ResponseContent<crate::datadogV2::model::ModelLabRunResponse>,
        datadog::Error<GetModelLabRunError>,
    > {
        let local_configuration = &self.config;
        let operation_id = "v2.get_model_lab_run";
        if local_configuration.is_unstable_operation_enabled(operation_id) {
            warn!("Using unstable operation {operation_id}");
        } else {
            let local_error = datadog::UnstableOperationDisabledError {
                msg: "Operation 'v2.get_model_lab_run' is not enabled".to_string(),
            };
            return Err(datadog::Error::UnstableOperationDisabledError(local_error));
        }

        let local_client = &self.client;

        let local_uri_str = format!(
            "{}/api/v2/model-lab-api/runs/{run_id}",
            local_configuration.get_operation_host(operation_id),
            run_id = run_id
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
            match serde_json::from_str::<crate::datadogV2::model::ModelLabRunResponse>(
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
            let local_entity: Option<GetModelLabRunError> =
                serde_json::from_str(&local_content).ok();
            let local_error = datadog::ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(datadog::Error::ResponseError(local_error))
        }
    }

    /// List all artifact files for a specific Model Lab project.
    pub async fn list_model_lab_project_artifacts(
        &self,
        project_id: i64,
    ) -> Result<
        crate::datadogV2::model::ModelLabProjectArtifactsResponse,
        datadog::Error<ListModelLabProjectArtifactsError>,
    > {
        match self
            .list_model_lab_project_artifacts_with_http_info(project_id)
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

    /// List all artifact files for a specific Model Lab project.
    pub async fn list_model_lab_project_artifacts_with_http_info(
        &self,
        project_id: i64,
    ) -> Result<
        datadog::ResponseContent<crate::datadogV2::model::ModelLabProjectArtifactsResponse>,
        datadog::Error<ListModelLabProjectArtifactsError>,
    > {
        let local_configuration = &self.config;
        let operation_id = "v2.list_model_lab_project_artifacts";
        if local_configuration.is_unstable_operation_enabled(operation_id) {
            warn!("Using unstable operation {operation_id}");
        } else {
            let local_error = datadog::UnstableOperationDisabledError {
                msg: "Operation 'v2.list_model_lab_project_artifacts' is not enabled".to_string(),
            };
            return Err(datadog::Error::UnstableOperationDisabledError(local_error));
        }

        let local_client = &self.client;

        let local_uri_str = format!(
            "{}/api/v2/model-lab-api/projects/{project_id}/artifacts",
            local_configuration.get_operation_host(operation_id),
            project_id = project_id
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
            match serde_json::from_str::<crate::datadogV2::model::ModelLabProjectArtifactsResponse>(
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
            let local_entity: Option<ListModelLabProjectArtifactsError> =
                serde_json::from_str(&local_content).ok();
            let local_error = datadog::ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(datadog::Error::ResponseError(local_error))
        }
    }

    /// List all available facet keys for filtering Model Lab projects.
    pub async fn list_model_lab_project_facet_keys(
        &self,
    ) -> Result<
        crate::datadogV2::model::ModelLabFacetKeysResponse,
        datadog::Error<ListModelLabProjectFacetKeysError>,
    > {
        match self
            .list_model_lab_project_facet_keys_with_http_info()
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

    /// List all available facet keys for filtering Model Lab projects.
    pub async fn list_model_lab_project_facet_keys_with_http_info(
        &self,
    ) -> Result<
        datadog::ResponseContent<crate::datadogV2::model::ModelLabFacetKeysResponse>,
        datadog::Error<ListModelLabProjectFacetKeysError>,
    > {
        let local_configuration = &self.config;
        let operation_id = "v2.list_model_lab_project_facet_keys";
        if local_configuration.is_unstable_operation_enabled(operation_id) {
            warn!("Using unstable operation {operation_id}");
        } else {
            let local_error = datadog::UnstableOperationDisabledError {
                msg: "Operation 'v2.list_model_lab_project_facet_keys' is not enabled".to_string(),
            };
            return Err(datadog::Error::UnstableOperationDisabledError(local_error));
        }

        let local_client = &self.client;

        let local_uri_str = format!(
            "{}/api/v2/model-lab-api/project-facet-keys",
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
            match serde_json::from_str::<crate::datadogV2::model::ModelLabFacetKeysResponse>(
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
            let local_entity: Option<ListModelLabProjectFacetKeysError> =
                serde_json::from_str(&local_content).ok();
            let local_error = datadog::ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(datadog::Error::ResponseError(local_error))
        }
    }

    /// List available facet values for a specific project facet key.
    pub async fn list_model_lab_project_facet_values(
        &self,
        facet_type: crate::datadogV2::model::ModelLabProjectFacetType,
        facet_name: String,
    ) -> Result<
        crate::datadogV2::model::ModelLabFacetValuesResponse,
        datadog::Error<ListModelLabProjectFacetValuesError>,
    > {
        match self
            .list_model_lab_project_facet_values_with_http_info(facet_type, facet_name)
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

    /// List available facet values for a specific project facet key.
    pub async fn list_model_lab_project_facet_values_with_http_info(
        &self,
        facet_type: crate::datadogV2::model::ModelLabProjectFacetType,
        facet_name: String,
    ) -> Result<
        datadog::ResponseContent<crate::datadogV2::model::ModelLabFacetValuesResponse>,
        datadog::Error<ListModelLabProjectFacetValuesError>,
    > {
        let local_configuration = &self.config;
        let operation_id = "v2.list_model_lab_project_facet_values";
        if local_configuration.is_unstable_operation_enabled(operation_id) {
            warn!("Using unstable operation {operation_id}");
        } else {
            let local_error = datadog::UnstableOperationDisabledError {
                msg: "Operation 'v2.list_model_lab_project_facet_values' is not enabled"
                    .to_string(),
            };
            return Err(datadog::Error::UnstableOperationDisabledError(local_error));
        }

        let local_client = &self.client;

        let local_uri_str = format!(
            "{}/api/v2/model-lab-api/project-facet-values",
            local_configuration.get_operation_host(operation_id)
        );
        let mut local_req_builder =
            local_client.request(reqwest::Method::GET, local_uri_str.as_str());

        local_req_builder = local_req_builder.query(&[("facet_type", &facet_type.to_string())]);
        local_req_builder = local_req_builder.query(&[("facet_name", &facet_name.to_string())]);

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
            match serde_json::from_str::<crate::datadogV2::model::ModelLabFacetValuesResponse>(
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
            let local_entity: Option<ListModelLabProjectFacetValuesError> =
                serde_json::from_str(&local_content).ok();
            let local_error = datadog::ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(datadog::Error::ResponseError(local_error))
        }
    }

    /// List all Model Lab projects for the current organization.
    pub async fn list_model_lab_projects(
        &self,
        params: ListModelLabProjectsOptionalParams,
    ) -> Result<
        crate::datadogV2::model::ModelLabProjectsResponse,
        datadog::Error<ListModelLabProjectsError>,
    > {
        match self.list_model_lab_projects_with_http_info(params).await {
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

    /// List all Model Lab projects for the current organization.
    pub async fn list_model_lab_projects_with_http_info(
        &self,
        params: ListModelLabProjectsOptionalParams,
    ) -> Result<
        datadog::ResponseContent<crate::datadogV2::model::ModelLabProjectsResponse>,
        datadog::Error<ListModelLabProjectsError>,
    > {
        let local_configuration = &self.config;
        let operation_id = "v2.list_model_lab_projects";
        if local_configuration.is_unstable_operation_enabled(operation_id) {
            warn!("Using unstable operation {operation_id}");
        } else {
            let local_error = datadog::UnstableOperationDisabledError {
                msg: "Operation 'v2.list_model_lab_projects' is not enabled".to_string(),
            };
            return Err(datadog::Error::UnstableOperationDisabledError(local_error));
        }

        // unbox and build optional parameters
        let filter = params.filter;
        let filter_owner_id = params.filter_owner_id;
        let filter_tags = params.filter_tags;
        let sort = params.sort;
        let page_size = params.page_size;
        let page_number = params.page_number;

        let local_client = &self.client;

        let local_uri_str = format!(
            "{}/api/v2/model-lab-api/projects",
            local_configuration.get_operation_host(operation_id)
        );
        let mut local_req_builder =
            local_client.request(reqwest::Method::GET, local_uri_str.as_str());

        if let Some(ref local_query_param) = filter {
            local_req_builder =
                local_req_builder.query(&[("filter", &local_query_param.to_string())]);
        };
        if let Some(ref local_query_param) = filter_owner_id {
            local_req_builder =
                local_req_builder.query(&[("filter[owner_id]", &local_query_param.to_string())]);
        };
        if let Some(ref local_query_param) = filter_tags {
            local_req_builder =
                local_req_builder.query(&[("filter[tags]", &local_query_param.to_string())]);
        };
        if let Some(ref local_query_param) = sort {
            local_req_builder =
                local_req_builder.query(&[("sort", &local_query_param.to_string())]);
        };
        if let Some(ref local_query_param) = page_size {
            local_req_builder =
                local_req_builder.query(&[("page[size]", &local_query_param.to_string())]);
        };
        if let Some(ref local_query_param) = page_number {
            local_req_builder =
                local_req_builder.query(&[("page[number]", &local_query_param.to_string())]);
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
            match serde_json::from_str::<crate::datadogV2::model::ModelLabProjectsResponse>(
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
            let local_entity: Option<ListModelLabProjectsError> =
                serde_json::from_str(&local_content).ok();
            let local_error = datadog::ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(datadog::Error::ResponseError(local_error))
        }
    }

    /// List artifact files for a specific Model Lab run.
    pub async fn list_model_lab_run_artifacts(
        &self,
        run_id: i64,
        params: ListModelLabRunArtifactsOptionalParams,
    ) -> Result<
        crate::datadogV2::model::ModelLabRunArtifactsResponse,
        datadog::Error<ListModelLabRunArtifactsError>,
    > {
        match self
            .list_model_lab_run_artifacts_with_http_info(run_id, params)
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

    /// List artifact files for a specific Model Lab run.
    pub async fn list_model_lab_run_artifacts_with_http_info(
        &self,
        run_id: i64,
        params: ListModelLabRunArtifactsOptionalParams,
    ) -> Result<
        datadog::ResponseContent<crate::datadogV2::model::ModelLabRunArtifactsResponse>,
        datadog::Error<ListModelLabRunArtifactsError>,
    > {
        let local_configuration = &self.config;
        let operation_id = "v2.list_model_lab_run_artifacts";
        if local_configuration.is_unstable_operation_enabled(operation_id) {
            warn!("Using unstable operation {operation_id}");
        } else {
            let local_error = datadog::UnstableOperationDisabledError {
                msg: "Operation 'v2.list_model_lab_run_artifacts' is not enabled".to_string(),
            };
            return Err(datadog::Error::UnstableOperationDisabledError(local_error));
        }

        // unbox and build optional parameters
        let path = params.path;

        let local_client = &self.client;

        let local_uri_str = format!(
            "{}/api/v2/model-lab-api/runs/{run_id}/artifacts",
            local_configuration.get_operation_host(operation_id),
            run_id = run_id
        );
        let mut local_req_builder =
            local_client.request(reqwest::Method::GET, local_uri_str.as_str());

        if let Some(ref local_query_param) = path {
            local_req_builder =
                local_req_builder.query(&[("path", &local_query_param.to_string())]);
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
            match serde_json::from_str::<crate::datadogV2::model::ModelLabRunArtifactsResponse>(
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
            let local_entity: Option<ListModelLabRunArtifactsError> =
                serde_json::from_str(&local_content).ok();
            let local_error = datadog::ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(datadog::Error::ResponseError(local_error))
        }
    }

    /// List all available facet keys for filtering Model Lab runs.
    pub async fn list_model_lab_run_facet_keys(
        &self,
        filter_project_id: i64,
    ) -> Result<
        crate::datadogV2::model::ModelLabFacetKeysResponse,
        datadog::Error<ListModelLabRunFacetKeysError>,
    > {
        match self
            .list_model_lab_run_facet_keys_with_http_info(filter_project_id)
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

    /// List all available facet keys for filtering Model Lab runs.
    pub async fn list_model_lab_run_facet_keys_with_http_info(
        &self,
        filter_project_id: i64,
    ) -> Result<
        datadog::ResponseContent<crate::datadogV2::model::ModelLabFacetKeysResponse>,
        datadog::Error<ListModelLabRunFacetKeysError>,
    > {
        let local_configuration = &self.config;
        let operation_id = "v2.list_model_lab_run_facet_keys";
        if local_configuration.is_unstable_operation_enabled(operation_id) {
            warn!("Using unstable operation {operation_id}");
        } else {
            let local_error = datadog::UnstableOperationDisabledError {
                msg: "Operation 'v2.list_model_lab_run_facet_keys' is not enabled".to_string(),
            };
            return Err(datadog::Error::UnstableOperationDisabledError(local_error));
        }

        let local_client = &self.client;

        let local_uri_str = format!(
            "{}/api/v2/model-lab-api/facet-keys",
            local_configuration.get_operation_host(operation_id)
        );
        let mut local_req_builder =
            local_client.request(reqwest::Method::GET, local_uri_str.as_str());

        local_req_builder =
            local_req_builder.query(&[("filter[project_id]", &filter_project_id.to_string())]);

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
            match serde_json::from_str::<crate::datadogV2::model::ModelLabFacetKeysResponse>(
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
            let local_entity: Option<ListModelLabRunFacetKeysError> =
                serde_json::from_str(&local_content).ok();
            let local_error = datadog::ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(datadog::Error::ResponseError(local_error))
        }
    }

    /// List available facet values for a specific run facet key.
    pub async fn list_model_lab_run_facet_values(
        &self,
        filter_project_id: i64,
        facet_type: crate::datadogV2::model::ModelLabFacetType,
        facet_name: String,
    ) -> Result<
        crate::datadogV2::model::ModelLabFacetValuesResponse,
        datadog::Error<ListModelLabRunFacetValuesError>,
    > {
        match self
            .list_model_lab_run_facet_values_with_http_info(
                filter_project_id,
                facet_type,
                facet_name,
            )
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

    /// List available facet values for a specific run facet key.
    pub async fn list_model_lab_run_facet_values_with_http_info(
        &self,
        filter_project_id: i64,
        facet_type: crate::datadogV2::model::ModelLabFacetType,
        facet_name: String,
    ) -> Result<
        datadog::ResponseContent<crate::datadogV2::model::ModelLabFacetValuesResponse>,
        datadog::Error<ListModelLabRunFacetValuesError>,
    > {
        let local_configuration = &self.config;
        let operation_id = "v2.list_model_lab_run_facet_values";
        if local_configuration.is_unstable_operation_enabled(operation_id) {
            warn!("Using unstable operation {operation_id}");
        } else {
            let local_error = datadog::UnstableOperationDisabledError {
                msg: "Operation 'v2.list_model_lab_run_facet_values' is not enabled".to_string(),
            };
            return Err(datadog::Error::UnstableOperationDisabledError(local_error));
        }

        let local_client = &self.client;

        let local_uri_str = format!(
            "{}/api/v2/model-lab-api/facet-values",
            local_configuration.get_operation_host(operation_id)
        );
        let mut local_req_builder =
            local_client.request(reqwest::Method::GET, local_uri_str.as_str());

        local_req_builder =
            local_req_builder.query(&[("filter[project_id]", &filter_project_id.to_string())]);
        local_req_builder = local_req_builder.query(&[("facet_type", &facet_type.to_string())]);
        local_req_builder = local_req_builder.query(&[("facet_name", &facet_name.to_string())]);

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
            match serde_json::from_str::<crate::datadogV2::model::ModelLabFacetValuesResponse>(
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
            let local_entity: Option<ListModelLabRunFacetValuesError> =
                serde_json::from_str(&local_content).ok();
            let local_error = datadog::ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(datadog::Error::ResponseError(local_error))
        }
    }

    /// List all Model Lab runs for the current organization.
    pub async fn list_model_lab_runs(
        &self,
        params: ListModelLabRunsOptionalParams,
    ) -> Result<crate::datadogV2::model::ModelLabRunsResponse, datadog::Error<ListModelLabRunsError>>
    {
        match self.list_model_lab_runs_with_http_info(params).await {
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

    /// List all Model Lab runs for the current organization.
    pub async fn list_model_lab_runs_with_http_info(
        &self,
        params: ListModelLabRunsOptionalParams,
    ) -> Result<
        datadog::ResponseContent<crate::datadogV2::model::ModelLabRunsResponse>,
        datadog::Error<ListModelLabRunsError>,
    > {
        let local_configuration = &self.config;
        let operation_id = "v2.list_model_lab_runs";
        if local_configuration.is_unstable_operation_enabled(operation_id) {
            warn!("Using unstable operation {operation_id}");
        } else {
            let local_error = datadog::UnstableOperationDisabledError {
                msg: "Operation 'v2.list_model_lab_runs' is not enabled".to_string(),
            };
            return Err(datadog::Error::UnstableOperationDisabledError(local_error));
        }

        // unbox and build optional parameters
        let filter_id = params.filter_id;
        let filter = params.filter;
        let filter_owner_id = params.filter_owner_id;
        let filter_status = params.filter_status;
        let filter_project_id = params.filter_project_id;
        let filter_tags = params.filter_tags;
        let filter_params = params.filter_params;
        let filter_parent_run_id = params.filter_parent_run_id;
        let pinned_first = params.pinned_first;
        let include_pinned = params.include_pinned;
        let include_descendant_matches = params.include_descendant_matches;
        let sort = params.sort;
        let page_size = params.page_size;
        let page_number = params.page_number;

        let local_client = &self.client;

        let local_uri_str = format!(
            "{}/api/v2/model-lab-api/runs",
            local_configuration.get_operation_host(operation_id)
        );
        let mut local_req_builder =
            local_client.request(reqwest::Method::GET, local_uri_str.as_str());

        if let Some(ref local_query_param) = filter_id {
            local_req_builder =
                local_req_builder.query(&[("filter[id]", &local_query_param.to_string())]);
        };
        if let Some(ref local_query_param) = filter {
            local_req_builder =
                local_req_builder.query(&[("filter", &local_query_param.to_string())]);
        };
        if let Some(ref local_query_param) = filter_owner_id {
            local_req_builder =
                local_req_builder.query(&[("filter[owner_id]", &local_query_param.to_string())]);
        };
        if let Some(ref local_query_param) = filter_status {
            local_req_builder =
                local_req_builder.query(&[("filter[status]", &local_query_param.to_string())]);
        };
        if let Some(ref local_query_param) = filter_project_id {
            local_req_builder =
                local_req_builder.query(&[("filter[project_id]", &local_query_param.to_string())]);
        };
        if let Some(ref local_query_param) = filter_tags {
            local_req_builder =
                local_req_builder.query(&[("filter[tags]", &local_query_param.to_string())]);
        };
        if let Some(ref local_query_param) = filter_params {
            local_req_builder =
                local_req_builder.query(&[("filter[params]", &local_query_param.to_string())]);
        };
        if let Some(ref local_query_param) = filter_parent_run_id {
            local_req_builder = local_req_builder
                .query(&[("filter[parent_run_id]", &local_query_param.to_string())]);
        };
        if let Some(ref local_query_param) = pinned_first {
            local_req_builder =
                local_req_builder.query(&[("pinned_first", &local_query_param.to_string())]);
        };
        if let Some(ref local_query_param) = include_pinned {
            local_req_builder =
                local_req_builder.query(&[("include_pinned", &local_query_param.to_string())]);
        };
        if let Some(ref local_query_param) = include_descendant_matches {
            local_req_builder = local_req_builder
                .query(&[("include_descendant_matches", &local_query_param.to_string())]);
        };
        if let Some(ref local_query_param) = sort {
            local_req_builder =
                local_req_builder.query(&[("sort", &local_query_param.to_string())]);
        };
        if let Some(ref local_query_param) = page_size {
            local_req_builder =
                local_req_builder.query(&[("page[size]", &local_query_param.to_string())]);
        };
        if let Some(ref local_query_param) = page_number {
            local_req_builder =
                local_req_builder.query(&[("page[number]", &local_query_param.to_string())]);
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
            match serde_json::from_str::<crate::datadogV2::model::ModelLabRunsResponse>(
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
            let local_entity: Option<ListModelLabRunsError> =
                serde_json::from_str(&local_content).ok();
            let local_error = datadog::ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(datadog::Error::ResponseError(local_error))
        }
    }

    /// Pin a Model Lab run for the current user.
    pub async fn pin_model_lab_run(
        &self,
        run_id: i64,
    ) -> Result<(), datadog::Error<PinModelLabRunError>> {
        match self.pin_model_lab_run_with_http_info(run_id).await {
            Ok(_) => Ok(()),
            Err(err) => Err(err),
        }
    }

    /// Pin a Model Lab run for the current user.
    pub async fn pin_model_lab_run_with_http_info(
        &self,
        run_id: i64,
    ) -> Result<datadog::ResponseContent<()>, datadog::Error<PinModelLabRunError>> {
        let local_configuration = &self.config;
        let operation_id = "v2.pin_model_lab_run";
        if local_configuration.is_unstable_operation_enabled(operation_id) {
            warn!("Using unstable operation {operation_id}");
        } else {
            let local_error = datadog::UnstableOperationDisabledError {
                msg: "Operation 'v2.pin_model_lab_run' is not enabled".to_string(),
            };
            return Err(datadog::Error::UnstableOperationDisabledError(local_error));
        }

        let local_client = &self.client;

        let local_uri_str = format!(
            "{}/api/v2/model-lab-api/runs/{run_id}/pin",
            local_configuration.get_operation_host(operation_id),
            run_id = run_id
        );
        let mut local_req_builder =
            local_client.request(reqwest::Method::POST, local_uri_str.as_str());

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
            let local_entity: Option<PinModelLabRunError> =
                serde_json::from_str(&local_content).ok();
            let local_error = datadog::ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(datadog::Error::ResponseError(local_error))
        }
    }

    /// Star a Model Lab project for the current user.
    pub async fn star_model_lab_project(
        &self,
        project_id: i64,
    ) -> Result<(), datadog::Error<StarModelLabProjectError>> {
        match self.star_model_lab_project_with_http_info(project_id).await {
            Ok(_) => Ok(()),
            Err(err) => Err(err),
        }
    }

    /// Star a Model Lab project for the current user.
    pub async fn star_model_lab_project_with_http_info(
        &self,
        project_id: i64,
    ) -> Result<datadog::ResponseContent<()>, datadog::Error<StarModelLabProjectError>> {
        let local_configuration = &self.config;
        let operation_id = "v2.star_model_lab_project";
        if local_configuration.is_unstable_operation_enabled(operation_id) {
            warn!("Using unstable operation {operation_id}");
        } else {
            let local_error = datadog::UnstableOperationDisabledError {
                msg: "Operation 'v2.star_model_lab_project' is not enabled".to_string(),
            };
            return Err(datadog::Error::UnstableOperationDisabledError(local_error));
        }

        let local_client = &self.client;

        let local_uri_str = format!(
            "{}/api/v2/model-lab-api/projects/{project_id}/star",
            local_configuration.get_operation_host(operation_id),
            project_id = project_id
        );
        let mut local_req_builder =
            local_client.request(reqwest::Method::POST, local_uri_str.as_str());

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
            let local_entity: Option<StarModelLabProjectError> =
                serde_json::from_str(&local_content).ok();
            let local_error = datadog::ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(datadog::Error::ResponseError(local_error))
        }
    }

    /// Remove the pin from a Model Lab run for the current user.
    pub async fn unpin_model_lab_run(
        &self,
        run_id: i64,
    ) -> Result<(), datadog::Error<UnpinModelLabRunError>> {
        match self.unpin_model_lab_run_with_http_info(run_id).await {
            Ok(_) => Ok(()),
            Err(err) => Err(err),
        }
    }

    /// Remove the pin from a Model Lab run for the current user.
    pub async fn unpin_model_lab_run_with_http_info(
        &self,
        run_id: i64,
    ) -> Result<datadog::ResponseContent<()>, datadog::Error<UnpinModelLabRunError>> {
        let local_configuration = &self.config;
        let operation_id = "v2.unpin_model_lab_run";
        if local_configuration.is_unstable_operation_enabled(operation_id) {
            warn!("Using unstable operation {operation_id}");
        } else {
            let local_error = datadog::UnstableOperationDisabledError {
                msg: "Operation 'v2.unpin_model_lab_run' is not enabled".to_string(),
            };
            return Err(datadog::Error::UnstableOperationDisabledError(local_error));
        }

        let local_client = &self.client;

        let local_uri_str = format!(
            "{}/api/v2/model-lab-api/runs/{run_id}/pin",
            local_configuration.get_operation_host(operation_id),
            run_id = run_id
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
            let local_entity: Option<UnpinModelLabRunError> =
                serde_json::from_str(&local_content).ok();
            let local_error = datadog::ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(datadog::Error::ResponseError(local_error))
        }
    }

    /// Remove the star from a Model Lab project for the current user.
    pub async fn unstar_model_lab_project(
        &self,
        project_id: i64,
    ) -> Result<(), datadog::Error<UnstarModelLabProjectError>> {
        match self
            .unstar_model_lab_project_with_http_info(project_id)
            .await
        {
            Ok(_) => Ok(()),
            Err(err) => Err(err),
        }
    }

    /// Remove the star from a Model Lab project for the current user.
    pub async fn unstar_model_lab_project_with_http_info(
        &self,
        project_id: i64,
    ) -> Result<datadog::ResponseContent<()>, datadog::Error<UnstarModelLabProjectError>> {
        let local_configuration = &self.config;
        let operation_id = "v2.unstar_model_lab_project";
        if local_configuration.is_unstable_operation_enabled(operation_id) {
            warn!("Using unstable operation {operation_id}");
        } else {
            let local_error = datadog::UnstableOperationDisabledError {
                msg: "Operation 'v2.unstar_model_lab_project' is not enabled".to_string(),
            };
            return Err(datadog::Error::UnstableOperationDisabledError(local_error));
        }

        let local_client = &self.client;

        let local_uri_str = format!(
            "{}/api/v2/model-lab-api/projects/{project_id}/star",
            local_configuration.get_operation_host(operation_id),
            project_id = project_id
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
            let local_entity: Option<UnstarModelLabProjectError> =
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
