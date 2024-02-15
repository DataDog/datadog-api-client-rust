// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use crate::datadog::*;
use reqwest;
use serde::{Deserialize, Serialize};

/// CheckAWSLogsLambdaAsyncError is a struct for typed errors of method [`AWSLogsIntegrationAPI::check_aws_logs_lambda_async`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CheckAWSLogsLambdaAsyncError {
    Status400(Option<crate::datadogV1::model::APIErrorResponse>),
    Status403(Option<crate::datadogV1::model::APIErrorResponse>),
    Status429(Option<crate::datadogV1::model::APIErrorResponse>),
    UnknownValue(serde_json::Value),
}

/// CheckAWSLogsServicesAsyncError is a struct for typed errors of method [`AWSLogsIntegrationAPI::check_aws_logs_services_async`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CheckAWSLogsServicesAsyncError {
    Status400(Option<crate::datadogV1::model::APIErrorResponse>),
    Status403(Option<crate::datadogV1::model::APIErrorResponse>),
    Status429(Option<crate::datadogV1::model::APIErrorResponse>),
    UnknownValue(serde_json::Value),
}

/// CreateAWSLambdaARNError is a struct for typed errors of method [`AWSLogsIntegrationAPI::create_aws_lambda_arn`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CreateAWSLambdaARNError {
    Status400(Option<crate::datadogV1::model::APIErrorResponse>),
    Status403(Option<crate::datadogV1::model::APIErrorResponse>),
    Status429(Option<crate::datadogV1::model::APIErrorResponse>),
    UnknownValue(serde_json::Value),
}

/// DeleteAWSLambdaARNError is a struct for typed errors of method [`AWSLogsIntegrationAPI::delete_aws_lambda_arn`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DeleteAWSLambdaARNError {
    Status400(Option<crate::datadogV1::model::APIErrorResponse>),
    Status403(Option<crate::datadogV1::model::APIErrorResponse>),
    Status429(Option<crate::datadogV1::model::APIErrorResponse>),
    UnknownValue(serde_json::Value),
}

/// EnableAWSLogServicesError is a struct for typed errors of method [`AWSLogsIntegrationAPI::enable_aws_log_services`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum EnableAWSLogServicesError {
    Status400(Option<crate::datadogV1::model::APIErrorResponse>),
    Status403(Option<crate::datadogV1::model::APIErrorResponse>),
    Status429(Option<crate::datadogV1::model::APIErrorResponse>),
    UnknownValue(serde_json::Value),
}

/// ListAWSLogsIntegrationsError is a struct for typed errors of method [`AWSLogsIntegrationAPI::list_aws_logs_integrations`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ListAWSLogsIntegrationsError {
    Status400(Option<crate::datadogV1::model::APIErrorResponse>),
    Status403(Option<crate::datadogV1::model::APIErrorResponse>),
    Status429(Option<crate::datadogV1::model::APIErrorResponse>),
    UnknownValue(serde_json::Value),
}

/// ListAWSLogsServicesError is a struct for typed errors of method [`AWSLogsIntegrationAPI::list_aws_logs_services`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ListAWSLogsServicesError {
    Status403(Option<crate::datadogV1::model::APIErrorResponse>),
    Status429(Option<crate::datadogV1::model::APIErrorResponse>),
    UnknownValue(serde_json::Value),
}

#[derive(Debug, Clone)]
pub struct AWSLogsIntegrationAPI {
    config: configuration::Configuration,
}

impl Default for AWSLogsIntegrationAPI {
    fn default() -> Self {
        Self {
            config: configuration::Configuration::new(),
        }
    }
}

impl AWSLogsIntegrationAPI {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn with_config(config: configuration::Configuration) -> Self {
        Self { config }
    }

    /// Test if permissions are present to add a log-forwarding triggers for the given services and AWS account. The input
    /// is the same as for Enable an AWS service log collection. Subsequent requests will always repeat the above, so this
    /// endpoint can be polled intermittently instead of blocking.
    ///
    /// - Returns a status of 'created' when it's checking if the Lambda exists in the account.
    /// - Returns a status of 'waiting' while checking.
    /// - Returns a status of 'checked and ok' if the Lambda exists.
    /// - Returns a status of 'error' if the Lambda does not exist.
    pub async fn check_aws_logs_lambda_async(
        &self,
        body: crate::datadogV1::model::AWSAccountAndLambdaRequest,
    ) -> Result<
        Option<crate::datadogV1::model::AWSLogsAsyncResponse>,
        Error<CheckAWSLogsLambdaAsyncError>,
    > {
        match self.check_aws_logs_lambda_async_with_http_info(body).await {
            Ok(response_content) => Ok(response_content.entity),
            Err(err) => Err(err),
        }
    }

    /// Test if permissions are present to add a log-forwarding triggers for the given services and AWS account. The input
    /// is the same as for Enable an AWS service log collection. Subsequent requests will always repeat the above, so this
    /// endpoint can be polled intermittently instead of blocking.
    ///
    /// - Returns a status of 'created' when it's checking if the Lambda exists in the account.
    /// - Returns a status of 'waiting' while checking.
    /// - Returns a status of 'checked and ok' if the Lambda exists.
    /// - Returns a status of 'error' if the Lambda does not exist.
    pub async fn check_aws_logs_lambda_async_with_http_info(
        &self,
        body: crate::datadogV1::model::AWSAccountAndLambdaRequest,
    ) -> Result<
        ResponseContent<crate::datadogV1::model::AWSLogsAsyncResponse>,
        Error<CheckAWSLogsLambdaAsyncError>,
    > {
        let local_configuration = &self.config;

        let local_client = &local_configuration.client;

        let local_uri_str = format!(
            "{}/api/v1/integration/aws/logs/check_async",
            local_configuration.get_operation_host("v1.check_aws_logs_lambda_async")
        );
        let mut local_req_builder =
            local_client.request(reqwest::Method::POST, local_uri_str.as_str());

        // build user agent
        local_req_builder = local_req_builder.header(
            reqwest::header::USER_AGENT,
            local_configuration.user_agent.clone(),
        );

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
            let local_entity: Option<crate::datadogV1::model::AWSLogsAsyncResponse> =
                serde_json::from_str(&local_content).ok();
            Ok(ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            })
        } else {
            let local_entity: Option<CheckAWSLogsLambdaAsyncError> =
                serde_json::from_str(&local_content).ok();
            let local_error = ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(Error::ResponseError(local_error))
        }
    }

    /// Test if permissions are present to add log-forwarding triggers for the
    /// given services and AWS account. Input is the same as for `EnableAWSLogServices`.
    /// Done async, so can be repeatedly polled in a non-blocking fashion until
    /// the async request completes.
    ///
    /// - Returns a status of `created` when it's checking if the permissions exists
    ///   in the AWS account.
    /// - Returns a status of `waiting` while checking.
    /// - Returns a status of `checked and ok` if the Lambda exists.
    /// - Returns a status of `error` if the Lambda does not exist.
    pub async fn check_aws_logs_services_async(
        &self,
        body: crate::datadogV1::model::AWSLogsServicesRequest,
    ) -> Result<
        Option<crate::datadogV1::model::AWSLogsAsyncResponse>,
        Error<CheckAWSLogsServicesAsyncError>,
    > {
        match self
            .check_aws_logs_services_async_with_http_info(body)
            .await
        {
            Ok(response_content) => Ok(response_content.entity),
            Err(err) => Err(err),
        }
    }

    /// Test if permissions are present to add log-forwarding triggers for the
    /// given services and AWS account. Input is the same as for `EnableAWSLogServices`.
    /// Done async, so can be repeatedly polled in a non-blocking fashion until
    /// the async request completes.
    ///
    /// - Returns a status of `created` when it's checking if the permissions exists
    ///   in the AWS account.
    /// - Returns a status of `waiting` while checking.
    /// - Returns a status of `checked and ok` if the Lambda exists.
    /// - Returns a status of `error` if the Lambda does not exist.
    pub async fn check_aws_logs_services_async_with_http_info(
        &self,
        body: crate::datadogV1::model::AWSLogsServicesRequest,
    ) -> Result<
        ResponseContent<crate::datadogV1::model::AWSLogsAsyncResponse>,
        Error<CheckAWSLogsServicesAsyncError>,
    > {
        let local_configuration = &self.config;

        let local_client = &local_configuration.client;

        let local_uri_str = format!(
            "{}/api/v1/integration/aws/logs/services_async",
            local_configuration.get_operation_host("v1.check_aws_logs_services_async")
        );
        let mut local_req_builder =
            local_client.request(reqwest::Method::POST, local_uri_str.as_str());

        // build user agent
        local_req_builder = local_req_builder.header(
            reqwest::header::USER_AGENT,
            local_configuration.user_agent.clone(),
        );

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
            let local_entity: Option<crate::datadogV1::model::AWSLogsAsyncResponse> =
                serde_json::from_str(&local_content).ok();
            Ok(ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            })
        } else {
            let local_entity: Option<CheckAWSLogsServicesAsyncError> =
                serde_json::from_str(&local_content).ok();
            let local_error = ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(Error::ResponseError(local_error))
        }
    }

    /// Attach the Lambda ARN of the Lambda created for the Datadog-AWS log collection to your AWS account ID to enable log collection.
    pub async fn create_aws_lambda_arn(
        &self,
        body: crate::datadogV1::model::AWSAccountAndLambdaRequest,
    ) -> Result<
        Option<std::collections::BTreeMap<String, serde_json::Value>>,
        Error<CreateAWSLambdaARNError>,
    > {
        match self.create_aws_lambda_arn_with_http_info(body).await {
            Ok(response_content) => Ok(response_content.entity),
            Err(err) => Err(err),
        }
    }

    /// Attach the Lambda ARN of the Lambda created for the Datadog-AWS log collection to your AWS account ID to enable log collection.
    pub async fn create_aws_lambda_arn_with_http_info(
        &self,
        body: crate::datadogV1::model::AWSAccountAndLambdaRequest,
    ) -> Result<
        ResponseContent<std::collections::BTreeMap<String, serde_json::Value>>,
        Error<CreateAWSLambdaARNError>,
    > {
        let local_configuration = &self.config;

        let local_client = &local_configuration.client;

        let local_uri_str = format!(
            "{}/api/v1/integration/aws/logs",
            local_configuration.get_operation_host("v1.create_aws_lambda_arn")
        );
        let mut local_req_builder =
            local_client.request(reqwest::Method::POST, local_uri_str.as_str());

        // build user agent
        local_req_builder = local_req_builder.header(
            reqwest::header::USER_AGENT,
            local_configuration.user_agent.clone(),
        );

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
            let local_entity: Option<std::collections::BTreeMap<String, serde_json::Value>> =
                serde_json::from_str(&local_content).ok();
            Ok(ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            })
        } else {
            let local_entity: Option<CreateAWSLambdaARNError> =
                serde_json::from_str(&local_content).ok();
            let local_error = ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(Error::ResponseError(local_error))
        }
    }

    /// Delete a Datadog-AWS logs configuration by removing the specific Lambda ARN associated with a given AWS account.
    pub async fn delete_aws_lambda_arn(
        &self,
        body: crate::datadogV1::model::AWSAccountAndLambdaRequest,
    ) -> Result<
        Option<std::collections::BTreeMap<String, serde_json::Value>>,
        Error<DeleteAWSLambdaARNError>,
    > {
        match self.delete_aws_lambda_arn_with_http_info(body).await {
            Ok(response_content) => Ok(response_content.entity),
            Err(err) => Err(err),
        }
    }

    /// Delete a Datadog-AWS logs configuration by removing the specific Lambda ARN associated with a given AWS account.
    pub async fn delete_aws_lambda_arn_with_http_info(
        &self,
        body: crate::datadogV1::model::AWSAccountAndLambdaRequest,
    ) -> Result<
        ResponseContent<std::collections::BTreeMap<String, serde_json::Value>>,
        Error<DeleteAWSLambdaARNError>,
    > {
        let local_configuration = &self.config;

        let local_client = &local_configuration.client;

        let local_uri_str = format!(
            "{}/api/v1/integration/aws/logs",
            local_configuration.get_operation_host("v1.delete_aws_lambda_arn")
        );
        let mut local_req_builder =
            local_client.request(reqwest::Method::DELETE, local_uri_str.as_str());

        // build user agent
        local_req_builder = local_req_builder.header(
            reqwest::header::USER_AGENT,
            local_configuration.user_agent.clone(),
        );

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
            let local_entity: Option<std::collections::BTreeMap<String, serde_json::Value>> =
                serde_json::from_str(&local_content).ok();
            Ok(ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            })
        } else {
            let local_entity: Option<DeleteAWSLambdaARNError> =
                serde_json::from_str(&local_content).ok();
            let local_error = ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(Error::ResponseError(local_error))
        }
    }

    /// Enable automatic log collection for a list of services. This should be run after running `CreateAWSLambdaARN` to save the configuration.
    pub async fn enable_aws_log_services(
        &self,
        body: crate::datadogV1::model::AWSLogsServicesRequest,
    ) -> Result<
        Option<std::collections::BTreeMap<String, serde_json::Value>>,
        Error<EnableAWSLogServicesError>,
    > {
        match self.enable_aws_log_services_with_http_info(body).await {
            Ok(response_content) => Ok(response_content.entity),
            Err(err) => Err(err),
        }
    }

    /// Enable automatic log collection for a list of services. This should be run after running `CreateAWSLambdaARN` to save the configuration.
    pub async fn enable_aws_log_services_with_http_info(
        &self,
        body: crate::datadogV1::model::AWSLogsServicesRequest,
    ) -> Result<
        ResponseContent<std::collections::BTreeMap<String, serde_json::Value>>,
        Error<EnableAWSLogServicesError>,
    > {
        let local_configuration = &self.config;

        let local_client = &local_configuration.client;

        let local_uri_str = format!(
            "{}/api/v1/integration/aws/logs/services",
            local_configuration.get_operation_host("v1.enable_aws_log_services")
        );
        let mut local_req_builder =
            local_client.request(reqwest::Method::POST, local_uri_str.as_str());

        // build user agent
        local_req_builder = local_req_builder.header(
            reqwest::header::USER_AGENT,
            local_configuration.user_agent.clone(),
        );

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
            let local_entity: Option<std::collections::BTreeMap<String, serde_json::Value>> =
                serde_json::from_str(&local_content).ok();
            Ok(ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            })
        } else {
            let local_entity: Option<EnableAWSLogServicesError> =
                serde_json::from_str(&local_content).ok();
            let local_error = ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(Error::ResponseError(local_error))
        }
    }

    /// List all Datadog-AWS Logs integrations configured in your Datadog account.
    pub async fn list_aws_logs_integrations(
        &self,
    ) -> Result<
        Option<Vec<crate::datadogV1::model::AWSLogsListResponse>>,
        Error<ListAWSLogsIntegrationsError>,
    > {
        match self.list_aws_logs_integrations_with_http_info().await {
            Ok(response_content) => Ok(response_content.entity),
            Err(err) => Err(err),
        }
    }

    /// List all Datadog-AWS Logs integrations configured in your Datadog account.
    pub async fn list_aws_logs_integrations_with_http_info(
        &self,
    ) -> Result<
        ResponseContent<Vec<crate::datadogV1::model::AWSLogsListResponse>>,
        Error<ListAWSLogsIntegrationsError>,
    > {
        let local_configuration = &self.config;

        let local_client = &local_configuration.client;

        let local_uri_str = format!(
            "{}/api/v1/integration/aws/logs",
            local_configuration.get_operation_host("v1.list_aws_logs_integrations")
        );
        let mut local_req_builder =
            local_client.request(reqwest::Method::GET, local_uri_str.as_str());

        // build user agent
        local_req_builder = local_req_builder.header(
            reqwest::header::USER_AGENT,
            local_configuration.user_agent.clone(),
        );

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
            let local_entity: Option<Vec<crate::datadogV1::model::AWSLogsListResponse>> =
                serde_json::from_str(&local_content).ok();
            Ok(ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            })
        } else {
            let local_entity: Option<ListAWSLogsIntegrationsError> =
                serde_json::from_str(&local_content).ok();
            let local_error = ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(Error::ResponseError(local_error))
        }
    }

    /// Get the list of current AWS services that Datadog offers automatic log collection. Use returned service IDs with the services parameter for the Enable an AWS service log collection API endpoint.
    pub async fn list_aws_logs_services(
        &self,
    ) -> Result<
        Option<Vec<crate::datadogV1::model::AWSLogsListServicesResponse>>,
        Error<ListAWSLogsServicesError>,
    > {
        match self.list_aws_logs_services_with_http_info().await {
            Ok(response_content) => Ok(response_content.entity),
            Err(err) => Err(err),
        }
    }

    /// Get the list of current AWS services that Datadog offers automatic log collection. Use returned service IDs with the services parameter for the Enable an AWS service log collection API endpoint.
    pub async fn list_aws_logs_services_with_http_info(
        &self,
    ) -> Result<
        ResponseContent<Vec<crate::datadogV1::model::AWSLogsListServicesResponse>>,
        Error<ListAWSLogsServicesError>,
    > {
        let local_configuration = &self.config;

        let local_client = &local_configuration.client;

        let local_uri_str = format!(
            "{}/api/v1/integration/aws/logs/services",
            local_configuration.get_operation_host("v1.list_aws_logs_services")
        );
        let mut local_req_builder =
            local_client.request(reqwest::Method::GET, local_uri_str.as_str());

        // build user agent
        local_req_builder = local_req_builder.header(
            reqwest::header::USER_AGENT,
            local_configuration.user_agent.clone(),
        );

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
            let local_entity: Option<Vec<crate::datadogV1::model::AWSLogsListServicesResponse>> =
                serde_json::from_str(&local_content).ok();
            Ok(ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            })
        } else {
            let local_entity: Option<ListAWSLogsServicesError> =
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
