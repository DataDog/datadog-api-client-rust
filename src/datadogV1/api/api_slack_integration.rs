// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

use crate::datadog::*;
use reqwest;
use serde::{Deserialize, Serialize};

/// CreateSlackIntegrationChannelParams is a struct for passing parameters to the method [`CreateSlackIntegrationChannel`]
#[derive(Clone, Debug, Default)]
pub struct CreateSlackIntegrationChannelParams {
    /// Your Slack account name.
    pub account_name: String,
    /// Payload describing Slack channel to be created
    pub body: crate::datadogV1::model::SlackIntegrationChannel,
}

/// GetSlackIntegrationChannelParams is a struct for passing parameters to the method [`GetSlackIntegrationChannel`]
#[derive(Clone, Debug, Default)]
pub struct GetSlackIntegrationChannelParams {
    /// Your Slack account name.
    pub account_name: String,
    /// The name of the Slack channel being operated on.
    pub channel_name: String,
}

/// GetSlackIntegrationChannelsParams is a struct for passing parameters to the method [`GetSlackIntegrationChannels`]
#[derive(Clone, Debug, Default)]
pub struct GetSlackIntegrationChannelsParams {
    /// Your Slack account name.
    pub account_name: String,
}

/// RemoveSlackIntegrationChannelParams is a struct for passing parameters to the method [`RemoveSlackIntegrationChannel`]
#[derive(Clone, Debug, Default)]
pub struct RemoveSlackIntegrationChannelParams {
    /// Your Slack account name.
    pub account_name: String,
    /// The name of the Slack channel being operated on.
    pub channel_name: String,
}

/// UpdateSlackIntegrationChannelParams is a struct for passing parameters to the method [`UpdateSlackIntegrationChannel`]
#[derive(Clone, Debug, Default)]
pub struct UpdateSlackIntegrationChannelParams {
    /// Your Slack account name.
    pub account_name: String,
    /// The name of the Slack channel being operated on.
    pub channel_name: String,
    /// Payload describing fields and values to be updated.
    pub body: crate::datadogV1::model::SlackIntegrationChannel,
}

/// CreateSlackIntegrationChannelError is a struct for typed errors of method [`CreateSlackIntegrationChannel`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CreateSlackIntegrationChannelError {
    Status400(Option<crate::datadogV1::model::APIErrorResponse>),
    Status403(Option<crate::datadogV1::model::APIErrorResponse>),
    Status404(Option<crate::datadogV1::model::APIErrorResponse>),
    Status429(Option<crate::datadogV1::model::APIErrorResponse>),
    UnknownValue(serde_json::Value),
}

/// GetSlackIntegrationChannelError is a struct for typed errors of method [`GetSlackIntegrationChannel`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetSlackIntegrationChannelError {
    Status400(Option<crate::datadogV1::model::APIErrorResponse>),
    Status403(Option<crate::datadogV1::model::APIErrorResponse>),
    Status404(Option<crate::datadogV1::model::APIErrorResponse>),
    Status429(Option<crate::datadogV1::model::APIErrorResponse>),
    UnknownValue(serde_json::Value),
}

/// GetSlackIntegrationChannelsError is a struct for typed errors of method [`GetSlackIntegrationChannels`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetSlackIntegrationChannelsError {
    Status400(Option<crate::datadogV1::model::APIErrorResponse>),
    Status403(Option<crate::datadogV1::model::APIErrorResponse>),
    Status404(Option<crate::datadogV1::model::APIErrorResponse>),
    Status429(Option<crate::datadogV1::model::APIErrorResponse>),
    UnknownValue(serde_json::Value),
}

/// RemoveSlackIntegrationChannelError is a struct for typed errors of method [`RemoveSlackIntegrationChannel`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum RemoveSlackIntegrationChannelError {
    Status400(Option<crate::datadogV1::model::APIErrorResponse>),
    Status403(Option<crate::datadogV1::model::APIErrorResponse>),
    Status404(Option<crate::datadogV1::model::APIErrorResponse>),
    Status429(Option<crate::datadogV1::model::APIErrorResponse>),
    UnknownValue(serde_json::Value),
}

/// UpdateSlackIntegrationChannelError is a struct for typed errors of method [`UpdateSlackIntegrationChannel`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum UpdateSlackIntegrationChannelError {
    Status400(Option<crate::datadogV1::model::APIErrorResponse>),
    Status403(Option<crate::datadogV1::model::APIErrorResponse>),
    Status404(Option<crate::datadogV1::model::APIErrorResponse>),
    Status429(Option<crate::datadogV1::model::APIErrorResponse>),
    UnknownValue(serde_json::Value),
}

#[derive(Debug, Clone)]
pub struct SlackIntegrationAPI {
    config: configuration::Configuration,
}

impl Default for SlackIntegrationAPI {
    fn default() -> Self {
        Self {
            config: configuration::Configuration::new(),
        }
    }
}

impl SlackIntegrationAPI {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn with_config(config: configuration::Configuration) -> Self {
        Self { config }
    }

    /// Add a channel to your Datadog-Slack integration.
    pub async fn create_slack_integration_channel(
        &self,
        params: CreateSlackIntegrationChannelParams,
    ) -> Result<
        Option<crate::datadogV1::model::SlackIntegrationChannel>,
        Error<CreateSlackIntegrationChannelError>,
    > {
        match self
            .create_slack_integration_channel_with_http_info(params)
            .await
        {
            Ok(response_content) => Ok(response_content.entity),
            Err(err) => Err(err),
        }
    }

    /// Add a channel to your Datadog-Slack integration.
    pub async fn create_slack_integration_channel_with_http_info(
        &self,
        params: CreateSlackIntegrationChannelParams,
    ) -> Result<
        ResponseContent<crate::datadogV1::model::SlackIntegrationChannel>,
        Error<CreateSlackIntegrationChannelError>,
    > {
        let local_configuration = &self.config;

        // unbox and build parameters
        let account_name = params.account_name;
        let body = params.body;

        let local_client = &local_configuration.client;

        let local_uri_str = format!(
            "{}/api/v1/integration/slack/configuration/accounts/{account_name}/channels",
            local_configuration.base_path,
            account_name = urlencode(account_name)
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
            let local_entity: Option<crate::datadogV1::model::SlackIntegrationChannel> =
                serde_json::from_str(&local_content).ok();
            Ok(ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            })
        } else {
            let local_entity: Option<CreateSlackIntegrationChannelError> =
                serde_json::from_str(&local_content).ok();
            let local_error = ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(Error::ResponseError(local_error))
        }
    }

    /// Get a channel configured for your Datadog-Slack integration.
    pub async fn get_slack_integration_channel(
        &self,
        params: GetSlackIntegrationChannelParams,
    ) -> Result<
        Option<crate::datadogV1::model::SlackIntegrationChannel>,
        Error<GetSlackIntegrationChannelError>,
    > {
        match self
            .get_slack_integration_channel_with_http_info(params)
            .await
        {
            Ok(response_content) => Ok(response_content.entity),
            Err(err) => Err(err),
        }
    }

    /// Get a channel configured for your Datadog-Slack integration.
    pub async fn get_slack_integration_channel_with_http_info(
        &self,
        params: GetSlackIntegrationChannelParams,
    ) -> Result<
        ResponseContent<crate::datadogV1::model::SlackIntegrationChannel>,
        Error<GetSlackIntegrationChannelError>,
    > {
        let local_configuration = &self.config;

        // unbox and build parameters
        let account_name = params.account_name;
        let channel_name = params.channel_name;

        let local_client = &local_configuration.client;

        let local_uri_str = format!(
            "{}/api/v1/integration/slack/configuration/accounts/{account_name}/channels/{channel_name}", 
            local_configuration.base_path, account_name=
            urlencode(account_name)
            , channel_name=
            urlencode(channel_name)
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
            let local_entity: Option<crate::datadogV1::model::SlackIntegrationChannel> =
                serde_json::from_str(&local_content).ok();
            Ok(ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            })
        } else {
            let local_entity: Option<GetSlackIntegrationChannelError> =
                serde_json::from_str(&local_content).ok();
            let local_error = ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(Error::ResponseError(local_error))
        }
    }

    /// Get a list of all channels configured for your Datadog-Slack integration.
    pub async fn get_slack_integration_channels(
        &self,
        params: GetSlackIntegrationChannelsParams,
    ) -> Result<
        Option<Vec<crate::datadogV1::model::SlackIntegrationChannel>>,
        Error<GetSlackIntegrationChannelsError>,
    > {
        match self
            .get_slack_integration_channels_with_http_info(params)
            .await
        {
            Ok(response_content) => Ok(response_content.entity),
            Err(err) => Err(err),
        }
    }

    /// Get a list of all channels configured for your Datadog-Slack integration.
    pub async fn get_slack_integration_channels_with_http_info(
        &self,
        params: GetSlackIntegrationChannelsParams,
    ) -> Result<
        ResponseContent<Vec<crate::datadogV1::model::SlackIntegrationChannel>>,
        Error<GetSlackIntegrationChannelsError>,
    > {
        let local_configuration = &self.config;

        // unbox and build parameters
        let account_name = params.account_name;

        let local_client = &local_configuration.client;

        let local_uri_str = format!(
            "{}/api/v1/integration/slack/configuration/accounts/{account_name}/channels",
            local_configuration.base_path,
            account_name = urlencode(account_name)
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
            let local_entity: Option<Vec<crate::datadogV1::model::SlackIntegrationChannel>> =
                serde_json::from_str(&local_content).ok();
            Ok(ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            })
        } else {
            let local_entity: Option<GetSlackIntegrationChannelsError> =
                serde_json::from_str(&local_content).ok();
            let local_error = ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(Error::ResponseError(local_error))
        }
    }

    /// Remove a channel from your Datadog-Slack integration.
    pub async fn remove_slack_integration_channel(
        &self,
        params: RemoveSlackIntegrationChannelParams,
    ) -> Result<Option<()>, Error<RemoveSlackIntegrationChannelError>> {
        match self
            .remove_slack_integration_channel_with_http_info(params)
            .await
        {
            Ok(response_content) => Ok(response_content.entity),
            Err(err) => Err(err),
        }
    }

    /// Remove a channel from your Datadog-Slack integration.
    pub async fn remove_slack_integration_channel_with_http_info(
        &self,
        params: RemoveSlackIntegrationChannelParams,
    ) -> Result<ResponseContent<()>, Error<RemoveSlackIntegrationChannelError>> {
        let local_configuration = &self.config;

        // unbox and build parameters
        let account_name = params.account_name;
        let channel_name = params.channel_name;

        let local_client = &local_configuration.client;

        let local_uri_str = format!(
            "{}/api/v1/integration/slack/configuration/accounts/{account_name}/channels/{channel_name}", 
            local_configuration.base_path, account_name=
            urlencode(account_name)
            , channel_name=
            urlencode(channel_name)
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
            let local_entity: Option<RemoveSlackIntegrationChannelError> =
                serde_json::from_str(&local_content).ok();
            let local_error = ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(Error::ResponseError(local_error))
        }
    }

    /// Update a channel used in your Datadog-Slack integration.
    pub async fn update_slack_integration_channel(
        &self,
        params: UpdateSlackIntegrationChannelParams,
    ) -> Result<
        Option<crate::datadogV1::model::SlackIntegrationChannel>,
        Error<UpdateSlackIntegrationChannelError>,
    > {
        match self
            .update_slack_integration_channel_with_http_info(params)
            .await
        {
            Ok(response_content) => Ok(response_content.entity),
            Err(err) => Err(err),
        }
    }

    /// Update a channel used in your Datadog-Slack integration.
    pub async fn update_slack_integration_channel_with_http_info(
        &self,
        params: UpdateSlackIntegrationChannelParams,
    ) -> Result<
        ResponseContent<crate::datadogV1::model::SlackIntegrationChannel>,
        Error<UpdateSlackIntegrationChannelError>,
    > {
        let local_configuration = &self.config;

        // unbox and build parameters
        let account_name = params.account_name;
        let channel_name = params.channel_name;
        let body = params.body;

        let local_client = &local_configuration.client;

        let local_uri_str = format!(
            "{}/api/v1/integration/slack/configuration/accounts/{account_name}/channels/{channel_name}", 
            local_configuration.base_path, account_name=
            urlencode(account_name)
            , channel_name=
            urlencode(channel_name)
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
            let local_entity: Option<crate::datadogV1::model::SlackIntegrationChannel> =
                serde_json::from_str(&local_content).ok();
            Ok(ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            })
        } else {
            let local_entity: Option<UpdateSlackIntegrationChannelError> =
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
