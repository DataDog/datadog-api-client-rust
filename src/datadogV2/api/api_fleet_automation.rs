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

/// GetFleetAgentDetailV2OptionalParams is a struct for passing parameters to the method [`FleetAutomationAPI::get_fleet_agent_detail_v2`]
#[non_exhaustive]
#[derive(Clone, Default, Debug)]
pub struct GetFleetAgentDetailV2OptionalParams {
    /// Comma-separated list of additional fields to include in the response. Valid values are `integrations` and `configuration_files`. Omitting this parameter returns only `agent_infos`. Unrecognized values are silently ignored rather than causing an error.
    pub include: Option<String>,
}

impl GetFleetAgentDetailV2OptionalParams {
    /// Comma-separated list of additional fields to include in the response. Valid values are `integrations` and `configuration_files`. Omitting this parameter returns only `agent_infos`. Unrecognized values are silently ignored rather than causing an error.
    pub fn include(mut self, value: String) -> Self {
        self.include = Some(value);
        self
    }
}

/// ListFleetAgentTracersOptionalParams is a struct for passing parameters to the method [`FleetAutomationAPI::list_fleet_agent_tracers`]
#[non_exhaustive]
#[derive(Clone, Default, Debug)]
pub struct ListFleetAgentTracersOptionalParams {
    /// Page number for pagination (starts at 0).
    pub page_number: Option<i64>,
    /// Number of results per page (must be greater than 0 and less than or equal to 100).
    pub page_size: Option<i64>,
    /// Attribute to sort by.
    pub sort_attribute: Option<String>,
    /// Sort order (true for descending, false for ascending).
    pub sort_descending: Option<bool>,
}

impl ListFleetAgentTracersOptionalParams {
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
}

/// ListFleetAgentsV2OptionalParams is a struct for passing parameters to the method [`FleetAutomationAPI::list_fleet_agents_v2`]
#[non_exhaustive]
#[derive(Clone, Default, Debug)]
pub struct ListFleetAgentsV2OptionalParams {
    /// Page number for pagination, starting at 0.
    pub page_number: Option<i64>,
    /// Number of agents to return per page. Maximum value is 100. Defaults to 10.
    pub page_size: Option<i64>,
    /// Filter string to narrow down agent results.
    pub filter: Option<String>,
    /// Comma-separated list of tag keys to select which tags are included in each agent's `tags` attribute. Does not filter which agents are returned.
    pub tags: Option<String>,
    /// Agent attribute to sort results by. Must be a supported attribute name; unsupported values return a 400 error.
    pub sort_attribute: Option<String>,
    /// Set to `true` to sort results in descending order. Defaults to ascending.
    pub sort_descending: Option<bool>,
}

impl ListFleetAgentsV2OptionalParams {
    /// Page number for pagination, starting at 0.
    pub fn page_number(mut self, value: i64) -> Self {
        self.page_number = Some(value);
        self
    }
    /// Number of agents to return per page. Maximum value is 100. Defaults to 10.
    pub fn page_size(mut self, value: i64) -> Self {
        self.page_size = Some(value);
        self
    }
    /// Filter string to narrow down agent results.
    pub fn filter(mut self, value: String) -> Self {
        self.filter = Some(value);
        self
    }
    /// Comma-separated list of tag keys to select which tags are included in each agent's `tags` attribute. Does not filter which agents are returned.
    pub fn tags(mut self, value: String) -> Self {
        self.tags = Some(value);
        self
    }
    /// Agent attribute to sort results by. Must be a supported attribute name; unsupported values return a 400 error.
    pub fn sort_attribute(mut self, value: String) -> Self {
        self.sort_attribute = Some(value);
        self
    }
    /// Set to `true` to sort results in descending order. Defaults to ascending.
    pub fn sort_descending(mut self, value: bool) -> Self {
        self.sort_descending = Some(value);
        self
    }
}

/// ListFleetDeploymentsV2OptionalParams is a struct for passing parameters to the method [`FleetAutomationAPI::list_fleet_deployments_v2`]
#[non_exhaustive]
#[derive(Clone, Default, Debug)]
pub struct ListFleetDeploymentsV2OptionalParams {
    /// Number of deployments to return per page. Maximum value is 100.
    pub page_size: Option<i64>,
    /// Page number for pagination, starting at 0.
    pub page_number: Option<i64>,
    /// Field to sort results by (for example, `start_date`). Must be a supported field
    /// name; unsupported values return a 400 error.
    pub sort: Option<String>,
    /// Set to `true` to sort in ascending order. This setting has no effect unless `sort` is also set.
    /// Defaults to descending order.
    pub ascending: Option<bool>,
    /// Query used to filter deployments. Uses the Datadog query syntax. Filtering on an
    /// unsupported field returns a 400 error. For example:
    /// - `status:failed` or `status:done_with_errors`: deployments that need investigation.
    /// - `status:running`: deployments currently in flight.
    /// - `update_type:update_package` or `update_type:update_config_operations`: deployments of a given type.
    pub filter: Option<String>,
}

impl ListFleetDeploymentsV2OptionalParams {
    /// Number of deployments to return per page. Maximum value is 100.
    pub fn page_size(mut self, value: i64) -> Self {
        self.page_size = Some(value);
        self
    }
    /// Page number for pagination, starting at 0.
    pub fn page_number(mut self, value: i64) -> Self {
        self.page_number = Some(value);
        self
    }
    /// Field to sort results by (for example, `start_date`). Must be a supported field
    /// name; unsupported values return a 400 error.
    pub fn sort(mut self, value: String) -> Self {
        self.sort = Some(value);
        self
    }
    /// Set to `true` to sort in ascending order. This setting has no effect unless `sort` is also set.
    /// Defaults to descending order.
    pub fn ascending(mut self, value: bool) -> Self {
        self.ascending = Some(value);
        self
    }
    /// Query used to filter deployments. Uses the Datadog query syntax. Filtering on an
    /// unsupported field returns a 400 error. For example:
    /// - `status:failed` or `status:done_with_errors`: deployments that need investigation.
    /// - `status:running`: deployments currently in flight.
    /// - `update_type:update_package` or `update_type:update_config_operations`: deployments of a given type.
    pub fn filter(mut self, value: String) -> Self {
        self.filter = Some(value);
        self
    }
}

/// ListFleetTracersOptionalParams is a struct for passing parameters to the method [`FleetAutomationAPI::list_fleet_tracers`]
#[non_exhaustive]
#[derive(Clone, Default, Debug)]
pub struct ListFleetTracersOptionalParams {
    /// Page number for pagination (starts at 0).
    pub page_number: Option<i64>,
    /// Number of results per page (must be greater than 0 and less than or equal to 100).
    pub page_size: Option<i64>,
    /// Attribute to sort by.
    pub sort_attribute: Option<String>,
    /// Sort order (true for descending, false for ascending).
    pub sort_descending: Option<bool>,
    /// Filter string for narrowing down tracer results.
    pub filter: Option<String>,
}

impl ListFleetTracersOptionalParams {
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
    /// Filter string for narrowing down tracer results.
    pub fn filter(mut self, value: String) -> Self {
        self.filter = Some(value);
        self
    }
}

/// CancelFleetDeploymentV2Error is a struct for typed errors of method [`FleetAutomationAPI::cancel_fleet_deployment_v2`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CancelFleetDeploymentV2Error {
    APIErrorResponse(crate::datadogV2::model::APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// CreateFleetDeploymentConfigureV2Error is a struct for typed errors of method [`FleetAutomationAPI::create_fleet_deployment_configure_v2`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CreateFleetDeploymentConfigureV2Error {
    APIErrorResponse(crate::datadogV2::model::APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// CreateFleetDeploymentUpgradeV2Error is a struct for typed errors of method [`FleetAutomationAPI::create_fleet_deployment_upgrade_v2`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CreateFleetDeploymentUpgradeV2Error {
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

/// GetFleetAgentDetailV2Error is a struct for typed errors of method [`FleetAutomationAPI::get_fleet_agent_detail_v2`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetFleetAgentDetailV2Error {
    APIErrorResponse(crate::datadogV2::model::APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// GetFleetDeploymentV2Error is a struct for typed errors of method [`FleetAutomationAPI::get_fleet_deployment_v2`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetFleetDeploymentV2Error {
    APIErrorResponse(crate::datadogV2::model::APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// GetFleetScheduleV2Error is a struct for typed errors of method [`FleetAutomationAPI::get_fleet_schedule_v2`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetFleetScheduleV2Error {
    APIErrorResponse(crate::datadogV2::model::APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// ListFleetAgentTracersError is a struct for typed errors of method [`FleetAutomationAPI::list_fleet_agent_tracers`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ListFleetAgentTracersError {
    APIErrorResponse(crate::datadogV2::model::APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// ListFleetAgentVersionsV2Error is a struct for typed errors of method [`FleetAutomationAPI::list_fleet_agent_versions_v2`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ListFleetAgentVersionsV2Error {
    APIErrorResponse(crate::datadogV2::model::APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// ListFleetAgentsV2Error is a struct for typed errors of method [`FleetAutomationAPI::list_fleet_agents_v2`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ListFleetAgentsV2Error {
    APIErrorResponse(crate::datadogV2::model::APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// ListFleetDeploymentsV2Error is a struct for typed errors of method [`FleetAutomationAPI::list_fleet_deployments_v2`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ListFleetDeploymentsV2Error {
    APIErrorResponse(crate::datadogV2::model::APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// ListFleetSchedulesV2Error is a struct for typed errors of method [`FleetAutomationAPI::list_fleet_schedules_v2`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ListFleetSchedulesV2Error {
    APIErrorResponse(crate::datadogV2::model::APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// ListFleetTracersError is a struct for typed errors of method [`FleetAutomationAPI::list_fleet_tracers`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ListFleetTracersError {
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

    /// Cancel an active deployment and stop all pending operations.
    /// When you cancel a deployment:
    /// - All pending operations on hosts that haven't started yet are stopped.
    /// - Operations currently in progress on hosts may complete or be interrupted, depending on their current status.
    /// - Configuration changes or package upgrades already applied to hosts are not rolled back.
    ///
    /// After cancellation, you can view the final state of the deployment using the GET endpoint to see which hosts
    /// were successfully updated before the cancellation.
    ///
    /// Only deployments with a `pending` or `running` status can be canceled. Returns a 400 if the deployment is not in a cancelable status. Returns a 404 if no deployment matches the specified ID or if you do not have access to it.
    pub async fn cancel_fleet_deployment_v2(
        &self,
        deployment_id: String,
    ) -> Result<
        crate::datadogV2::model::FleetDeploymentV2CancelResponse,
        datadog::Error<CancelFleetDeploymentV2Error>,
    > {
        match self
            .cancel_fleet_deployment_v2_with_http_info(deployment_id)
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

    /// Cancel an active deployment and stop all pending operations.
    /// When you cancel a deployment:
    /// - All pending operations on hosts that haven't started yet are stopped.
    /// - Operations currently in progress on hosts may complete or be interrupted, depending on their current status.
    /// - Configuration changes or package upgrades already applied to hosts are not rolled back.
    ///
    /// After cancellation, you can view the final state of the deployment using the GET endpoint to see which hosts
    /// were successfully updated before the cancellation.
    ///
    /// Only deployments with a `pending` or `running` status can be canceled. Returns a 400 if the deployment is not in a cancelable status. Returns a 404 if no deployment matches the specified ID or if you do not have access to it.
    pub async fn cancel_fleet_deployment_v2_with_http_info(
        &self,
        deployment_id: String,
    ) -> Result<
        datadog::ResponseContent<crate::datadogV2::model::FleetDeploymentV2CancelResponse>,
        datadog::Error<CancelFleetDeploymentV2Error>,
    > {
        let local_configuration = &self.config;
        let local_operation_id = "v2.cancel_fleet_deployment_v2";

        let local_client = &self.client;

        let local_uri_str = format!(
            "{}/api/v2/fleet/deployments/{deployment_id}/cancel",
            local_configuration.get_operation_host(local_operation_id),
            deployment_id = datadog::urlencode(deployment_id)
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
            match serde_json::from_str::<crate::datadogV2::model::FleetDeploymentV2CancelResponse>(
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
            let local_entity: Option<CancelFleetDeploymentV2Error> =
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
    ///   creating the file if it doesn't exist.
    /// - `delete`: Removes the specified configuration file from the target hosts.
    ///
    /// You can optionally use `target_packages` to apply the configuration change only to specific package versions.
    ///
    /// The deployment is created and started automatically. You can specify multiple configuration
    /// operations to execute in order on each target host. Use the filter query to target
    /// specific hosts using the Datadog query syntax.
    ///
    /// Set `dry_run` to `true` to validate the configuration and resolve target hosts and packages without deploying anything. A dry run returns a 200 with the validation result instead of creating and starting a deployment.
    ///
    /// Returns a 400 if `filter_query` or `config_operations` is missing, a target package is missing a name or version or cannot be resolved, the configuration fails validation, or the filter query does not match any host eligible for the deployment.
    pub async fn create_fleet_deployment_configure_v2(
        &self,
        body: crate::datadogV2::model::FleetDeploymentConfigureV2CreateRequest,
    ) -> Result<
        crate::datadogV2::model::FleetDeploymentConfigureV2DryRunResponse,
        datadog::Error<CreateFleetDeploymentConfigureV2Error>,
    > {
        match self
            .create_fleet_deployment_configure_v2_with_http_info(body)
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
    ///   creating the file if it doesn't exist.
    /// - `delete`: Removes the specified configuration file from the target hosts.
    ///
    /// You can optionally use `target_packages` to apply the configuration change only to specific package versions.
    ///
    /// The deployment is created and started automatically. You can specify multiple configuration
    /// operations to execute in order on each target host. Use the filter query to target
    /// specific hosts using the Datadog query syntax.
    ///
    /// Set `dry_run` to `true` to validate the configuration and resolve target hosts and packages without deploying anything. A dry run returns a 200 with the validation result instead of creating and starting a deployment.
    ///
    /// Returns a 400 if `filter_query` or `config_operations` is missing, a target package is missing a name or version or cannot be resolved, the configuration fails validation, or the filter query does not match any host eligible for the deployment.
    pub async fn create_fleet_deployment_configure_v2_with_http_info(
        &self,
        body: crate::datadogV2::model::FleetDeploymentConfigureV2CreateRequest,
    ) -> Result<
        datadog::ResponseContent<crate::datadogV2::model::FleetDeploymentConfigureV2DryRunResponse>,
        datadog::Error<CreateFleetDeploymentConfigureV2Error>,
    > {
        let local_configuration = &self.config;
        let local_operation_id = "v2.create_fleet_deployment_configure_v2";

        let local_client = &self.client;

        let local_uri_str = format!(
            "{}/api/v2/fleet/deployments/configure",
            local_configuration.get_operation_host(local_operation_id)
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
            match serde_json::from_str::<
                crate::datadogV2::model::FleetDeploymentConfigureV2DryRunResponse,
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
            let local_entity: Option<CreateFleetDeploymentConfigureV2Error> =
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
    /// The deployment is created and started automatically. The system:
    /// 1. Identifies all hosts matching the filter query.
    /// 2. Validates that the specified version is available.
    /// 3. Begins rolling out the package upgrade to the target hosts.
    ///
    /// Returns a 400 if `filter_query` or `target_packages` is missing, a target package is missing a name or version, or the filter query does not match any host eligible for the upgrade. Returns a 409 if a conflicting upgrade is already running on one or more target hosts.
    pub async fn create_fleet_deployment_upgrade_v2(
        &self,
        body: crate::datadogV2::model::FleetDeploymentPackageUpgradeV2CreateRequest,
    ) -> Result<
        crate::datadogV2::model::FleetDeploymentV2CreateResponse,
        datadog::Error<CreateFleetDeploymentUpgradeV2Error>,
    > {
        match self
            .create_fleet_deployment_upgrade_v2_with_http_info(body)
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
    /// The deployment is created and started automatically. The system:
    /// 1. Identifies all hosts matching the filter query.
    /// 2. Validates that the specified version is available.
    /// 3. Begins rolling out the package upgrade to the target hosts.
    ///
    /// Returns a 400 if `filter_query` or `target_packages` is missing, a target package is missing a name or version, or the filter query does not match any host eligible for the upgrade. Returns a 409 if a conflicting upgrade is already running on one or more target hosts.
    pub async fn create_fleet_deployment_upgrade_v2_with_http_info(
        &self,
        body: crate::datadogV2::model::FleetDeploymentPackageUpgradeV2CreateRequest,
    ) -> Result<
        datadog::ResponseContent<crate::datadogV2::model::FleetDeploymentV2CreateResponse>,
        datadog::Error<CreateFleetDeploymentUpgradeV2Error>,
    > {
        let local_configuration = &self.config;
        let local_operation_id = "v2.create_fleet_deployment_upgrade_v2";

        let local_client = &self.client;

        let local_uri_str = format!(
            "{}/api/v2/fleet/deployments/upgrade",
            local_configuration.get_operation_host(local_operation_id)
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
            match serde_json::from_str::<crate::datadogV2::model::FleetDeploymentV2CreateResponse>(
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
            let local_entity: Option<CreateFleetDeploymentUpgradeV2Error> =
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
        let local_operation_id = "v2.create_fleet_schedule";
        if local_configuration.is_unstable_operation_enabled(local_operation_id) {
            warn!("Using unstable operation {local_operation_id}");
        } else {
            let local_error = datadog::UnstableOperationDisabledError {
                msg: "Operation 'v2.create_fleet_schedule' is not enabled".to_string(),
            };
            return Err(datadog::Error::UnstableOperationDisabledError(local_error));
        }

        let local_client = &self.client;

        let local_uri_str = format!(
            "{}/api/unstable/fleet/schedules",
            local_configuration.get_operation_host(local_operation_id)
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
        let local_operation_id = "v2.delete_fleet_schedule";
        if local_configuration.is_unstable_operation_enabled(local_operation_id) {
            warn!("Using unstable operation {local_operation_id}");
        } else {
            let local_error = datadog::UnstableOperationDisabledError {
                msg: "Operation 'v2.delete_fleet_schedule' is not enabled".to_string(),
            };
            return Err(datadog::Error::UnstableOperationDisabledError(local_error));
        }

        let local_client = &self.client;

        let local_uri_str = format!(
            "{}/api/unstable/fleet/schedules/{id}",
            local_configuration.get_operation_host(local_operation_id),
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
    ///
    /// By default, only `agent_infos` is returned. Use the `include` query parameter to
    /// request additional data: `integrations` and/or `configuration_files`.
    pub async fn get_fleet_agent_detail_v2(
        &self,
        agent_key: String,
        params: GetFleetAgentDetailV2OptionalParams,
    ) -> Result<
        crate::datadogV2::model::FleetAgentDetailV2Response,
        datadog::Error<GetFleetAgentDetailV2Error>,
    > {
        match self
            .get_fleet_agent_detail_v2_with_http_info(agent_key, params)
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

    /// Retrieve detailed information about a specific Datadog Agent.
    ///
    /// By default, only `agent_infos` is returned. Use the `include` query parameter to
    /// request additional data: `integrations` and/or `configuration_files`.
    pub async fn get_fleet_agent_detail_v2_with_http_info(
        &self,
        agent_key: String,
        params: GetFleetAgentDetailV2OptionalParams,
    ) -> Result<
        datadog::ResponseContent<crate::datadogV2::model::FleetAgentDetailV2Response>,
        datadog::Error<GetFleetAgentDetailV2Error>,
    > {
        let local_configuration = &self.config;
        let local_operation_id = "v2.get_fleet_agent_detail_v2";

        // unbox and build optional parameters
        let include = params.include;

        let local_client = &self.client;

        let local_uri_str = format!(
            "{}/api/v2/fleet/agents/{agent_key}",
            local_configuration.get_operation_host(local_operation_id),
            agent_key = datadog::urlencode(agent_key)
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
            match serde_json::from_str::<crate::datadogV2::model::FleetAgentDetailV2Response>(
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
            let local_entity: Option<GetFleetAgentDetailV2Error> =
                serde_json::from_str(&local_content).ok();
            let local_error = datadog::ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(datadog::Error::ResponseError(local_error))
        }
    }

    /// Retrieve detailed information about a specific deployment, including its current status,
    /// configuration operations, and per-host execution status.
    ///
    /// Returns a 404 if no deployment matches the given ID or if you do not have access to it.
    pub async fn get_fleet_deployment_v2(
        &self,
        deployment_id: String,
    ) -> Result<
        crate::datadogV2::model::FleetDeploymentV2DetailResponse,
        datadog::Error<GetFleetDeploymentV2Error>,
    > {
        match self
            .get_fleet_deployment_v2_with_http_info(deployment_id)
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

    /// Retrieve detailed information about a specific deployment, including its current status,
    /// configuration operations, and per-host execution status.
    ///
    /// Returns a 404 if no deployment matches the given ID or if you do not have access to it.
    pub async fn get_fleet_deployment_v2_with_http_info(
        &self,
        deployment_id: String,
    ) -> Result<
        datadog::ResponseContent<crate::datadogV2::model::FleetDeploymentV2DetailResponse>,
        datadog::Error<GetFleetDeploymentV2Error>,
    > {
        let local_configuration = &self.config;
        let local_operation_id = "v2.get_fleet_deployment_v2";

        let local_client = &self.client;

        let local_uri_str = format!(
            "{}/api/v2/fleet/deployments/{deployment_id}",
            local_configuration.get_operation_host(local_operation_id),
            deployment_id = datadog::urlencode(deployment_id)
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
            match serde_json::from_str::<crate::datadogV2::model::FleetDeploymentV2DetailResponse>(
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
            let local_entity: Option<GetFleetDeploymentV2Error> =
                serde_json::from_str(&local_content).ok();
            let local_error = datadog::ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(datadog::Error::ResponseError(local_error))
        }
    }

    /// Retrieve detailed information about a specific schedule by its unique identifier.
    pub async fn get_fleet_schedule_v2(
        &self,
        id: String,
    ) -> Result<
        crate::datadogV2::model::FleetScheduleV2Response,
        datadog::Error<GetFleetScheduleV2Error>,
    > {
        match self.get_fleet_schedule_v2_with_http_info(id).await {
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

    /// Retrieve detailed information about a specific schedule by its unique identifier.
    pub async fn get_fleet_schedule_v2_with_http_info(
        &self,
        id: String,
    ) -> Result<
        datadog::ResponseContent<crate::datadogV2::model::FleetScheduleV2Response>,
        datadog::Error<GetFleetScheduleV2Error>,
    > {
        let local_configuration = &self.config;
        let local_operation_id = "v2.get_fleet_schedule_v2";

        let local_client = &self.client;

        let local_uri_str = format!(
            "{}/api/v2/fleet/schedules/{id}",
            local_configuration.get_operation_host(local_operation_id),
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
            match serde_json::from_str::<crate::datadogV2::model::FleetScheduleV2Response>(
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
            let local_entity: Option<GetFleetScheduleV2Error> =
                serde_json::from_str(&local_content).ok();
            let local_error = datadog::ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(datadog::Error::ResponseError(local_error))
        }
    }

    /// Retrieve a paginated list of tracers for a specific agent.
    ///
    /// This endpoint returns tracers associated with a given agent key, identified by the
    /// agent's hostname. Use this to discover telemetry-derived service names for a particular host.
    pub async fn list_fleet_agent_tracers(
        &self,
        agent_key: String,
        params: ListFleetAgentTracersOptionalParams,
    ) -> Result<
        crate::datadogV2::model::FleetTracersResponse,
        datadog::Error<ListFleetAgentTracersError>,
    > {
        match self
            .list_fleet_agent_tracers_with_http_info(agent_key, params)
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

    /// Retrieve a paginated list of tracers for a specific agent.
    ///
    /// This endpoint returns tracers associated with a given agent key, identified by the
    /// agent's hostname. Use this to discover telemetry-derived service names for a particular host.
    pub async fn list_fleet_agent_tracers_with_http_info(
        &self,
        agent_key: String,
        params: ListFleetAgentTracersOptionalParams,
    ) -> Result<
        datadog::ResponseContent<crate::datadogV2::model::FleetTracersResponse>,
        datadog::Error<ListFleetAgentTracersError>,
    > {
        let local_configuration = &self.config;
        let local_operation_id = "v2.list_fleet_agent_tracers";
        if local_configuration.is_unstable_operation_enabled(local_operation_id) {
            warn!("Using unstable operation {local_operation_id}");
        } else {
            let local_error = datadog::UnstableOperationDisabledError {
                msg: "Operation 'v2.list_fleet_agent_tracers' is not enabled".to_string(),
            };
            return Err(datadog::Error::UnstableOperationDisabledError(local_error));
        }

        // unbox and build optional parameters
        let page_number = params.page_number;
        let page_size = params.page_size;
        let sort_attribute = params.sort_attribute;
        let sort_descending = params.sort_descending;

        let local_client = &self.client;

        let local_uri_str = format!(
            "{}/api/unstable/fleet/agents/{agent_key}/tracers",
            local_configuration.get_operation_host(local_operation_id),
            agent_key = datadog::urlencode(agent_key)
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
            match serde_json::from_str::<crate::datadogV2::model::FleetTracersResponse>(
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
            let local_entity: Option<ListFleetAgentTracersError> =
                serde_json::from_str(&local_content).ok();
            let local_error = datadog::ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(datadog::Error::ResponseError(local_error))
        }
    }

    /// Retrieve the list of Datadog Agent versions available for deployment.
    ///
    /// Returns `200` with an empty `data` array if the Agent package exists in the catalog
    /// but has no available versions, and `404` only if the Agent package itself is absent
    /// from the catalog.
    pub async fn list_fleet_agent_versions_v2(
        &self,
    ) -> Result<
        crate::datadogV2::model::FleetAgentVersionsV2Response,
        datadog::Error<ListFleetAgentVersionsV2Error>,
    > {
        match self.list_fleet_agent_versions_v2_with_http_info().await {
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

    /// Retrieve the list of Datadog Agent versions available for deployment.
    ///
    /// Returns `200` with an empty `data` array if the Agent package exists in the catalog
    /// but has no available versions, and `404` only if the Agent package itself is absent
    /// from the catalog.
    pub async fn list_fleet_agent_versions_v2_with_http_info(
        &self,
    ) -> Result<
        datadog::ResponseContent<crate::datadogV2::model::FleetAgentVersionsV2Response>,
        datadog::Error<ListFleetAgentVersionsV2Error>,
    > {
        let local_configuration = &self.config;
        let local_operation_id = "v2.list_fleet_agent_versions_v2";

        let local_client = &self.client;

        let local_uri_str = format!(
            "{}/api/v2/fleet/agent_versions",
            local_configuration.get_operation_host(local_operation_id)
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
            match serde_json::from_str::<crate::datadogV2::model::FleetAgentVersionsV2Response>(
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
            let local_entity: Option<ListFleetAgentVersionsV2Error> =
                serde_json::from_str(&local_content).ok();
            let local_error = datadog::ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(datadog::Error::ResponseError(local_error))
        }
    }

    /// Retrieve a paginated list of Datadog Agents.
    ///
    /// Returns agents with support for pagination, sorting, and filtering.
    /// Use `page_number` and `page_size` to navigate pages, `filter` to narrow by field values,
    /// and `tags` to filter by agent tags.
    pub async fn list_fleet_agents_v2(
        &self,
        params: ListFleetAgentsV2OptionalParams,
    ) -> Result<
        crate::datadogV2::model::FleetAgentsV2Response,
        datadog::Error<ListFleetAgentsV2Error>,
    > {
        match self.list_fleet_agents_v2_with_http_info(params).await {
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

    /// Retrieve a paginated list of Datadog Agents.
    ///
    /// Returns agents with support for pagination, sorting, and filtering.
    /// Use `page_number` and `page_size` to navigate pages, `filter` to narrow by field values,
    /// and `tags` to filter by agent tags.
    pub async fn list_fleet_agents_v2_with_http_info(
        &self,
        params: ListFleetAgentsV2OptionalParams,
    ) -> Result<
        datadog::ResponseContent<crate::datadogV2::model::FleetAgentsV2Response>,
        datadog::Error<ListFleetAgentsV2Error>,
    > {
        let local_configuration = &self.config;
        let local_operation_id = "v2.list_fleet_agents_v2";

        // unbox and build optional parameters
        let page_number = params.page_number;
        let page_size = params.page_size;
        let filter = params.filter;
        let tags = params.tags;
        let sort_attribute = params.sort_attribute;
        let sort_descending = params.sort_descending;

        let local_client = &self.client;

        let local_uri_str = format!(
            "{}/api/v2/fleet/agents",
            local_configuration.get_operation_host(local_operation_id)
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
        if let Some(ref local_query_param) = filter {
            local_req_builder =
                local_req_builder.query(&[("filter", &local_query_param.to_string())]);
        };
        if let Some(ref local_query_param) = tags {
            local_req_builder =
                local_req_builder.query(&[("tags", &local_query_param.to_string())]);
        };
        if let Some(ref local_query_param) = sort_attribute {
            local_req_builder =
                local_req_builder.query(&[("sort_attribute", &local_query_param.to_string())]);
        };
        if let Some(ref local_query_param) = sort_descending {
            local_req_builder =
                local_req_builder.query(&[("sort_descending", &local_query_param.to_string())]);
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
            match serde_json::from_str::<crate::datadogV2::model::FleetAgentsV2Response>(
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
            let local_entity: Option<ListFleetAgentsV2Error> =
                serde_json::from_str(&local_content).ok();
            let local_error = datadog::ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(datadog::Error::ResponseError(local_error))
        }
    }

    /// Retrieve a paginated list of all deployments for fleet automation.
    pub async fn list_fleet_deployments_v2(
        &self,
        params: ListFleetDeploymentsV2OptionalParams,
    ) -> Result<
        crate::datadogV2::model::FleetDeploymentsV2Response,
        datadog::Error<ListFleetDeploymentsV2Error>,
    > {
        match self.list_fleet_deployments_v2_with_http_info(params).await {
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

    /// Retrieve a paginated list of all deployments for fleet automation.
    pub async fn list_fleet_deployments_v2_with_http_info(
        &self,
        params: ListFleetDeploymentsV2OptionalParams,
    ) -> Result<
        datadog::ResponseContent<crate::datadogV2::model::FleetDeploymentsV2Response>,
        datadog::Error<ListFleetDeploymentsV2Error>,
    > {
        let local_configuration = &self.config;
        let local_operation_id = "v2.list_fleet_deployments_v2";

        // unbox and build optional parameters
        let page_size = params.page_size;
        let page_number = params.page_number;
        let sort = params.sort;
        let ascending = params.ascending;
        let filter = params.filter;

        let local_client = &self.client;

        let local_uri_str = format!(
            "{}/api/v2/fleet/deployments",
            local_configuration.get_operation_host(local_operation_id)
        );
        let mut local_req_builder =
            local_client.request(reqwest::Method::GET, local_uri_str.as_str());

        if let Some(ref local_query_param) = page_size {
            local_req_builder =
                local_req_builder.query(&[("page_size", &local_query_param.to_string())]);
        };
        if let Some(ref local_query_param) = page_number {
            local_req_builder =
                local_req_builder.query(&[("page_number", &local_query_param.to_string())]);
        };
        if let Some(ref local_query_param) = sort {
            local_req_builder =
                local_req_builder.query(&[("sort", &local_query_param.to_string())]);
        };
        if let Some(ref local_query_param) = ascending {
            local_req_builder =
                local_req_builder.query(&[("ascending", &local_query_param.to_string())]);
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
            match serde_json::from_str::<crate::datadogV2::model::FleetDeploymentsV2Response>(
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
            let local_entity: Option<ListFleetDeploymentsV2Error> =
                serde_json::from_str(&local_content).ok();
            let local_error = datadog::ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(datadog::Error::ResponseError(local_error))
        }
    }

    /// Retrieve all upgrade schedules for the organization.
    ///
    /// Schedules automate package upgrades by defining maintenance windows and recurrence rules.
    /// Each schedule automatically creates deployments based on its configuration.
    pub async fn list_fleet_schedules_v2(
        &self,
    ) -> Result<
        crate::datadogV2::model::FleetSchedulesV2Response,
        datadog::Error<ListFleetSchedulesV2Error>,
    > {
        match self.list_fleet_schedules_v2_with_http_info().await {
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

    /// Retrieve all upgrade schedules for the organization.
    ///
    /// Schedules automate package upgrades by defining maintenance windows and recurrence rules.
    /// Each schedule automatically creates deployments based on its configuration.
    pub async fn list_fleet_schedules_v2_with_http_info(
        &self,
    ) -> Result<
        datadog::ResponseContent<crate::datadogV2::model::FleetSchedulesV2Response>,
        datadog::Error<ListFleetSchedulesV2Error>,
    > {
        let local_configuration = &self.config;
        let local_operation_id = "v2.list_fleet_schedules_v2";

        let local_client = &self.client;

        let local_uri_str = format!(
            "{}/api/v2/fleet/schedules",
            local_configuration.get_operation_host(local_operation_id)
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
            match serde_json::from_str::<crate::datadogV2::model::FleetSchedulesV2Response>(
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
            let local_entity: Option<ListFleetSchedulesV2Error> =
                serde_json::from_str(&local_content).ok();
            let local_error = datadog::ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(datadog::Error::ResponseError(local_error))
        }
    }

    /// Retrieve a paginated list of all fleet tracers.
    ///
    /// This endpoint returns telemetry-derived service names from the SDK telemetry pipeline.
    /// These names may differ from span-derived names in APM and are useful for querying
    /// service library configurations.
    /// Use the `page_number` and `page_size` query parameters to paginate through results.
    pub async fn list_fleet_tracers(
        &self,
        params: ListFleetTracersOptionalParams,
    ) -> Result<crate::datadogV2::model::FleetTracersResponse, datadog::Error<ListFleetTracersError>>
    {
        match self.list_fleet_tracers_with_http_info(params).await {
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

    /// Retrieve a paginated list of all fleet tracers.
    ///
    /// This endpoint returns telemetry-derived service names from the SDK telemetry pipeline.
    /// These names may differ from span-derived names in APM and are useful for querying
    /// service library configurations.
    /// Use the `page_number` and `page_size` query parameters to paginate through results.
    pub async fn list_fleet_tracers_with_http_info(
        &self,
        params: ListFleetTracersOptionalParams,
    ) -> Result<
        datadog::ResponseContent<crate::datadogV2::model::FleetTracersResponse>,
        datadog::Error<ListFleetTracersError>,
    > {
        let local_configuration = &self.config;
        let local_operation_id = "v2.list_fleet_tracers";
        if local_configuration.is_unstable_operation_enabled(local_operation_id) {
            warn!("Using unstable operation {local_operation_id}");
        } else {
            let local_error = datadog::UnstableOperationDisabledError {
                msg: "Operation 'v2.list_fleet_tracers' is not enabled".to_string(),
            };
            return Err(datadog::Error::UnstableOperationDisabledError(local_error));
        }

        // unbox and build optional parameters
        let page_number = params.page_number;
        let page_size = params.page_size;
        let sort_attribute = params.sort_attribute;
        let sort_descending = params.sort_descending;
        let filter = params.filter;

        let local_client = &self.client;

        let local_uri_str = format!(
            "{}/api/unstable/fleet/tracers",
            local_configuration.get_operation_host(local_operation_id)
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
            match serde_json::from_str::<crate::datadogV2::model::FleetTracersResponse>(
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
            let local_entity: Option<ListFleetTracersError> =
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
        let local_operation_id = "v2.trigger_fleet_schedule";
        if local_configuration.is_unstable_operation_enabled(local_operation_id) {
            warn!("Using unstable operation {local_operation_id}");
        } else {
            let local_error = datadog::UnstableOperationDisabledError {
                msg: "Operation 'v2.trigger_fleet_schedule' is not enabled".to_string(),
            };
            return Err(datadog::Error::UnstableOperationDisabledError(local_error));
        }

        let local_client = &self.client;

        let local_uri_str = format!(
            "{}/api/unstable/fleet/schedules/{id}/trigger",
            local_configuration.get_operation_host(local_operation_id),
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
        let local_operation_id = "v2.update_fleet_schedule";
        if local_configuration.is_unstable_operation_enabled(local_operation_id) {
            warn!("Using unstable operation {local_operation_id}");
        } else {
            let local_error = datadog::UnstableOperationDisabledError {
                msg: "Operation 'v2.update_fleet_schedule' is not enabled".to_string(),
            };
            return Err(datadog::Error::UnstableOperationDisabledError(local_error));
        }

        let local_client = &self.client;

        let local_uri_str = format!(
            "{}/api/unstable/fleet/schedules/{id}",
            local_configuration.get_operation_host(local_operation_id),
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
