// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use crate::datadog::*;
use reqwest;
use serde::{Deserialize, Serialize};

/// CreateSlackIntegrationChannelError is a struct for typed errors of method [`SlackIntegrationAPI::create_slack_integration_channel`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CreateSlackIntegrationChannelError {
    Status400(Option<crate::datadogV1::model::APIErrorResponse>),
    Status403(Option<crate::datadogV1::model::APIErrorResponse>),
    Status404(Option<crate::datadogV1::model::APIErrorResponse>),
    Status429(Option<crate::datadogV1::model::APIErrorResponse>),
    UnknownValue(serde_json::Value),
}

/// GetSlackIntegrationChannelError is a struct for typed errors of method [`SlackIntegrationAPI::get_slack_integration_channel`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetSlackIntegrationChannelError {
    Status400(Option<crate::datadogV1::model::APIErrorResponse>),
    Status403(Option<crate::datadogV1::model::APIErrorResponse>),
    Status404(Option<crate::datadogV1::model::APIErrorResponse>),
    Status429(Option<crate::datadogV1::model::APIErrorResponse>),
    UnknownValue(serde_json::Value),
}

/// GetSlackIntegrationChannelsError is a struct for typed errors of method [`SlackIntegrationAPI::get_slack_integration_channels`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetSlackIntegrationChannelsError {
    Status400(Option<crate::datadogV1::model::APIErrorResponse>),
    Status403(Option<crate::datadogV1::model::APIErrorResponse>),
    Status404(Option<crate::datadogV1::model::APIErrorResponse>),
    Status429(Option<crate::datadogV1::model::APIErrorResponse>),
    UnknownValue(serde_json::Value),
}

/// RemoveSlackIntegrationChannelError is a struct for typed errors of method [`SlackIntegrationAPI::remove_slack_integration_channel`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum RemoveSlackIntegrationChannelError {
    Status400(Option<crate::datadogV1::model::APIErrorResponse>),
    Status403(Option<crate::datadogV1::model::APIErrorResponse>),
    Status404(Option<crate::datadogV1::model::APIErrorResponse>),
    Status429(Option<crate::datadogV1::model::APIErrorResponse>),
    UnknownValue(serde_json::Value),
}

/// UpdateSlackIntegrationChannelError is a struct for typed errors of method [`SlackIntegrationAPI::update_slack_integration_channel`]
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
        account_name: String,
        body: crate::datadogV1::model::SlackIntegrationChannel,
    ) -> Result<
        crate::datadogV1::model::SlackIntegrationChannel,
        Error<CreateSlackIntegrationChannelError>,
    > {
        match self
            .create_slack_integration_channel_with_http_info(account_name, body)
            .await
        {
            Ok(response_content) => Ok(response_content.entity.unwrap()),
            Err(err) => Err(err),
        }
    }

    /// Add a channel to your Datadog-Slack integration.
    pub async fn create_slack_integration_channel_with_http_info(
        &self,
        account_name: String,
        body: crate::datadogV1::model::SlackIntegrationChannel,
    ) -> Result<
        ResponseContent<crate::datadogV1::model::SlackIntegrationChannel>,
        Error<CreateSlackIntegrationChannelError>,
    > {
        let local_configuration = &self.config;
        let operation_id = "v1.create_slack_integration_channel";

        let local_client = &local_configuration.client;

        let local_uri_str = format!(
            "{}/api/v1/integration/slack/configuration/accounts/{account_name}/channels",
            local_configuration.get_operation_host(operation_id),
            account_name = urlencode(account_name)
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
            match serde_json::from_str::<crate::datadogV1::model::SlackIntegrationChannel>(
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
        account_name: String,
        channel_name: String,
    ) -> Result<
        crate::datadogV1::model::SlackIntegrationChannel,
        Error<GetSlackIntegrationChannelError>,
    > {
        match self
            .get_slack_integration_channel_with_http_info(account_name, channel_name)
            .await
        {
            Ok(response_content) => Ok(response_content.entity.unwrap()),
            Err(err) => Err(err),
        }
    }

    /// Get a channel configured for your Datadog-Slack integration.
    pub async fn get_slack_integration_channel_with_http_info(
        &self,
        account_name: String,
        channel_name: String,
    ) -> Result<
        ResponseContent<crate::datadogV1::model::SlackIntegrationChannel>,
        Error<GetSlackIntegrationChannelError>,
    > {
        let local_configuration = &self.config;
        let operation_id = "v1.get_slack_integration_channel";

        let local_client = &local_configuration.client;

        let local_uri_str = format!(
            "{}/api/v1/integration/slack/configuration/accounts/{account_name}/channels/{channel_name}",
            local_configuration.get_operation_host(operation_id), account_name=
            urlencode(account_name)
            , channel_name=
            urlencode(channel_name)
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
            match serde_json::from_str::<crate::datadogV1::model::SlackIntegrationChannel>(
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
        account_name: String,
    ) -> Result<
        Vec<crate::datadogV1::model::SlackIntegrationChannel>,
        Error<GetSlackIntegrationChannelsError>,
    > {
        match self
            .get_slack_integration_channels_with_http_info(account_name)
            .await
        {
            Ok(response_content) => Ok(response_content.entity.unwrap()),
            Err(err) => Err(err),
        }
    }

    /// Get a list of all channels configured for your Datadog-Slack integration.
    pub async fn get_slack_integration_channels_with_http_info(
        &self,
        account_name: String,
    ) -> Result<
        ResponseContent<Vec<crate::datadogV1::model::SlackIntegrationChannel>>,
        Error<GetSlackIntegrationChannelsError>,
    > {
        let local_configuration = &self.config;
        let operation_id = "v1.get_slack_integration_channels";

        let local_client = &local_configuration.client;

        let local_uri_str = format!(
            "{}/api/v1/integration/slack/configuration/accounts/{account_name}/channels",
            local_configuration.get_operation_host(operation_id),
            account_name = urlencode(account_name)
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
            match serde_json::from_str::<Vec<crate::datadogV1::model::SlackIntegrationChannel>>(
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
        account_name: String,
        channel_name: String,
    ) -> Result<(), Error<RemoveSlackIntegrationChannelError>> {
        match self
            .remove_slack_integration_channel_with_http_info(account_name, channel_name)
            .await
        {
            Ok(_) => Ok(()),
            Err(err) => Err(err),
        }
    }

    /// Remove a channel from your Datadog-Slack integration.
    pub async fn remove_slack_integration_channel_with_http_info(
        &self,
        account_name: String,
        channel_name: String,
    ) -> Result<ResponseContent<()>, Error<RemoveSlackIntegrationChannelError>> {
        let local_configuration = &self.config;
        let operation_id = "v1.remove_slack_integration_channel";

        let local_client = &local_configuration.client;

        let local_uri_str = format!(
            "{}/api/v1/integration/slack/configuration/accounts/{account_name}/channels/{channel_name}",
            local_configuration.get_operation_host(operation_id), account_name=
            urlencode(account_name)
            , channel_name=
            urlencode(channel_name)
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
        account_name: String,
        channel_name: String,
        body: crate::datadogV1::model::SlackIntegrationChannel,
    ) -> Result<
        crate::datadogV1::model::SlackIntegrationChannel,
        Error<UpdateSlackIntegrationChannelError>,
    > {
        match self
            .update_slack_integration_channel_with_http_info(account_name, channel_name, body)
            .await
        {
            Ok(response_content) => Ok(response_content.entity.unwrap()),
            Err(err) => Err(err),
        }
    }

    /// Update a channel used in your Datadog-Slack integration.
    pub async fn update_slack_integration_channel_with_http_info(
        &self,
        account_name: String,
        channel_name: String,
        body: crate::datadogV1::model::SlackIntegrationChannel,
    ) -> Result<
        ResponseContent<crate::datadogV1::model::SlackIntegrationChannel>,
        Error<UpdateSlackIntegrationChannelError>,
    > {
        let local_configuration = &self.config;
        let operation_id = "v1.update_slack_integration_channel";

        let local_client = &local_configuration.client;

        let local_uri_str = format!(
            "{}/api/v1/integration/slack/configuration/accounts/{account_name}/channels/{channel_name}",
            local_configuration.get_operation_host(operation_id), account_name=
            urlencode(account_name)
            , channel_name=
            urlencode(channel_name)
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
            match serde_json::from_str::<crate::datadogV1::model::SlackIntegrationChannel>(
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
