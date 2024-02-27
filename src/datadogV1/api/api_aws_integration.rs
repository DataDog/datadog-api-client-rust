// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use crate::datadog::*;
use reqwest;
use serde::{Deserialize, Serialize};

/// ListAWSAccountsOptionalParams is a struct for passing parameters to the method [`AWSIntegrationAPI::list_aws_accounts`]
#[non_exhaustive]
#[derive(Clone, Default, Debug)]
pub struct ListAWSAccountsOptionalParams {
    /// Only return AWS accounts that matches this `account_id`.
    pub account_id: Option<String>,
    /// Only return AWS accounts that matches this role_name.
    pub role_name: Option<String>,
    /// Only return AWS accounts that matches this `access_key_id`.
    pub access_key_id: Option<String>,
}

impl ListAWSAccountsOptionalParams {
    /// Only return AWS accounts that matches this `account_id`.
    pub fn account_id(&mut self, value: String) -> &mut Self {
        self.account_id = Some(value);
        self
    }
    /// Only return AWS accounts that matches this role_name.
    pub fn role_name(&mut self, value: String) -> &mut Self {
        self.role_name = Some(value);
        self
    }
    /// Only return AWS accounts that matches this `access_key_id`.
    pub fn access_key_id(&mut self, value: String) -> &mut Self {
        self.access_key_id = Some(value);
        self
    }
}

/// UpdateAWSAccountOptionalParams is a struct for passing parameters to the method [`AWSIntegrationAPI::update_aws_account`]
#[non_exhaustive]
#[derive(Clone, Default, Debug)]
pub struct UpdateAWSAccountOptionalParams {
    /// Only return AWS accounts that matches this `account_id`.
    pub account_id: Option<String>,
    /// Only return AWS accounts that match this `role_name`.
    /// Required if `account_id` is specified.
    pub role_name: Option<String>,
    /// Only return AWS accounts that matches this `access_key_id`.
    /// Required if none of the other two options are specified.
    pub access_key_id: Option<String>,
}

impl UpdateAWSAccountOptionalParams {
    /// Only return AWS accounts that matches this `account_id`.
    pub fn account_id(&mut self, value: String) -> &mut Self {
        self.account_id = Some(value);
        self
    }
    /// Only return AWS accounts that match this `role_name`.
    /// Required if `account_id` is specified.
    pub fn role_name(&mut self, value: String) -> &mut Self {
        self.role_name = Some(value);
        self
    }
    /// Only return AWS accounts that matches this `access_key_id`.
    /// Required if none of the other two options are specified.
    pub fn access_key_id(&mut self, value: String) -> &mut Self {
        self.access_key_id = Some(value);
        self
    }
}

/// CreateAWSAccountError is a struct for typed errors of method [`AWSIntegrationAPI::create_aws_account`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CreateAWSAccountError {
    Status400(Option<crate::datadogV1::model::APIErrorResponse>),
    Status403(Option<crate::datadogV1::model::APIErrorResponse>),
    Status409(Option<crate::datadogV1::model::APIErrorResponse>),
    Status429(Option<crate::datadogV1::model::APIErrorResponse>),
    UnknownValue(serde_json::Value),
}

/// CreateAWSEventBridgeSourceError is a struct for typed errors of method [`AWSIntegrationAPI::create_aws_event_bridge_source`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CreateAWSEventBridgeSourceError {
    Status400(Option<crate::datadogV1::model::APIErrorResponse>),
    Status403(Option<crate::datadogV1::model::APIErrorResponse>),
    Status429(Option<crate::datadogV1::model::APIErrorResponse>),
    UnknownValue(serde_json::Value),
}

/// CreateAWSTagFilterError is a struct for typed errors of method [`AWSIntegrationAPI::create_aws_tag_filter`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CreateAWSTagFilterError {
    Status400(Option<crate::datadogV1::model::APIErrorResponse>),
    Status403(Option<crate::datadogV1::model::APIErrorResponse>),
    Status429(Option<crate::datadogV1::model::APIErrorResponse>),
    UnknownValue(serde_json::Value),
}

/// CreateNewAWSExternalIDError is a struct for typed errors of method [`AWSIntegrationAPI::create_new_aws_external_id`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CreateNewAWSExternalIDError {
    Status400(Option<crate::datadogV1::model::APIErrorResponse>),
    Status403(Option<crate::datadogV1::model::APIErrorResponse>),
    Status429(Option<crate::datadogV1::model::APIErrorResponse>),
    UnknownValue(serde_json::Value),
}

/// DeleteAWSAccountError is a struct for typed errors of method [`AWSIntegrationAPI::delete_aws_account`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DeleteAWSAccountError {
    Status400(Option<crate::datadogV1::model::APIErrorResponse>),
    Status403(Option<crate::datadogV1::model::APIErrorResponse>),
    Status409(Option<crate::datadogV1::model::APIErrorResponse>),
    Status429(Option<crate::datadogV1::model::APIErrorResponse>),
    UnknownValue(serde_json::Value),
}

/// DeleteAWSEventBridgeSourceError is a struct for typed errors of method [`AWSIntegrationAPI::delete_aws_event_bridge_source`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DeleteAWSEventBridgeSourceError {
    Status400(Option<crate::datadogV1::model::APIErrorResponse>),
    Status403(Option<crate::datadogV1::model::APIErrorResponse>),
    Status429(Option<crate::datadogV1::model::APIErrorResponse>),
    UnknownValue(serde_json::Value),
}

/// DeleteAWSTagFilterError is a struct for typed errors of method [`AWSIntegrationAPI::delete_aws_tag_filter`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DeleteAWSTagFilterError {
    Status400(Option<crate::datadogV1::model::APIErrorResponse>),
    Status403(Option<crate::datadogV1::model::APIErrorResponse>),
    Status429(Option<crate::datadogV1::model::APIErrorResponse>),
    UnknownValue(serde_json::Value),
}

/// ListAWSAccountsError is a struct for typed errors of method [`AWSIntegrationAPI::list_aws_accounts`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ListAWSAccountsError {
    Status400(Option<crate::datadogV1::model::APIErrorResponse>),
    Status403(Option<crate::datadogV1::model::APIErrorResponse>),
    Status429(Option<crate::datadogV1::model::APIErrorResponse>),
    UnknownValue(serde_json::Value),
}

/// ListAWSEventBridgeSourcesError is a struct for typed errors of method [`AWSIntegrationAPI::list_aws_event_bridge_sources`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ListAWSEventBridgeSourcesError {
    Status400(Option<crate::datadogV1::model::APIErrorResponse>),
    Status403(Option<crate::datadogV1::model::APIErrorResponse>),
    Status429(Option<crate::datadogV1::model::APIErrorResponse>),
    UnknownValue(serde_json::Value),
}

/// ListAWSTagFiltersError is a struct for typed errors of method [`AWSIntegrationAPI::list_aws_tag_filters`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ListAWSTagFiltersError {
    Status400(Option<crate::datadogV1::model::APIErrorResponse>),
    Status403(Option<crate::datadogV1::model::APIErrorResponse>),
    Status429(Option<crate::datadogV1::model::APIErrorResponse>),
    UnknownValue(serde_json::Value),
}

/// ListAvailableAWSNamespacesError is a struct for typed errors of method [`AWSIntegrationAPI::list_available_aws_namespaces`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ListAvailableAWSNamespacesError {
    Status403(Option<crate::datadogV1::model::APIErrorResponse>),
    Status429(Option<crate::datadogV1::model::APIErrorResponse>),
    UnknownValue(serde_json::Value),
}

/// UpdateAWSAccountError is a struct for typed errors of method [`AWSIntegrationAPI::update_aws_account`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum UpdateAWSAccountError {
    Status400(Option<crate::datadogV1::model::APIErrorResponse>),
    Status403(Option<crate::datadogV1::model::APIErrorResponse>),
    Status409(Option<crate::datadogV1::model::APIErrorResponse>),
    Status429(Option<crate::datadogV1::model::APIErrorResponse>),
    UnknownValue(serde_json::Value),
}

#[derive(Debug, Clone)]
pub struct AWSIntegrationAPI {
    config: configuration::Configuration,
}

impl Default for AWSIntegrationAPI {
    fn default() -> Self {
        Self {
            config: configuration::Configuration::new(),
        }
    }
}

impl AWSIntegrationAPI {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn with_config(config: configuration::Configuration) -> Self {
        Self { config }
    }

    /// Create a Datadog-Amazon Web Services integration.
    /// Using the `POST` method updates your integration configuration
    /// by adding your new configuration to the existing one in your Datadog organization.
    /// A unique AWS Account ID for role based authentication.
    pub async fn create_aws_account(
        &self,
        body: crate::datadogV1::model::AWSAccount,
    ) -> Result<crate::datadogV1::model::AWSAccountCreateResponse, Error<CreateAWSAccountError>>
    {
        match self.create_aws_account_with_http_info(body).await {
            Ok(response_content) => Ok(response_content.entity.unwrap()),
            Err(err) => Err(err),
        }
    }

    /// Create a Datadog-Amazon Web Services integration.
    /// Using the `POST` method updates your integration configuration
    /// by adding your new configuration to the existing one in your Datadog organization.
    /// A unique AWS Account ID for role based authentication.
    pub async fn create_aws_account_with_http_info(
        &self,
        body: crate::datadogV1::model::AWSAccount,
    ) -> Result<
        ResponseContent<crate::datadogV1::model::AWSAccountCreateResponse>,
        Error<CreateAWSAccountError>,
    > {
        let local_configuration = &self.config;

        let local_client = &local_configuration.client;

        let local_uri_str = format!("{}/api/v1/integration/aws", local_configuration.base_path);
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
            match serde_json::from_str::<crate::datadogV1::model::AWSAccountCreateResponse>(
                &local_content,
            ) {
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
            let local_entity: Option<CreateAWSAccountError> =
                serde_json::from_str(&local_content).ok();
            let local_error = ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(Error::ResponseError(local_error))
        }
    }

    /// Create an Amazon EventBridge source.
    pub async fn create_aws_event_bridge_source(
        &self,
        body: crate::datadogV1::model::AWSEventBridgeCreateRequest,
    ) -> Result<
        crate::datadogV1::model::AWSEventBridgeCreateResponse,
        Error<CreateAWSEventBridgeSourceError>,
    > {
        match self
            .create_aws_event_bridge_source_with_http_info(body)
            .await
        {
            Ok(response_content) => Ok(response_content.entity.unwrap()),
            Err(err) => Err(err),
        }
    }

    /// Create an Amazon EventBridge source.
    pub async fn create_aws_event_bridge_source_with_http_info(
        &self,
        body: crate::datadogV1::model::AWSEventBridgeCreateRequest,
    ) -> Result<
        ResponseContent<crate::datadogV1::model::AWSEventBridgeCreateResponse>,
        Error<CreateAWSEventBridgeSourceError>,
    > {
        let local_configuration = &self.config;

        let local_client = &local_configuration.client;

        let local_uri_str = format!(
            "{}/api/v1/integration/aws/event_bridge",
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
            match serde_json::from_str::<crate::datadogV1::model::AWSEventBridgeCreateResponse>(
                &local_content,
            ) {
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
            let local_entity: Option<CreateAWSEventBridgeSourceError> =
                serde_json::from_str(&local_content).ok();
            let local_error = ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(Error::ResponseError(local_error))
        }
    }

    /// Set an AWS tag filter.
    pub async fn create_aws_tag_filter(
        &self,
        body: crate::datadogV1::model::AWSTagFilterCreateRequest,
    ) -> Result<std::collections::BTreeMap<String, serde_json::Value>, Error<CreateAWSTagFilterError>>
    {
        match self.create_aws_tag_filter_with_http_info(body).await {
            Ok(response_content) => Ok(response_content.entity.unwrap()),
            Err(err) => Err(err),
        }
    }

    /// Set an AWS tag filter.
    pub async fn create_aws_tag_filter_with_http_info(
        &self,
        body: crate::datadogV1::model::AWSTagFilterCreateRequest,
    ) -> Result<
        ResponseContent<std::collections::BTreeMap<String, serde_json::Value>>,
        Error<CreateAWSTagFilterError>,
    > {
        let local_configuration = &self.config;

        let local_client = &local_configuration.client;

        let local_uri_str = format!(
            "{}/api/v1/integration/aws/filtering",
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
            match serde_json::from_str::<std::collections::BTreeMap<String, serde_json::Value>>(
                &local_content,
            ) {
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
            let local_entity: Option<CreateAWSTagFilterError> =
                serde_json::from_str(&local_content).ok();
            let local_error = ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(Error::ResponseError(local_error))
        }
    }

    /// Generate a new AWS external ID for a given AWS account ID and role name pair.
    pub async fn create_new_aws_external_id(
        &self,
        body: crate::datadogV1::model::AWSAccount,
    ) -> Result<crate::datadogV1::model::AWSAccountCreateResponse, Error<CreateNewAWSExternalIDError>>
    {
        match self.create_new_aws_external_id_with_http_info(body).await {
            Ok(response_content) => Ok(response_content.entity.unwrap()),
            Err(err) => Err(err),
        }
    }

    /// Generate a new AWS external ID for a given AWS account ID and role name pair.
    pub async fn create_new_aws_external_id_with_http_info(
        &self,
        body: crate::datadogV1::model::AWSAccount,
    ) -> Result<
        ResponseContent<crate::datadogV1::model::AWSAccountCreateResponse>,
        Error<CreateNewAWSExternalIDError>,
    > {
        let local_configuration = &self.config;

        let local_client = &local_configuration.client;

        let local_uri_str = format!(
            "{}/api/v1/integration/aws/generate_new_external_id",
            local_configuration.base_path
        );
        let mut local_req_builder =
            local_client.request(reqwest::Method::PUT, local_uri_str.as_str());

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
            match serde_json::from_str::<crate::datadogV1::model::AWSAccountCreateResponse>(
                &local_content,
            ) {
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
            let local_entity: Option<CreateNewAWSExternalIDError> =
                serde_json::from_str(&local_content).ok();
            let local_error = ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(Error::ResponseError(local_error))
        }
    }

    /// Delete a Datadog-AWS integration matching the specified `account_id` and `role_name parameters`.
    pub async fn delete_aws_account(
        &self,
        body: crate::datadogV1::model::AWSAccountDeleteRequest,
    ) -> Result<std::collections::BTreeMap<String, serde_json::Value>, Error<DeleteAWSAccountError>>
    {
        match self.delete_aws_account_with_http_info(body).await {
            Ok(response_content) => Ok(response_content.entity.unwrap()),
            Err(err) => Err(err),
        }
    }

    /// Delete a Datadog-AWS integration matching the specified `account_id` and `role_name parameters`.
    pub async fn delete_aws_account_with_http_info(
        &self,
        body: crate::datadogV1::model::AWSAccountDeleteRequest,
    ) -> Result<
        ResponseContent<std::collections::BTreeMap<String, serde_json::Value>>,
        Error<DeleteAWSAccountError>,
    > {
        let local_configuration = &self.config;

        let local_client = &local_configuration.client;

        let local_uri_str = format!("{}/api/v1/integration/aws", local_configuration.base_path);
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
            match serde_json::from_str::<std::collections::BTreeMap<String, serde_json::Value>>(
                &local_content,
            ) {
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
            let local_entity: Option<DeleteAWSAccountError> =
                serde_json::from_str(&local_content).ok();
            let local_error = ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(Error::ResponseError(local_error))
        }
    }

    /// Delete an Amazon EventBridge source.
    pub async fn delete_aws_event_bridge_source(
        &self,
        body: crate::datadogV1::model::AWSEventBridgeDeleteRequest,
    ) -> Result<
        crate::datadogV1::model::AWSEventBridgeDeleteResponse,
        Error<DeleteAWSEventBridgeSourceError>,
    > {
        match self
            .delete_aws_event_bridge_source_with_http_info(body)
            .await
        {
            Ok(response_content) => Ok(response_content.entity.unwrap()),
            Err(err) => Err(err),
        }
    }

    /// Delete an Amazon EventBridge source.
    pub async fn delete_aws_event_bridge_source_with_http_info(
        &self,
        body: crate::datadogV1::model::AWSEventBridgeDeleteRequest,
    ) -> Result<
        ResponseContent<crate::datadogV1::model::AWSEventBridgeDeleteResponse>,
        Error<DeleteAWSEventBridgeSourceError>,
    > {
        let local_configuration = &self.config;

        let local_client = &local_configuration.client;

        let local_uri_str = format!(
            "{}/api/v1/integration/aws/event_bridge",
            local_configuration.base_path
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
            match serde_json::from_str::<crate::datadogV1::model::AWSEventBridgeDeleteResponse>(
                &local_content,
            ) {
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
            let local_entity: Option<DeleteAWSEventBridgeSourceError> =
                serde_json::from_str(&local_content).ok();
            let local_error = ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(Error::ResponseError(local_error))
        }
    }

    /// Delete a tag filtering entry.
    pub async fn delete_aws_tag_filter(
        &self,
        body: crate::datadogV1::model::AWSTagFilterDeleteRequest,
    ) -> Result<std::collections::BTreeMap<String, serde_json::Value>, Error<DeleteAWSTagFilterError>>
    {
        match self.delete_aws_tag_filter_with_http_info(body).await {
            Ok(response_content) => Ok(response_content.entity.unwrap()),
            Err(err) => Err(err),
        }
    }

    /// Delete a tag filtering entry.
    pub async fn delete_aws_tag_filter_with_http_info(
        &self,
        body: crate::datadogV1::model::AWSTagFilterDeleteRequest,
    ) -> Result<
        ResponseContent<std::collections::BTreeMap<String, serde_json::Value>>,
        Error<DeleteAWSTagFilterError>,
    > {
        let local_configuration = &self.config;

        let local_client = &local_configuration.client;

        let local_uri_str = format!(
            "{}/api/v1/integration/aws/filtering",
            local_configuration.base_path
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
            match serde_json::from_str::<std::collections::BTreeMap<String, serde_json::Value>>(
                &local_content,
            ) {
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
            let local_entity: Option<DeleteAWSTagFilterError> =
                serde_json::from_str(&local_content).ok();
            let local_error = ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(Error::ResponseError(local_error))
        }
    }

    /// List all Datadog-AWS integrations available in your Datadog organization.
    pub async fn list_aws_accounts(
        &self,
        params: ListAWSAccountsOptionalParams,
    ) -> Result<crate::datadogV1::model::AWSAccountListResponse, Error<ListAWSAccountsError>> {
        match self.list_aws_accounts_with_http_info(params).await {
            Ok(response_content) => Ok(response_content.entity.unwrap()),
            Err(err) => Err(err),
        }
    }

    /// List all Datadog-AWS integrations available in your Datadog organization.
    pub async fn list_aws_accounts_with_http_info(
        &self,
        params: ListAWSAccountsOptionalParams,
    ) -> Result<
        ResponseContent<crate::datadogV1::model::AWSAccountListResponse>,
        Error<ListAWSAccountsError>,
    > {
        let local_configuration = &self.config;

        // unbox and build optional parameters
        let account_id = params.account_id;
        let role_name = params.role_name;
        let access_key_id = params.access_key_id;

        let local_client = &local_configuration.client;

        let local_uri_str = format!("{}/api/v1/integration/aws", local_configuration.base_path);
        let mut local_req_builder =
            local_client.request(reqwest::Method::GET, local_uri_str.as_str());

        if let Some(ref local_query_param) = account_id {
            local_req_builder =
                local_req_builder.query(&[("account_id", &local_query_param.to_string())]);
        };
        if let Some(ref local_query_param) = role_name {
            local_req_builder =
                local_req_builder.query(&[("role_name", &local_query_param.to_string())]);
        };
        if let Some(ref local_query_param) = access_key_id {
            local_req_builder =
                local_req_builder.query(&[("access_key_id", &local_query_param.to_string())]);
        };

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
            match serde_json::from_str::<crate::datadogV1::model::AWSAccountListResponse>(
                &local_content,
            ) {
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
            let local_entity: Option<ListAWSAccountsError> =
                serde_json::from_str(&local_content).ok();
            let local_error = ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(Error::ResponseError(local_error))
        }
    }

    /// Get all Amazon EventBridge sources.
    pub async fn list_aws_event_bridge_sources(
        &self,
    ) -> Result<
        crate::datadogV1::model::AWSEventBridgeListResponse,
        Error<ListAWSEventBridgeSourcesError>,
    > {
        match self.list_aws_event_bridge_sources_with_http_info().await {
            Ok(response_content) => Ok(response_content.entity.unwrap()),
            Err(err) => Err(err),
        }
    }

    /// Get all Amazon EventBridge sources.
    pub async fn list_aws_event_bridge_sources_with_http_info(
        &self,
    ) -> Result<
        ResponseContent<crate::datadogV1::model::AWSEventBridgeListResponse>,
        Error<ListAWSEventBridgeSourcesError>,
    > {
        let local_configuration = &self.config;

        let local_client = &local_configuration.client;

        let local_uri_str = format!(
            "{}/api/v1/integration/aws/event_bridge",
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
            match serde_json::from_str::<crate::datadogV1::model::AWSEventBridgeListResponse>(
                &local_content,
            ) {
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
            let local_entity: Option<ListAWSEventBridgeSourcesError> =
                serde_json::from_str(&local_content).ok();
            let local_error = ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(Error::ResponseError(local_error))
        }
    }

    /// Get all AWS tag filters.
    pub async fn list_aws_tag_filters(
        &self,
        account_id: String,
    ) -> Result<crate::datadogV1::model::AWSTagFilterListResponse, Error<ListAWSTagFiltersError>>
    {
        match self.list_aws_tag_filters_with_http_info(account_id).await {
            Ok(response_content) => Ok(response_content.entity.unwrap()),
            Err(err) => Err(err),
        }
    }

    /// Get all AWS tag filters.
    pub async fn list_aws_tag_filters_with_http_info(
        &self,
        account_id: String,
    ) -> Result<
        ResponseContent<crate::datadogV1::model::AWSTagFilterListResponse>,
        Error<ListAWSTagFiltersError>,
    > {
        let local_configuration = &self.config;

        let local_client = &local_configuration.client;

        let local_uri_str = format!(
            "{}/api/v1/integration/aws/filtering",
            local_configuration.base_path
        );
        let mut local_req_builder =
            local_client.request(reqwest::Method::GET, local_uri_str.as_str());

        local_req_builder = local_req_builder.query(&[("account_id", &account_id.to_string())]);

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
            match serde_json::from_str::<crate::datadogV1::model::AWSTagFilterListResponse>(
                &local_content,
            ) {
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
            let local_entity: Option<ListAWSTagFiltersError> =
                serde_json::from_str(&local_content).ok();
            let local_error = ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(Error::ResponseError(local_error))
        }
    }

    /// List all namespace rules for a given Datadog-AWS integration. This endpoint takes no arguments.
    pub async fn list_available_aws_namespaces(
        &self,
    ) -> Result<Vec<String>, Error<ListAvailableAWSNamespacesError>> {
        match self.list_available_aws_namespaces_with_http_info().await {
            Ok(response_content) => Ok(response_content.entity.unwrap()),
            Err(err) => Err(err),
        }
    }

    /// List all namespace rules for a given Datadog-AWS integration. This endpoint takes no arguments.
    pub async fn list_available_aws_namespaces_with_http_info(
        &self,
    ) -> Result<ResponseContent<Vec<String>>, Error<ListAvailableAWSNamespacesError>> {
        let local_configuration = &self.config;

        let local_client = &local_configuration.client;

        let local_uri_str = format!(
            "{}/api/v1/integration/aws/available_namespace_rules",
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
            match serde_json::from_str::<Vec<String>>(&local_content) {
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
            let local_entity: Option<ListAvailableAWSNamespacesError> =
                serde_json::from_str(&local_content).ok();
            let local_error = ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(Error::ResponseError(local_error))
        }
    }

    /// Update a Datadog-Amazon Web Services integration.
    pub async fn update_aws_account(
        &self,
        body: crate::datadogV1::model::AWSAccount,
        params: UpdateAWSAccountOptionalParams,
    ) -> Result<std::collections::BTreeMap<String, serde_json::Value>, Error<UpdateAWSAccountError>>
    {
        match self.update_aws_account_with_http_info(body, params).await {
            Ok(response_content) => Ok(response_content.entity.unwrap()),
            Err(err) => Err(err),
        }
    }

    /// Update a Datadog-Amazon Web Services integration.
    pub async fn update_aws_account_with_http_info(
        &self,
        body: crate::datadogV1::model::AWSAccount,
        params: UpdateAWSAccountOptionalParams,
    ) -> Result<
        ResponseContent<std::collections::BTreeMap<String, serde_json::Value>>,
        Error<UpdateAWSAccountError>,
    > {
        let local_configuration = &self.config;

        // unbox and build optional parameters
        let account_id = params.account_id;
        let role_name = params.role_name;
        let access_key_id = params.access_key_id;

        let local_client = &local_configuration.client;

        let local_uri_str = format!("{}/api/v1/integration/aws", local_configuration.base_path);
        let mut local_req_builder =
            local_client.request(reqwest::Method::PUT, local_uri_str.as_str());

        if let Some(ref local_query_param) = account_id {
            local_req_builder =
                local_req_builder.query(&[("account_id", &local_query_param.to_string())]);
        };
        if let Some(ref local_query_param) = role_name {
            local_req_builder =
                local_req_builder.query(&[("role_name", &local_query_param.to_string())]);
        };
        if let Some(ref local_query_param) = access_key_id {
            local_req_builder =
                local_req_builder.query(&[("access_key_id", &local_query_param.to_string())]);
        };

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
            match serde_json::from_str::<std::collections::BTreeMap<String, serde_json::Value>>(
                &local_content,
            ) {
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
            let local_entity: Option<UpdateAWSAccountError> =
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
