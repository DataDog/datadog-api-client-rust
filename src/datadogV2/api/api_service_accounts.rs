// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use crate::datadog::*;
use reqwest;
use serde::{Deserialize, Serialize};

/// ListServiceAccountApplicationKeysOptionalParams is a struct for passing parameters to the method [`ServiceAccountsAPI::list_service_account_application_keys`]
#[non_exhaustive]
#[derive(Clone, Default, Debug)]
pub struct ListServiceAccountApplicationKeysOptionalParams {
    /// Size for a given page. The maximum allowed value is 100.
    pub page_size: Option<i64>,
    /// Specific page number to return.
    pub page_number: Option<i64>,
    /// Application key attribute used to sort results. Sort order is ascending
    /// by default. In order to specify a descending sort, prefix the
    /// attribute with a minus sign.
    pub sort: Option<crate::datadogV2::model::ApplicationKeysSort>,
    /// Filter application keys by the specified string.
    pub filter: Option<String>,
    /// Only include application keys created on or after the specified date.
    pub filter_created_at_start: Option<String>,
    /// Only include application keys created on or before the specified date.
    pub filter_created_at_end: Option<String>,
}

impl ListServiceAccountApplicationKeysOptionalParams {
    /// Size for a given page. The maximum allowed value is 100.
    pub fn page_size(&mut self, value: i64) -> &mut Self {
        self.page_size = Some(value);
        self
    }
    /// Specific page number to return.
    pub fn page_number(&mut self, value: i64) -> &mut Self {
        self.page_number = Some(value);
        self
    }
    /// Application key attribute used to sort results. Sort order is ascending
    /// by default. In order to specify a descending sort, prefix the
    /// attribute with a minus sign.
    pub fn sort(&mut self, value: crate::datadogV2::model::ApplicationKeysSort) -> &mut Self {
        self.sort = Some(value);
        self
    }
    /// Filter application keys by the specified string.
    pub fn filter(&mut self, value: String) -> &mut Self {
        self.filter = Some(value);
        self
    }
    /// Only include application keys created on or after the specified date.
    pub fn filter_created_at_start(&mut self, value: String) -> &mut Self {
        self.filter_created_at_start = Some(value);
        self
    }
    /// Only include application keys created on or before the specified date.
    pub fn filter_created_at_end(&mut self, value: String) -> &mut Self {
        self.filter_created_at_end = Some(value);
        self
    }
}

/// CreateServiceAccountError is a struct for typed errors of method [`ServiceAccountsAPI::create_service_account`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CreateServiceAccountError {
    Status400(Option<crate::datadogV2::model::APIErrorResponse>),
    Status403(Option<crate::datadogV2::model::APIErrorResponse>),
    Status429(Option<crate::datadogV2::model::APIErrorResponse>),
    UnknownValue(serde_json::Value),
}

/// CreateServiceAccountApplicationKeyError is a struct for typed errors of method [`ServiceAccountsAPI::create_service_account_application_key`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CreateServiceAccountApplicationKeyError {
    Status400(Option<crate::datadogV2::model::APIErrorResponse>),
    Status403(Option<crate::datadogV2::model::APIErrorResponse>),
    Status429(Option<crate::datadogV2::model::APIErrorResponse>),
    UnknownValue(serde_json::Value),
}

/// DeleteServiceAccountApplicationKeyError is a struct for typed errors of method [`ServiceAccountsAPI::delete_service_account_application_key`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DeleteServiceAccountApplicationKeyError {
    Status403(Option<crate::datadogV2::model::APIErrorResponse>),
    Status404(Option<crate::datadogV2::model::APIErrorResponse>),
    Status429(Option<crate::datadogV2::model::APIErrorResponse>),
    UnknownValue(serde_json::Value),
}

/// GetServiceAccountApplicationKeyError is a struct for typed errors of method [`ServiceAccountsAPI::get_service_account_application_key`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetServiceAccountApplicationKeyError {
    Status403(Option<crate::datadogV2::model::APIErrorResponse>),
    Status404(Option<crate::datadogV2::model::APIErrorResponse>),
    Status429(Option<crate::datadogV2::model::APIErrorResponse>),
    UnknownValue(serde_json::Value),
}

/// ListServiceAccountApplicationKeysError is a struct for typed errors of method [`ServiceAccountsAPI::list_service_account_application_keys`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ListServiceAccountApplicationKeysError {
    Status400(Option<crate::datadogV2::model::APIErrorResponse>),
    Status403(Option<crate::datadogV2::model::APIErrorResponse>),
    Status404(Option<crate::datadogV2::model::APIErrorResponse>),
    Status429(Option<crate::datadogV2::model::APIErrorResponse>),
    UnknownValue(serde_json::Value),
}

/// UpdateServiceAccountApplicationKeyError is a struct for typed errors of method [`ServiceAccountsAPI::update_service_account_application_key`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum UpdateServiceAccountApplicationKeyError {
    Status400(Option<crate::datadogV2::model::APIErrorResponse>),
    Status403(Option<crate::datadogV2::model::APIErrorResponse>),
    Status404(Option<crate::datadogV2::model::APIErrorResponse>),
    Status429(Option<crate::datadogV2::model::APIErrorResponse>),
    UnknownValue(serde_json::Value),
}

#[derive(Debug, Clone)]
pub struct ServiceAccountsAPI {
    config: configuration::Configuration,
}

impl Default for ServiceAccountsAPI {
    fn default() -> Self {
        Self {
            config: configuration::Configuration::new(),
        }
    }
}

impl ServiceAccountsAPI {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn with_config(config: configuration::Configuration) -> Self {
        Self { config }
    }

    /// Create a service account for your organization.
    pub async fn create_service_account(
        &self,
        body: crate::datadogV2::model::ServiceAccountCreateRequest,
    ) -> Result<Option<crate::datadogV2::model::UserResponse>, Error<CreateServiceAccountError>>
    {
        match self.create_service_account_with_http_info(body).await {
            Ok(response_content) => Ok(response_content.entity),
            Err(err) => Err(err),
        }
    }

    /// Create a service account for your organization.
    pub async fn create_service_account_with_http_info(
        &self,
        body: crate::datadogV2::model::ServiceAccountCreateRequest,
    ) -> Result<
        ResponseContent<crate::datadogV2::model::UserResponse>,
        Error<CreateServiceAccountError>,
    > {
        let local_configuration = &self.config;

        let local_client = &local_configuration.client;

        let local_uri_str = format!(
            "{}/api/v2/service_accounts",
            local_configuration.get_operation_host("v2.create_service_account")
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
            let local_entity: Option<crate::datadogV2::model::UserResponse> =
                serde_json::from_str(&local_content).ok();
            Ok(ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            })
        } else {
            let local_entity: Option<CreateServiceAccountError> =
                serde_json::from_str(&local_content).ok();
            let local_error = ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(Error::ResponseError(local_error))
        }
    }

    /// Create an application key for this service account.
    pub async fn create_service_account_application_key(
        &self,
        service_account_id: String,
        body: crate::datadogV2::model::ApplicationKeyCreateRequest,
    ) -> Result<
        Option<crate::datadogV2::model::ApplicationKeyResponse>,
        Error<CreateServiceAccountApplicationKeyError>,
    > {
        match self
            .create_service_account_application_key_with_http_info(service_account_id, body)
            .await
        {
            Ok(response_content) => Ok(response_content.entity),
            Err(err) => Err(err),
        }
    }

    /// Create an application key for this service account.
    pub async fn create_service_account_application_key_with_http_info(
        &self,
        service_account_id: String,
        body: crate::datadogV2::model::ApplicationKeyCreateRequest,
    ) -> Result<
        ResponseContent<crate::datadogV2::model::ApplicationKeyResponse>,
        Error<CreateServiceAccountApplicationKeyError>,
    > {
        let local_configuration = &self.config;

        let local_client = &local_configuration.client;

        let local_uri_str = format!(
            "{}/api/v2/service_accounts/{service_account_id}/application_keys",
            local_configuration.get_operation_host("v2.create_service_account_application_key"),
            service_account_id = urlencode(service_account_id)
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
            let local_entity: Option<crate::datadogV2::model::ApplicationKeyResponse> =
                serde_json::from_str(&local_content).ok();
            Ok(ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            })
        } else {
            let local_entity: Option<CreateServiceAccountApplicationKeyError> =
                serde_json::from_str(&local_content).ok();
            let local_error = ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(Error::ResponseError(local_error))
        }
    }

    /// Delete an application key owned by this service account.
    pub async fn delete_service_account_application_key(
        &self,
        service_account_id: String,
        app_key_id: String,
    ) -> Result<Option<()>, Error<DeleteServiceAccountApplicationKeyError>> {
        match self
            .delete_service_account_application_key_with_http_info(service_account_id, app_key_id)
            .await
        {
            Ok(response_content) => Ok(response_content.entity),
            Err(err) => Err(err),
        }
    }

    /// Delete an application key owned by this service account.
    pub async fn delete_service_account_application_key_with_http_info(
        &self,
        service_account_id: String,
        app_key_id: String,
    ) -> Result<ResponseContent<()>, Error<DeleteServiceAccountApplicationKeyError>> {
        let local_configuration = &self.config;

        let local_client = &local_configuration.client;

        let local_uri_str = format!(
            "{}/api/v2/service_accounts/{service_account_id}/application_keys/{app_key_id}",
            local_configuration.get_operation_host("v2.delete_service_account_application_key"),
            service_account_id = urlencode(service_account_id),
            app_key_id = urlencode(app_key_id)
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
            let local_entity: Option<DeleteServiceAccountApplicationKeyError> =
                serde_json::from_str(&local_content).ok();
            let local_error = ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(Error::ResponseError(local_error))
        }
    }

    /// Get an application key owned by this service account.
    pub async fn get_service_account_application_key(
        &self,
        service_account_id: String,
        app_key_id: String,
    ) -> Result<
        Option<crate::datadogV2::model::PartialApplicationKeyResponse>,
        Error<GetServiceAccountApplicationKeyError>,
    > {
        match self
            .get_service_account_application_key_with_http_info(service_account_id, app_key_id)
            .await
        {
            Ok(response_content) => Ok(response_content.entity),
            Err(err) => Err(err),
        }
    }

    /// Get an application key owned by this service account.
    pub async fn get_service_account_application_key_with_http_info(
        &self,
        service_account_id: String,
        app_key_id: String,
    ) -> Result<
        ResponseContent<crate::datadogV2::model::PartialApplicationKeyResponse>,
        Error<GetServiceAccountApplicationKeyError>,
    > {
        let local_configuration = &self.config;

        let local_client = &local_configuration.client;

        let local_uri_str = format!(
            "{}/api/v2/service_accounts/{service_account_id}/application_keys/{app_key_id}",
            local_configuration.get_operation_host("v2.get_service_account_application_key"),
            service_account_id = urlencode(service_account_id),
            app_key_id = urlencode(app_key_id)
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
            let local_entity: Option<crate::datadogV2::model::PartialApplicationKeyResponse> =
                serde_json::from_str(&local_content).ok();
            Ok(ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            })
        } else {
            let local_entity: Option<GetServiceAccountApplicationKeyError> =
                serde_json::from_str(&local_content).ok();
            let local_error = ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(Error::ResponseError(local_error))
        }
    }

    /// List all application keys available for this service account.
    pub async fn list_service_account_application_keys(
        &self,
        service_account_id: String,
        params: ListServiceAccountApplicationKeysOptionalParams,
    ) -> Result<
        Option<crate::datadogV2::model::ListApplicationKeysResponse>,
        Error<ListServiceAccountApplicationKeysError>,
    > {
        match self
            .list_service_account_application_keys_with_http_info(service_account_id, params)
            .await
        {
            Ok(response_content) => Ok(response_content.entity),
            Err(err) => Err(err),
        }
    }

    /// List all application keys available for this service account.
    pub async fn list_service_account_application_keys_with_http_info(
        &self,
        service_account_id: String,
        params: ListServiceAccountApplicationKeysOptionalParams,
    ) -> Result<
        ResponseContent<crate::datadogV2::model::ListApplicationKeysResponse>,
        Error<ListServiceAccountApplicationKeysError>,
    > {
        let local_configuration = &self.config;

        // unbox and build optional parameters
        let page_size = params.page_size;
        let page_number = params.page_number;
        let sort = params.sort;
        let filter = params.filter;
        let filter_created_at_start = params.filter_created_at_start;
        let filter_created_at_end = params.filter_created_at_end;

        let local_client = &local_configuration.client;

        let local_uri_str = format!(
            "{}/api/v2/service_accounts/{service_account_id}/application_keys",
            local_configuration.get_operation_host("v2.list_service_account_application_keys"),
            service_account_id = urlencode(service_account_id)
        );
        let mut local_req_builder =
            local_client.request(reqwest::Method::GET, local_uri_str.as_str());

        if let Some(ref local_query_param) = page_size {
            local_req_builder =
                local_req_builder.query(&[("page[size]", &local_query_param.to_string())]);
        };
        if let Some(ref local_query_param) = page_number {
            local_req_builder =
                local_req_builder.query(&[("page[number]", &local_query_param.to_string())]);
        };
        if let Some(ref local_query_param) = sort {
            local_req_builder =
                local_req_builder.query(&[("sort", &local_query_param.to_string())]);
        };
        if let Some(ref local_query_param) = filter {
            local_req_builder =
                local_req_builder.query(&[("filter", &local_query_param.to_string())]);
        };
        if let Some(ref local_query_param) = filter_created_at_start {
            local_req_builder = local_req_builder
                .query(&[("filter[created_at][start]", &local_query_param.to_string())]);
        };
        if let Some(ref local_query_param) = filter_created_at_end {
            local_req_builder = local_req_builder
                .query(&[("filter[created_at][end]", &local_query_param.to_string())]);
        };

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
            let local_entity: Option<crate::datadogV2::model::ListApplicationKeysResponse> =
                serde_json::from_str(&local_content).ok();
            Ok(ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            })
        } else {
            let local_entity: Option<ListServiceAccountApplicationKeysError> =
                serde_json::from_str(&local_content).ok();
            let local_error = ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(Error::ResponseError(local_error))
        }
    }

    /// Edit an application key owned by this service account.
    pub async fn update_service_account_application_key(
        &self,
        service_account_id: String,
        app_key_id: String,
        body: crate::datadogV2::model::ApplicationKeyUpdateRequest,
    ) -> Result<
        Option<crate::datadogV2::model::PartialApplicationKeyResponse>,
        Error<UpdateServiceAccountApplicationKeyError>,
    > {
        match self
            .update_service_account_application_key_with_http_info(
                service_account_id,
                app_key_id,
                body,
            )
            .await
        {
            Ok(response_content) => Ok(response_content.entity),
            Err(err) => Err(err),
        }
    }

    /// Edit an application key owned by this service account.
    pub async fn update_service_account_application_key_with_http_info(
        &self,
        service_account_id: String,
        app_key_id: String,
        body: crate::datadogV2::model::ApplicationKeyUpdateRequest,
    ) -> Result<
        ResponseContent<crate::datadogV2::model::PartialApplicationKeyResponse>,
        Error<UpdateServiceAccountApplicationKeyError>,
    > {
        let local_configuration = &self.config;

        let local_client = &local_configuration.client;

        let local_uri_str = format!(
            "{}/api/v2/service_accounts/{service_account_id}/application_keys/{app_key_id}",
            local_configuration.get_operation_host("v2.update_service_account_application_key"),
            service_account_id = urlencode(service_account_id),
            app_key_id = urlencode(app_key_id)
        );
        let mut local_req_builder =
            local_client.request(reqwest::Method::PATCH, local_uri_str.as_str());

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
            let local_entity: Option<crate::datadogV2::model::PartialApplicationKeyResponse> =
                serde_json::from_str(&local_content).ok();
            Ok(ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            })
        } else {
            let local_entity: Option<UpdateServiceAccountApplicationKeyError> =
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
