// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use crate::datadog::*;
use reqwest;
use serde::{Deserialize, Serialize};

/// CreateScanningGroupError is a struct for typed errors of method [`SensitiveDataScannerAPI::create_scanning_group`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CreateScanningGroupError {
    Status400(Option<crate::datadogV2::model::APIErrorResponse>),
    Status403(Option<crate::datadogV2::model::APIErrorResponse>),
    Status429(Option<crate::datadogV2::model::APIErrorResponse>),
    UnknownValue(serde_json::Value),
}

/// CreateScanningRuleError is a struct for typed errors of method [`SensitiveDataScannerAPI::create_scanning_rule`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CreateScanningRuleError {
    Status400(Option<crate::datadogV2::model::APIErrorResponse>),
    Status403(Option<crate::datadogV2::model::APIErrorResponse>),
    Status429(Option<crate::datadogV2::model::APIErrorResponse>),
    UnknownValue(serde_json::Value),
}

/// DeleteScanningGroupError is a struct for typed errors of method [`SensitiveDataScannerAPI::delete_scanning_group`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DeleteScanningGroupError {
    Status400(Option<crate::datadogV2::model::APIErrorResponse>),
    Status403(Option<crate::datadogV2::model::APIErrorResponse>),
    Status404(Option<crate::datadogV2::model::APIErrorResponse>),
    Status429(Option<crate::datadogV2::model::APIErrorResponse>),
    UnknownValue(serde_json::Value),
}

/// DeleteScanningRuleError is a struct for typed errors of method [`SensitiveDataScannerAPI::delete_scanning_rule`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DeleteScanningRuleError {
    Status400(Option<crate::datadogV2::model::APIErrorResponse>),
    Status403(Option<crate::datadogV2::model::APIErrorResponse>),
    Status404(Option<crate::datadogV2::model::APIErrorResponse>),
    Status429(Option<crate::datadogV2::model::APIErrorResponse>),
    UnknownValue(serde_json::Value),
}

/// ListScanningGroupsError is a struct for typed errors of method [`SensitiveDataScannerAPI::list_scanning_groups`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ListScanningGroupsError {
    Status400(Option<crate::datadogV2::model::APIErrorResponse>),
    Status403(Option<crate::datadogV2::model::APIErrorResponse>),
    Status429(Option<crate::datadogV2::model::APIErrorResponse>),
    UnknownValue(serde_json::Value),
}

/// ListStandardPatternsError is a struct for typed errors of method [`SensitiveDataScannerAPI::list_standard_patterns`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ListStandardPatternsError {
    Status400(Option<crate::datadogV2::model::APIErrorResponse>),
    Status403(Option<crate::datadogV2::model::APIErrorResponse>),
    Status429(Option<crate::datadogV2::model::APIErrorResponse>),
    UnknownValue(serde_json::Value),
}

/// ReorderScanningGroupsError is a struct for typed errors of method [`SensitiveDataScannerAPI::reorder_scanning_groups`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ReorderScanningGroupsError {
    Status400(Option<crate::datadogV2::model::APIErrorResponse>),
    Status403(Option<crate::datadogV2::model::APIErrorResponse>),
    Status429(Option<crate::datadogV2::model::APIErrorResponse>),
    UnknownValue(serde_json::Value),
}

/// UpdateScanningGroupError is a struct for typed errors of method [`SensitiveDataScannerAPI::update_scanning_group`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum UpdateScanningGroupError {
    Status400(Option<crate::datadogV2::model::APIErrorResponse>),
    Status403(Option<crate::datadogV2::model::APIErrorResponse>),
    Status404(Option<crate::datadogV2::model::APIErrorResponse>),
    Status429(Option<crate::datadogV2::model::APIErrorResponse>),
    UnknownValue(serde_json::Value),
}

/// UpdateScanningRuleError is a struct for typed errors of method [`SensitiveDataScannerAPI::update_scanning_rule`]
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
    client: reqwest_middleware::ClientWithMiddleware,
}

impl Default for SensitiveDataScannerAPI {
    fn default() -> Self {
        Self {
            config: configuration::Configuration::new(),
            client: reqwest_middleware::ClientBuilder::new(reqwest::Client::new()).build(),
        }
    }
}

impl SensitiveDataScannerAPI {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn with_config(config: configuration::Configuration) -> Self {
        let mut reqwest_client_builder = reqwest::Client::builder();

        if let Some(proxy_url) = &config.proxy_url {
            let proxy = reqwest::Proxy::all(proxy_url).expect("Failed to parse proxy URL");
            reqwest_client_builder = reqwest_client_builder.proxy(proxy);
        }

        let mut middleware_client_builder =
            reqwest_middleware::ClientBuilder::new(reqwest_client_builder.build().unwrap());
        let client = middleware_client_builder.build();
        Self { config, client }
    }

    pub fn with_client_and_config(
        config: configuration::Configuration,
        client: reqwest_middleware::ClientWithMiddleware,
    ) -> Self {
        Self { config, client }
    }

    /// Create a scanning group.
    /// The request MAY include a configuration relationship.
    /// A rules relationship can be omitted entirely, but if it is included it MUST be
    /// null or an empty array (rules cannot be created at the same time).
    /// The new group will be ordered last within the configuration.
    pub async fn create_scanning_group(
        &self,
        body: crate::datadogV2::model::SensitiveDataScannerGroupCreateRequest,
    ) -> Result<
        crate::datadogV2::model::SensitiveDataScannerCreateGroupResponse,
        Error<CreateScanningGroupError>,
    > {
        match self.create_scanning_group_with_http_info(body).await {
            Ok(response_content) => {
                if let Some(e) = response_content.entity {
                    Ok(e)
                } else {
                    Err(Error::Serde(serde::de::Error::custom(
                        "response content was None",
                    )))
                }
            }
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
        body: crate::datadogV2::model::SensitiveDataScannerGroupCreateRequest,
    ) -> Result<
        ResponseContent<crate::datadogV2::model::SensitiveDataScannerCreateGroupResponse>,
        Error<CreateScanningGroupError>,
    > {
        let local_configuration = &self.config;
        let operation_id = "v2.create_scanning_group";

        let local_client = &self.client;

        let local_uri_str = format!(
            "{}/api/v2/sensitive-data-scanner/config/groups",
            local_configuration.get_operation_host(operation_id)
        );
        let mut local_req_builder =
            local_client.request(reqwest::Method::POST, local_uri_str.as_str());

        // build user agent
        local_req_builder = local_req_builder.header(
            reqwest::header::USER_AGENT,
            local_configuration.user_agent.clone(),
        );

        // build auth
        if let Some(local_key) = local_configuration.auth_keys.get("apiKeyAuth") {
            local_req_builder = local_req_builder.header("DD-API-KEY", &local_key.key);
        };
        if let Some(local_key) = local_configuration.auth_keys.get("appKeyAuth") {
            local_req_builder = local_req_builder.header("DD-APPLICATION-KEY", &local_key.key);
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
            match serde_json::from_str::<
                crate::datadogV2::model::SensitiveDataScannerCreateGroupResponse,
            >(&local_content)
            {
                Ok(e) => {
                    return Ok(ResponseContent {
                        status: local_status,
                        content: local_content,
                        entity: Some(e),
                    })
                }
                Err(e) => return Err(crate::datadog::Error::Serde(e)),
            };
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
        body: crate::datadogV2::model::SensitiveDataScannerRuleCreateRequest,
    ) -> Result<
        crate::datadogV2::model::SensitiveDataScannerCreateRuleResponse,
        Error<CreateScanningRuleError>,
    > {
        match self.create_scanning_rule_with_http_info(body).await {
            Ok(response_content) => {
                if let Some(e) = response_content.entity {
                    Ok(e)
                } else {
                    Err(Error::Serde(serde::de::Error::custom(
                        "response content was None",
                    )))
                }
            }
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
        body: crate::datadogV2::model::SensitiveDataScannerRuleCreateRequest,
    ) -> Result<
        ResponseContent<crate::datadogV2::model::SensitiveDataScannerCreateRuleResponse>,
        Error<CreateScanningRuleError>,
    > {
        let local_configuration = &self.config;
        let operation_id = "v2.create_scanning_rule";

        let local_client = &self.client;

        let local_uri_str = format!(
            "{}/api/v2/sensitive-data-scanner/config/rules",
            local_configuration.get_operation_host(operation_id)
        );
        let mut local_req_builder =
            local_client.request(reqwest::Method::POST, local_uri_str.as_str());

        // build user agent
        local_req_builder = local_req_builder.header(
            reqwest::header::USER_AGENT,
            local_configuration.user_agent.clone(),
        );

        // build auth
        if let Some(local_key) = local_configuration.auth_keys.get("apiKeyAuth") {
            local_req_builder = local_req_builder.header("DD-API-KEY", &local_key.key);
        };
        if let Some(local_key) = local_configuration.auth_keys.get("appKeyAuth") {
            local_req_builder = local_req_builder.header("DD-APPLICATION-KEY", &local_key.key);
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
            match serde_json::from_str::<
                crate::datadogV2::model::SensitiveDataScannerCreateRuleResponse,
            >(&local_content)
            {
                Ok(e) => {
                    return Ok(ResponseContent {
                        status: local_status,
                        content: local_content,
                        entity: Some(e),
                    })
                }
                Err(e) => return Err(crate::datadog::Error::Serde(e)),
            };
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
        group_id: String,
        body: crate::datadogV2::model::SensitiveDataScannerGroupDeleteRequest,
    ) -> Result<
        crate::datadogV2::model::SensitiveDataScannerGroupDeleteResponse,
        Error<DeleteScanningGroupError>,
    > {
        match self
            .delete_scanning_group_with_http_info(group_id, body)
            .await
        {
            Ok(response_content) => {
                if let Some(e) = response_content.entity {
                    Ok(e)
                } else {
                    Err(Error::Serde(serde::de::Error::custom(
                        "response content was None",
                    )))
                }
            }
            Err(err) => Err(err),
        }
    }

    /// Delete a given group.
    pub async fn delete_scanning_group_with_http_info(
        &self,
        group_id: String,
        body: crate::datadogV2::model::SensitiveDataScannerGroupDeleteRequest,
    ) -> Result<
        ResponseContent<crate::datadogV2::model::SensitiveDataScannerGroupDeleteResponse>,
        Error<DeleteScanningGroupError>,
    > {
        let local_configuration = &self.config;
        let operation_id = "v2.delete_scanning_group";

        let local_client = &self.client;

        let local_uri_str = format!(
            "{}/api/v2/sensitive-data-scanner/config/groups/{group_id}",
            local_configuration.get_operation_host(operation_id),
            group_id = urlencode(group_id)
        );
        let mut local_req_builder =
            local_client.request(reqwest::Method::DELETE, local_uri_str.as_str());

        // build user agent
        local_req_builder = local_req_builder.header(
            reqwest::header::USER_AGENT,
            local_configuration.user_agent.clone(),
        );

        // build auth
        if let Some(local_key) = local_configuration.auth_keys.get("apiKeyAuth") {
            local_req_builder = local_req_builder.header("DD-API-KEY", &local_key.key);
        };
        if let Some(local_key) = local_configuration.auth_keys.get("appKeyAuth") {
            local_req_builder = local_req_builder.header("DD-APPLICATION-KEY", &local_key.key);
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
            match serde_json::from_str::<
                crate::datadogV2::model::SensitiveDataScannerGroupDeleteResponse,
            >(&local_content)
            {
                Ok(e) => {
                    return Ok(ResponseContent {
                        status: local_status,
                        content: local_content,
                        entity: Some(e),
                    })
                }
                Err(e) => return Err(crate::datadog::Error::Serde(e)),
            };
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
        rule_id: String,
        body: crate::datadogV2::model::SensitiveDataScannerRuleDeleteRequest,
    ) -> Result<
        crate::datadogV2::model::SensitiveDataScannerRuleDeleteResponse,
        Error<DeleteScanningRuleError>,
    > {
        match self
            .delete_scanning_rule_with_http_info(rule_id, body)
            .await
        {
            Ok(response_content) => {
                if let Some(e) = response_content.entity {
                    Ok(e)
                } else {
                    Err(Error::Serde(serde::de::Error::custom(
                        "response content was None",
                    )))
                }
            }
            Err(err) => Err(err),
        }
    }

    /// Delete a given rule.
    pub async fn delete_scanning_rule_with_http_info(
        &self,
        rule_id: String,
        body: crate::datadogV2::model::SensitiveDataScannerRuleDeleteRequest,
    ) -> Result<
        ResponseContent<crate::datadogV2::model::SensitiveDataScannerRuleDeleteResponse>,
        Error<DeleteScanningRuleError>,
    > {
        let local_configuration = &self.config;
        let operation_id = "v2.delete_scanning_rule";

        let local_client = &self.client;

        let local_uri_str = format!(
            "{}/api/v2/sensitive-data-scanner/config/rules/{rule_id}",
            local_configuration.get_operation_host(operation_id),
            rule_id = urlencode(rule_id)
        );
        let mut local_req_builder =
            local_client.request(reqwest::Method::DELETE, local_uri_str.as_str());

        // build user agent
        local_req_builder = local_req_builder.header(
            reqwest::header::USER_AGENT,
            local_configuration.user_agent.clone(),
        );

        // build auth
        if let Some(local_key) = local_configuration.auth_keys.get("apiKeyAuth") {
            local_req_builder = local_req_builder.header("DD-API-KEY", &local_key.key);
        };
        if let Some(local_key) = local_configuration.auth_keys.get("appKeyAuth") {
            local_req_builder = local_req_builder.header("DD-APPLICATION-KEY", &local_key.key);
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
            match serde_json::from_str::<
                crate::datadogV2::model::SensitiveDataScannerRuleDeleteResponse,
            >(&local_content)
            {
                Ok(e) => {
                    return Ok(ResponseContent {
                        status: local_status,
                        content: local_content,
                        entity: Some(e),
                    })
                }
                Err(e) => return Err(crate::datadog::Error::Serde(e)),
            };
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
        crate::datadogV2::model::SensitiveDataScannerGetConfigResponse,
        Error<ListScanningGroupsError>,
    > {
        match self.list_scanning_groups_with_http_info().await {
            Ok(response_content) => {
                if let Some(e) = response_content.entity {
                    Ok(e)
                } else {
                    Err(Error::Serde(serde::de::Error::custom(
                        "response content was None",
                    )))
                }
            }
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
        let operation_id = "v2.list_scanning_groups";

        let local_client = &self.client;

        let local_uri_str = format!(
            "{}/api/v2/sensitive-data-scanner/config",
            local_configuration.get_operation_host(operation_id)
        );
        let mut local_req_builder =
            local_client.request(reqwest::Method::GET, local_uri_str.as_str());

        // build user agent
        local_req_builder = local_req_builder.header(
            reqwest::header::USER_AGENT,
            local_configuration.user_agent.clone(),
        );

        // build auth
        if let Some(local_key) = local_configuration.auth_keys.get("apiKeyAuth") {
            local_req_builder = local_req_builder.header("DD-API-KEY", &local_key.key);
        };
        if let Some(local_key) = local_configuration.auth_keys.get("appKeyAuth") {
            local_req_builder = local_req_builder.header("DD-APPLICATION-KEY", &local_key.key);
        };

        let local_req = local_req_builder.build()?;
        let local_resp = local_client.execute(local_req).await?;

        let local_status = local_resp.status();
        let local_content = local_resp.text().await?;

        if !local_status.is_client_error() && !local_status.is_server_error() {
            match serde_json::from_str::<
                crate::datadogV2::model::SensitiveDataScannerGetConfigResponse,
            >(&local_content)
            {
                Ok(e) => {
                    return Ok(ResponseContent {
                        status: local_status,
                        content: local_content,
                        entity: Some(e),
                    })
                }
                Err(e) => return Err(crate::datadog::Error::Serde(e)),
            };
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
        crate::datadogV2::model::SensitiveDataScannerStandardPatternsResponseData,
        Error<ListStandardPatternsError>,
    > {
        match self.list_standard_patterns_with_http_info().await {
            Ok(response_content) => {
                if let Some(e) = response_content.entity {
                    Ok(e)
                } else {
                    Err(Error::Serde(serde::de::Error::custom(
                        "response content was None",
                    )))
                }
            }
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
        let operation_id = "v2.list_standard_patterns";

        let local_client = &self.client;

        let local_uri_str = format!(
            "{}/api/v2/sensitive-data-scanner/config/standard-patterns",
            local_configuration.get_operation_host(operation_id)
        );
        let mut local_req_builder =
            local_client.request(reqwest::Method::GET, local_uri_str.as_str());

        // build user agent
        local_req_builder = local_req_builder.header(
            reqwest::header::USER_AGENT,
            local_configuration.user_agent.clone(),
        );

        // build auth
        if let Some(local_key) = local_configuration.auth_keys.get("apiKeyAuth") {
            local_req_builder = local_req_builder.header("DD-API-KEY", &local_key.key);
        };
        if let Some(local_key) = local_configuration.auth_keys.get("appKeyAuth") {
            local_req_builder = local_req_builder.header("DD-APPLICATION-KEY", &local_key.key);
        };

        let local_req = local_req_builder.build()?;
        let local_resp = local_client.execute(local_req).await?;

        let local_status = local_resp.status();
        let local_content = local_resp.text().await?;

        if !local_status.is_client_error() && !local_status.is_server_error() {
            match serde_json::from_str::<
                crate::datadogV2::model::SensitiveDataScannerStandardPatternsResponseData,
            >(&local_content)
            {
                Ok(e) => {
                    return Ok(ResponseContent {
                        status: local_status,
                        content: local_content,
                        entity: Some(e),
                    })
                }
                Err(e) => return Err(crate::datadog::Error::Serde(e)),
            };
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
        body: crate::datadogV2::model::SensitiveDataScannerConfigRequest,
    ) -> Result<
        crate::datadogV2::model::SensitiveDataScannerReorderGroupsResponse,
        Error<ReorderScanningGroupsError>,
    > {
        match self.reorder_scanning_groups_with_http_info(body).await {
            Ok(response_content) => {
                if let Some(e) = response_content.entity {
                    Ok(e)
                } else {
                    Err(Error::Serde(serde::de::Error::custom(
                        "response content was None",
                    )))
                }
            }
            Err(err) => Err(err),
        }
    }

    /// Reorder the list of groups.
    pub async fn reorder_scanning_groups_with_http_info(
        &self,
        body: crate::datadogV2::model::SensitiveDataScannerConfigRequest,
    ) -> Result<
        ResponseContent<crate::datadogV2::model::SensitiveDataScannerReorderGroupsResponse>,
        Error<ReorderScanningGroupsError>,
    > {
        let local_configuration = &self.config;
        let operation_id = "v2.reorder_scanning_groups";

        let local_client = &self.client;

        let local_uri_str = format!(
            "{}/api/v2/sensitive-data-scanner/config",
            local_configuration.get_operation_host(operation_id)
        );
        let mut local_req_builder =
            local_client.request(reqwest::Method::PATCH, local_uri_str.as_str());

        // build user agent
        local_req_builder = local_req_builder.header(
            reqwest::header::USER_AGENT,
            local_configuration.user_agent.clone(),
        );

        // build auth
        if let Some(local_key) = local_configuration.auth_keys.get("apiKeyAuth") {
            local_req_builder = local_req_builder.header("DD-API-KEY", &local_key.key);
        };
        if let Some(local_key) = local_configuration.auth_keys.get("appKeyAuth") {
            local_req_builder = local_req_builder.header("DD-APPLICATION-KEY", &local_key.key);
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
            match serde_json::from_str::<
                crate::datadogV2::model::SensitiveDataScannerReorderGroupsResponse,
            >(&local_content)
            {
                Ok(e) => {
                    return Ok(ResponseContent {
                        status: local_status,
                        content: local_content,
                        entity: Some(e),
                    })
                }
                Err(e) => return Err(crate::datadog::Error::Serde(e)),
            };
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
        group_id: String,
        body: crate::datadogV2::model::SensitiveDataScannerGroupUpdateRequest,
    ) -> Result<
        crate::datadogV2::model::SensitiveDataScannerGroupUpdateResponse,
        Error<UpdateScanningGroupError>,
    > {
        match self
            .update_scanning_group_with_http_info(group_id, body)
            .await
        {
            Ok(response_content) => {
                if let Some(e) = response_content.entity {
                    Ok(e)
                } else {
                    Err(Error::Serde(serde::de::Error::custom(
                        "response content was None",
                    )))
                }
            }
            Err(err) => Err(err),
        }
    }

    /// Update a group, including the order of the rules.
    /// Rules within the group are reordered by including a rules relationship. If the rules
    /// relationship is present, its data section MUST contain linkages for all of the rules
    /// currently in the group, and MUST NOT contain any others.
    pub async fn update_scanning_group_with_http_info(
        &self,
        group_id: String,
        body: crate::datadogV2::model::SensitiveDataScannerGroupUpdateRequest,
    ) -> Result<
        ResponseContent<crate::datadogV2::model::SensitiveDataScannerGroupUpdateResponse>,
        Error<UpdateScanningGroupError>,
    > {
        let local_configuration = &self.config;
        let operation_id = "v2.update_scanning_group";

        let local_client = &self.client;

        let local_uri_str = format!(
            "{}/api/v2/sensitive-data-scanner/config/groups/{group_id}",
            local_configuration.get_operation_host(operation_id),
            group_id = urlencode(group_id)
        );
        let mut local_req_builder =
            local_client.request(reqwest::Method::PATCH, local_uri_str.as_str());

        // build user agent
        local_req_builder = local_req_builder.header(
            reqwest::header::USER_AGENT,
            local_configuration.user_agent.clone(),
        );

        // build auth
        if let Some(local_key) = local_configuration.auth_keys.get("apiKeyAuth") {
            local_req_builder = local_req_builder.header("DD-API-KEY", &local_key.key);
        };
        if let Some(local_key) = local_configuration.auth_keys.get("appKeyAuth") {
            local_req_builder = local_req_builder.header("DD-APPLICATION-KEY", &local_key.key);
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
            match serde_json::from_str::<
                crate::datadogV2::model::SensitiveDataScannerGroupUpdateResponse,
            >(&local_content)
            {
                Ok(e) => {
                    return Ok(ResponseContent {
                        status: local_status,
                        content: local_content,
                        entity: Some(e),
                    })
                }
                Err(e) => return Err(crate::datadog::Error::Serde(e)),
            };
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
        rule_id: String,
        body: crate::datadogV2::model::SensitiveDataScannerRuleUpdateRequest,
    ) -> Result<
        crate::datadogV2::model::SensitiveDataScannerRuleUpdateResponse,
        Error<UpdateScanningRuleError>,
    > {
        match self
            .update_scanning_rule_with_http_info(rule_id, body)
            .await
        {
            Ok(response_content) => {
                if let Some(e) = response_content.entity {
                    Ok(e)
                } else {
                    Err(Error::Serde(serde::de::Error::custom(
                        "response content was None",
                    )))
                }
            }
            Err(err) => Err(err),
        }
    }

    /// Update a scanning rule.
    /// The request body MUST NOT include a standard_pattern relationship, as that relationship
    /// is non-editable. Trying to edit the regex attribute of a rule with a standard_pattern
    /// relationship will also result in an error.
    pub async fn update_scanning_rule_with_http_info(
        &self,
        rule_id: String,
        body: crate::datadogV2::model::SensitiveDataScannerRuleUpdateRequest,
    ) -> Result<
        ResponseContent<crate::datadogV2::model::SensitiveDataScannerRuleUpdateResponse>,
        Error<UpdateScanningRuleError>,
    > {
        let local_configuration = &self.config;
        let operation_id = "v2.update_scanning_rule";

        let local_client = &self.client;

        let local_uri_str = format!(
            "{}/api/v2/sensitive-data-scanner/config/rules/{rule_id}",
            local_configuration.get_operation_host(operation_id),
            rule_id = urlencode(rule_id)
        );
        let mut local_req_builder =
            local_client.request(reqwest::Method::PATCH, local_uri_str.as_str());

        // build user agent
        local_req_builder = local_req_builder.header(
            reqwest::header::USER_AGENT,
            local_configuration.user_agent.clone(),
        );

        // build auth
        if let Some(local_key) = local_configuration.auth_keys.get("apiKeyAuth") {
            local_req_builder = local_req_builder.header("DD-API-KEY", &local_key.key);
        };
        if let Some(local_key) = local_configuration.auth_keys.get("appKeyAuth") {
            local_req_builder = local_req_builder.header("DD-APPLICATION-KEY", &local_key.key);
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
            match serde_json::from_str::<
                crate::datadogV2::model::SensitiveDataScannerRuleUpdateResponse,
            >(&local_content)
            {
                Ok(e) => {
                    return Ok(ResponseContent {
                        status: local_status,
                        content: local_content,
                        entity: Some(e),
                    })
                }
                Err(e) => return Err(crate::datadog::Error::Serde(e)),
            };
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
