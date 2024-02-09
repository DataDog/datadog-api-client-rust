// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use crate::datadog::*;
use reqwest;
use serde::{Deserialize, Serialize};

/// CreateCloudWorkloadSecurityAgentRuleError is a struct for typed errors of method [`CloudWorkloadSecurityAPI::create_cloud_workload_security_agent_rule`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CreateCloudWorkloadSecurityAgentRuleError {
    Status400(Option<crate::datadogV2::model::APIErrorResponse>),
    Status403(Option<crate::datadogV2::model::APIErrorResponse>),
    Status409(Option<crate::datadogV2::model::APIErrorResponse>),
    Status429(Option<crate::datadogV2::model::APIErrorResponse>),
    UnknownValue(serde_json::Value),
}

/// DeleteCloudWorkloadSecurityAgentRuleError is a struct for typed errors of method [`CloudWorkloadSecurityAPI::delete_cloud_workload_security_agent_rule`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DeleteCloudWorkloadSecurityAgentRuleError {
    Status403(Option<crate::datadogV2::model::APIErrorResponse>),
    Status404(Option<crate::datadogV2::model::APIErrorResponse>),
    Status429(Option<crate::datadogV2::model::APIErrorResponse>),
    UnknownValue(serde_json::Value),
}

/// DownloadCloudWorkloadPolicyFileError is a struct for typed errors of method [`CloudWorkloadSecurityAPI::download_cloud_workload_policy_file`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DownloadCloudWorkloadPolicyFileError {
    Status403(Option<crate::datadogV2::model::APIErrorResponse>),
    Status429(Option<crate::datadogV2::model::APIErrorResponse>),
    UnknownValue(serde_json::Value),
}

/// GetCloudWorkloadSecurityAgentRuleError is a struct for typed errors of method [`CloudWorkloadSecurityAPI::get_cloud_workload_security_agent_rule`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetCloudWorkloadSecurityAgentRuleError {
    Status403(Option<crate::datadogV2::model::APIErrorResponse>),
    Status404(Option<crate::datadogV2::model::APIErrorResponse>),
    Status429(Option<crate::datadogV2::model::APIErrorResponse>),
    UnknownValue(serde_json::Value),
}

/// ListCloudWorkloadSecurityAgentRulesError is a struct for typed errors of method [`CloudWorkloadSecurityAPI::list_cloud_workload_security_agent_rules`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ListCloudWorkloadSecurityAgentRulesError {
    Status403(Option<crate::datadogV2::model::APIErrorResponse>),
    Status429(Option<crate::datadogV2::model::APIErrorResponse>),
    UnknownValue(serde_json::Value),
}

/// UpdateCloudWorkloadSecurityAgentRuleError is a struct for typed errors of method [`CloudWorkloadSecurityAPI::update_cloud_workload_security_agent_rule`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum UpdateCloudWorkloadSecurityAgentRuleError {
    Status400(Option<crate::datadogV2::model::APIErrorResponse>),
    Status403(Option<crate::datadogV2::model::APIErrorResponse>),
    Status404(Option<crate::datadogV2::model::APIErrorResponse>),
    Status409(Option<crate::datadogV2::model::APIErrorResponse>),
    Status429(Option<crate::datadogV2::model::APIErrorResponse>),
    UnknownValue(serde_json::Value),
}

#[derive(Debug, Clone)]
pub struct CloudWorkloadSecurityAPI {
    config: configuration::Configuration,
}

impl Default for CloudWorkloadSecurityAPI {
    fn default() -> Self {
        Self {
            config: configuration::Configuration::new(),
        }
    }
}

impl CloudWorkloadSecurityAPI {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn with_config(config: configuration::Configuration) -> Self {
        Self { config }
    }

    /// Create a new Agent rule with the given parameters.
    pub async fn create_cloud_workload_security_agent_rule(
        &self,
        body: crate::datadogV2::model::CloudWorkloadSecurityAgentRuleCreateRequest,
    ) -> Result<
        Option<crate::datadogV2::model::CloudWorkloadSecurityAgentRuleResponse>,
        Error<CreateCloudWorkloadSecurityAgentRuleError>,
    > {
        match self
            .create_cloud_workload_security_agent_rule_with_http_info(body)
            .await
        {
            Ok(response_content) => Ok(response_content.entity),
            Err(err) => Err(err),
        }
    }

    /// Create a new Agent rule with the given parameters.
    pub async fn create_cloud_workload_security_agent_rule_with_http_info(
        &self,
        body: crate::datadogV2::model::CloudWorkloadSecurityAgentRuleCreateRequest,
    ) -> Result<
        ResponseContent<crate::datadogV2::model::CloudWorkloadSecurityAgentRuleResponse>,
        Error<CreateCloudWorkloadSecurityAgentRuleError>,
    > {
        let local_configuration = &self.config;

        let local_client = &local_configuration.client;

        let local_uri_str = format!(
            "{}/api/v2/security_monitoring/cloud_workload_security/agent_rules",
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
                crate::datadogV2::model::CloudWorkloadSecurityAgentRuleResponse,
            > = serde_json::from_str(&local_content).ok();
            Ok(ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            })
        } else {
            let local_entity: Option<CreateCloudWorkloadSecurityAgentRuleError> =
                serde_json::from_str(&local_content).ok();
            let local_error = ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(Error::ResponseError(local_error))
        }
    }

    /// Delete a specific Agent rule.
    pub async fn delete_cloud_workload_security_agent_rule(
        &self,
        agent_rule_id: String,
    ) -> Result<Option<()>, Error<DeleteCloudWorkloadSecurityAgentRuleError>> {
        match self
            .delete_cloud_workload_security_agent_rule_with_http_info(agent_rule_id)
            .await
        {
            Ok(response_content) => Ok(response_content.entity),
            Err(err) => Err(err),
        }
    }

    /// Delete a specific Agent rule.
    pub async fn delete_cloud_workload_security_agent_rule_with_http_info(
        &self,
        agent_rule_id: String,
    ) -> Result<ResponseContent<()>, Error<DeleteCloudWorkloadSecurityAgentRuleError>> {
        let local_configuration = &self.config;

        let local_client = &local_configuration.client;

        let local_uri_str = format!(
            "{}/api/v2/security_monitoring/cloud_workload_security/agent_rules/{agent_rule_id}",
            local_configuration.base_path,
            agent_rule_id = urlencode(agent_rule_id)
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
            let local_entity: Option<DeleteCloudWorkloadSecurityAgentRuleError> =
                serde_json::from_str(&local_content).ok();
            let local_error = ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(Error::ResponseError(local_error))
        }
    }

    /// The download endpoint generates a Cloud Workload Security policy file from your currently active
    /// Cloud Workload Security rules, and downloads them as a .policy file. This file can then be deployed to
    /// your Agents to update the policy running in your environment.
    pub async fn download_cloud_workload_policy_file(
        &self,
    ) -> Result<Option<Vec<u8>>, Error<DownloadCloudWorkloadPolicyFileError>> {
        match self
            .download_cloud_workload_policy_file_with_http_info()
            .await
        {
            Ok(response_content) => Ok(response_content.entity),
            Err(err) => Err(err),
        }
    }

    /// The download endpoint generates a Cloud Workload Security policy file from your currently active
    /// Cloud Workload Security rules, and downloads them as a .policy file. This file can then be deployed to
    /// your Agents to update the policy running in your environment.
    pub async fn download_cloud_workload_policy_file_with_http_info(
        &self,
    ) -> Result<ResponseContent<Vec<u8>>, Error<DownloadCloudWorkloadPolicyFileError>> {
        let local_configuration = &self.config;

        let local_client = &local_configuration.client;

        let local_uri_str = format!(
            "{}/api/v2/security/cloud_workload/policy/download",
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
            let local_entity: Option<Vec<u8>> = serde_json::from_str(&local_content).ok();
            Ok(ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            })
        } else {
            let local_entity: Option<DownloadCloudWorkloadPolicyFileError> =
                serde_json::from_str(&local_content).ok();
            let local_error = ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(Error::ResponseError(local_error))
        }
    }

    /// Get the details of a specific Agent rule.
    pub async fn get_cloud_workload_security_agent_rule(
        &self,
        agent_rule_id: String,
    ) -> Result<
        Option<crate::datadogV2::model::CloudWorkloadSecurityAgentRuleResponse>,
        Error<GetCloudWorkloadSecurityAgentRuleError>,
    > {
        match self
            .get_cloud_workload_security_agent_rule_with_http_info(agent_rule_id)
            .await
        {
            Ok(response_content) => Ok(response_content.entity),
            Err(err) => Err(err),
        }
    }

    /// Get the details of a specific Agent rule.
    pub async fn get_cloud_workload_security_agent_rule_with_http_info(
        &self,
        agent_rule_id: String,
    ) -> Result<
        ResponseContent<crate::datadogV2::model::CloudWorkloadSecurityAgentRuleResponse>,
        Error<GetCloudWorkloadSecurityAgentRuleError>,
    > {
        let local_configuration = &self.config;

        let local_client = &local_configuration.client;

        let local_uri_str = format!(
            "{}/api/v2/security_monitoring/cloud_workload_security/agent_rules/{agent_rule_id}",
            local_configuration.base_path,
            agent_rule_id = urlencode(agent_rule_id)
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
                crate::datadogV2::model::CloudWorkloadSecurityAgentRuleResponse,
            > = serde_json::from_str(&local_content).ok();
            Ok(ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            })
        } else {
            let local_entity: Option<GetCloudWorkloadSecurityAgentRuleError> =
                serde_json::from_str(&local_content).ok();
            let local_error = ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(Error::ResponseError(local_error))
        }
    }

    /// Get the list of Agent rules.
    pub async fn list_cloud_workload_security_agent_rules(
        &self,
    ) -> Result<
        Option<crate::datadogV2::model::CloudWorkloadSecurityAgentRulesListResponse>,
        Error<ListCloudWorkloadSecurityAgentRulesError>,
    > {
        match self
            .list_cloud_workload_security_agent_rules_with_http_info()
            .await
        {
            Ok(response_content) => Ok(response_content.entity),
            Err(err) => Err(err),
        }
    }

    /// Get the list of Agent rules.
    pub async fn list_cloud_workload_security_agent_rules_with_http_info(
        &self,
    ) -> Result<
        ResponseContent<crate::datadogV2::model::CloudWorkloadSecurityAgentRulesListResponse>,
        Error<ListCloudWorkloadSecurityAgentRulesError>,
    > {
        let local_configuration = &self.config;

        let local_client = &local_configuration.client;

        let local_uri_str = format!(
            "{}/api/v2/security_monitoring/cloud_workload_security/agent_rules",
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
                crate::datadogV2::model::CloudWorkloadSecurityAgentRulesListResponse,
            > = serde_json::from_str(&local_content).ok();
            Ok(ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            })
        } else {
            let local_entity: Option<ListCloudWorkloadSecurityAgentRulesError> =
                serde_json::from_str(&local_content).ok();
            let local_error = ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(Error::ResponseError(local_error))
        }
    }

    /// Update a specific Agent rule.
    /// Returns the Agent rule object when the request is successful.
    pub async fn update_cloud_workload_security_agent_rule(
        &self,
        agent_rule_id: String,
        body: crate::datadogV2::model::CloudWorkloadSecurityAgentRuleUpdateRequest,
    ) -> Result<
        Option<crate::datadogV2::model::CloudWorkloadSecurityAgentRuleResponse>,
        Error<UpdateCloudWorkloadSecurityAgentRuleError>,
    > {
        match self
            .update_cloud_workload_security_agent_rule_with_http_info(agent_rule_id, body)
            .await
        {
            Ok(response_content) => Ok(response_content.entity),
            Err(err) => Err(err),
        }
    }

    /// Update a specific Agent rule.
    /// Returns the Agent rule object when the request is successful.
    pub async fn update_cloud_workload_security_agent_rule_with_http_info(
        &self,
        agent_rule_id: String,
        body: crate::datadogV2::model::CloudWorkloadSecurityAgentRuleUpdateRequest,
    ) -> Result<
        ResponseContent<crate::datadogV2::model::CloudWorkloadSecurityAgentRuleResponse>,
        Error<UpdateCloudWorkloadSecurityAgentRuleError>,
    > {
        let local_configuration = &self.config;

        let local_client = &local_configuration.client;

        let local_uri_str = format!(
            "{}/api/v2/security_monitoring/cloud_workload_security/agent_rules/{agent_rule_id}",
            local_configuration.base_path,
            agent_rule_id = urlencode(agent_rule_id)
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
                crate::datadogV2::model::CloudWorkloadSecurityAgentRuleResponse,
            > = serde_json::from_str(&local_content).ok();
            Ok(ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            })
        } else {
            let local_entity: Option<UpdateCloudWorkloadSecurityAgentRuleError> =
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
