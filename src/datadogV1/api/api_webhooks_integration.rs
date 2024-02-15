// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use crate::datadog::*;
use reqwest;
use serde::{Deserialize, Serialize};

/// CreateWebhooksIntegrationError is a struct for typed errors of method [`WebhooksIntegrationAPI::create_webhooks_integration`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CreateWebhooksIntegrationError {
    Status400(Option<crate::datadogV1::model::APIErrorResponse>),
    Status403(Option<crate::datadogV1::model::APIErrorResponse>),
    Status429(Option<crate::datadogV1::model::APIErrorResponse>),
    UnknownValue(serde_json::Value),
}

/// CreateWebhooksIntegrationCustomVariableError is a struct for typed errors of method [`WebhooksIntegrationAPI::create_webhooks_integration_custom_variable`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CreateWebhooksIntegrationCustomVariableError {
    Status400(Option<crate::datadogV1::model::APIErrorResponse>),
    Status403(Option<crate::datadogV1::model::APIErrorResponse>),
    Status429(Option<crate::datadogV1::model::APIErrorResponse>),
    UnknownValue(serde_json::Value),
}

/// DeleteWebhooksIntegrationError is a struct for typed errors of method [`WebhooksIntegrationAPI::delete_webhooks_integration`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DeleteWebhooksIntegrationError {
    Status403(Option<crate::datadogV1::model::APIErrorResponse>),
    Status404(Option<crate::datadogV1::model::APIErrorResponse>),
    Status429(Option<crate::datadogV1::model::APIErrorResponse>),
    UnknownValue(serde_json::Value),
}

/// DeleteWebhooksIntegrationCustomVariableError is a struct for typed errors of method [`WebhooksIntegrationAPI::delete_webhooks_integration_custom_variable`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DeleteWebhooksIntegrationCustomVariableError {
    Status403(Option<crate::datadogV1::model::APIErrorResponse>),
    Status404(Option<crate::datadogV1::model::APIErrorResponse>),
    Status429(Option<crate::datadogV1::model::APIErrorResponse>),
    UnknownValue(serde_json::Value),
}

/// GetWebhooksIntegrationError is a struct for typed errors of method [`WebhooksIntegrationAPI::get_webhooks_integration`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetWebhooksIntegrationError {
    Status400(Option<crate::datadogV1::model::APIErrorResponse>),
    Status403(Option<crate::datadogV1::model::APIErrorResponse>),
    Status404(Option<crate::datadogV1::model::APIErrorResponse>),
    Status429(Option<crate::datadogV1::model::APIErrorResponse>),
    UnknownValue(serde_json::Value),
}

/// GetWebhooksIntegrationCustomVariableError is a struct for typed errors of method [`WebhooksIntegrationAPI::get_webhooks_integration_custom_variable`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetWebhooksIntegrationCustomVariableError {
    Status400(Option<crate::datadogV1::model::APIErrorResponse>),
    Status403(Option<crate::datadogV1::model::APIErrorResponse>),
    Status404(Option<crate::datadogV1::model::APIErrorResponse>),
    Status429(Option<crate::datadogV1::model::APIErrorResponse>),
    UnknownValue(serde_json::Value),
}

/// UpdateWebhooksIntegrationError is a struct for typed errors of method [`WebhooksIntegrationAPI::update_webhooks_integration`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum UpdateWebhooksIntegrationError {
    Status400(Option<crate::datadogV1::model::APIErrorResponse>),
    Status403(Option<crate::datadogV1::model::APIErrorResponse>),
    Status404(Option<crate::datadogV1::model::APIErrorResponse>),
    Status429(Option<crate::datadogV1::model::APIErrorResponse>),
    UnknownValue(serde_json::Value),
}

/// UpdateWebhooksIntegrationCustomVariableError is a struct for typed errors of method [`WebhooksIntegrationAPI::update_webhooks_integration_custom_variable`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum UpdateWebhooksIntegrationCustomVariableError {
    Status400(Option<crate::datadogV1::model::APIErrorResponse>),
    Status403(Option<crate::datadogV1::model::APIErrorResponse>),
    Status404(Option<crate::datadogV1::model::APIErrorResponse>),
    Status429(Option<crate::datadogV1::model::APIErrorResponse>),
    UnknownValue(serde_json::Value),
}

#[derive(Debug, Clone)]
pub struct WebhooksIntegrationAPI {
    config: configuration::Configuration,
}

impl Default for WebhooksIntegrationAPI {
    fn default() -> Self {
        Self {
            config: configuration::Configuration::new(),
        }
    }
}

impl WebhooksIntegrationAPI {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn with_config(config: configuration::Configuration) -> Self {
        Self { config }
    }

    /// Creates an endpoint with the name `<WEBHOOK_NAME>`.
    pub async fn create_webhooks_integration(
        &self,
        body: crate::datadogV1::model::WebhooksIntegration,
    ) -> Result<
        Option<crate::datadogV1::model::WebhooksIntegration>,
        Error<CreateWebhooksIntegrationError>,
    > {
        match self.create_webhooks_integration_with_http_info(body).await {
            Ok(response_content) => Ok(response_content.entity),
            Err(err) => Err(err),
        }
    }

    /// Creates an endpoint with the name `<WEBHOOK_NAME>`.
    pub async fn create_webhooks_integration_with_http_info(
        &self,
        body: crate::datadogV1::model::WebhooksIntegration,
    ) -> Result<
        ResponseContent<crate::datadogV1::model::WebhooksIntegration>,
        Error<CreateWebhooksIntegrationError>,
    > {
        let local_configuration = &self.config;
        let operation_id = "v1.create_webhooks_integration";

        let local_client = &local_configuration.client;

        let local_uri_str = format!(
            "{}/api/v1/integration/webhooks/configuration/webhooks",
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
            let local_entity: Option<crate::datadogV1::model::WebhooksIntegration> =
                serde_json::from_str(&local_content).ok();
            Ok(ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            })
        } else {
            let local_entity: Option<CreateWebhooksIntegrationError> =
                serde_json::from_str(&local_content).ok();
            let local_error = ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(Error::ResponseError(local_error))
        }
    }

    /// Creates an endpoint with the name `<CUSTOM_VARIABLE_NAME>`.
    pub async fn create_webhooks_integration_custom_variable(
        &self,
        body: crate::datadogV1::model::WebhooksIntegrationCustomVariable,
    ) -> Result<
        Option<crate::datadogV1::model::WebhooksIntegrationCustomVariableResponse>,
        Error<CreateWebhooksIntegrationCustomVariableError>,
    > {
        match self
            .create_webhooks_integration_custom_variable_with_http_info(body)
            .await
        {
            Ok(response_content) => Ok(response_content.entity),
            Err(err) => Err(err),
        }
    }

    /// Creates an endpoint with the name `<CUSTOM_VARIABLE_NAME>`.
    pub async fn create_webhooks_integration_custom_variable_with_http_info(
        &self,
        body: crate::datadogV1::model::WebhooksIntegrationCustomVariable,
    ) -> Result<
        ResponseContent<crate::datadogV1::model::WebhooksIntegrationCustomVariableResponse>,
        Error<CreateWebhooksIntegrationCustomVariableError>,
    > {
        let local_configuration = &self.config;
        let operation_id = "v1.create_webhooks_integration_custom_variable";

        let local_client = &local_configuration.client;

        let local_uri_str = format!(
            "{}/api/v1/integration/webhooks/configuration/custom-variables",
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
            let local_entity: Option<
                crate::datadogV1::model::WebhooksIntegrationCustomVariableResponse,
            > = serde_json::from_str(&local_content).ok();
            Ok(ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            })
        } else {
            let local_entity: Option<CreateWebhooksIntegrationCustomVariableError> =
                serde_json::from_str(&local_content).ok();
            let local_error = ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(Error::ResponseError(local_error))
        }
    }

    /// Deletes the endpoint with the name `<WEBHOOK NAME>`.
    pub async fn delete_webhooks_integration(
        &self,
        webhook_name: String,
    ) -> Result<Option<()>, Error<DeleteWebhooksIntegrationError>> {
        match self
            .delete_webhooks_integration_with_http_info(webhook_name)
            .await
        {
            Ok(response_content) => Ok(response_content.entity),
            Err(err) => Err(err),
        }
    }

    /// Deletes the endpoint with the name `<WEBHOOK NAME>`.
    pub async fn delete_webhooks_integration_with_http_info(
        &self,
        webhook_name: String,
    ) -> Result<ResponseContent<()>, Error<DeleteWebhooksIntegrationError>> {
        let local_configuration = &self.config;
        let operation_id = "v1.delete_webhooks_integration";

        let local_client = &local_configuration.client;

        let local_uri_str = format!(
            "{}/api/v1/integration/webhooks/configuration/webhooks/{webhook_name}",
            local_configuration.get_operation_host(operation_id),
            webhook_name = urlencode(webhook_name)
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
            let local_entity: Option<DeleteWebhooksIntegrationError> =
                serde_json::from_str(&local_content).ok();
            let local_error = ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(Error::ResponseError(local_error))
        }
    }

    /// Deletes the endpoint with the name `<CUSTOM_VARIABLE_NAME>`.
    pub async fn delete_webhooks_integration_custom_variable(
        &self,
        custom_variable_name: String,
    ) -> Result<Option<()>, Error<DeleteWebhooksIntegrationCustomVariableError>> {
        match self
            .delete_webhooks_integration_custom_variable_with_http_info(custom_variable_name)
            .await
        {
            Ok(response_content) => Ok(response_content.entity),
            Err(err) => Err(err),
        }
    }

    /// Deletes the endpoint with the name `<CUSTOM_VARIABLE_NAME>`.
    pub async fn delete_webhooks_integration_custom_variable_with_http_info(
        &self,
        custom_variable_name: String,
    ) -> Result<ResponseContent<()>, Error<DeleteWebhooksIntegrationCustomVariableError>> {
        let local_configuration = &self.config;
        let operation_id = "v1.delete_webhooks_integration_custom_variable";

        let local_client = &local_configuration.client;

        let local_uri_str = format!(
            "{}/api/v1/integration/webhooks/configuration/custom-variables/{custom_variable_name}",
            local_configuration.get_operation_host(operation_id),
            custom_variable_name = urlencode(custom_variable_name)
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
            let local_entity: Option<DeleteWebhooksIntegrationCustomVariableError> =
                serde_json::from_str(&local_content).ok();
            let local_error = ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(Error::ResponseError(local_error))
        }
    }

    /// Gets the content of the webhook with the name `<WEBHOOK_NAME>`.
    pub async fn get_webhooks_integration(
        &self,
        webhook_name: String,
    ) -> Result<
        Option<crate::datadogV1::model::WebhooksIntegration>,
        Error<GetWebhooksIntegrationError>,
    > {
        match self
            .get_webhooks_integration_with_http_info(webhook_name)
            .await
        {
            Ok(response_content) => Ok(response_content.entity),
            Err(err) => Err(err),
        }
    }

    /// Gets the content of the webhook with the name `<WEBHOOK_NAME>`.
    pub async fn get_webhooks_integration_with_http_info(
        &self,
        webhook_name: String,
    ) -> Result<
        ResponseContent<crate::datadogV1::model::WebhooksIntegration>,
        Error<GetWebhooksIntegrationError>,
    > {
        let local_configuration = &self.config;
        let operation_id = "v1.get_webhooks_integration";

        let local_client = &local_configuration.client;

        let local_uri_str = format!(
            "{}/api/v1/integration/webhooks/configuration/webhooks/{webhook_name}",
            local_configuration.get_operation_host(operation_id),
            webhook_name = urlencode(webhook_name)
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
            let local_entity: Option<crate::datadogV1::model::WebhooksIntegration> =
                serde_json::from_str(&local_content).ok();
            Ok(ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            })
        } else {
            let local_entity: Option<GetWebhooksIntegrationError> =
                serde_json::from_str(&local_content).ok();
            let local_error = ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(Error::ResponseError(local_error))
        }
    }

    /// Shows the content of the custom variable with the name `<CUSTOM_VARIABLE_NAME>`.
    ///
    /// If the custom variable is secret, the value does not return in the
    /// response payload.
    pub async fn get_webhooks_integration_custom_variable(
        &self,
        custom_variable_name: String,
    ) -> Result<
        Option<crate::datadogV1::model::WebhooksIntegrationCustomVariableResponse>,
        Error<GetWebhooksIntegrationCustomVariableError>,
    > {
        match self
            .get_webhooks_integration_custom_variable_with_http_info(custom_variable_name)
            .await
        {
            Ok(response_content) => Ok(response_content.entity),
            Err(err) => Err(err),
        }
    }

    /// Shows the content of the custom variable with the name `<CUSTOM_VARIABLE_NAME>`.
    ///
    /// If the custom variable is secret, the value does not return in the
    /// response payload.
    pub async fn get_webhooks_integration_custom_variable_with_http_info(
        &self,
        custom_variable_name: String,
    ) -> Result<
        ResponseContent<crate::datadogV1::model::WebhooksIntegrationCustomVariableResponse>,
        Error<GetWebhooksIntegrationCustomVariableError>,
    > {
        let local_configuration = &self.config;
        let operation_id = "v1.get_webhooks_integration_custom_variable";

        let local_client = &local_configuration.client;

        let local_uri_str = format!(
            "{}/api/v1/integration/webhooks/configuration/custom-variables/{custom_variable_name}",
            local_configuration.get_operation_host(operation_id),
            custom_variable_name = urlencode(custom_variable_name)
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
            let local_entity: Option<
                crate::datadogV1::model::WebhooksIntegrationCustomVariableResponse,
            > = serde_json::from_str(&local_content).ok();
            Ok(ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            })
        } else {
            let local_entity: Option<GetWebhooksIntegrationCustomVariableError> =
                serde_json::from_str(&local_content).ok();
            let local_error = ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(Error::ResponseError(local_error))
        }
    }

    /// Updates the endpoint with the name `<WEBHOOK_NAME>`.
    pub async fn update_webhooks_integration(
        &self,
        webhook_name: String,
        body: crate::datadogV1::model::WebhooksIntegrationUpdateRequest,
    ) -> Result<
        Option<crate::datadogV1::model::WebhooksIntegration>,
        Error<UpdateWebhooksIntegrationError>,
    > {
        match self
            .update_webhooks_integration_with_http_info(webhook_name, body)
            .await
        {
            Ok(response_content) => Ok(response_content.entity),
            Err(err) => Err(err),
        }
    }

    /// Updates the endpoint with the name `<WEBHOOK_NAME>`.
    pub async fn update_webhooks_integration_with_http_info(
        &self,
        webhook_name: String,
        body: crate::datadogV1::model::WebhooksIntegrationUpdateRequest,
    ) -> Result<
        ResponseContent<crate::datadogV1::model::WebhooksIntegration>,
        Error<UpdateWebhooksIntegrationError>,
    > {
        let local_configuration = &self.config;
        let operation_id = "v1.update_webhooks_integration";

        let local_client = &local_configuration.client;

        let local_uri_str = format!(
            "{}/api/v1/integration/webhooks/configuration/webhooks/{webhook_name}",
            local_configuration.get_operation_host(operation_id),
            webhook_name = urlencode(webhook_name)
        );
        let mut local_req_builder =
            local_client.request(reqwest::Method::PUT, local_uri_str.as_str());

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
            let local_entity: Option<crate::datadogV1::model::WebhooksIntegration> =
                serde_json::from_str(&local_content).ok();
            Ok(ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            })
        } else {
            let local_entity: Option<UpdateWebhooksIntegrationError> =
                serde_json::from_str(&local_content).ok();
            let local_error = ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(Error::ResponseError(local_error))
        }
    }

    /// Updates the endpoint with the name `<CUSTOM_VARIABLE_NAME>`.
    pub async fn update_webhooks_integration_custom_variable(
        &self,
        custom_variable_name: String,
        body: crate::datadogV1::model::WebhooksIntegrationCustomVariableUpdateRequest,
    ) -> Result<
        Option<crate::datadogV1::model::WebhooksIntegrationCustomVariableResponse>,
        Error<UpdateWebhooksIntegrationCustomVariableError>,
    > {
        match self
            .update_webhooks_integration_custom_variable_with_http_info(custom_variable_name, body)
            .await
        {
            Ok(response_content) => Ok(response_content.entity),
            Err(err) => Err(err),
        }
    }

    /// Updates the endpoint with the name `<CUSTOM_VARIABLE_NAME>`.
    pub async fn update_webhooks_integration_custom_variable_with_http_info(
        &self,
        custom_variable_name: String,
        body: crate::datadogV1::model::WebhooksIntegrationCustomVariableUpdateRequest,
    ) -> Result<
        ResponseContent<crate::datadogV1::model::WebhooksIntegrationCustomVariableResponse>,
        Error<UpdateWebhooksIntegrationCustomVariableError>,
    > {
        let local_configuration = &self.config;
        let operation_id = "v1.update_webhooks_integration_custom_variable";

        let local_client = &local_configuration.client;

        let local_uri_str = format!(
            "{}/api/v1/integration/webhooks/configuration/custom-variables/{custom_variable_name}",
            local_configuration.get_operation_host(operation_id),
            custom_variable_name = urlencode(custom_variable_name)
        );
        let mut local_req_builder =
            local_client.request(reqwest::Method::PUT, local_uri_str.as_str());

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
            let local_entity: Option<
                crate::datadogV1::model::WebhooksIntegrationCustomVariableResponse,
            > = serde_json::from_str(&local_content).ok();
            Ok(ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            })
        } else {
            let local_entity: Option<UpdateWebhooksIntegrationCustomVariableError> =
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
