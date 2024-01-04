// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

use crate::datadog::*;
use reqwest;
use serde::{Deserialize, Serialize};

/// CreateScanningGroupParams is a struct for passing parameters to the method [`CreateScanningGroup`]
#[derive(Clone, Debug)]
pub struct CreateScanningGroupParams {
    pub body: crate::datadogV2::model::SensitiveDataScannerGroupCreateRequest,
}

/// CreateScanningRuleParams is a struct for passing parameters to the method [`CreateScanningRule`]
#[derive(Clone, Debug)]
pub struct CreateScanningRuleParams {
    pub body: crate::datadogV2::model::SensitiveDataScannerRuleCreateRequest,
}

/// DeleteScanningGroupParams is a struct for passing parameters to the method [`DeleteScanningGroup`]
#[derive(Clone, Debug)]
pub struct DeleteScanningGroupParams {
    /// The ID of a group of rules.
    pub group_id: String,
    pub body: crate::datadogV2::model::SensitiveDataScannerGroupDeleteRequest,
}

/// DeleteScanningRuleParams is a struct for passing parameters to the method [`DeleteScanningRule`]
#[derive(Clone, Debug)]
pub struct DeleteScanningRuleParams {
    /// The ID of the rule.
    pub rule_id: String,
    pub body: crate::datadogV2::model::SensitiveDataScannerRuleDeleteRequest,
}

/// ReorderScanningGroupsParams is a struct for passing parameters to the method [`ReorderScanningGroups`]
#[derive(Clone, Debug)]
pub struct ReorderScanningGroupsParams {
    pub body: crate::datadogV2::model::SensitiveDataScannerConfigRequest,
}

/// UpdateScanningGroupParams is a struct for passing parameters to the method [`UpdateScanningGroup`]
#[derive(Clone, Debug)]
pub struct UpdateScanningGroupParams {
    /// The ID of a group of rules.
    pub group_id: String,
    pub body: crate::datadogV2::model::SensitiveDataScannerGroupUpdateRequest,
}

/// UpdateScanningRuleParams is a struct for passing parameters to the method [`UpdateScanningRule`]
#[derive(Clone, Debug)]
pub struct UpdateScanningRuleParams {
    /// The ID of the rule.
    pub rule_id: String,
    pub body: crate::datadogV2::model::SensitiveDataScannerRuleUpdateRequest,
}

/// CreateScanningGroupError is a struct for typed errors of method [`CreateScanningGroup`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CreateScanningGroupError {
    Status400(Option<crate::datadogV2::model::APIErrorResponse>),
    Status403(Option<crate::datadogV2::model::APIErrorResponse>),
    Status429(Option<crate::datadogV2::model::APIErrorResponse>),
    UnknownValue(serde_json::Value),
}

/// CreateScanningRuleError is a struct for typed errors of method [`CreateScanningRule`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CreateScanningRuleError {
    Status400(Option<crate::datadogV2::model::APIErrorResponse>),
    Status403(Option<crate::datadogV2::model::APIErrorResponse>),
    Status429(Option<crate::datadogV2::model::APIErrorResponse>),
    UnknownValue(serde_json::Value),
}

/// DeleteScanningGroupError is a struct for typed errors of method [`DeleteScanningGroup`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DeleteScanningGroupError {
    Status400(Option<crate::datadogV2::model::APIErrorResponse>),
    Status403(Option<crate::datadogV2::model::APIErrorResponse>),
    Status404(Option<crate::datadogV2::model::APIErrorResponse>),
    Status429(Option<crate::datadogV2::model::APIErrorResponse>),
    UnknownValue(serde_json::Value),
}

/// DeleteScanningRuleError is a struct for typed errors of method [`DeleteScanningRule`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DeleteScanningRuleError {
    Status400(Option<crate::datadogV2::model::APIErrorResponse>),
    Status403(Option<crate::datadogV2::model::APIErrorResponse>),
    Status404(Option<crate::datadogV2::model::APIErrorResponse>),
    Status429(Option<crate::datadogV2::model::APIErrorResponse>),
    UnknownValue(serde_json::Value),
}

/// ListScanningGroupsError is a struct for typed errors of method [`ListScanningGroups`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ListScanningGroupsError {
    Status400(Option<crate::datadogV2::model::APIErrorResponse>),
    Status403(Option<crate::datadogV2::model::APIErrorResponse>),
    Status429(Option<crate::datadogV2::model::APIErrorResponse>),
    UnknownValue(serde_json::Value),
}

/// ListStandardPatternsError is a struct for typed errors of method [`ListStandardPatterns`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ListStandardPatternsError {
    Status400(Option<crate::datadogV2::model::APIErrorResponse>),
    Status403(Option<crate::datadogV2::model::APIErrorResponse>),
    Status429(Option<crate::datadogV2::model::APIErrorResponse>),
    UnknownValue(serde_json::Value),
}

/// ReorderScanningGroupsError is a struct for typed errors of method [`ReorderScanningGroups`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ReorderScanningGroupsError {
    Status400(Option<crate::datadogV2::model::APIErrorResponse>),
    Status403(Option<crate::datadogV2::model::APIErrorResponse>),
    Status429(Option<crate::datadogV2::model::APIErrorResponse>),
    UnknownValue(serde_json::Value),
}

/// UpdateScanningGroupError is a struct for typed errors of method [`UpdateScanningGroup`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum UpdateScanningGroupError {
    Status400(Option<crate::datadogV2::model::APIErrorResponse>),
    Status403(Option<crate::datadogV2::model::APIErrorResponse>),
    Status404(Option<crate::datadogV2::model::APIErrorResponse>),
    Status429(Option<crate::datadogV2::model::APIErrorResponse>),
    UnknownValue(serde_json::Value),
}

/// UpdateScanningRuleError is a struct for typed errors of method [`UpdateScanningRule`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum UpdateScanningRuleError {
    Status400(Option<crate::datadogV2::model::APIErrorResponse>),
    Status403(Option<crate::datadogV2::model::APIErrorResponse>),
    Status404(Option<crate::datadogV2::model::APIErrorResponse>),
    Status429(Option<crate::datadogV2::model::APIErrorResponse>),
    UnknownValue(serde_json::Value),
}

#[derive(Debug, Clone)]
pub struct SensitiveDataScannerAPI {
    config: configuration::Configuration,
}

impl Default for SensitiveDataScannerAPI {
    fn default() -> Self {
        Self {
            config: configuration::Configuration::new(),
        }
    }
}

impl SensitiveDataScannerAPI {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn with_config(config: configuration::Configuration) -> Self {
        Self { config }
    }

    /// Create a scanning group.
    /// The request MAY include a configuration relationship.
    /// A rules relationship can be omitted entirely, but if it is included it MUST be
    /// null or an empty array (rules cannot be created at the same time).
    /// The new group will be ordered last within the configuration.
    pub async fn create_scanning_group(
        &self,
        params: CreateScanningGroupParams,
    ) -> Result<
        Option<crate::datadogV2::model::SensitiveDataScannerCreateGroupResponse>,
        Error<CreateScanningGroupError>,
    > {
        match self.create_scanning_group_with_http_info(params).await {
            Ok(response_content) => Ok(response_content.entity),
            Err(err) => Err(err),
        }
    }

    /// Create a scanning group.
    /// The request MAY include a configuration relationship.
    /// A rules relationship can be omitted entirely, but if it is included it MUST be
    /// null or an empty array (rules cannot be created at the same time).
    /// The new group will be ordered last within the configuration.
    pub async fn create_scanning_group_with_http_info(
        &self,
        params: CreateScanningGroupParams,
    ) -> Result<
        ResponseContent<crate::datadogV2::model::SensitiveDataScannerCreateGroupResponse>,
        Error<CreateScanningGroupError>,
    > {
        let local_configuration = &self.config;

        // unbox and build parameters
        let body = params.body;

        let local_client = &local_configuration.client;

        let local_uri_str = format!(
            "{}/api/v2/sensitive-data-scanner/config/groups",
            local_configuration.base_path
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
            let local_entity: Option<
                crate::datadogV2::model::SensitiveDataScannerCreateGroupResponse,
            > = serde_json::from_str(&local_content).ok();
            Ok(ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            })
        } else {
            let local_entity: Option<CreateScanningGroupError> =
                serde_json::from_str(&local_content).ok();
            let local_error = ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(Error::ResponseError(local_error))
        }
    }

    /// Create a scanning rule in a sensitive data scanner group, ordered last.
    /// The posted rule MUST include a group relationship.
    /// It MUST include either a standard_pattern relationship or a regex attribute, but not both.
    /// If included_attributes is empty or missing, we will scan all attributes except
    /// excluded_attributes. If both are missing, we will scan the whole event.
    pub async fn create_scanning_rule(
        &self,
        params: CreateScanningRuleParams,
    ) -> Result<
        Option<crate::datadogV2::model::SensitiveDataScannerCreateRuleResponse>,
        Error<CreateScanningRuleError>,
    > {
        match self.create_scanning_rule_with_http_info(params).await {
            Ok(response_content) => Ok(response_content.entity),
            Err(err) => Err(err),
        }
    }

    /// Create a scanning rule in a sensitive data scanner group, ordered last.
    /// The posted rule MUST include a group relationship.
    /// It MUST include either a standard_pattern relationship or a regex attribute, but not both.
    /// If included_attributes is empty or missing, we will scan all attributes except
    /// excluded_attributes. If both are missing, we will scan the whole event.
    pub async fn create_scanning_rule_with_http_info(
        &self,
        params: CreateScanningRuleParams,
    ) -> Result<
        ResponseContent<crate::datadogV2::model::SensitiveDataScannerCreateRuleResponse>,
        Error<CreateScanningRuleError>,
    > {
        let local_configuration = &self.config;

        // unbox and build parameters
        let body = params.body;

        let local_client = &local_configuration.client;

        let local_uri_str = format!(
            "{}/api/v2/sensitive-data-scanner/config/rules",
            local_configuration.base_path
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
            let local_entity: Option<
                crate::datadogV2::model::SensitiveDataScannerCreateRuleResponse,
            > = serde_json::from_str(&local_content).ok();
            Ok(ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            })
        } else {
            let local_entity: Option<CreateScanningRuleError> =
                serde_json::from_str(&local_content).ok();
            let local_error = ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(Error::ResponseError(local_error))
        }
    }

    /// Delete a given group.
    pub async fn delete_scanning_group(
        &self,
        params: DeleteScanningGroupParams,
    ) -> Result<
        Option<crate::datadogV2::model::SensitiveDataScannerGroupDeleteResponse>,
        Error<DeleteScanningGroupError>,
    > {
        match self.delete_scanning_group_with_http_info(params).await {
            Ok(response_content) => Ok(response_content.entity),
            Err(err) => Err(err),
        }
    }

    /// Delete a given group.
    pub async fn delete_scanning_group_with_http_info(
        &self,
        params: DeleteScanningGroupParams,
    ) -> Result<
        ResponseContent<crate::datadogV2::model::SensitiveDataScannerGroupDeleteResponse>,
        Error<DeleteScanningGroupError>,
    > {
        let local_configuration = &self.config;

        // unbox and build parameters
        let group_id = params.group_id;
        let body = params.body;

        let local_client = &local_configuration.client;

        let local_uri_str = format!(
            "{}/api/v2/sensitive-data-scanner/config/groups/{group_id}",
            local_configuration.base_path,
            group_id = urlencode(group_id)
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
            let local_entity: Option<
                crate::datadogV2::model::SensitiveDataScannerGroupDeleteResponse,
            > = serde_json::from_str(&local_content).ok();
            Ok(ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            })
        } else {
            let local_entity: Option<DeleteScanningGroupError> =
                serde_json::from_str(&local_content).ok();
            let local_error = ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(Error::ResponseError(local_error))
        }
    }

    /// Delete a given rule.
    pub async fn delete_scanning_rule(
        &self,
        params: DeleteScanningRuleParams,
    ) -> Result<
        Option<crate::datadogV2::model::SensitiveDataScannerRuleDeleteResponse>,
        Error<DeleteScanningRuleError>,
    > {
        match self.delete_scanning_rule_with_http_info(params).await {
            Ok(response_content) => Ok(response_content.entity),
            Err(err) => Err(err),
        }
    }

    /// Delete a given rule.
    pub async fn delete_scanning_rule_with_http_info(
        &self,
        params: DeleteScanningRuleParams,
    ) -> Result<
        ResponseContent<crate::datadogV2::model::SensitiveDataScannerRuleDeleteResponse>,
        Error<DeleteScanningRuleError>,
    > {
        let local_configuration = &self.config;

        // unbox and build parameters
        let rule_id = params.rule_id;
        let body = params.body;

        let local_client = &local_configuration.client;

        let local_uri_str = format!(
            "{}/api/v2/sensitive-data-scanner/config/rules/{rule_id}",
            local_configuration.base_path,
            rule_id = urlencode(rule_id)
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
            let local_entity: Option<
                crate::datadogV2::model::SensitiveDataScannerRuleDeleteResponse,
            > = serde_json::from_str(&local_content).ok();
            Ok(ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            })
        } else {
            let local_entity: Option<DeleteScanningRuleError> =
                serde_json::from_str(&local_content).ok();
            let local_error = ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(Error::ResponseError(local_error))
        }
    }

    /// List all the Scanning groups in your organization.
    pub async fn list_scanning_groups(
        &self,
    ) -> Result<
        Option<crate::datadogV2::model::SensitiveDataScannerGetConfigResponse>,
        Error<ListScanningGroupsError>,
    > {
        match self.list_scanning_groups_with_http_info().await {
            Ok(response_content) => Ok(response_content.entity),
            Err(err) => Err(err),
        }
    }

    /// List all the Scanning groups in your organization.
    pub async fn list_scanning_groups_with_http_info(
        &self,
    ) -> Result<
        ResponseContent<crate::datadogV2::model::SensitiveDataScannerGetConfigResponse>,
        Error<ListScanningGroupsError>,
    > {
        let local_configuration = &self.config;

        // unbox and build parameters

        let local_client = &local_configuration.client;

        let local_uri_str = format!(
            "{}/api/v2/sensitive-data-scanner/config",
            local_configuration.base_path
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
            let local_entity: Option<
                crate::datadogV2::model::SensitiveDataScannerGetConfigResponse,
            > = serde_json::from_str(&local_content).ok();
            Ok(ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            })
        } else {
            let local_entity: Option<ListScanningGroupsError> =
                serde_json::from_str(&local_content).ok();
            let local_error = ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(Error::ResponseError(local_error))
        }
    }

    /// Returns all standard patterns.
    pub async fn list_standard_patterns(
        &self,
    ) -> Result<
        Option<crate::datadogV2::model::SensitiveDataScannerStandardPatternsResponseData>,
        Error<ListStandardPatternsError>,
    > {
        match self.list_standard_patterns_with_http_info().await {
            Ok(response_content) => Ok(response_content.entity),
            Err(err) => Err(err),
        }
    }

    /// Returns all standard patterns.
    pub async fn list_standard_patterns_with_http_info(
        &self,
    ) -> Result<
        ResponseContent<crate::datadogV2::model::SensitiveDataScannerStandardPatternsResponseData>,
        Error<ListStandardPatternsError>,
    > {
        let local_configuration = &self.config;

        // unbox and build parameters

        let local_client = &local_configuration.client;

        let local_uri_str = format!(
            "{}/api/v2/sensitive-data-scanner/config/standard-patterns",
            local_configuration.base_path
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
            let local_entity: Option<
                crate::datadogV2::model::SensitiveDataScannerStandardPatternsResponseData,
            > = serde_json::from_str(&local_content).ok();
            Ok(ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            })
        } else {
            let local_entity: Option<ListStandardPatternsError> =
                serde_json::from_str(&local_content).ok();
            let local_error = ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(Error::ResponseError(local_error))
        }
    }

    /// Reorder the list of groups.
    pub async fn reorder_scanning_groups(
        &self,
        params: ReorderScanningGroupsParams,
    ) -> Result<
        Option<crate::datadogV2::model::SensitiveDataScannerReorderGroupsResponse>,
        Error<ReorderScanningGroupsError>,
    > {
        match self.reorder_scanning_groups_with_http_info(params).await {
            Ok(response_content) => Ok(response_content.entity),
            Err(err) => Err(err),
        }
    }

    /// Reorder the list of groups.
    pub async fn reorder_scanning_groups_with_http_info(
        &self,
        params: ReorderScanningGroupsParams,
    ) -> Result<
        ResponseContent<crate::datadogV2::model::SensitiveDataScannerReorderGroupsResponse>,
        Error<ReorderScanningGroupsError>,
    > {
        let local_configuration = &self.config;

        // unbox and build parameters
        let body = params.body;

        let local_client = &local_configuration.client;

        let local_uri_str = format!(
            "{}/api/v2/sensitive-data-scanner/config",
            local_configuration.base_path
        );
        let mut local_req_builder =
            local_client.request(reqwest::Method::PATCH, local_uri_str.as_str());

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
            let local_entity: Option<
                crate::datadogV2::model::SensitiveDataScannerReorderGroupsResponse,
            > = serde_json::from_str(&local_content).ok();
            Ok(ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            })
        } else {
            let local_entity: Option<ReorderScanningGroupsError> =
                serde_json::from_str(&local_content).ok();
            let local_error = ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(Error::ResponseError(local_error))
        }
    }

    /// Update a group, including the order of the rules.
    /// Rules within the group are reordered by including a rules relationship. If the rules
    /// relationship is present, its data section MUST contain linkages for all of the rules
    /// currently in the group, and MUST NOT contain any others.
    pub async fn update_scanning_group(
        &self,
        params: UpdateScanningGroupParams,
    ) -> Result<
        Option<crate::datadogV2::model::SensitiveDataScannerGroupUpdateResponse>,
        Error<UpdateScanningGroupError>,
    > {
        match self.update_scanning_group_with_http_info(params).await {
            Ok(response_content) => Ok(response_content.entity),
            Err(err) => Err(err),
        }
    }

    /// Update a group, including the order of the rules.
    /// Rules within the group are reordered by including a rules relationship. If the rules
    /// relationship is present, its data section MUST contain linkages for all of the rules
    /// currently in the group, and MUST NOT contain any others.
    pub async fn update_scanning_group_with_http_info(
        &self,
        params: UpdateScanningGroupParams,
    ) -> Result<
        ResponseContent<crate::datadogV2::model::SensitiveDataScannerGroupUpdateResponse>,
        Error<UpdateScanningGroupError>,
    > {
        let local_configuration = &self.config;

        // unbox and build parameters
        let group_id = params.group_id;
        let body = params.body;

        let local_client = &local_configuration.client;

        let local_uri_str = format!(
            "{}/api/v2/sensitive-data-scanner/config/groups/{group_id}",
            local_configuration.base_path,
            group_id = urlencode(group_id)
        );
        let mut local_req_builder =
            local_client.request(reqwest::Method::PATCH, local_uri_str.as_str());

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
            let local_entity: Option<
                crate::datadogV2::model::SensitiveDataScannerGroupUpdateResponse,
            > = serde_json::from_str(&local_content).ok();
            Ok(ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            })
        } else {
            let local_entity: Option<UpdateScanningGroupError> =
                serde_json::from_str(&local_content).ok();
            let local_error = ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(Error::ResponseError(local_error))
        }
    }

    /// Update a scanning rule.
    /// The request body MUST NOT include a standard_pattern relationship, as that relationship
    /// is non-editable. Trying to edit the regex attribute of a rule with a standard_pattern
    /// relationship will also result in an error.
    pub async fn update_scanning_rule(
        &self,
        params: UpdateScanningRuleParams,
    ) -> Result<
        Option<crate::datadogV2::model::SensitiveDataScannerRuleUpdateResponse>,
        Error<UpdateScanningRuleError>,
    > {
        match self.update_scanning_rule_with_http_info(params).await {
            Ok(response_content) => Ok(response_content.entity),
            Err(err) => Err(err),
        }
    }

    /// Update a scanning rule.
    /// The request body MUST NOT include a standard_pattern relationship, as that relationship
    /// is non-editable. Trying to edit the regex attribute of a rule with a standard_pattern
    /// relationship will also result in an error.
    pub async fn update_scanning_rule_with_http_info(
        &self,
        params: UpdateScanningRuleParams,
    ) -> Result<
        ResponseContent<crate::datadogV2::model::SensitiveDataScannerRuleUpdateResponse>,
        Error<UpdateScanningRuleError>,
    > {
        let local_configuration = &self.config;

        // unbox and build parameters
        let rule_id = params.rule_id;
        let body = params.body;

        let local_client = &local_configuration.client;

        let local_uri_str = format!(
            "{}/api/v2/sensitive-data-scanner/config/rules/{rule_id}",
            local_configuration.base_path,
            rule_id = urlencode(rule_id)
        );
        let mut local_req_builder =
            local_client.request(reqwest::Method::PATCH, local_uri_str.as_str());

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
            let local_entity: Option<
                crate::datadogV2::model::SensitiveDataScannerRuleUpdateResponse,
            > = serde_json::from_str(&local_content).ok();
            Ok(ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            })
        } else {
            let local_entity: Option<UpdateScanningRuleError> =
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
