// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use crate::datadog;
use flate2::{
    write::{GzEncoder, ZlibEncoder},
    Compression,
};
use log::warn;
use reqwest::header::{HeaderMap, HeaderValue};
use serde::{Deserialize, Serialize};
use std::io::Write;

/// GetFleetDeploymentOptionalParams is a struct for passing parameters to the method [`FleetAutomationAPI::get_fleet_deployment`]
#[non_exhaustive]
#[derive(Clone, Default, Debug)]
pub struct GetFleetDeploymentOptionalParams {
    /// Maximum number of hosts to return per page. Default is 50, maximum is 100.
    pub limit: Option<i64>,
    /// Page index for pagination (zero-based). Use this to retrieve subsequent pages of hosts.
    pub page: Option<i64>,
}

impl GetFleetDeploymentOptionalParams {
    /// Maximum number of hosts to return per page. Default is 50, maximum is 100.
    pub fn limit(mut self, value: i64) -> Self {
        self.limit = Some(value);
        self
    }
    /// Page index for pagination (zero-based). Use this to retrieve subsequent pages of hosts.
    pub fn page(mut self, value: i64) -> Self {
        self.page = Some(value);
        self
    }
}

/// ListFleetAgentsOptionalParams is a struct for passing parameters to the method [`FleetAutomationAPI::list_fleet_agents`]
#[non_exhaustive]
#[derive(Clone, Default, Debug)]
pub struct ListFleetAgentsOptionalParams {
    /// Page number for pagination (starts at 0).
    pub page_number: Option<i64>,
    /// Number of results per page (must be greater than 0 and less than or equal to 100).
    pub page_size: Option<i64>,
    /// Attribute to sort by.
    pub sort_attribute: Option<String>,
    /// Sort order (true for descending, false for ascending).
    pub sort_descending: Option<bool>,
    /// Comma-separated list of tags to filter agents.
    pub tags: Option<String>,
    /// Filter string for narrowing down agent results.
    pub filter: Option<String>,
}

impl ListFleetAgentsOptionalParams {
    /// Page number for pagination (starts at 0).
    pub fn page_number(mut self, value: i64) -> Self {
        self.page_number = Some(value);
        self
    }
    /// Number of results per page (must be greater than 0 and less than or equal to 100).
    pub fn page_size(mut self, value: i64) -> Self {
        self.page_size = Some(value);
        self
    }
    /// Attribute to sort by.
    pub fn sort_attribute(mut self, value: String) -> Self {
        self.sort_attribute = Some(value);
        self
    }
    /// Sort order (true for descending, false for ascending).
    pub fn sort_descending(mut self, value: bool) -> Self {
        self.sort_descending = Some(value);
        self
    }
    /// Comma-separated list of tags to filter agents.
    pub fn tags(mut self, value: String) -> Self {
        self.tags = Some(value);
        self
    }
    /// Filter string for narrowing down agent results.
    pub fn filter(mut self, value: String) -> Self {
        self.filter = Some(value);
        self
    }
}

/// ListFleetDeploymentsOptionalParams is a struct for passing parameters to the method [`FleetAutomationAPI::list_fleet_deployments`]
#[non_exhaustive]
#[derive(Clone, Default, Debug)]
pub struct ListFleetDeploymentsOptionalParams {
    /// Number of deployments to return per page. Maximum value is 100.
    pub page_size: Option<i64>,
    /// Index of the first deployment to return. Use this with `page_size` to paginate through results.
    pub page_offset: Option<i64>,
}

impl ListFleetDeploymentsOptionalParams {
    /// Number of deployments to return per page. Maximum value is 100.
    pub fn page_size(mut self, value: i64) -> Self {
        self.page_size = Some(value);
        self
    }
    /// Index of the first deployment to return. Use this with `page_size` to paginate through results.
    pub fn page_offset(mut self, value: i64) -> Self {
        self.page_offset = Some(value);
        self
    }
}

/// CancelFleetDeploymentError is a struct for typed errors of method [`FleetAutomationAPI::cancel_fleet_deployment`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CancelFleetDeploymentError {
    APIErrorResponse(crate::datadogV2::model::APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// CreateFleetDeploymentConfigureError is a struct for typed errors of method [`FleetAutomationAPI::create_fleet_deployment_configure`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CreateFleetDeploymentConfigureError {
    APIErrorResponse(crate::datadogV2::model::APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// CreateFleetDeploymentUpgradeError is a struct for typed errors of method [`FleetAutomationAPI::create_fleet_deployment_upgrade`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CreateFleetDeploymentUpgradeError {
    APIErrorResponse(crate::datadogV2::model::APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// CreateFleetScheduleError is a struct for typed errors of method [`FleetAutomationAPI::create_fleet_schedule`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CreateFleetScheduleError {
    APIErrorResponse(crate::datadogV2::model::APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// DeleteFleetScheduleError is a struct for typed errors of method [`FleetAutomationAPI::delete_fleet_schedule`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DeleteFleetScheduleError {
    APIErrorResponse(crate::datadogV2::model::APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// GetFleetAgentInfoError is a struct for typed errors of method [`FleetAutomationAPI::get_fleet_agent_info`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetFleetAgentInfoError {
    APIErrorResponse(crate::datadogV2::model::APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// GetFleetDeploymentError is a struct for typed errors of method [`FleetAutomationAPI::get_fleet_deployment`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetFleetDeploymentError {
    APIErrorResponse(crate::datadogV2::model::APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// GetFleetScheduleError is a struct for typed errors of method [`FleetAutomationAPI::get_fleet_schedule`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetFleetScheduleError {
    APIErrorResponse(crate::datadogV2::model::APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// ListFleetAgentVersionsError is a struct for typed errors of method [`FleetAutomationAPI::list_fleet_agent_versions`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ListFleetAgentVersionsError {
    APIErrorResponse(crate::datadogV2::model::APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// ListFleetAgentsError is a struct for typed errors of method [`FleetAutomationAPI::list_fleet_agents`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ListFleetAgentsError {
    APIErrorResponse(crate::datadogV2::model::APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// ListFleetDeploymentsError is a struct for typed errors of method [`FleetAutomationAPI::list_fleet_deployments`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ListFleetDeploymentsError {
    APIErrorResponse(crate::datadogV2::model::APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// ListFleetSchedulesError is a struct for typed errors of method [`FleetAutomationAPI::list_fleet_schedules`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ListFleetSchedulesError {
    APIErrorResponse(crate::datadogV2::model::APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// TriggerFleetScheduleError is a struct for typed errors of method [`FleetAutomationAPI::trigger_fleet_schedule`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum TriggerFleetScheduleError {
    APIErrorResponse(crate::datadogV2::model::APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// UpdateFleetScheduleError is a struct for typed errors of method [`FleetAutomationAPI::update_fleet_schedule`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum UpdateFleetScheduleError {
    APIErrorResponse(crate::datadogV2::model::APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// Manage automated deployments across your fleet of hosts.
///
/// Fleet Automation provides two types of deployments:
///
/// Configuration Deployments (`/configure`):
/// - Apply configuration file changes to target hosts
/// - Support merge-patch operations to update specific configuration fields
/// - Support delete operations to remove configuration files
/// - Useful for updating Datadog Agent settings, integration configs, and more
///
/// Package Upgrade Deployments (`/upgrade`):
/// - Upgrade the Datadog Agent to specific versions
#[derive(Debug, Clone)]
pub struct FleetAutomationAPI {
    config: datadog::Configuration,
    client: reqwest_middleware::ClientWithMiddleware,
}

impl Default for FleetAutomationAPI {
    fn default() -> Self {
        Self::with_config(datadog::Configuration::default())
    }
}

impl FleetAutomationAPI {
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

    /// Cancel an active deployment and stop all pending operations.
    /// When you cancel a deployment:
    /// - All pending operations on hosts that haven't started yet are stopped
    /// - Operations currently in progress on hosts may complete or be interrupted, depending on their current state
    /// - Configuration changes or package upgrades already applied to hosts are not rolled back
    ///
    /// After cancellation, you can view the final state of the deployment using the GET endpoint to see which hosts
    /// were successfully updated before the cancellation.
    pub async fn cancel_fleet_deployment(
        &self,
        deployment_id: String,
    ) -> Result<(), datadog::Error<CancelFleetDeploymentError>> {
        match self
            .cancel_fleet_deployment_with_http_info(deployment_id)
            .await
        {
            Ok(_) => Ok(()),
            Err(err) => Err(err),
        }
    }

    /// Cancel an active deployment and stop all pending operations.
    /// When you cancel a deployment:
    /// - All pending operations on hosts that haven't started yet are stopped
    /// - Operations currently in progress on hosts may complete or be interrupted, depending on their current state
    /// - Configuration changes or package upgrades already applied to hosts are not rolled back
    ///
    /// After cancellation, you can view the final state of the deployment using the GET endpoint to see which hosts
    /// were successfully updated before the cancellation.
    pub async fn cancel_fleet_deployment_with_http_info(
        &self,
        deployment_id: String,
    ) -> Result<datadog::ResponseContent<()>, datadog::Error<CancelFleetDeploymentError>> {
        let local_configuration = &self.config;
        let operation_id = "v2.cancel_fleet_deployment";
        if local_configuration.is_unstable_operation_enabled(operation_id) {
            warn!("Using unstable operation {operation_id}");
        } else {
            let local_error = datadog::UnstableOperationDisabledError {
                msg: "Operation 'v2.cancel_fleet_deployment' is not enabled".to_string(),
            };
            return Err(datadog::Error::UnstableOperationDisabledError(local_error));
        }

        let local_client = &self.client;

        let local_uri_str = format!(
            "{}/api/unstable/fleet/deployments/{deployment_id}/cancel",
            local_configuration.get_operation_host(operation_id),
            deployment_id = datadog::urlencode(deployment_id)
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
            let local_entity: Option<CancelFleetDeploymentError> =
                serde_json::from_str(&local_content).ok();
            let local_error = datadog::ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(datadog::Error::ResponseError(local_error))
        }
    }

    /// Create a new deployment to apply configuration changes
    /// to a fleet of hosts matching the specified filter query.
    ///
    /// This endpoint supports two types of configuration operations:
    /// - `merge-patch`: Merges the provided patch data with the existing configuration file,
    ///   creating the file if it doesn't exist
    /// - `delete`: Removes the specified configuration file from the target hosts
    ///
    /// The deployment is created and started automatically. You can specify multiple configuration
    /// operations that will be executed in order on each target host. Use the filter query to target
    /// specific hosts using the Datadog query syntax.
    pub async fn create_fleet_deployment_configure(
        &self,
        body: crate::datadogV2::model::FleetDeploymentConfigureCreateRequest,
    ) -> Result<
        crate::datadogV2::model::FleetDeploymentResponse,
        datadog::Error<CreateFleetDeploymentConfigureError>,
    > {
        match self
            .create_fleet_deployment_configure_with_http_info(body)
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

    /// Create a new deployment to apply configuration changes
    /// to a fleet of hosts matching the specified filter query.
    ///
    /// This endpoint supports two types of configuration operations:
    /// - `merge-patch`: Merges the provided patch data with the existing configuration file,
    ///   creating the file if it doesn't exist
    /// - `delete`: Removes the specified configuration file from the target hosts
    ///
    /// The deployment is created and started automatically. You can specify multiple configuration
    /// operations that will be executed in order on each target host. Use the filter query to target
    /// specific hosts using the Datadog query syntax.
    pub async fn create_fleet_deployment_configure_with_http_info(
        &self,
        body: crate::datadogV2::model::FleetDeploymentConfigureCreateRequest,
    ) -> Result<
        datadog::ResponseContent<crate::datadogV2::model::FleetDeploymentResponse>,
        datadog::Error<CreateFleetDeploymentConfigureError>,
    > {
        let local_configuration = &self.config;
        let operation_id = "v2.create_fleet_deployment_configure";
        if local_configuration.is_unstable_operation_enabled(operation_id) {
            warn!("Using unstable operation {operation_id}");
        } else {
            let local_error = datadog::UnstableOperationDisabledError {
                msg: "Operation 'v2.create_fleet_deployment_configure' is not enabled".to_string(),
            };
            return Err(datadog::Error::UnstableOperationDisabledError(local_error));
        }

        let local_client = &self.client;

        let local_uri_str = format!(
            "{}/api/unstable/fleet/deployments/configure",
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
            match serde_json::from_str::<crate::datadogV2::model::FleetDeploymentResponse>(
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
            let local_entity: Option<CreateFleetDeploymentConfigureError> =
                serde_json::from_str(&local_content).ok();
            let local_error = datadog::ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(datadog::Error::ResponseError(local_error))
        }
    }

    /// Create and immediately start a new package upgrade
    /// on hosts matching the specified filter query.
    ///
    /// This endpoint allows you to upgrade the Datadog Agent to a specific version
    /// on hosts matching the specified filter query.
    ///
    /// The deployment is created and started automatically. The system will:
    /// 1. Identify all hosts matching the filter query
    /// 2. Validate that the specified version is available
    /// 3. Begin rolling out the package upgrade to the target hosts
    pub async fn create_fleet_deployment_upgrade(
        &self,
        body: crate::datadogV2::model::FleetDeploymentPackageUpgradeCreateRequest,
    ) -> Result<
        crate::datadogV2::model::FleetDeploymentResponse,
        datadog::Error<CreateFleetDeploymentUpgradeError>,
    > {
        match self
            .create_fleet_deployment_upgrade_with_http_info(body)
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

    /// Create and immediately start a new package upgrade
    /// on hosts matching the specified filter query.
    ///
    /// This endpoint allows you to upgrade the Datadog Agent to a specific version
    /// on hosts matching the specified filter query.
    ///
    /// The deployment is created and started automatically. The system will:
    /// 1. Identify all hosts matching the filter query
    /// 2. Validate that the specified version is available
    /// 3. Begin rolling out the package upgrade to the target hosts
    pub async fn create_fleet_deployment_upgrade_with_http_info(
        &self,
        body: crate::datadogV2::model::FleetDeploymentPackageUpgradeCreateRequest,
    ) -> Result<
        datadog::ResponseContent<crate::datadogV2::model::FleetDeploymentResponse>,
        datadog::Error<CreateFleetDeploymentUpgradeError>,
    > {
        let local_configuration = &self.config;
        let operation_id = "v2.create_fleet_deployment_upgrade";
        if local_configuration.is_unstable_operation_enabled(operation_id) {
            warn!("Using unstable operation {operation_id}");
        } else {
            let local_error = datadog::UnstableOperationDisabledError {
                msg: "Operation 'v2.create_fleet_deployment_upgrade' is not enabled".to_string(),
            };
            return Err(datadog::Error::UnstableOperationDisabledError(local_error));
        }

        let local_client = &self.client;

        let local_uri_str = format!(
            "{}/api/unstable/fleet/deployments/upgrade",
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
            match serde_json::from_str::<crate::datadogV2::model::FleetDeploymentResponse>(
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
            let local_entity: Option<CreateFleetDeploymentUpgradeError> =
                serde_json::from_str(&local_content).ok();
            let local_error = datadog::ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(datadog::Error::ResponseError(local_error))
        }
    }

    /// Create a new schedule for automated package upgrades.
    ///
    /// Schedules define when and how often to automatically deploy package upgrades to a fleet
    /// of hosts. Each schedule includes:
    /// - A filter query to select target hosts
    /// - A recurrence rule defining maintenance windows
    /// - A version strategy (e.g., always latest, or N versions behind latest)
    ///
    /// When the schedule triggers during a maintenance window, it automatically creates a
    /// deployment that upgrades the Datadog Agent to the specified version on all matching hosts.
    pub async fn create_fleet_schedule(
        &self,
        body: crate::datadogV2::model::FleetScheduleCreateRequest,
    ) -> Result<
        crate::datadogV2::model::FleetScheduleResponse,
        datadog::Error<CreateFleetScheduleError>,
    > {
        match self.create_fleet_schedule_with_http_info(body).await {
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

    /// Create a new schedule for automated package upgrades.
    ///
    /// Schedules define when and how often to automatically deploy package upgrades to a fleet
    /// of hosts. Each schedule includes:
    /// - A filter query to select target hosts
    /// - A recurrence rule defining maintenance windows
    /// - A version strategy (e.g., always latest, or N versions behind latest)
    ///
    /// When the schedule triggers during a maintenance window, it automatically creates a
    /// deployment that upgrades the Datadog Agent to the specified version on all matching hosts.
    pub async fn create_fleet_schedule_with_http_info(
        &self,
        body: crate::datadogV2::model::FleetScheduleCreateRequest,
    ) -> Result<
        datadog::ResponseContent<crate::datadogV2::model::FleetScheduleResponse>,
        datadog::Error<CreateFleetScheduleError>,
    > {
        let local_configuration = &self.config;
        let operation_id = "v2.create_fleet_schedule";
        if local_configuration.is_unstable_operation_enabled(operation_id) {
            warn!("Using unstable operation {operation_id}");
        } else {
            let local_error = datadog::UnstableOperationDisabledError {
                msg: "Operation 'v2.create_fleet_schedule' is not enabled".to_string(),
            };
            return Err(datadog::Error::UnstableOperationDisabledError(local_error));
        }

        let local_client = &self.client;

        let local_uri_str = format!(
            "{}/api/unstable/fleet/schedules",
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
            match serde_json::from_str::<crate::datadogV2::model::FleetScheduleResponse>(
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
            let local_entity: Option<CreateFleetScheduleError> =
                serde_json::from_str(&local_content).ok();
            let local_error = datadog::ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(datadog::Error::ResponseError(local_error))
        }
    }

    /// Delete a schedule permanently.
    ///
    /// When you delete a schedule:
    /// - The schedule is permanently removed and will no longer create deployments
    /// - Any deployments already created by this schedule are not affected
    /// - This action cannot be undone
    ///
    /// If you want to temporarily stop a schedule from creating deployments, consider
    /// updating its status to "inactive" instead of deleting it.
    pub async fn delete_fleet_schedule(
        &self,
        id: String,
    ) -> Result<(), datadog::Error<DeleteFleetScheduleError>> {
        match self.delete_fleet_schedule_with_http_info(id).await {
            Ok(_) => Ok(()),
            Err(err) => Err(err),
        }
    }

    /// Delete a schedule permanently.
    ///
    /// When you delete a schedule:
    /// - The schedule is permanently removed and will no longer create deployments
    /// - Any deployments already created by this schedule are not affected
    /// - This action cannot be undone
    ///
    /// If you want to temporarily stop a schedule from creating deployments, consider
    /// updating its status to "inactive" instead of deleting it.
    pub async fn delete_fleet_schedule_with_http_info(
        &self,
        id: String,
    ) -> Result<datadog::ResponseContent<()>, datadog::Error<DeleteFleetScheduleError>> {
        let local_configuration = &self.config;
        let operation_id = "v2.delete_fleet_schedule";
        if local_configuration.is_unstable_operation_enabled(operation_id) {
            warn!("Using unstable operation {operation_id}");
        } else {
            let local_error = datadog::UnstableOperationDisabledError {
                msg: "Operation 'v2.delete_fleet_schedule' is not enabled".to_string(),
            };
            return Err(datadog::Error::UnstableOperationDisabledError(local_error));
        }

        let local_client = &self.client;

        let local_uri_str = format!(
            "{}/api/unstable/fleet/schedules/{id}",
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
            let local_entity: Option<DeleteFleetScheduleError> =
                serde_json::from_str(&local_content).ok();
            let local_error = datadog::ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(datadog::Error::ResponseError(local_error))
        }
    }

    /// Retrieve detailed information about a specific Datadog Agent.
    /// This endpoint returns comprehensive information about an agent including:
    /// - Agent details and metadata
    /// - Configured integrations organized by status (working, warning, error, missing)
    /// - Detected integrations
    /// - Configuration files and layers
    pub async fn get_fleet_agent_info(
        &self,
        agent_key: String,
    ) -> Result<
        crate::datadogV2::model::FleetAgentInfoResponse,
        datadog::Error<GetFleetAgentInfoError>,
    > {
        match self.get_fleet_agent_info_with_http_info(agent_key).await {
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

    /// Retrieve detailed information about a specific Datadog Agent.
    /// This endpoint returns comprehensive information about an agent including:
    /// - Agent details and metadata
    /// - Configured integrations organized by status (working, warning, error, missing)
    /// - Detected integrations
    /// - Configuration files and layers
    pub async fn get_fleet_agent_info_with_http_info(
        &self,
        agent_key: String,
    ) -> Result<
        datadog::ResponseContent<crate::datadogV2::model::FleetAgentInfoResponse>,
        datadog::Error<GetFleetAgentInfoError>,
    > {
        let local_configuration = &self.config;
        let operation_id = "v2.get_fleet_agent_info";
        if local_configuration.is_unstable_operation_enabled(operation_id) {
            warn!("Using unstable operation {operation_id}");
        } else {
            let local_error = datadog::UnstableOperationDisabledError {
                msg: "Operation 'v2.get_fleet_agent_info' is not enabled".to_string(),
            };
            return Err(datadog::Error::UnstableOperationDisabledError(local_error));
        }

        let local_client = &self.client;

        let local_uri_str = format!(
            "{}/api/unstable/fleet/agents/{agent_key}",
            local_configuration.get_operation_host(operation_id),
            agent_key = datadog::urlencode(agent_key)
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
            match serde_json::from_str::<crate::datadogV2::model::FleetAgentInfoResponse>(
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
            let local_entity: Option<GetFleetAgentInfoError> =
                serde_json::from_str(&local_content).ok();
            let local_error = datadog::ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(datadog::Error::ResponseError(local_error))
        }
    }

    /// Retrieve detailed information about a specific deployment using its unique identifier.
    /// This endpoint returns comprehensive information about a deployment, including:
    /// - Deployment metadata (ID, type, filter query)
    /// - Total number of target hosts
    /// - Current high-level status (pending, running, succeeded, failed)
    /// - Estimated completion time
    /// - Configuration operations that were or are being applied
    /// - Detailed host list: A paginated array of hosts included in this deployment with individual
    ///   host status, current package versions, and any errors
    ///
    /// The host list provides visibility into the per-host execution status, allowing you to:
    /// - Monitor which hosts have completed successfully
    /// - Identify hosts that are still in progress
    /// - Investigate failures on specific hosts
    /// - View current package versions installed on each host (including initial, target, and current
    ///   versions for each package)
    ///
    /// Pagination: Use the `limit` and `page` query parameters to paginate through hosts. The response
    /// includes pagination metadata in the `meta.hosts` field with information about the current page,
    /// total pages, and total host count. The default page size is 50 hosts, with a maximum of 100.
    pub async fn get_fleet_deployment(
        &self,
        deployment_id: String,
        params: GetFleetDeploymentOptionalParams,
    ) -> Result<
        crate::datadogV2::model::FleetDeploymentResponse,
        datadog::Error<GetFleetDeploymentError>,
    > {
        match self
            .get_fleet_deployment_with_http_info(deployment_id, params)
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

    /// Retrieve detailed information about a specific deployment using its unique identifier.
    /// This endpoint returns comprehensive information about a deployment, including:
    /// - Deployment metadata (ID, type, filter query)
    /// - Total number of target hosts
    /// - Current high-level status (pending, running, succeeded, failed)
    /// - Estimated completion time
    /// - Configuration operations that were or are being applied
    /// - Detailed host list: A paginated array of hosts included in this deployment with individual
    ///   host status, current package versions, and any errors
    ///
    /// The host list provides visibility into the per-host execution status, allowing you to:
    /// - Monitor which hosts have completed successfully
    /// - Identify hosts that are still in progress
    /// - Investigate failures on specific hosts
    /// - View current package versions installed on each host (including initial, target, and current
    ///   versions for each package)
    ///
    /// Pagination: Use the `limit` and `page` query parameters to paginate through hosts. The response
    /// includes pagination metadata in the `meta.hosts` field with information about the current page,
    /// total pages, and total host count. The default page size is 50 hosts, with a maximum of 100.
    pub async fn get_fleet_deployment_with_http_info(
        &self,
        deployment_id: String,
        params: GetFleetDeploymentOptionalParams,
    ) -> Result<
        datadog::ResponseContent<crate::datadogV2::model::FleetDeploymentResponse>,
        datadog::Error<GetFleetDeploymentError>,
    > {
        let local_configuration = &self.config;
        let operation_id = "v2.get_fleet_deployment";
        if local_configuration.is_unstable_operation_enabled(operation_id) {
            warn!("Using unstable operation {operation_id}");
        } else {
            let local_error = datadog::UnstableOperationDisabledError {
                msg: "Operation 'v2.get_fleet_deployment' is not enabled".to_string(),
            };
            return Err(datadog::Error::UnstableOperationDisabledError(local_error));
        }

        // unbox and build optional parameters
        let limit = params.limit;
        let page = params.page;

        let local_client = &self.client;

        let local_uri_str = format!(
            "{}/api/unstable/fleet/deployments/{deployment_id}",
            local_configuration.get_operation_host(operation_id),
            deployment_id = datadog::urlencode(deployment_id)
        );
        let mut local_req_builder =
            local_client.request(reqwest::Method::GET, local_uri_str.as_str());

        if let Some(ref local_query_param) = limit {
            local_req_builder =
                local_req_builder.query(&[("limit", &local_query_param.to_string())]);
        };
        if let Some(ref local_query_param) = page {
            local_req_builder =
                local_req_builder.query(&[("page", &local_query_param.to_string())]);
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
            match serde_json::from_str::<crate::datadogV2::model::FleetDeploymentResponse>(
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
            let local_entity: Option<GetFleetDeploymentError> =
                serde_json::from_str(&local_content).ok();
            let local_error = datadog::ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(datadog::Error::ResponseError(local_error))
        }
    }

    /// Retrieve detailed information about a specific schedule using its unique identifier.
    ///
    /// This endpoint returns comprehensive information about a schedule, including:
    /// - Schedule metadata (ID, name, creation/update timestamps)
    /// - Filter query for selecting target hosts
    /// - Recurrence rule defining when deployments are triggered
    /// - Version strategy for package upgrades
    /// - Current status (active or inactive)
    pub async fn get_fleet_schedule(
        &self,
        id: String,
    ) -> Result<crate::datadogV2::model::FleetScheduleResponse, datadog::Error<GetFleetScheduleError>>
    {
        match self.get_fleet_schedule_with_http_info(id).await {
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

    /// Retrieve detailed information about a specific schedule using its unique identifier.
    ///
    /// This endpoint returns comprehensive information about a schedule, including:
    /// - Schedule metadata (ID, name, creation/update timestamps)
    /// - Filter query for selecting target hosts
    /// - Recurrence rule defining when deployments are triggered
    /// - Version strategy for package upgrades
    /// - Current status (active or inactive)
    pub async fn get_fleet_schedule_with_http_info(
        &self,
        id: String,
    ) -> Result<
        datadog::ResponseContent<crate::datadogV2::model::FleetScheduleResponse>,
        datadog::Error<GetFleetScheduleError>,
    > {
        let local_configuration = &self.config;
        let operation_id = "v2.get_fleet_schedule";
        if local_configuration.is_unstable_operation_enabled(operation_id) {
            warn!("Using unstable operation {operation_id}");
        } else {
            let local_error = datadog::UnstableOperationDisabledError {
                msg: "Operation 'v2.get_fleet_schedule' is not enabled".to_string(),
            };
            return Err(datadog::Error::UnstableOperationDisabledError(local_error));
        }

        let local_client = &self.client;

        let local_uri_str = format!(
            "{}/api/unstable/fleet/schedules/{id}",
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
        log::debug!("request content: {:?}", local_req.body());
        let local_resp = local_client.execute(local_req).await?;

        let local_status = local_resp.status();
        let local_content = local_resp.text().await?;
        log::debug!("response content: {}", local_content);

        if !local_status.is_client_error() && !local_status.is_server_error() {
            match serde_json::from_str::<crate::datadogV2::model::FleetScheduleResponse>(
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
            let local_entity: Option<GetFleetScheduleError> =
                serde_json::from_str(&local_content).ok();
            let local_error = datadog::ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(datadog::Error::ResponseError(local_error))
        }
    }

    /// Retrieve a list of all available Datadog Agent versions.
    ///
    /// This endpoint returns the available Agent versions that can be deployed to your fleet.
    /// These versions are used when creating deployments or configuring schedules for
    /// automated Agent upgrades.
    pub async fn list_fleet_agent_versions(
        &self,
    ) -> Result<
        crate::datadogV2::model::FleetAgentVersionsResponse,
        datadog::Error<ListFleetAgentVersionsError>,
    > {
        match self.list_fleet_agent_versions_with_http_info().await {
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

    /// Retrieve a list of all available Datadog Agent versions.
    ///
    /// This endpoint returns the available Agent versions that can be deployed to your fleet.
    /// These versions are used when creating deployments or configuring schedules for
    /// automated Agent upgrades.
    pub async fn list_fleet_agent_versions_with_http_info(
        &self,
    ) -> Result<
        datadog::ResponseContent<crate::datadogV2::model::FleetAgentVersionsResponse>,
        datadog::Error<ListFleetAgentVersionsError>,
    > {
        let local_configuration = &self.config;
        let operation_id = "v2.list_fleet_agent_versions";
        if local_configuration.is_unstable_operation_enabled(operation_id) {
            warn!("Using unstable operation {operation_id}");
        } else {
            let local_error = datadog::UnstableOperationDisabledError {
                msg: "Operation 'v2.list_fleet_agent_versions' is not enabled".to_string(),
            };
            return Err(datadog::Error::UnstableOperationDisabledError(local_error));
        }

        let local_client = &self.client;

        let local_uri_str = format!(
            "{}/api/unstable/fleet/agent_versions",
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
            match serde_json::from_str::<crate::datadogV2::model::FleetAgentVersionsResponse>(
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
            let local_entity: Option<ListFleetAgentVersionsError> =
                serde_json::from_str(&local_content).ok();
            let local_error = datadog::ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(datadog::Error::ResponseError(local_error))
        }
    }

    /// Retrieve a paginated list of all Datadog Agents.
    /// This endpoint returns a paginated list of all Datadog Agents with support for pagination, sorting, and filtering.
    /// Use the `page_number` and `page_size` query parameters to paginate through results.
    pub async fn list_fleet_agents(
        &self,
        params: ListFleetAgentsOptionalParams,
    ) -> Result<crate::datadogV2::model::FleetAgentsResponse, datadog::Error<ListFleetAgentsError>>
    {
        match self.list_fleet_agents_with_http_info(params).await {
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

    /// Retrieve a paginated list of all Datadog Agents.
    /// This endpoint returns a paginated list of all Datadog Agents with support for pagination, sorting, and filtering.
    /// Use the `page_number` and `page_size` query parameters to paginate through results.
    pub async fn list_fleet_agents_with_http_info(
        &self,
        params: ListFleetAgentsOptionalParams,
    ) -> Result<
        datadog::ResponseContent<crate::datadogV2::model::FleetAgentsResponse>,
        datadog::Error<ListFleetAgentsError>,
    > {
        let local_configuration = &self.config;
        let operation_id = "v2.list_fleet_agents";
        if local_configuration.is_unstable_operation_enabled(operation_id) {
            warn!("Using unstable operation {operation_id}");
        } else {
            let local_error = datadog::UnstableOperationDisabledError {
                msg: "Operation 'v2.list_fleet_agents' is not enabled".to_string(),
            };
            return Err(datadog::Error::UnstableOperationDisabledError(local_error));
        }

        // unbox and build optional parameters
        let page_number = params.page_number;
        let page_size = params.page_size;
        let sort_attribute = params.sort_attribute;
        let sort_descending = params.sort_descending;
        let tags = params.tags;
        let filter = params.filter;

        let local_client = &self.client;

        let local_uri_str = format!(
            "{}/api/unstable/fleet/agents",
            local_configuration.get_operation_host(operation_id)
        );
        let mut local_req_builder =
            local_client.request(reqwest::Method::GET, local_uri_str.as_str());

        if let Some(ref local_query_param) = page_number {
            local_req_builder =
                local_req_builder.query(&[("page_number", &local_query_param.to_string())]);
        };
        if let Some(ref local_query_param) = page_size {
            local_req_builder =
                local_req_builder.query(&[("page_size", &local_query_param.to_string())]);
        };
        if let Some(ref local_query_param) = sort_attribute {
            local_req_builder =
                local_req_builder.query(&[("sort_attribute", &local_query_param.to_string())]);
        };
        if let Some(ref local_query_param) = sort_descending {
            local_req_builder =
                local_req_builder.query(&[("sort_descending", &local_query_param.to_string())]);
        };
        if let Some(ref local_query_param) = tags {
            local_req_builder =
                local_req_builder.query(&[("tags", &local_query_param.to_string())]);
        };
        if let Some(ref local_query_param) = filter {
            local_req_builder =
                local_req_builder.query(&[("filter", &local_query_param.to_string())]);
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
            match serde_json::from_str::<crate::datadogV2::model::FleetAgentsResponse>(
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
            let local_entity: Option<ListFleetAgentsError> =
                serde_json::from_str(&local_content).ok();
            let local_error = datadog::ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(datadog::Error::ResponseError(local_error))
        }
    }

    /// Retrieve a list of all deployments for fleet automation.
    /// Use the `page_size` and `page_offset` parameters to paginate results.
    pub async fn list_fleet_deployments(
        &self,
        params: ListFleetDeploymentsOptionalParams,
    ) -> Result<
        crate::datadogV2::model::FleetDeploymentsResponse,
        datadog::Error<ListFleetDeploymentsError>,
    > {
        match self.list_fleet_deployments_with_http_info(params).await {
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

    /// Retrieve a list of all deployments for fleet automation.
    /// Use the `page_size` and `page_offset` parameters to paginate results.
    pub async fn list_fleet_deployments_with_http_info(
        &self,
        params: ListFleetDeploymentsOptionalParams,
    ) -> Result<
        datadog::ResponseContent<crate::datadogV2::model::FleetDeploymentsResponse>,
        datadog::Error<ListFleetDeploymentsError>,
    > {
        let local_configuration = &self.config;
        let operation_id = "v2.list_fleet_deployments";
        if local_configuration.is_unstable_operation_enabled(operation_id) {
            warn!("Using unstable operation {operation_id}");
        } else {
            let local_error = datadog::UnstableOperationDisabledError {
                msg: "Operation 'v2.list_fleet_deployments' is not enabled".to_string(),
            };
            return Err(datadog::Error::UnstableOperationDisabledError(local_error));
        }

        // unbox and build optional parameters
        let page_size = params.page_size;
        let page_offset = params.page_offset;

        let local_client = &self.client;

        let local_uri_str = format!(
            "{}/api/unstable/fleet/deployments",
            local_configuration.get_operation_host(operation_id)
        );
        let mut local_req_builder =
            local_client.request(reqwest::Method::GET, local_uri_str.as_str());

        if let Some(ref local_query_param) = page_size {
            local_req_builder =
                local_req_builder.query(&[("page_size", &local_query_param.to_string())]);
        };
        if let Some(ref local_query_param) = page_offset {
            local_req_builder =
                local_req_builder.query(&[("page_offset", &local_query_param.to_string())]);
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
            match serde_json::from_str::<crate::datadogV2::model::FleetDeploymentsResponse>(
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
            let local_entity: Option<ListFleetDeploymentsError> =
                serde_json::from_str(&local_content).ok();
            let local_error = datadog::ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(datadog::Error::ResponseError(local_error))
        }
    }

    /// Retrieve a list of all schedules for automated fleet deployments.
    ///
    /// Schedules allow you to automate package upgrades by defining maintenance windows
    /// and recurrence rules. Each schedule automatically creates deployments based on its
    /// configuration.
    pub async fn list_fleet_schedules(
        &self,
    ) -> Result<
        crate::datadogV2::model::FleetSchedulesResponse,
        datadog::Error<ListFleetSchedulesError>,
    > {
        match self.list_fleet_schedules_with_http_info().await {
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

    /// Retrieve a list of all schedules for automated fleet deployments.
    ///
    /// Schedules allow you to automate package upgrades by defining maintenance windows
    /// and recurrence rules. Each schedule automatically creates deployments based on its
    /// configuration.
    pub async fn list_fleet_schedules_with_http_info(
        &self,
    ) -> Result<
        datadog::ResponseContent<crate::datadogV2::model::FleetSchedulesResponse>,
        datadog::Error<ListFleetSchedulesError>,
    > {
        let local_configuration = &self.config;
        let operation_id = "v2.list_fleet_schedules";
        if local_configuration.is_unstable_operation_enabled(operation_id) {
            warn!("Using unstable operation {operation_id}");
        } else {
            let local_error = datadog::UnstableOperationDisabledError {
                msg: "Operation 'v2.list_fleet_schedules' is not enabled".to_string(),
            };
            return Err(datadog::Error::UnstableOperationDisabledError(local_error));
        }

        let local_client = &self.client;

        let local_uri_str = format!(
            "{}/api/unstable/fleet/schedules",
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
            match serde_json::from_str::<crate::datadogV2::model::FleetSchedulesResponse>(
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
            let local_entity: Option<ListFleetSchedulesError> =
                serde_json::from_str(&local_content).ok();
            let local_error = datadog::ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(datadog::Error::ResponseError(local_error))
        }
    }

    /// Manually trigger a schedule to immediately create and start a deployment.
    ///
    /// This endpoint allows you to manually initiate a deployment using the schedule's
    /// configuration, without waiting for the next scheduled maintenance window. This is
    /// useful for:
    /// - Testing a schedule before it runs automatically
    /// - Performing an emergency update outside the regular maintenance window
    /// - Creating an ad-hoc deployment with the same settings as a schedule
    ///
    /// The deployment is created immediately with:
    /// - The same filter query as the schedule
    /// - The package version determined by the schedule's version strategy
    /// - All matching hosts as targets
    ///
    /// The manually triggered deployment is independent of the schedule and does not
    /// affect the schedule's normal recurrence pattern.
    pub async fn trigger_fleet_schedule(
        &self,
        id: String,
    ) -> Result<
        crate::datadogV2::model::FleetDeploymentResponse,
        datadog::Error<TriggerFleetScheduleError>,
    > {
        match self.trigger_fleet_schedule_with_http_info(id).await {
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

    /// Manually trigger a schedule to immediately create and start a deployment.
    ///
    /// This endpoint allows you to manually initiate a deployment using the schedule's
    /// configuration, without waiting for the next scheduled maintenance window. This is
    /// useful for:
    /// - Testing a schedule before it runs automatically
    /// - Performing an emergency update outside the regular maintenance window
    /// - Creating an ad-hoc deployment with the same settings as a schedule
    ///
    /// The deployment is created immediately with:
    /// - The same filter query as the schedule
    /// - The package version determined by the schedule's version strategy
    /// - All matching hosts as targets
    ///
    /// The manually triggered deployment is independent of the schedule and does not
    /// affect the schedule's normal recurrence pattern.
    pub async fn trigger_fleet_schedule_with_http_info(
        &self,
        id: String,
    ) -> Result<
        datadog::ResponseContent<crate::datadogV2::model::FleetDeploymentResponse>,
        datadog::Error<TriggerFleetScheduleError>,
    > {
        let local_configuration = &self.config;
        let operation_id = "v2.trigger_fleet_schedule";
        if local_configuration.is_unstable_operation_enabled(operation_id) {
            warn!("Using unstable operation {operation_id}");
        } else {
            let local_error = datadog::UnstableOperationDisabledError {
                msg: "Operation 'v2.trigger_fleet_schedule' is not enabled".to_string(),
            };
            return Err(datadog::Error::UnstableOperationDisabledError(local_error));
        }

        let local_client = &self.client;

        let local_uri_str = format!(
            "{}/api/unstable/fleet/schedules/{id}/trigger",
            local_configuration.get_operation_host(operation_id),
            id = datadog::urlencode(id)
        );
        let mut local_req_builder =
            local_client.request(reqwest::Method::POST, local_uri_str.as_str());

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
            match serde_json::from_str::<crate::datadogV2::model::FleetDeploymentResponse>(
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
            let local_entity: Option<TriggerFleetScheduleError> =
                serde_json::from_str(&local_content).ok();
            let local_error = datadog::ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(datadog::Error::ResponseError(local_error))
        }
    }

    /// Partially update a schedule by providing only the fields you want to change.
    ///
    /// This endpoint allows you to modify specific attributes of a schedule without
    /// affecting other fields. Common use cases include:
    /// - Changing the schedule status between active and inactive
    /// - Updating the maintenance window times
    /// - Modifying the filter query to target different hosts
    /// - Adjusting the version strategy
    ///
    /// Only include the fields you want to update in the request body. All fields
    /// are optional in a PATCH request.
    pub async fn update_fleet_schedule(
        &self,
        id: String,
        body: crate::datadogV2::model::FleetSchedulePatchRequest,
    ) -> Result<
        crate::datadogV2::model::FleetScheduleResponse,
        datadog::Error<UpdateFleetScheduleError>,
    > {
        match self.update_fleet_schedule_with_http_info(id, body).await {
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

    /// Partially update a schedule by providing only the fields you want to change.
    ///
    /// This endpoint allows you to modify specific attributes of a schedule without
    /// affecting other fields. Common use cases include:
    /// - Changing the schedule status between active and inactive
    /// - Updating the maintenance window times
    /// - Modifying the filter query to target different hosts
    /// - Adjusting the version strategy
    ///
    /// Only include the fields you want to update in the request body. All fields
    /// are optional in a PATCH request.
    pub async fn update_fleet_schedule_with_http_info(
        &self,
        id: String,
        body: crate::datadogV2::model::FleetSchedulePatchRequest,
    ) -> Result<
        datadog::ResponseContent<crate::datadogV2::model::FleetScheduleResponse>,
        datadog::Error<UpdateFleetScheduleError>,
    > {
        let local_configuration = &self.config;
        let operation_id = "v2.update_fleet_schedule";
        if local_configuration.is_unstable_operation_enabled(operation_id) {
            warn!("Using unstable operation {operation_id}");
        } else {
            let local_error = datadog::UnstableOperationDisabledError {
                msg: "Operation 'v2.update_fleet_schedule' is not enabled".to_string(),
            };
            return Err(datadog::Error::UnstableOperationDisabledError(local_error));
        }

        let local_client = &self.client;

        let local_uri_str = format!(
            "{}/api/unstable/fleet/schedules/{id}",
            local_configuration.get_operation_host(operation_id),
            id = datadog::urlencode(id)
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
            match serde_json::from_str::<crate::datadogV2::model::FleetScheduleResponse>(
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
            let local_entity: Option<UpdateFleetScheduleError> =
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
