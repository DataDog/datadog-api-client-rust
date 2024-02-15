// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use crate::datadog::*;
use reqwest;
use serde::{Deserialize, Serialize};

/// DeleteRestrictionPolicyError is a struct for typed errors of method [`RestrictionPoliciesAPI::delete_restriction_policy`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DeleteRestrictionPolicyError {
    Status400(Option<crate::datadogV2::model::APIErrorResponse>),
    Status403(Option<crate::datadogV2::model::APIErrorResponse>),
    Status429(Option<crate::datadogV2::model::APIErrorResponse>),
    UnknownValue(serde_json::Value),
}

/// GetRestrictionPolicyError is a struct for typed errors of method [`RestrictionPoliciesAPI::get_restriction_policy`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetRestrictionPolicyError {
    Status400(Option<crate::datadogV2::model::APIErrorResponse>),
    Status403(Option<crate::datadogV2::model::APIErrorResponse>),
    Status429(Option<crate::datadogV2::model::APIErrorResponse>),
    UnknownValue(serde_json::Value),
}

/// UpdateRestrictionPolicyError is a struct for typed errors of method [`RestrictionPoliciesAPI::update_restriction_policy`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum UpdateRestrictionPolicyError {
    Status400(Option<crate::datadogV2::model::APIErrorResponse>),
    Status403(Option<crate::datadogV2::model::APIErrorResponse>),
    Status429(Option<crate::datadogV2::model::APIErrorResponse>),
    UnknownValue(serde_json::Value),
}

#[derive(Debug, Clone)]
pub struct RestrictionPoliciesAPI {
    config: configuration::Configuration,
}

impl Default for RestrictionPoliciesAPI {
    fn default() -> Self {
        Self {
            config: configuration::Configuration::new(),
        }
    }
}

impl RestrictionPoliciesAPI {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn with_config(config: configuration::Configuration) -> Self {
        Self { config }
    }

    /// Deletes the restriction policy associated with a specified resource.
    pub async fn delete_restriction_policy(
        &self,
        resource_id: String,
    ) -> Result<Option<()>, Error<DeleteRestrictionPolicyError>> {
        match self
            .delete_restriction_policy_with_http_info(resource_id)
            .await
        {
            Ok(response_content) => Ok(response_content.entity),
            Err(err) => Err(err),
        }
    }

    /// Deletes the restriction policy associated with a specified resource.
    pub async fn delete_restriction_policy_with_http_info(
        &self,
        resource_id: String,
    ) -> Result<ResponseContent<()>, Error<DeleteRestrictionPolicyError>> {
        let local_configuration = &self.config;
        let operation_id = "v2.delete_restriction_policy";

        let local_client = &local_configuration.client;

        let local_uri_str = format!(
            "{}/api/v2/restriction_policy/{resource_id}",
            local_configuration.get_operation_host(operation_id),
            resource_id = urlencode(resource_id)
        );
        let mut local_req_builder =
            local_client.request(reqwest::Method::DELETE, local_uri_str.as_str());

        // build user agent
        local_req_builder = local_req_builder.header(
            reqwest::header::USER_AGENT,
            local_configuration.user_agent.clone(),
        );

        // build auth
        if let Some(ref local_key) = local_configuration.api_key {
            local_req_builder = local_req_builder.header("DD-API-KEY", local_key);
        };
        if let Some(ref local_key) = local_configuration.app_key {
            local_req_builder = local_req_builder.header("DD-APPLICATION-KEY", local_key);
        };

        let local_req = local_req_builder.build()?;
        let local_resp = local_client.execute(local_req).await?;

        let local_status = local_resp.status();
        let local_content = local_resp.text().await?;

        if !local_status.is_client_error() && !local_status.is_server_error() {
            Ok(ResponseContent {
                status: local_status,
                content: local_content,
                entity: None,
            })
        } else {
            let local_entity: Option<DeleteRestrictionPolicyError> =
                serde_json::from_str(&local_content).ok();
            let local_error = ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(Error::ResponseError(local_error))
        }
    }

    /// Retrieves the restriction policy associated with a specified resource.
    pub async fn get_restriction_policy(
        &self,
        resource_id: String,
    ) -> Result<
        Option<crate::datadogV2::model::RestrictionPolicyResponse>,
        Error<GetRestrictionPolicyError>,
    > {
        match self
            .get_restriction_policy_with_http_info(resource_id)
            .await
        {
            Ok(response_content) => Ok(response_content.entity),
            Err(err) => Err(err),
        }
    }

    /// Retrieves the restriction policy associated with a specified resource.
    pub async fn get_restriction_policy_with_http_info(
        &self,
        resource_id: String,
    ) -> Result<
        ResponseContent<crate::datadogV2::model::RestrictionPolicyResponse>,
        Error<GetRestrictionPolicyError>,
    > {
        let local_configuration = &self.config;
        let operation_id = "v2.get_restriction_policy";

        let local_client = &local_configuration.client;

        let local_uri_str = format!(
            "{}/api/v2/restriction_policy/{resource_id}",
            local_configuration.get_operation_host(operation_id),
            resource_id = urlencode(resource_id)
        );
        let mut local_req_builder =
            local_client.request(reqwest::Method::GET, local_uri_str.as_str());

        // build user agent
        local_req_builder = local_req_builder.header(
            reqwest::header::USER_AGENT,
            local_configuration.user_agent.clone(),
        );

        // build auth
        if let Some(ref local_key) = local_configuration.api_key {
            local_req_builder = local_req_builder.header("DD-API-KEY", local_key);
        };
        if let Some(ref local_key) = local_configuration.app_key {
            local_req_builder = local_req_builder.header("DD-APPLICATION-KEY", local_key);
        };

        let local_req = local_req_builder.build()?;
        let local_resp = local_client.execute(local_req).await?;

        let local_status = local_resp.status();
        let local_content = local_resp.text().await?;

        if !local_status.is_client_error() && !local_status.is_server_error() {
            let local_entity: Option<crate::datadogV2::model::RestrictionPolicyResponse> =
                serde_json::from_str(&local_content).ok();
            Ok(ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            })
        } else {
            let local_entity: Option<GetRestrictionPolicyError> =
                serde_json::from_str(&local_content).ok();
            let local_error = ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(Error::ResponseError(local_error))
        }
    }

    /// Updates the restriction policy associated with a resource.
    ///
    /// #### Supported resources
    /// Restriction policies can be applied to the following resources:
    /// - Connections: `connection`
    /// - Dashboards: `dashboard`
    /// - Notebooks: `notebook`
    /// - Security Rules: `security-rule`
    /// - Service Level Objectives: `slo`
    ///
    /// #### Supported relations for resources
    /// Resource Type            | Supported Relations
    /// -------------------------|--------------------------
    /// Connections              | `viewer`, `editor`, `resolver`
    /// Dashboards               | `viewer`, `editor`
    /// Notebooks                | `viewer`, `editor`
    /// Security Rules           | `viewer`, `editor`
    /// Service Level Objectives | `viewer`, `editor`
    pub async fn update_restriction_policy(
        &self,
        resource_id: String,
        body: crate::datadogV2::model::RestrictionPolicyUpdateRequest,
    ) -> Result<
        Option<crate::datadogV2::model::RestrictionPolicyResponse>,
        Error<UpdateRestrictionPolicyError>,
    > {
        match self
            .update_restriction_policy_with_http_info(resource_id, body)
            .await
        {
            Ok(response_content) => Ok(response_content.entity),
            Err(err) => Err(err),
        }
    }

    /// Updates the restriction policy associated with a resource.
    ///
    /// #### Supported resources
    /// Restriction policies can be applied to the following resources:
    /// - Connections: `connection`
    /// - Dashboards: `dashboard`
    /// - Notebooks: `notebook`
    /// - Security Rules: `security-rule`
    /// - Service Level Objectives: `slo`
    ///
    /// #### Supported relations for resources
    /// Resource Type            | Supported Relations
    /// -------------------------|--------------------------
    /// Connections              | `viewer`, `editor`, `resolver`
    /// Dashboards               | `viewer`, `editor`
    /// Notebooks                | `viewer`, `editor`
    /// Security Rules           | `viewer`, `editor`
    /// Service Level Objectives | `viewer`, `editor`
    pub async fn update_restriction_policy_with_http_info(
        &self,
        resource_id: String,
        body: crate::datadogV2::model::RestrictionPolicyUpdateRequest,
    ) -> Result<
        ResponseContent<crate::datadogV2::model::RestrictionPolicyResponse>,
        Error<UpdateRestrictionPolicyError>,
    > {
        let local_configuration = &self.config;
        let operation_id = "v2.update_restriction_policy";

        let local_client = &local_configuration.client;

        let local_uri_str = format!(
            "{}/api/v2/restriction_policy/{resource_id}",
            local_configuration.get_operation_host(operation_id),
            resource_id = urlencode(resource_id)
        );
        let mut local_req_builder =
            local_client.request(reqwest::Method::POST, local_uri_str.as_str());

        // build user agent
        local_req_builder = local_req_builder.header(
            reqwest::header::USER_AGENT,
            local_configuration.user_agent.clone(),
        );

        // build auth
        if let Some(ref local_key) = local_configuration.api_key {
            local_req_builder = local_req_builder.header("DD-API-KEY", local_key);
        };
        if let Some(ref local_key) = local_configuration.app_key {
            local_req_builder = local_req_builder.header("DD-APPLICATION-KEY", local_key);
        };

        // build body parameters
        let output = Vec::new();
        let mut ser = serde_json::Serializer::with_formatter(output, DDFormatter);
        if body.serialize(&mut ser).is_ok() {
            local_req_builder = local_req_builder.body(ser.into_inner());
        }

        let local_req = local_req_builder.build()?;
        let local_resp = local_client.execute(local_req).await?;

        let local_status = local_resp.status();
        let local_content = local_resp.text().await?;

        if !local_status.is_client_error() && !local_status.is_server_error() {
            let local_entity: Option<crate::datadogV2::model::RestrictionPolicyResponse> =
                serde_json::from_str(&local_content).ok();
            Ok(ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            })
        } else {
            let local_entity: Option<UpdateRestrictionPolicyError> =
                serde_json::from_str(&local_content).ok();
            let local_error = ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(Error::ResponseError(local_error))
        }
    }
}
