// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

use crate::datadog::*;
use reqwest;
use serde::{Deserialize, Serialize};

/// DeleteRestrictionPolicyParams is a struct for passing parameters to the method [`DeleteRestrictionPolicy`]
#[derive(Clone, Debug)]
pub struct DeleteRestrictionPolicyParams {
    /// Identifier, formatted as `type:id`. Supported types: `connection`, `dashboard`, `notebook`, `security-rule`, `slo`.
    pub resource_id: String,
}

/// GetRestrictionPolicyParams is a struct for passing parameters to the method [`GetRestrictionPolicy`]
#[derive(Clone, Debug)]
pub struct GetRestrictionPolicyParams {
    /// Identifier, formatted as `type:id`. Supported types: `connection`, `dashboard`, `notebook`, `security-rule`, `slo`.
    pub resource_id: String,
}

/// UpdateRestrictionPolicyParams is a struct for passing parameters to the method [`UpdateRestrictionPolicy`]
#[derive(Clone, Debug)]
pub struct UpdateRestrictionPolicyParams {
    /// Identifier, formatted as `type:id`. Supported types: `connection`, `dashboard`, `notebook`, `security-rule`, `slo`.
    pub resource_id: String,
    /// Restriction policy payload
    pub body: crate::datadogV2::model::RestrictionPolicyUpdateRequest,
}

/// DeleteRestrictionPolicyError is a struct for typed errors of method [`DeleteRestrictionPolicy`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DeleteRestrictionPolicyError {
    Status400(Option<crate::datadogV2::model::APIErrorResponse>),
    Status403(Option<crate::datadogV2::model::APIErrorResponse>),
    Status429(Option<crate::datadogV2::model::APIErrorResponse>),
    UnknownValue(serde_json::Value),
}

/// GetRestrictionPolicyError is a struct for typed errors of method [`GetRestrictionPolicy`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetRestrictionPolicyError {
    Status400(Option<crate::datadogV2::model::APIErrorResponse>),
    Status403(Option<crate::datadogV2::model::APIErrorResponse>),
    Status429(Option<crate::datadogV2::model::APIErrorResponse>),
    UnknownValue(serde_json::Value),
}

/// UpdateRestrictionPolicyError is a struct for typed errors of method [`UpdateRestrictionPolicy`]
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
        params: DeleteRestrictionPolicyParams,
    ) -> Result<Option<()>, Error<DeleteRestrictionPolicyError>> {
        match self.delete_restriction_policy_with_http_info(params).await {
            Ok(response_content) => Ok(response_content.entity),
            Err(err) => Err(err),
        }
    }

    /// Deletes the restriction policy associated with a specified resource.
    pub async fn delete_restriction_policy_with_http_info(
        &self,
        params: DeleteRestrictionPolicyParams,
    ) -> Result<ResponseContent<()>, Error<DeleteRestrictionPolicyError>> {
        let local_configuration = &self.config;

        // unbox and build parameters
        let resource_id = params.resource_id;

        let local_client = &local_configuration.client;

        let local_uri_str = format!(
            "{}/api/v2/restriction_policy/{resource_id}",
            local_configuration.base_path,
            resource_id = urlencode(resource_id)
        );
        let mut local_req_builder =
            local_client.request(reqwest::Method::DELETE, local_uri_str.as_str());

        // build user agent
        if let Some(ref local_user_agent) = local_configuration.user_agent {
            local_req_builder =
                local_req_builder.header(reqwest::header::USER_AGENT, local_user_agent.clone());
        }

        // build auth
        if let Some(ref local_apikey) = local_configuration.api_key_auth {
            local_req_builder = local_req_builder.header("DD-API-KEY", local_apikey);
        };
        if let Some(ref local_apikey) = local_configuration.app_key_auth {
            local_req_builder = local_req_builder.header("DD-APPLICATION-KEY", local_apikey);
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
        params: GetRestrictionPolicyParams,
    ) -> Result<
        Option<crate::datadogV2::model::RestrictionPolicyResponse>,
        Error<GetRestrictionPolicyError>,
    > {
        match self.get_restriction_policy_with_http_info(params).await {
            Ok(response_content) => Ok(response_content.entity),
            Err(err) => Err(err),
        }
    }

    /// Retrieves the restriction policy associated with a specified resource.
    pub async fn get_restriction_policy_with_http_info(
        &self,
        params: GetRestrictionPolicyParams,
    ) -> Result<
        ResponseContent<crate::datadogV2::model::RestrictionPolicyResponse>,
        Error<GetRestrictionPolicyError>,
    > {
        let local_configuration = &self.config;

        // unbox and build parameters
        let resource_id = params.resource_id;

        let local_client = &local_configuration.client;

        let local_uri_str = format!(
            "{}/api/v2/restriction_policy/{resource_id}",
            local_configuration.base_path,
            resource_id = urlencode(resource_id)
        );
        let mut local_req_builder =
            local_client.request(reqwest::Method::GET, local_uri_str.as_str());

        // build user agent
        if let Some(ref local_user_agent) = local_configuration.user_agent {
            local_req_builder =
                local_req_builder.header(reqwest::header::USER_AGENT, local_user_agent.clone());
        }

        // build auth
        if let Some(ref local_apikey) = local_configuration.api_key_auth {
            local_req_builder = local_req_builder.header("DD-API-KEY", local_apikey);
        };
        if let Some(ref local_apikey) = local_configuration.app_key_auth {
            local_req_builder = local_req_builder.header("DD-APPLICATION-KEY", local_apikey);
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
        params: UpdateRestrictionPolicyParams,
    ) -> Result<
        Option<crate::datadogV2::model::RestrictionPolicyResponse>,
        Error<UpdateRestrictionPolicyError>,
    > {
        match self.update_restriction_policy_with_http_info(params).await {
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
        params: UpdateRestrictionPolicyParams,
    ) -> Result<
        ResponseContent<crate::datadogV2::model::RestrictionPolicyResponse>,
        Error<UpdateRestrictionPolicyError>,
    > {
        let local_configuration = &self.config;

        // unbox and build parameters
        let resource_id = params.resource_id;
        let body = params.body;

        let local_client = &local_configuration.client;

        let local_uri_str = format!(
            "{}/api/v2/restriction_policy/{resource_id}",
            local_configuration.base_path,
            resource_id = urlencode(resource_id)
        );
        let mut local_req_builder =
            local_client.request(reqwest::Method::POST, local_uri_str.as_str());

        // build user agent
        if let Some(ref local_user_agent) = local_configuration.user_agent {
            local_req_builder =
                local_req_builder.header(reqwest::header::USER_AGENT, local_user_agent.clone());
        }

        // build auth
        if let Some(ref local_apikey) = local_configuration.api_key_auth {
            local_req_builder = local_req_builder.header("DD-API-KEY", local_apikey);
        };
        if let Some(ref local_apikey) = local_configuration.app_key_auth {
            local_req_builder = local_req_builder.header("DD-APPLICATION-KEY", local_apikey);
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
