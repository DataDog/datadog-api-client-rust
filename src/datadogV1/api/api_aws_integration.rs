// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

use crate::datadog::*;
use reqwest;
use serde::{Deserialize, Serialize};

/// CreateAWSAccountParams is a struct for passing parameters to the method [`CreateAWSAccount`]
#[derive(Clone, Debug)]
pub struct CreateAWSAccountParams {
    /// AWS Request Object
    pub body: crate::datadogV1::model::AWSAccount,
}

/// CreateAWSEventBridgeSourceParams is a struct for passing parameters to the method [`CreateAWSEventBridgeSource`]
#[derive(Clone, Debug)]
pub struct CreateAWSEventBridgeSourceParams {
    /// Create an Amazon EventBridge source for an AWS account with a given name and region.
    pub body: crate::datadogV1::model::AWSEventBridgeCreateRequest,
}

/// CreateAWSTagFilterParams is a struct for passing parameters to the method [`CreateAWSTagFilter`]
#[derive(Clone, Debug)]
pub struct CreateAWSTagFilterParams {
    /// Set an AWS tag filter using an `aws_account_identifier`, `namespace`, and filtering string.
    /// Namespace options are `application_elb`, `elb`, `lambda`, `network_elb`, `rds`, `sqs`, and `custom`.
    pub body: crate::datadogV1::model::AWSTagFilterCreateRequest,
}

/// CreateNewAWSExternalIDParams is a struct for passing parameters to the method [`CreateNewAWSExternalID`]
#[derive(Clone, Debug)]
pub struct CreateNewAWSExternalIDParams {
    /// Your Datadog role delegation name.
    /// For more information about your AWS account Role name,
    /// see the [Datadog AWS integration configuration info](https://docs.datadoghq.com/integrations/amazon_web_services/#setup).
    pub body: crate::datadogV1::model::AWSAccount,
}

/// DeleteAWSAccountParams is a struct for passing parameters to the method [`DeleteAWSAccount`]
#[derive(Clone, Debug)]
pub struct DeleteAWSAccountParams {
    /// AWS request object
    pub body: crate::datadogV1::model::AWSAccountDeleteRequest,
}

/// DeleteAWSEventBridgeSourceParams is a struct for passing parameters to the method [`DeleteAWSEventBridgeSource`]
#[derive(Clone, Debug)]
pub struct DeleteAWSEventBridgeSourceParams {
    /// Delete the Amazon EventBridge source with the given name, region, and associated AWS account.
    pub body: crate::datadogV1::model::AWSEventBridgeDeleteRequest,
}

/// DeleteAWSTagFilterParams is a struct for passing parameters to the method [`DeleteAWSTagFilter`]
#[derive(Clone, Debug)]
pub struct DeleteAWSTagFilterParams {
    /// Delete a tag filtering entry for a given AWS account and `dd-aws` namespace.
    pub body: crate::datadogV1::model::AWSTagFilterDeleteRequest,
}

/// ListAWSAccountsParams is a struct for passing parameters to the method [`ListAWSAccounts`]
#[derive(Clone, Debug)]
pub struct ListAWSAccountsParams {
    /// Only return AWS accounts that matches this `account_id`.
    pub account_id: Option<String>,
    /// Only return AWS accounts that matches this role_name.
    pub role_name: Option<String>,
    /// Only return AWS accounts that matches this `access_key_id`.
    pub access_key_id: Option<String>,
}

/// ListAWSTagFiltersParams is a struct for passing parameters to the method [`ListAWSTagFilters`]
#[derive(Clone, Debug)]
pub struct ListAWSTagFiltersParams {
    /// Only return AWS filters that matches this `account_id`.
    pub account_id: String,
}

/// UpdateAWSAccountParams is a struct for passing parameters to the method [`UpdateAWSAccount`]
#[derive(Clone, Debug)]
pub struct UpdateAWSAccountParams {
    /// AWS request object
    pub body: crate::datadogV1::model::AWSAccount,
    /// Only return AWS accounts that matches this `account_id`.
    pub account_id: Option<String>,
    /// Only return AWS accounts that match this `role_name`.
    /// Required if `account_id` is specified.
    pub role_name: Option<String>,
    /// Only return AWS accounts that matches this `access_key_id`.
    /// Required if none of the other two options are specified.
    pub access_key_id: Option<String>,
}

/// CreateAWSAccountError is a struct for typed errors of method [`CreateAWSAccount`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CreateAWSAccountError {
    Status400(Option<crate::datadogV1::model::APIErrorResponse>),
    Status403(Option<crate::datadogV1::model::APIErrorResponse>),
    Status409(Option<crate::datadogV1::model::APIErrorResponse>),
    Status429(Option<crate::datadogV1::model::APIErrorResponse>),
    UnknownValue(serde_json::Value),
}

/// CreateAWSEventBridgeSourceError is a struct for typed errors of method [`CreateAWSEventBridgeSource`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CreateAWSEventBridgeSourceError {
    Status400(Option<crate::datadogV1::model::APIErrorResponse>),
    Status403(Option<crate::datadogV1::model::APIErrorResponse>),
    Status429(Option<crate::datadogV1::model::APIErrorResponse>),
    UnknownValue(serde_json::Value),
}

/// CreateAWSTagFilterError is a struct for typed errors of method [`CreateAWSTagFilter`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CreateAWSTagFilterError {
    Status400(Option<crate::datadogV1::model::APIErrorResponse>),
    Status403(Option<crate::datadogV1::model::APIErrorResponse>),
    Status429(Option<crate::datadogV1::model::APIErrorResponse>),
    UnknownValue(serde_json::Value),
}

/// CreateNewAWSExternalIDError is a struct for typed errors of method [`CreateNewAWSExternalID`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CreateNewAWSExternalIDError {
    Status400(Option<crate::datadogV1::model::APIErrorResponse>),
    Status403(Option<crate::datadogV1::model::APIErrorResponse>),
    Status429(Option<crate::datadogV1::model::APIErrorResponse>),
    UnknownValue(serde_json::Value),
}

/// DeleteAWSAccountError is a struct for typed errors of method [`DeleteAWSAccount`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DeleteAWSAccountError {
    Status400(Option<crate::datadogV1::model::APIErrorResponse>),
    Status403(Option<crate::datadogV1::model::APIErrorResponse>),
    Status409(Option<crate::datadogV1::model::APIErrorResponse>),
    Status429(Option<crate::datadogV1::model::APIErrorResponse>),
    UnknownValue(serde_json::Value),
}

/// DeleteAWSEventBridgeSourceError is a struct for typed errors of method [`DeleteAWSEventBridgeSource`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DeleteAWSEventBridgeSourceError {
    Status400(Option<crate::datadogV1::model::APIErrorResponse>),
    Status403(Option<crate::datadogV1::model::APIErrorResponse>),
    Status429(Option<crate::datadogV1::model::APIErrorResponse>),
    UnknownValue(serde_json::Value),
}

/// DeleteAWSTagFilterError is a struct for typed errors of method [`DeleteAWSTagFilter`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DeleteAWSTagFilterError {
    Status400(Option<crate::datadogV1::model::APIErrorResponse>),
    Status403(Option<crate::datadogV1::model::APIErrorResponse>),
    Status429(Option<crate::datadogV1::model::APIErrorResponse>),
    UnknownValue(serde_json::Value),
}

/// ListAWSAccountsError is a struct for typed errors of method [`ListAWSAccounts`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ListAWSAccountsError {
    Status400(Option<crate::datadogV1::model::APIErrorResponse>),
    Status403(Option<crate::datadogV1::model::APIErrorResponse>),
    Status429(Option<crate::datadogV1::model::APIErrorResponse>),
    UnknownValue(serde_json::Value),
}

/// ListAWSEventBridgeSourcesError is a struct for typed errors of method [`ListAWSEventBridgeSources`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ListAWSEventBridgeSourcesError {
    Status400(Option<crate::datadogV1::model::APIErrorResponse>),
    Status403(Option<crate::datadogV1::model::APIErrorResponse>),
    Status429(Option<crate::datadogV1::model::APIErrorResponse>),
    UnknownValue(serde_json::Value),
}

/// ListAWSTagFiltersError is a struct for typed errors of method [`ListAWSTagFilters`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ListAWSTagFiltersError {
    Status400(Option<crate::datadogV1::model::APIErrorResponse>),
    Status403(Option<crate::datadogV1::model::APIErrorResponse>),
    Status429(Option<crate::datadogV1::model::APIErrorResponse>),
    UnknownValue(serde_json::Value),
}

/// ListAvailableAWSNamespacesError is a struct for typed errors of method [`ListAvailableAWSNamespaces`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ListAvailableAWSNamespacesError {
    Status403(Option<crate::datadogV1::model::APIErrorResponse>),
    Status429(Option<crate::datadogV1::model::APIErrorResponse>),
    UnknownValue(serde_json::Value),
}

/// UpdateAWSAccountError is a struct for typed errors of method [`UpdateAWSAccount`]
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
pub struct AwsIntegrationAPI {
    config: configuration::Configuration,
}

impl Default for AwsIntegrationAPI {
    fn default() -> Self {
        Self {
            config: configuration::Configuration::new(),
        }
    }
}

impl AwsIntegrationAPI {
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
        params: CreateAWSAccountParams,
    ) -> Result<
        Option<crate::datadogV1::model::AWSAccountCreateResponse>,
        Error<CreateAWSAccountError>,
    > {
        match self.create_aws_account_with_http_info(params).await {
            Ok(response_content) => Ok(response_content.entity),
            Err(err) => Err(err),
        }
    }

    /// Create a Datadog-Amazon Web Services integration.
    /// Using the `POST` method updates your integration configuration
    /// by adding your new configuration to the existing one in your Datadog organization.
    /// A unique AWS Account ID for role based authentication.
    pub async fn create_aws_account_with_http_info(
        &self,
        params: CreateAWSAccountParams,
    ) -> Result<
        ResponseContent<crate::datadogV1::model::AWSAccountCreateResponse>,
        Error<CreateAWSAccountError>,
    > {
        let local_configuration = &self.config;

        // unbox and build parameters
        let body = params.body;

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
            let local_entity: Option<crate::datadogV1::model::AWSAccountCreateResponse> =
                serde_json::from_str(&local_content).ok();
            Ok(ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            })
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
        params: CreateAWSEventBridgeSourceParams,
    ) -> Result<
        Option<crate::datadogV1::model::AWSEventBridgeCreateResponse>,
        Error<CreateAWSEventBridgeSourceError>,
    > {
        match self
            .create_aws_event_bridge_source_with_http_info(params)
            .await
        {
            Ok(response_content) => Ok(response_content.entity),
            Err(err) => Err(err),
        }
    }

    /// Create an Amazon EventBridge source.
    pub async fn create_aws_event_bridge_source_with_http_info(
        &self,
        params: CreateAWSEventBridgeSourceParams,
    ) -> Result<
        ResponseContent<crate::datadogV1::model::AWSEventBridgeCreateResponse>,
        Error<CreateAWSEventBridgeSourceError>,
    > {
        let local_configuration = &self.config;

        // unbox and build parameters
        let body = params.body;

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
            let local_entity: Option<crate::datadogV1::model::AWSEventBridgeCreateResponse> =
                serde_json::from_str(&local_content).ok();
            Ok(ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            })
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
        params: CreateAWSTagFilterParams,
    ) -> Result<
        Option<std::collections::HashMap<String, serde_json::Value>>,
        Error<CreateAWSTagFilterError>,
    > {
        match self.create_aws_tag_filter_with_http_info(params).await {
            Ok(response_content) => Ok(response_content.entity),
            Err(err) => Err(err),
        }
    }

    /// Set an AWS tag filter.
    pub async fn create_aws_tag_filter_with_http_info(
        &self,
        params: CreateAWSTagFilterParams,
    ) -> Result<
        ResponseContent<std::collections::HashMap<String, serde_json::Value>>,
        Error<CreateAWSTagFilterError>,
    > {
        let local_configuration = &self.config;

        // unbox and build parameters
        let body = params.body;

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
            let local_entity: Option<std::collections::HashMap<String, serde_json::Value>> =
                serde_json::from_str(&local_content).ok();
            Ok(ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            })
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
        params: CreateNewAWSExternalIDParams,
    ) -> Result<
        Option<crate::datadogV1::model::AWSAccountCreateResponse>,
        Error<CreateNewAWSExternalIDError>,
    > {
        match self.create_new_aws_external_id_with_http_info(params).await {
            Ok(response_content) => Ok(response_content.entity),
            Err(err) => Err(err),
        }
    }

    /// Generate a new AWS external ID for a given AWS account ID and role name pair.
    pub async fn create_new_aws_external_id_with_http_info(
        &self,
        params: CreateNewAWSExternalIDParams,
    ) -> Result<
        ResponseContent<crate::datadogV1::model::AWSAccountCreateResponse>,
        Error<CreateNewAWSExternalIDError>,
    > {
        let local_configuration = &self.config;

        // unbox and build parameters
        let body = params.body;

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
            let local_entity: Option<crate::datadogV1::model::AWSAccountCreateResponse> =
                serde_json::from_str(&local_content).ok();
            Ok(ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            })
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
        params: DeleteAWSAccountParams,
    ) -> Result<
        Option<std::collections::HashMap<String, serde_json::Value>>,
        Error<DeleteAWSAccountError>,
    > {
        match self.delete_aws_account_with_http_info(params).await {
            Ok(response_content) => Ok(response_content.entity),
            Err(err) => Err(err),
        }
    }

    /// Delete a Datadog-AWS integration matching the specified `account_id` and `role_name parameters`.
    pub async fn delete_aws_account_with_http_info(
        &self,
        params: DeleteAWSAccountParams,
    ) -> Result<
        ResponseContent<std::collections::HashMap<String, serde_json::Value>>,
        Error<DeleteAWSAccountError>,
    > {
        let local_configuration = &self.config;

        // unbox and build parameters
        let body = params.body;

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
            let local_entity: Option<std::collections::HashMap<String, serde_json::Value>> =
                serde_json::from_str(&local_content).ok();
            Ok(ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            })
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
        params: DeleteAWSEventBridgeSourceParams,
    ) -> Result<
        Option<crate::datadogV1::model::AWSEventBridgeDeleteResponse>,
        Error<DeleteAWSEventBridgeSourceError>,
    > {
        match self
            .delete_aws_event_bridge_source_with_http_info(params)
            .await
        {
            Ok(response_content) => Ok(response_content.entity),
            Err(err) => Err(err),
        }
    }

    /// Delete an Amazon EventBridge source.
    pub async fn delete_aws_event_bridge_source_with_http_info(
        &self,
        params: DeleteAWSEventBridgeSourceParams,
    ) -> Result<
        ResponseContent<crate::datadogV1::model::AWSEventBridgeDeleteResponse>,
        Error<DeleteAWSEventBridgeSourceError>,
    > {
        let local_configuration = &self.config;

        // unbox and build parameters
        let body = params.body;

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
            let local_entity: Option<crate::datadogV1::model::AWSEventBridgeDeleteResponse> =
                serde_json::from_str(&local_content).ok();
            Ok(ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            })
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
        params: DeleteAWSTagFilterParams,
    ) -> Result<
        Option<std::collections::HashMap<String, serde_json::Value>>,
        Error<DeleteAWSTagFilterError>,
    > {
        match self.delete_aws_tag_filter_with_http_info(params).await {
            Ok(response_content) => Ok(response_content.entity),
            Err(err) => Err(err),
        }
    }

    /// Delete a tag filtering entry.
    pub async fn delete_aws_tag_filter_with_http_info(
        &self,
        params: DeleteAWSTagFilterParams,
    ) -> Result<
        ResponseContent<std::collections::HashMap<String, serde_json::Value>>,
        Error<DeleteAWSTagFilterError>,
    > {
        let local_configuration = &self.config;

        // unbox and build parameters
        let body = params.body;

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
            let local_entity: Option<std::collections::HashMap<String, serde_json::Value>> =
                serde_json::from_str(&local_content).ok();
            Ok(ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            })
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
        params: ListAWSAccountsParams,
    ) -> Result<Option<crate::datadogV1::model::AWSAccountListResponse>, Error<ListAWSAccountsError>>
    {
        match self.list_aws_accounts_with_http_info(params).await {
            Ok(response_content) => Ok(response_content.entity),
            Err(err) => Err(err),
        }
    }

    /// List all Datadog-AWS integrations available in your Datadog organization.
    pub async fn list_aws_accounts_with_http_info(
        &self,
        params: ListAWSAccountsParams,
    ) -> Result<
        ResponseContent<crate::datadogV1::model::AWSAccountListResponse>,
        Error<ListAWSAccountsError>,
    > {
        let local_configuration = &self.config;

        // unbox and build parameters
        let account_id = params.account_id;
        let role_name = params.role_name;
        let access_key_id = params.access_key_id;

        let local_client = &local_configuration.client;

        let local_uri_str = format!("{}/api/v1/integration/aws", local_configuration.base_path);
        let mut local_req_builder =
            local_client.request(reqwest::Method::GET, local_uri_str.as_str());

        if let Some(ref local_str) = account_id {
            local_req_builder = local_req_builder.query(&[("account_id", &local_str.to_string())]);
        };
        if let Some(ref local_str) = role_name {
            local_req_builder = local_req_builder.query(&[("role_name", &local_str.to_string())]);
        };
        if let Some(ref local_str) = access_key_id {
            local_req_builder =
                local_req_builder.query(&[("access_key_id", &local_str.to_string())]);
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
            let local_entity: Option<crate::datadogV1::model::AWSAccountListResponse> =
                serde_json::from_str(&local_content).ok();
            Ok(ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            })
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
        Option<crate::datadogV1::model::AWSEventBridgeListResponse>,
        Error<ListAWSEventBridgeSourcesError>,
    > {
        match self.list_aws_event_bridge_sources_with_http_info().await {
            Ok(response_content) => Ok(response_content.entity),
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

        // unbox and build parameters

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
            let local_entity: Option<crate::datadogV1::model::AWSEventBridgeListResponse> =
                serde_json::from_str(&local_content).ok();
            Ok(ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            })
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
        params: ListAWSTagFiltersParams,
    ) -> Result<
        Option<crate::datadogV1::model::AWSTagFilterListResponse>,
        Error<ListAWSTagFiltersError>,
    > {
        match self.list_aws_tag_filters_with_http_info(params).await {
            Ok(response_content) => Ok(response_content.entity),
            Err(err) => Err(err),
        }
    }

    /// Get all AWS tag filters.
    pub async fn list_aws_tag_filters_with_http_info(
        &self,
        params: ListAWSTagFiltersParams,
    ) -> Result<
        ResponseContent<crate::datadogV1::model::AWSTagFilterListResponse>,
        Error<ListAWSTagFiltersError>,
    > {
        let local_configuration = &self.config;

        // unbox and build parameters
        let account_id = params.account_id;

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
            let local_entity: Option<crate::datadogV1::model::AWSTagFilterListResponse> =
                serde_json::from_str(&local_content).ok();
            Ok(ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            })
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
    ) -> Result<Option<Vec<String>>, Error<ListAvailableAWSNamespacesError>> {
        match self.list_available_aws_namespaces_with_http_info().await {
            Ok(response_content) => Ok(response_content.entity),
            Err(err) => Err(err),
        }
    }

    /// List all namespace rules for a given Datadog-AWS integration. This endpoint takes no arguments.
    pub async fn list_available_aws_namespaces_with_http_info(
        &self,
    ) -> Result<ResponseContent<Vec<String>>, Error<ListAvailableAWSNamespacesError>> {
        let local_configuration = &self.config;

        // unbox and build parameters

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
            let local_entity: Option<Vec<String>> = serde_json::from_str(&local_content).ok();
            Ok(ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            })
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
        params: UpdateAWSAccountParams,
    ) -> Result<
        Option<std::collections::HashMap<String, serde_json::Value>>,
        Error<UpdateAWSAccountError>,
    > {
        match self.update_aws_account_with_http_info(params).await {
            Ok(response_content) => Ok(response_content.entity),
            Err(err) => Err(err),
        }
    }

    /// Update a Datadog-Amazon Web Services integration.
    pub async fn update_aws_account_with_http_info(
        &self,
        params: UpdateAWSAccountParams,
    ) -> Result<
        ResponseContent<std::collections::HashMap<String, serde_json::Value>>,
        Error<UpdateAWSAccountError>,
    > {
        let local_configuration = &self.config;

        // unbox and build parameters
        let body = params.body;
        let account_id = params.account_id;
        let role_name = params.role_name;
        let access_key_id = params.access_key_id;

        let local_client = &local_configuration.client;

        let local_uri_str = format!("{}/api/v1/integration/aws", local_configuration.base_path);
        let mut local_req_builder =
            local_client.request(reqwest::Method::PUT, local_uri_str.as_str());

        if let Some(ref local_str) = account_id {
            local_req_builder = local_req_builder.query(&[("account_id", &local_str.to_string())]);
        };
        if let Some(ref local_str) = role_name {
            local_req_builder = local_req_builder.query(&[("role_name", &local_str.to_string())]);
        };
        if let Some(ref local_str) = access_key_id {
            local_req_builder =
                local_req_builder.query(&[("access_key_id", &local_str.to_string())]);
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
            let local_entity: Option<std::collections::HashMap<String, serde_json::Value>> =
                serde_json::from_str(&local_content).ok();
            Ok(ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            })
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
