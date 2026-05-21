// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use crate::datadog;
use flate2::{
    write::{GzEncoder, ZlibEncoder},
    Compression,
};
use reqwest::header::{HeaderMap, HeaderValue};
use serde::{Deserialize, Serialize};
use std::io::Write;

/// GetAppOptionalParams is a struct for passing parameters to the method [`AppBuilderAPI::get_app`]
#[non_exhaustive]
#[derive(Clone, Default, Debug)]
pub struct GetAppOptionalParams {
    /// The version number of the app to retrieve. If not specified, the latest version is returned. Version numbers start at 1 and increment with each update. The special values `latest` and `deployed` can be used to retrieve the latest version or the published version, respectively.
    pub version: Option<String>,
}

impl GetAppOptionalParams {
    /// The version number of the app to retrieve. If not specified, the latest version is returned. Version numbers start at 1 and increment with each update. The special values `latest` and `deployed` can be used to retrieve the latest version or the published version, respectively.
    pub fn version(mut self, value: String) -> Self {
        self.version = Some(value);
        self
    }
}

/// ListAppVersionsOptionalParams is a struct for passing parameters to the method [`AppBuilderAPI::list_app_versions`]
#[non_exhaustive]
#[derive(Clone, Default, Debug)]
pub struct ListAppVersionsOptionalParams {
    /// The number of versions to return per page.
    pub limit: Option<i64>,
    /// The page number to return.
    pub page: Option<i64>,
}

impl ListAppVersionsOptionalParams {
    /// The number of versions to return per page.
    pub fn limit(mut self, value: i64) -> Self {
        self.limit = Some(value);
        self
    }
    /// The page number to return.
    pub fn page(mut self, value: i64) -> Self {
        self.page = Some(value);
        self
    }
}

/// ListAppsOptionalParams is a struct for passing parameters to the method [`AppBuilderAPI::list_apps`]
#[non_exhaustive]
#[derive(Clone, Default, Debug)]
pub struct ListAppsOptionalParams {
    /// The number of apps to return per page.
    pub limit: Option<i64>,
    /// The page number to return.
    pub page: Option<i64>,
    /// Filter apps by the app creator. Usually the user's email.
    pub filter_user_name: Option<String>,
    /// Filter apps by the app creator's UUID.
    pub filter_user_uuid: Option<uuid::Uuid>,
    /// Filter by app name.
    pub filter_name: Option<String>,
    /// Filter apps by the app name or the app creator.
    pub filter_query: Option<String>,
    /// Filter apps by whether they are published.
    pub filter_deployed: Option<bool>,
    /// Filter apps by tags.
    pub filter_tags: Option<String>,
    /// Filter apps by whether you have added them to your favorites.
    pub filter_favorite: Option<bool>,
    /// Filter apps by whether they are enabled for self-service.
    pub filter_self_service: Option<bool>,
    /// The fields and direction to sort apps by.
    pub sort: Option<Vec<crate::datadogV2::model::AppsSortField>>,
}

impl ListAppsOptionalParams {
    /// The number of apps to return per page.
    pub fn limit(mut self, value: i64) -> Self {
        self.limit = Some(value);
        self
    }
    /// The page number to return.
    pub fn page(mut self, value: i64) -> Self {
        self.page = Some(value);
        self
    }
    /// Filter apps by the app creator. Usually the user's email.
    pub fn filter_user_name(mut self, value: String) -> Self {
        self.filter_user_name = Some(value);
        self
    }
    /// Filter apps by the app creator's UUID.
    pub fn filter_user_uuid(mut self, value: uuid::Uuid) -> Self {
        self.filter_user_uuid = Some(value);
        self
    }
    /// Filter by app name.
    pub fn filter_name(mut self, value: String) -> Self {
        self.filter_name = Some(value);
        self
    }
    /// Filter apps by the app name or the app creator.
    pub fn filter_query(mut self, value: String) -> Self {
        self.filter_query = Some(value);
        self
    }
    /// Filter apps by whether they are published.
    pub fn filter_deployed(mut self, value: bool) -> Self {
        self.filter_deployed = Some(value);
        self
    }
    /// Filter apps by tags.
    pub fn filter_tags(mut self, value: String) -> Self {
        self.filter_tags = Some(value);
        self
    }
    /// Filter apps by whether you have added them to your favorites.
    pub fn filter_favorite(mut self, value: bool) -> Self {
        self.filter_favorite = Some(value);
        self
    }
    /// Filter apps by whether they are enabled for self-service.
    pub fn filter_self_service(mut self, value: bool) -> Self {
        self.filter_self_service = Some(value);
        self
    }
    /// The fields and direction to sort apps by.
    pub fn sort(mut self, value: Vec<crate::datadogV2::model::AppsSortField>) -> Self {
        self.sort = Some(value);
        self
    }
}

/// ListBlueprintsOptionalParams is a struct for passing parameters to the method [`AppBuilderAPI::list_blueprints`]
#[non_exhaustive]
#[derive(Clone, Default, Debug)]
pub struct ListBlueprintsOptionalParams {
    /// The number of blueprints to return per page. Defaults to 10. Maximum is 100.
    pub limit: Option<i64>,
    /// The page of results to return. Starts at 0.
    pub page: Option<i64>,
}

impl ListBlueprintsOptionalParams {
    /// The number of blueprints to return per page. Defaults to 10. Maximum is 100.
    pub fn limit(mut self, value: i64) -> Self {
        self.limit = Some(value);
        self
    }
    /// The page of results to return. Starts at 0.
    pub fn page(mut self, value: i64) -> Self {
        self.page = Some(value);
        self
    }
}

/// CreateAppError is a struct for typed errors of method [`AppBuilderAPI::create_app`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CreateAppError {
    JSONAPIErrorResponse(crate::datadogV2::model::JSONAPIErrorResponse),
    APIErrorResponse(crate::datadogV2::model::APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// CreatePublishRequestError is a struct for typed errors of method [`AppBuilderAPI::create_publish_request`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CreatePublishRequestError {
    JSONAPIErrorResponse(crate::datadogV2::model::JSONAPIErrorResponse),
    APIErrorResponse(crate::datadogV2::model::APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// DeleteAppError is a struct for typed errors of method [`AppBuilderAPI::delete_app`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DeleteAppError {
    JSONAPIErrorResponse(crate::datadogV2::model::JSONAPIErrorResponse),
    APIErrorResponse(crate::datadogV2::model::APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// DeleteAppsError is a struct for typed errors of method [`AppBuilderAPI::delete_apps`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DeleteAppsError {
    JSONAPIErrorResponse(crate::datadogV2::model::JSONAPIErrorResponse),
    APIErrorResponse(crate::datadogV2::model::APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// GetAppError is a struct for typed errors of method [`AppBuilderAPI::get_app`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetAppError {
    JSONAPIErrorResponse(crate::datadogV2::model::JSONAPIErrorResponse),
    APIErrorResponse(crate::datadogV2::model::APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// GetBlueprintError is a struct for typed errors of method [`AppBuilderAPI::get_blueprint`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetBlueprintError {
    JSONAPIErrorResponse(crate::datadogV2::model::JSONAPIErrorResponse),
    APIErrorResponse(crate::datadogV2::model::APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// GetBlueprintsByIntegrationIdError is a struct for typed errors of method [`AppBuilderAPI::get_blueprints_by_integration_id`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetBlueprintsByIntegrationIdError {
    JSONAPIErrorResponse(crate::datadogV2::model::JSONAPIErrorResponse),
    APIErrorResponse(crate::datadogV2::model::APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// GetBlueprintsBySlugsError is a struct for typed errors of method [`AppBuilderAPI::get_blueprints_by_slugs`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetBlueprintsBySlugsError {
    JSONAPIErrorResponse(crate::datadogV2::model::JSONAPIErrorResponse),
    APIErrorResponse(crate::datadogV2::model::APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// ListAppVersionsError is a struct for typed errors of method [`AppBuilderAPI::list_app_versions`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ListAppVersionsError {
    JSONAPIErrorResponse(crate::datadogV2::model::JSONAPIErrorResponse),
    APIErrorResponse(crate::datadogV2::model::APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// ListAppsError is a struct for typed errors of method [`AppBuilderAPI::list_apps`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ListAppsError {
    JSONAPIErrorResponse(crate::datadogV2::model::JSONAPIErrorResponse),
    APIErrorResponse(crate::datadogV2::model::APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// ListBlueprintsError is a struct for typed errors of method [`AppBuilderAPI::list_blueprints`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ListBlueprintsError {
    JSONAPIErrorResponse(crate::datadogV2::model::JSONAPIErrorResponse),
    APIErrorResponse(crate::datadogV2::model::APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// ListTagsError is a struct for typed errors of method [`AppBuilderAPI::list_tags`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ListTagsError {
    JSONAPIErrorResponse(crate::datadogV2::model::JSONAPIErrorResponse),
    APIErrorResponse(crate::datadogV2::model::APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// PublishAppError is a struct for typed errors of method [`AppBuilderAPI::publish_app`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum PublishAppError {
    JSONAPIErrorResponse(crate::datadogV2::model::JSONAPIErrorResponse),
    APIErrorResponse(crate::datadogV2::model::APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// RevertAppError is a struct for typed errors of method [`AppBuilderAPI::revert_app`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum RevertAppError {
    JSONAPIErrorResponse(crate::datadogV2::model::JSONAPIErrorResponse),
    APIErrorResponse(crate::datadogV2::model::APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// UnpublishAppError is a struct for typed errors of method [`AppBuilderAPI::unpublish_app`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum UnpublishAppError {
    JSONAPIErrorResponse(crate::datadogV2::model::JSONAPIErrorResponse),
    APIErrorResponse(crate::datadogV2::model::APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// UpdateAppError is a struct for typed errors of method [`AppBuilderAPI::update_app`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum UpdateAppError {
    JSONAPIErrorResponse(crate::datadogV2::model::JSONAPIErrorResponse),
    APIErrorResponse(crate::datadogV2::model::APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// UpdateAppFavoriteError is a struct for typed errors of method [`AppBuilderAPI::update_app_favorite`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum UpdateAppFavoriteError {
    JSONAPIErrorResponse(crate::datadogV2::model::JSONAPIErrorResponse),
    APIErrorResponse(crate::datadogV2::model::APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// UpdateAppSelfServiceError is a struct for typed errors of method [`AppBuilderAPI::update_app_self_service`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum UpdateAppSelfServiceError {
    JSONAPIErrorResponse(crate::datadogV2::model::JSONAPIErrorResponse),
    APIErrorResponse(crate::datadogV2::model::APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// UpdateAppTagsError is a struct for typed errors of method [`AppBuilderAPI::update_app_tags`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum UpdateAppTagsError {
    JSONAPIErrorResponse(crate::datadogV2::model::JSONAPIErrorResponse),
    APIErrorResponse(crate::datadogV2::model::APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// UpdateAppVersionNameError is a struct for typed errors of method [`AppBuilderAPI::update_app_version_name`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum UpdateAppVersionNameError {
    JSONAPIErrorResponse(crate::datadogV2::model::JSONAPIErrorResponse),
    APIErrorResponse(crate::datadogV2::model::APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// UpdateProtectionLevelError is a struct for typed errors of method [`AppBuilderAPI::update_protection_level`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum UpdateProtectionLevelError {
    JSONAPIErrorResponse(crate::datadogV2::model::JSONAPIErrorResponse),
    APIErrorResponse(crate::datadogV2::model::APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// Datadog App Builder provides a low-code solution to rapidly develop and integrate secure, customized applications into your monitoring stack that are built to accelerate remediation at scale. These API endpoints allow you to create, read, update, delete, and publish apps.
#[derive(Debug, Clone)]
pub struct AppBuilderAPI {
    config: datadog::Configuration,
    client: reqwest_middleware::ClientWithMiddleware,
}

impl Default for AppBuilderAPI {
    fn default() -> Self {
        Self::with_config(datadog::Configuration::default())
    }
}

impl AppBuilderAPI {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn with_config(config: datadog::Configuration) -> Self {
        let reqwest_client_builder = {
            let builder = reqwest::Client::builder();
            #[cfg(not(target_arch = "wasm32"))]
            let builder = if let Some(proxy_url) = &config.proxy_url {
                builder.proxy(reqwest::Proxy::all(proxy_url).expect("Failed to parse proxy URL"))
            } else {
                builder
            };
            builder
        };

        let middleware_client_builder = {
            let builder =
                reqwest_middleware::ClientBuilder::new(reqwest_client_builder.build().unwrap());
            #[cfg(feature = "retry")]
            let builder = if config.enable_retry {
                struct RetryableStatus;
                impl reqwest_retry::RetryableStrategy for RetryableStatus {
                    fn handle(
                        &self,
                        res: &Result<reqwest::Response, reqwest_middleware::Error>,
                    ) -> Option<reqwest_retry::Retryable> {
                        match res {
                            Ok(success) => reqwest_retry::default_on_request_success(success),
                            Err(_) => None,
                        }
                    }
                }
                let backoff_policy = reqwest_retry::policies::ExponentialBackoff::builder()
                    .build_with_max_retries(config.max_retries);

                let retry_middleware =
                    reqwest_retry::RetryTransientMiddleware::new_with_policy_and_strategy(
                        backoff_policy,
                        RetryableStatus,
                    );

                builder.with(retry_middleware)
            } else {
                builder
            };
            builder
        };

        let client = middleware_client_builder.build();

        Self { config, client }
    }

    pub fn with_client_and_config(
        config: datadog::Configuration,
        client: reqwest_middleware::ClientWithMiddleware,
    ) -> Self {
        Self { config, client }
    }

    /// Create a new app, returning the app ID. This API requires a [registered application key](<https://docs.datadoghq.com/api/latest/action-connection/#register-a-new-app-key>). Alternatively, you can configure these permissions [in the UI](<https://docs.datadoghq.com/account_management/api-app-keys/#actions-api-access>).
    pub async fn create_app(
        &self,
        body: crate::datadogV2::model::CreateAppRequest,
    ) -> Result<crate::datadogV2::model::CreateAppResponse, datadog::Error<CreateAppError>> {
        match self.create_app_with_http_info(body).await {
            Ok(response_content) => {
                if let Some(e) = response_content.entity {
                    Ok(e)
                } else {
                    Err(datadog::Error::Serde(serde::de::Error::custom(
                        "response content was None",
                    )))
                }
            }
            Err(err) => Err(err),
        }
    }

    /// Create a new app, returning the app ID. This API requires a [registered application key](<https://docs.datadoghq.com/api/latest/action-connection/#register-a-new-app-key>). Alternatively, you can configure these permissions [in the UI](<https://docs.datadoghq.com/account_management/api-app-keys/#actions-api-access>).
    pub async fn create_app_with_http_info(
        &self,
        body: crate::datadogV2::model::CreateAppRequest,
    ) -> Result<
        datadog::ResponseContent<crate::datadogV2::model::CreateAppResponse>,
        datadog::Error<CreateAppError>,
    > {
        let local_configuration = &self.config;
        let operation_id = "v2.create_app";

        let local_client = &self.client;

        let local_uri_str = format!(
            "{}/api/v2/app-builder/apps",
            local_configuration.get_operation_host(operation_id)
        );
        let mut local_req_builder =
            local_client.request(reqwest::Method::POST, local_uri_str.as_str());

        // build headers
        let mut headers = HeaderMap::new();
        headers.insert("Content-Type", HeaderValue::from_static("application/json"));
        headers.insert("Accept", HeaderValue::from_static("application/json"));

        // build user agent
        match HeaderValue::from_str(local_configuration.user_agent.as_str()) {
            Ok(user_agent) => headers.insert(reqwest::header::USER_AGENT, user_agent),
            Err(e) => {
                log::warn!("Failed to parse user agent header: {e}, falling back to default");
                headers.insert(
                    reqwest::header::USER_AGENT,
                    HeaderValue::from_static(datadog::DEFAULT_USER_AGENT.as_str()),
                )
            }
        };

        // build auth
        if let Some(local_key) = local_configuration.auth_keys.get("apiKeyAuth") {
            headers.insert(
                "DD-API-KEY",
                HeaderValue::from_str(local_key.key.as_str())
                    .expect("failed to parse DD-API-KEY header"),
            );
        };
        if let Some(local_key) = local_configuration.auth_keys.get("appKeyAuth") {
            headers.insert(
                "DD-APPLICATION-KEY",
                HeaderValue::from_str(local_key.key.as_str())
                    .expect("failed to parse DD-APPLICATION-KEY header"),
            );
        };

        // build body parameters
        let output = Vec::new();
        let mut ser = serde_json::Serializer::with_formatter(output, datadog::DDFormatter);
        if body.serialize(&mut ser).is_ok() {
            if let Some(content_encoding) = headers.get("Content-Encoding") {
                match content_encoding.to_str().unwrap_or_default() {
                    "gzip" => {
                        let mut enc = GzEncoder::new(Vec::new(), Compression::default());
                        let _ = enc.write_all(ser.into_inner().as_slice());
                        match enc.finish() {
                            Ok(buf) => {
                                local_req_builder = local_req_builder.body(buf);
                            }
                            Err(e) => return Err(datadog::Error::Io(e)),
                        }
                    }
                    "deflate" => {
                        let mut enc = ZlibEncoder::new(Vec::new(), Compression::default());
                        let _ = enc.write_all(ser.into_inner().as_slice());
                        match enc.finish() {
                            Ok(buf) => {
                                local_req_builder = local_req_builder.body(buf);
                            }
                            Err(e) => return Err(datadog::Error::Io(e)),
                        }
                    }
                    #[cfg(feature = "zstd")]
                    "zstd1" => {
                        let mut enc = zstd::stream::Encoder::new(Vec::new(), 0).unwrap();
                        let _ = enc.write_all(ser.into_inner().as_slice());
                        match enc.finish() {
                            Ok(buf) => {
                                local_req_builder = local_req_builder.body(buf);
                            }
                            Err(e) => return Err(datadog::Error::Io(e)),
                        }
                    }
                    _ => {
                        local_req_builder = local_req_builder.body(ser.into_inner());
                    }
                }
            } else {
                local_req_builder = local_req_builder.body(ser.into_inner());
            }
        }

        local_req_builder = local_req_builder.headers(headers);
        let local_req = local_req_builder.build()?;
        log::debug!("request content: {:?}", local_req.body());
        let local_resp = local_client.execute(local_req).await?;

        let local_status = local_resp.status();
        let local_content = local_resp.text().await?;
        log::debug!("response content: {}", local_content);

        if !local_status.is_client_error() && !local_status.is_server_error() {
            match serde_json::from_str::<crate::datadogV2::model::CreateAppResponse>(&local_content)
            {
                Ok(e) => {
                    return Ok(datadog::ResponseContent {
                        status: local_status,
                        content: local_content,
                        entity: Some(e),
                    })
                }
                Err(e) => return Err(datadog::Error::Serde(e)),
            };
        } else {
            let local_entity: Option<CreateAppError> = serde_json::from_str(&local_content).ok();
            let local_error = datadog::ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(datadog::Error::ResponseError(local_error))
        }
    }

    /// Create a publish request to ask for approval to publish an app whose protection level is `approval_required`. Publishing happens automatically once the request is approved by a user with the appropriate permissions.
    pub async fn create_publish_request(
        &self,
        app_id: uuid::Uuid,
        body: crate::datadogV2::model::CreatePublishRequestRequest,
    ) -> Result<
        crate::datadogV2::model::PublishAppResponse,
        datadog::Error<CreatePublishRequestError>,
    > {
        match self
            .create_publish_request_with_http_info(app_id, body)
            .await
        {
            Ok(response_content) => {
                if let Some(e) = response_content.entity {
                    Ok(e)
                } else {
                    Err(datadog::Error::Serde(serde::de::Error::custom(
                        "response content was None",
                    )))
                }
            }
            Err(err) => Err(err),
        }
    }

    /// Create a publish request to ask for approval to publish an app whose protection level is `approval_required`. Publishing happens automatically once the request is approved by a user with the appropriate permissions.
    pub async fn create_publish_request_with_http_info(
        &self,
        app_id: uuid::Uuid,
        body: crate::datadogV2::model::CreatePublishRequestRequest,
    ) -> Result<
        datadog::ResponseContent<crate::datadogV2::model::PublishAppResponse>,
        datadog::Error<CreatePublishRequestError>,
    > {
        let local_configuration = &self.config;
        let operation_id = "v2.create_publish_request";

        let local_client = &self.client;

        let local_uri_str = format!(
            "{}/api/v2/app-builder/apps/{app_id}/publish-request",
            local_configuration.get_operation_host(operation_id),
            app_id = datadog::urlencode(app_id.to_string())
        );
        let mut local_req_builder =
            local_client.request(reqwest::Method::POST, local_uri_str.as_str());

        // build headers
        let mut headers = HeaderMap::new();
        headers.insert("Content-Type", HeaderValue::from_static("application/json"));
        headers.insert("Accept", HeaderValue::from_static("application/json"));

        // build user agent
        match HeaderValue::from_str(local_configuration.user_agent.as_str()) {
            Ok(user_agent) => headers.insert(reqwest::header::USER_AGENT, user_agent),
            Err(e) => {
                log::warn!("Failed to parse user agent header: {e}, falling back to default");
                headers.insert(
                    reqwest::header::USER_AGENT,
                    HeaderValue::from_static(datadog::DEFAULT_USER_AGENT.as_str()),
                )
            }
        };

        // build auth
        if let Some(local_key) = local_configuration.auth_keys.get("apiKeyAuth") {
            headers.insert(
                "DD-API-KEY",
                HeaderValue::from_str(local_key.key.as_str())
                    .expect("failed to parse DD-API-KEY header"),
            );
        };
        if let Some(local_key) = local_configuration.auth_keys.get("appKeyAuth") {
            headers.insert(
                "DD-APPLICATION-KEY",
                HeaderValue::from_str(local_key.key.as_str())
                    .expect("failed to parse DD-APPLICATION-KEY header"),
            );
        };

        // build body parameters
        let output = Vec::new();
        let mut ser = serde_json::Serializer::with_formatter(output, datadog::DDFormatter);
        if body.serialize(&mut ser).is_ok() {
            if let Some(content_encoding) = headers.get("Content-Encoding") {
                match content_encoding.to_str().unwrap_or_default() {
                    "gzip" => {
                        let mut enc = GzEncoder::new(Vec::new(), Compression::default());
                        let _ = enc.write_all(ser.into_inner().as_slice());
                        match enc.finish() {
                            Ok(buf) => {
                                local_req_builder = local_req_builder.body(buf);
                            }
                            Err(e) => return Err(datadog::Error::Io(e)),
                        }
                    }
                    "deflate" => {
                        let mut enc = ZlibEncoder::new(Vec::new(), Compression::default());
                        let _ = enc.write_all(ser.into_inner().as_slice());
                        match enc.finish() {
                            Ok(buf) => {
                                local_req_builder = local_req_builder.body(buf);
                            }
                            Err(e) => return Err(datadog::Error::Io(e)),
                        }
                    }
                    #[cfg(feature = "zstd")]
                    "zstd1" => {
                        let mut enc = zstd::stream::Encoder::new(Vec::new(), 0).unwrap();
                        let _ = enc.write_all(ser.into_inner().as_slice());
                        match enc.finish() {
                            Ok(buf) => {
                                local_req_builder = local_req_builder.body(buf);
                            }
                            Err(e) => return Err(datadog::Error::Io(e)),
                        }
                    }
                    _ => {
                        local_req_builder = local_req_builder.body(ser.into_inner());
                    }
                }
            } else {
                local_req_builder = local_req_builder.body(ser.into_inner());
            }
        }

        local_req_builder = local_req_builder.headers(headers);
        let local_req = local_req_builder.build()?;
        log::debug!("request content: {:?}", local_req.body());
        let local_resp = local_client.execute(local_req).await?;

        let local_status = local_resp.status();
        let local_content = local_resp.text().await?;
        log::debug!("response content: {}", local_content);

        if !local_status.is_client_error() && !local_status.is_server_error() {
            match serde_json::from_str::<crate::datadogV2::model::PublishAppResponse>(
                &local_content,
            ) {
                Ok(e) => {
                    return Ok(datadog::ResponseContent {
                        status: local_status,
                        content: local_content,
                        entity: Some(e),
                    })
                }
                Err(e) => return Err(datadog::Error::Serde(e)),
            };
        } else {
            let local_entity: Option<CreatePublishRequestError> =
                serde_json::from_str(&local_content).ok();
            let local_error = datadog::ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(datadog::Error::ResponseError(local_error))
        }
    }

    /// Delete a single app. This API requires a [registered application key](<https://docs.datadoghq.com/api/latest/action-connection/#register-a-new-app-key>). Alternatively, you can configure these permissions [in the UI](<https://docs.datadoghq.com/account_management/api-app-keys/#actions-api-access>).
    pub async fn delete_app(
        &self,
        app_id: uuid::Uuid,
    ) -> Result<crate::datadogV2::model::DeleteAppResponse, datadog::Error<DeleteAppError>> {
        match self.delete_app_with_http_info(app_id).await {
            Ok(response_content) => {
                if let Some(e) = response_content.entity {
                    Ok(e)
                } else {
                    Err(datadog::Error::Serde(serde::de::Error::custom(
                        "response content was None",
                    )))
                }
            }
            Err(err) => Err(err),
        }
    }

    /// Delete a single app. This API requires a [registered application key](<https://docs.datadoghq.com/api/latest/action-connection/#register-a-new-app-key>). Alternatively, you can configure these permissions [in the UI](<https://docs.datadoghq.com/account_management/api-app-keys/#actions-api-access>).
    pub async fn delete_app_with_http_info(
        &self,
        app_id: uuid::Uuid,
    ) -> Result<
        datadog::ResponseContent<crate::datadogV2::model::DeleteAppResponse>,
        datadog::Error<DeleteAppError>,
    > {
        let local_configuration = &self.config;
        let operation_id = "v2.delete_app";

        let local_client = &self.client;

        let local_uri_str = format!(
            "{}/api/v2/app-builder/apps/{app_id}",
            local_configuration.get_operation_host(operation_id),
            app_id = datadog::urlencode(app_id.to_string())
        );
        let mut local_req_builder =
            local_client.request(reqwest::Method::DELETE, local_uri_str.as_str());

        // build headers
        let mut headers = HeaderMap::new();
        headers.insert("Accept", HeaderValue::from_static("application/json"));

        // build user agent
        match HeaderValue::from_str(local_configuration.user_agent.as_str()) {
            Ok(user_agent) => headers.insert(reqwest::header::USER_AGENT, user_agent),
            Err(e) => {
                log::warn!("Failed to parse user agent header: {e}, falling back to default");
                headers.insert(
                    reqwest::header::USER_AGENT,
                    HeaderValue::from_static(datadog::DEFAULT_USER_AGENT.as_str()),
                )
            }
        };

        // build auth
        if let Some(local_key) = local_configuration.auth_keys.get("apiKeyAuth") {
            headers.insert(
                "DD-API-KEY",
                HeaderValue::from_str(local_key.key.as_str())
                    .expect("failed to parse DD-API-KEY header"),
            );
        };
        if let Some(local_key) = local_configuration.auth_keys.get("appKeyAuth") {
            headers.insert(
                "DD-APPLICATION-KEY",
                HeaderValue::from_str(local_key.key.as_str())
                    .expect("failed to parse DD-APPLICATION-KEY header"),
            );
        };

        local_req_builder = local_req_builder.headers(headers);
        let local_req = local_req_builder.build()?;
        log::debug!("request content: {:?}", local_req.body());
        let local_resp = local_client.execute(local_req).await?;

        let local_status = local_resp.status();
        let local_content = local_resp.text().await?;
        log::debug!("response content: {}", local_content);

        if !local_status.is_client_error() && !local_status.is_server_error() {
            match serde_json::from_str::<crate::datadogV2::model::DeleteAppResponse>(&local_content)
            {
                Ok(e) => {
                    return Ok(datadog::ResponseContent {
                        status: local_status,
                        content: local_content,
                        entity: Some(e),
                    })
                }
                Err(e) => return Err(datadog::Error::Serde(e)),
            };
        } else {
            let local_entity: Option<DeleteAppError> = serde_json::from_str(&local_content).ok();
            let local_error = datadog::ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(datadog::Error::ResponseError(local_error))
        }
    }

    /// Delete multiple apps in a single request from a list of app IDs. This API requires a [registered application key](<https://docs.datadoghq.com/api/latest/action-connection/#register-a-new-app-key>). Alternatively, you can configure these permissions [in the UI](<https://docs.datadoghq.com/account_management/api-app-keys/#actions-api-access>).
    pub async fn delete_apps(
        &self,
        body: crate::datadogV2::model::DeleteAppsRequest,
    ) -> Result<crate::datadogV2::model::DeleteAppsResponse, datadog::Error<DeleteAppsError>> {
        match self.delete_apps_with_http_info(body).await {
            Ok(response_content) => {
                if let Some(e) = response_content.entity {
                    Ok(e)
                } else {
                    Err(datadog::Error::Serde(serde::de::Error::custom(
                        "response content was None",
                    )))
                }
            }
            Err(err) => Err(err),
        }
    }

    /// Delete multiple apps in a single request from a list of app IDs. This API requires a [registered application key](<https://docs.datadoghq.com/api/latest/action-connection/#register-a-new-app-key>). Alternatively, you can configure these permissions [in the UI](<https://docs.datadoghq.com/account_management/api-app-keys/#actions-api-access>).
    pub async fn delete_apps_with_http_info(
        &self,
        body: crate::datadogV2::model::DeleteAppsRequest,
    ) -> Result<
        datadog::ResponseContent<crate::datadogV2::model::DeleteAppsResponse>,
        datadog::Error<DeleteAppsError>,
    > {
        let local_configuration = &self.config;
        let operation_id = "v2.delete_apps";

        let local_client = &self.client;

        let local_uri_str = format!(
            "{}/api/v2/app-builder/apps",
            local_configuration.get_operation_host(operation_id)
        );
        let mut local_req_builder =
            local_client.request(reqwest::Method::DELETE, local_uri_str.as_str());

        // build headers
        let mut headers = HeaderMap::new();
        headers.insert("Content-Type", HeaderValue::from_static("application/json"));
        headers.insert("Accept", HeaderValue::from_static("application/json"));

        // build user agent
        match HeaderValue::from_str(local_configuration.user_agent.as_str()) {
            Ok(user_agent) => headers.insert(reqwest::header::USER_AGENT, user_agent),
            Err(e) => {
                log::warn!("Failed to parse user agent header: {e}, falling back to default");
                headers.insert(
                    reqwest::header::USER_AGENT,
                    HeaderValue::from_static(datadog::DEFAULT_USER_AGENT.as_str()),
                )
            }
        };

        // build auth
        if let Some(local_key) = local_configuration.auth_keys.get("apiKeyAuth") {
            headers.insert(
                "DD-API-KEY",
                HeaderValue::from_str(local_key.key.as_str())
                    .expect("failed to parse DD-API-KEY header"),
            );
        };
        if let Some(local_key) = local_configuration.auth_keys.get("appKeyAuth") {
            headers.insert(
                "DD-APPLICATION-KEY",
                HeaderValue::from_str(local_key.key.as_str())
                    .expect("failed to parse DD-APPLICATION-KEY header"),
            );
        };

        // build body parameters
        let output = Vec::new();
        let mut ser = serde_json::Serializer::with_formatter(output, datadog::DDFormatter);
        if body.serialize(&mut ser).is_ok() {
            if let Some(content_encoding) = headers.get("Content-Encoding") {
                match content_encoding.to_str().unwrap_or_default() {
                    "gzip" => {
                        let mut enc = GzEncoder::new(Vec::new(), Compression::default());
                        let _ = enc.write_all(ser.into_inner().as_slice());
                        match enc.finish() {
                            Ok(buf) => {
                                local_req_builder = local_req_builder.body(buf);
                            }
                            Err(e) => return Err(datadog::Error::Io(e)),
                        }
                    }
                    "deflate" => {
                        let mut enc = ZlibEncoder::new(Vec::new(), Compression::default());
                        let _ = enc.write_all(ser.into_inner().as_slice());
                        match enc.finish() {
                            Ok(buf) => {
                                local_req_builder = local_req_builder.body(buf);
                            }
                            Err(e) => return Err(datadog::Error::Io(e)),
                        }
                    }
                    #[cfg(feature = "zstd")]
                    "zstd1" => {
                        let mut enc = zstd::stream::Encoder::new(Vec::new(), 0).unwrap();
                        let _ = enc.write_all(ser.into_inner().as_slice());
                        match enc.finish() {
                            Ok(buf) => {
                                local_req_builder = local_req_builder.body(buf);
                            }
                            Err(e) => return Err(datadog::Error::Io(e)),
                        }
                    }
                    _ => {
                        local_req_builder = local_req_builder.body(ser.into_inner());
                    }
                }
            } else {
                local_req_builder = local_req_builder.body(ser.into_inner());
            }
        }

        local_req_builder = local_req_builder.headers(headers);
        let local_req = local_req_builder.build()?;
        log::debug!("request content: {:?}", local_req.body());
        let local_resp = local_client.execute(local_req).await?;

        let local_status = local_resp.status();
        let local_content = local_resp.text().await?;
        log::debug!("response content: {}", local_content);

        if !local_status.is_client_error() && !local_status.is_server_error() {
            match serde_json::from_str::<crate::datadogV2::model::DeleteAppsResponse>(
                &local_content,
            ) {
                Ok(e) => {
                    return Ok(datadog::ResponseContent {
                        status: local_status,
                        content: local_content,
                        entity: Some(e),
                    })
                }
                Err(e) => return Err(datadog::Error::Serde(e)),
            };
        } else {
            let local_entity: Option<DeleteAppsError> = serde_json::from_str(&local_content).ok();
            let local_error = datadog::ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(datadog::Error::ResponseError(local_error))
        }
    }

    /// Get the full definition of an app. This API requires a [registered application key](<https://docs.datadoghq.com/api/latest/action-connection/#register-a-new-app-key>). Alternatively, you can configure these permissions [in the UI](<https://docs.datadoghq.com/account_management/api-app-keys/#actions-api-access>).
    pub async fn get_app(
        &self,
        app_id: uuid::Uuid,
        params: GetAppOptionalParams,
    ) -> Result<crate::datadogV2::model::GetAppResponse, datadog::Error<GetAppError>> {
        match self.get_app_with_http_info(app_id, params).await {
            Ok(response_content) => {
                if let Some(e) = response_content.entity {
                    Ok(e)
                } else {
                    Err(datadog::Error::Serde(serde::de::Error::custom(
                        "response content was None",
                    )))
                }
            }
            Err(err) => Err(err),
        }
    }

    /// Get the full definition of an app. This API requires a [registered application key](<https://docs.datadoghq.com/api/latest/action-connection/#register-a-new-app-key>). Alternatively, you can configure these permissions [in the UI](<https://docs.datadoghq.com/account_management/api-app-keys/#actions-api-access>).
    pub async fn get_app_with_http_info(
        &self,
        app_id: uuid::Uuid,
        params: GetAppOptionalParams,
    ) -> Result<
        datadog::ResponseContent<crate::datadogV2::model::GetAppResponse>,
        datadog::Error<GetAppError>,
    > {
        let local_configuration = &self.config;
        let operation_id = "v2.get_app";

        // unbox and build optional parameters
        let version = params.version;

        let local_client = &self.client;

        let local_uri_str = format!(
            "{}/api/v2/app-builder/apps/{app_id}",
            local_configuration.get_operation_host(operation_id),
            app_id = datadog::urlencode(app_id.to_string())
        );
        let mut local_req_builder =
            local_client.request(reqwest::Method::GET, local_uri_str.as_str());

        if let Some(ref local_query_param) = version {
            local_req_builder =
                local_req_builder.query(&[("version", &local_query_param.to_string())]);
        };

        // build headers
        let mut headers = HeaderMap::new();
        headers.insert("Accept", HeaderValue::from_static("application/json"));

        // build user agent
        match HeaderValue::from_str(local_configuration.user_agent.as_str()) {
            Ok(user_agent) => headers.insert(reqwest::header::USER_AGENT, user_agent),
            Err(e) => {
                log::warn!("Failed to parse user agent header: {e}, falling back to default");
                headers.insert(
                    reqwest::header::USER_AGENT,
                    HeaderValue::from_static(datadog::DEFAULT_USER_AGENT.as_str()),
                )
            }
        };

        // build auth
        if let Some(local_key) = local_configuration.auth_keys.get("apiKeyAuth") {
            headers.insert(
                "DD-API-KEY",
                HeaderValue::from_str(local_key.key.as_str())
                    .expect("failed to parse DD-API-KEY header"),
            );
        };
        if let Some(local_key) = local_configuration.auth_keys.get("appKeyAuth") {
            headers.insert(
                "DD-APPLICATION-KEY",
                HeaderValue::from_str(local_key.key.as_str())
                    .expect("failed to parse DD-APPLICATION-KEY header"),
            );
        };

        local_req_builder = local_req_builder.headers(headers);
        let local_req = local_req_builder.build()?;
        log::debug!("request content: {:?}", local_req.body());
        let local_resp = local_client.execute(local_req).await?;

        let local_status = local_resp.status();
        let local_content = local_resp.text().await?;
        log::debug!("response content: {}", local_content);

        if !local_status.is_client_error() && !local_status.is_server_error() {
            match serde_json::from_str::<crate::datadogV2::model::GetAppResponse>(&local_content) {
                Ok(e) => {
                    return Ok(datadog::ResponseContent {
                        status: local_status,
                        content: local_content,
                        entity: Some(e),
                    })
                }
                Err(e) => return Err(datadog::Error::Serde(e)),
            };
        } else {
            let local_entity: Option<GetAppError> = serde_json::from_str(&local_content).ok();
            let local_error = datadog::ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(datadog::Error::ResponseError(local_error))
        }
    }

    /// Retrieve an app blueprint by its ID.
    pub async fn get_blueprint(
        &self,
        blueprint_id: uuid::Uuid,
    ) -> Result<crate::datadogV2::model::GetBlueprintResponse, datadog::Error<GetBlueprintError>>
    {
        match self.get_blueprint_with_http_info(blueprint_id).await {
            Ok(response_content) => {
                if let Some(e) = response_content.entity {
                    Ok(e)
                } else {
                    Err(datadog::Error::Serde(serde::de::Error::custom(
                        "response content was None",
                    )))
                }
            }
            Err(err) => Err(err),
        }
    }

    /// Retrieve an app blueprint by its ID.
    pub async fn get_blueprint_with_http_info(
        &self,
        blueprint_id: uuid::Uuid,
    ) -> Result<
        datadog::ResponseContent<crate::datadogV2::model::GetBlueprintResponse>,
        datadog::Error<GetBlueprintError>,
    > {
        let local_configuration = &self.config;
        let operation_id = "v2.get_blueprint";

        let local_client = &self.client;

        let local_uri_str = format!(
            "{}/api/v2/app-builder/blueprint/{blueprint_id}",
            local_configuration.get_operation_host(operation_id),
            blueprint_id = datadog::urlencode(blueprint_id.to_string())
        );
        let mut local_req_builder =
            local_client.request(reqwest::Method::GET, local_uri_str.as_str());

        // build headers
        let mut headers = HeaderMap::new();
        headers.insert("Accept", HeaderValue::from_static("application/json"));

        // build user agent
        match HeaderValue::from_str(local_configuration.user_agent.as_str()) {
            Ok(user_agent) => headers.insert(reqwest::header::USER_AGENT, user_agent),
            Err(e) => {
                log::warn!("Failed to parse user agent header: {e}, falling back to default");
                headers.insert(
                    reqwest::header::USER_AGENT,
                    HeaderValue::from_static(datadog::DEFAULT_USER_AGENT.as_str()),
                )
            }
        };

        // build auth
        if let Some(local_key) = local_configuration.auth_keys.get("apiKeyAuth") {
            headers.insert(
                "DD-API-KEY",
                HeaderValue::from_str(local_key.key.as_str())
                    .expect("failed to parse DD-API-KEY header"),
            );
        };
        if let Some(local_key) = local_configuration.auth_keys.get("appKeyAuth") {
            headers.insert(
                "DD-APPLICATION-KEY",
                HeaderValue::from_str(local_key.key.as_str())
                    .expect("failed to parse DD-APPLICATION-KEY header"),
            );
        };

        local_req_builder = local_req_builder.headers(headers);
        let local_req = local_req_builder.build()?;
        log::debug!("request content: {:?}", local_req.body());
        let local_resp = local_client.execute(local_req).await?;

        let local_status = local_resp.status();
        let local_content = local_resp.text().await?;
        log::debug!("response content: {}", local_content);

        if !local_status.is_client_error() && !local_status.is_server_error() {
            match serde_json::from_str::<crate::datadogV2::model::GetBlueprintResponse>(
                &local_content,
            ) {
                Ok(e) => {
                    return Ok(datadog::ResponseContent {
                        status: local_status,
                        content: local_content,
                        entity: Some(e),
                    })
                }
                Err(e) => return Err(datadog::Error::Serde(e)),
            };
        } else {
            let local_entity: Option<GetBlueprintError> = serde_json::from_str(&local_content).ok();
            let local_error = datadog::ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(datadog::Error::ResponseError(local_error))
        }
    }

    /// List app blueprints associated with a specific integration ID.
    pub async fn get_blueprints_by_integration_id(
        &self,
        integration_id: String,
    ) -> Result<
        crate::datadogV2::model::GetBlueprintsResponse,
        datadog::Error<GetBlueprintsByIntegrationIdError>,
    > {
        match self
            .get_blueprints_by_integration_id_with_http_info(integration_id)
            .await
        {
            Ok(response_content) => {
                if let Some(e) = response_content.entity {
                    Ok(e)
                } else {
                    Err(datadog::Error::Serde(serde::de::Error::custom(
                        "response content was None",
                    )))
                }
            }
            Err(err) => Err(err),
        }
    }

    /// List app blueprints associated with a specific integration ID.
    pub async fn get_blueprints_by_integration_id_with_http_info(
        &self,
        integration_id: String,
    ) -> Result<
        datadog::ResponseContent<crate::datadogV2::model::GetBlueprintsResponse>,
        datadog::Error<GetBlueprintsByIntegrationIdError>,
    > {
        let local_configuration = &self.config;
        let operation_id = "v2.get_blueprints_by_integration_id";

        let local_client = &self.client;

        let local_uri_str = format!(
            "{}/api/v2/app-builder/blueprints/integration-id/{integration_id}",
            local_configuration.get_operation_host(operation_id),
            integration_id = datadog::urlencode(integration_id)
        );
        let mut local_req_builder =
            local_client.request(reqwest::Method::GET, local_uri_str.as_str());

        // build headers
        let mut headers = HeaderMap::new();
        headers.insert("Accept", HeaderValue::from_static("application/json"));

        // build user agent
        match HeaderValue::from_str(local_configuration.user_agent.as_str()) {
            Ok(user_agent) => headers.insert(reqwest::header::USER_AGENT, user_agent),
            Err(e) => {
                log::warn!("Failed to parse user agent header: {e}, falling back to default");
                headers.insert(
                    reqwest::header::USER_AGENT,
                    HeaderValue::from_static(datadog::DEFAULT_USER_AGENT.as_str()),
                )
            }
        };

        // build auth
        if let Some(local_key) = local_configuration.auth_keys.get("apiKeyAuth") {
            headers.insert(
                "DD-API-KEY",
                HeaderValue::from_str(local_key.key.as_str())
                    .expect("failed to parse DD-API-KEY header"),
            );
        };
        if let Some(local_key) = local_configuration.auth_keys.get("appKeyAuth") {
            headers.insert(
                "DD-APPLICATION-KEY",
                HeaderValue::from_str(local_key.key.as_str())
                    .expect("failed to parse DD-APPLICATION-KEY header"),
            );
        };

        local_req_builder = local_req_builder.headers(headers);
        let local_req = local_req_builder.build()?;
        log::debug!("request content: {:?}", local_req.body());
        let local_resp = local_client.execute(local_req).await?;

        let local_status = local_resp.status();
        let local_content = local_resp.text().await?;
        log::debug!("response content: {}", local_content);

        if !local_status.is_client_error() && !local_status.is_server_error() {
            match serde_json::from_str::<crate::datadogV2::model::GetBlueprintsResponse>(
                &local_content,
            ) {
                Ok(e) => {
                    return Ok(datadog::ResponseContent {
                        status: local_status,
                        content: local_content,
                        entity: Some(e),
                    })
                }
                Err(e) => return Err(datadog::Error::Serde(e)),
            };
        } else {
            let local_entity: Option<GetBlueprintsByIntegrationIdError> =
                serde_json::from_str(&local_content).ok();
            let local_error = datadog::ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(datadog::Error::ResponseError(local_error))
        }
    }

    /// Retrieve app blueprints by their slugs.
    pub async fn get_blueprints_by_slugs(
        &self,
        slugs: String,
    ) -> Result<
        crate::datadogV2::model::GetBlueprintsResponse,
        datadog::Error<GetBlueprintsBySlugsError>,
    > {
        match self.get_blueprints_by_slugs_with_http_info(slugs).await {
            Ok(response_content) => {
                if let Some(e) = response_content.entity {
                    Ok(e)
                } else {
                    Err(datadog::Error::Serde(serde::de::Error::custom(
                        "response content was None",
                    )))
                }
            }
            Err(err) => Err(err),
        }
    }

    /// Retrieve app blueprints by their slugs.
    pub async fn get_blueprints_by_slugs_with_http_info(
        &self,
        slugs: String,
    ) -> Result<
        datadog::ResponseContent<crate::datadogV2::model::GetBlueprintsResponse>,
        datadog::Error<GetBlueprintsBySlugsError>,
    > {
        let local_configuration = &self.config;
        let operation_id = "v2.get_blueprints_by_slugs";

        let local_client = &self.client;

        let local_uri_str = format!(
            "{}/api/v2/app-builder/blueprints/slugs/{slugs}",
            local_configuration.get_operation_host(operation_id),
            slugs = datadog::urlencode(slugs)
        );
        let mut local_req_builder =
            local_client.request(reqwest::Method::GET, local_uri_str.as_str());

        // build headers
        let mut headers = HeaderMap::new();
        headers.insert("Accept", HeaderValue::from_static("application/json"));

        // build user agent
        match HeaderValue::from_str(local_configuration.user_agent.as_str()) {
            Ok(user_agent) => headers.insert(reqwest::header::USER_AGENT, user_agent),
            Err(e) => {
                log::warn!("Failed to parse user agent header: {e}, falling back to default");
                headers.insert(
                    reqwest::header::USER_AGENT,
                    HeaderValue::from_static(datadog::DEFAULT_USER_AGENT.as_str()),
                )
            }
        };

        // build auth
        if let Some(local_key) = local_configuration.auth_keys.get("apiKeyAuth") {
            headers.insert(
                "DD-API-KEY",
                HeaderValue::from_str(local_key.key.as_str())
                    .expect("failed to parse DD-API-KEY header"),
            );
        };
        if let Some(local_key) = local_configuration.auth_keys.get("appKeyAuth") {
            headers.insert(
                "DD-APPLICATION-KEY",
                HeaderValue::from_str(local_key.key.as_str())
                    .expect("failed to parse DD-APPLICATION-KEY header"),
            );
        };

        local_req_builder = local_req_builder.headers(headers);
        let local_req = local_req_builder.build()?;
        log::debug!("request content: {:?}", local_req.body());
        let local_resp = local_client.execute(local_req).await?;

        let local_status = local_resp.status();
        let local_content = local_resp.text().await?;
        log::debug!("response content: {}", local_content);

        if !local_status.is_client_error() && !local_status.is_server_error() {
            match serde_json::from_str::<crate::datadogV2::model::GetBlueprintsResponse>(
                &local_content,
            ) {
                Ok(e) => {
                    return Ok(datadog::ResponseContent {
                        status: local_status,
                        content: local_content,
                        entity: Some(e),
                    })
                }
                Err(e) => return Err(datadog::Error::Serde(e)),
            };
        } else {
            let local_entity: Option<GetBlueprintsBySlugsError> =
                serde_json::from_str(&local_content).ok();
            let local_error = datadog::ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(datadog::Error::ResponseError(local_error))
        }
    }

    /// List the versions of an app. This endpoint is paginated.
    pub async fn list_app_versions(
        &self,
        app_id: uuid::Uuid,
        params: ListAppVersionsOptionalParams,
    ) -> Result<
        crate::datadogV2::model::ListAppVersionsResponse,
        datadog::Error<ListAppVersionsError>,
    > {
        match self.list_app_versions_with_http_info(app_id, params).await {
            Ok(response_content) => {
                if let Some(e) = response_content.entity {
                    Ok(e)
                } else {
                    Err(datadog::Error::Serde(serde::de::Error::custom(
                        "response content was None",
                    )))
                }
            }
            Err(err) => Err(err),
        }
    }

    /// List the versions of an app. This endpoint is paginated.
    pub async fn list_app_versions_with_http_info(
        &self,
        app_id: uuid::Uuid,
        params: ListAppVersionsOptionalParams,
    ) -> Result<
        datadog::ResponseContent<crate::datadogV2::model::ListAppVersionsResponse>,
        datadog::Error<ListAppVersionsError>,
    > {
        let local_configuration = &self.config;
        let operation_id = "v2.list_app_versions";

        // unbox and build optional parameters
        let limit = params.limit;
        let page = params.page;

        let local_client = &self.client;

        let local_uri_str = format!(
            "{}/api/v2/app-builder/apps/{app_id}/versions",
            local_configuration.get_operation_host(operation_id),
            app_id = datadog::urlencode(app_id.to_string())
        );
        let mut local_req_builder =
            local_client.request(reqwest::Method::GET, local_uri_str.as_str());

        if let Some(ref local_query_param) = limit {
            local_req_builder =
                local_req_builder.query(&[("limit", &local_query_param.to_string())]);
        };
        if let Some(ref local_query_param) = page {
            local_req_builder =
                local_req_builder.query(&[("page", &local_query_param.to_string())]);
        };

        // build headers
        let mut headers = HeaderMap::new();
        headers.insert("Accept", HeaderValue::from_static("application/json"));

        // build user agent
        match HeaderValue::from_str(local_configuration.user_agent.as_str()) {
            Ok(user_agent) => headers.insert(reqwest::header::USER_AGENT, user_agent),
            Err(e) => {
                log::warn!("Failed to parse user agent header: {e}, falling back to default");
                headers.insert(
                    reqwest::header::USER_AGENT,
                    HeaderValue::from_static(datadog::DEFAULT_USER_AGENT.as_str()),
                )
            }
        };

        // build auth
        if let Some(local_key) = local_configuration.auth_keys.get("apiKeyAuth") {
            headers.insert(
                "DD-API-KEY",
                HeaderValue::from_str(local_key.key.as_str())
                    .expect("failed to parse DD-API-KEY header"),
            );
        };
        if let Some(local_key) = local_configuration.auth_keys.get("appKeyAuth") {
            headers.insert(
                "DD-APPLICATION-KEY",
                HeaderValue::from_str(local_key.key.as_str())
                    .expect("failed to parse DD-APPLICATION-KEY header"),
            );
        };

        local_req_builder = local_req_builder.headers(headers);
        let local_req = local_req_builder.build()?;
        log::debug!("request content: {:?}", local_req.body());
        let local_resp = local_client.execute(local_req).await?;

        let local_status = local_resp.status();
        let local_content = local_resp.text().await?;
        log::debug!("response content: {}", local_content);

        if !local_status.is_client_error() && !local_status.is_server_error() {
            match serde_json::from_str::<crate::datadogV2::model::ListAppVersionsResponse>(
                &local_content,
            ) {
                Ok(e) => {
                    return Ok(datadog::ResponseContent {
                        status: local_status,
                        content: local_content,
                        entity: Some(e),
                    })
                }
                Err(e) => return Err(datadog::Error::Serde(e)),
            };
        } else {
            let local_entity: Option<ListAppVersionsError> =
                serde_json::from_str(&local_content).ok();
            let local_error = datadog::ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(datadog::Error::ResponseError(local_error))
        }
    }

    /// List all apps, with optional filters and sorting. This endpoint is paginated. Only basic app information such as the app ID, name, and description is returned by this endpoint. This API requires a [registered application key](<https://docs.datadoghq.com/api/latest/action-connection/#register-a-new-app-key>). Alternatively, you can configure these permissions [in the UI](<https://docs.datadoghq.com/account_management/api-app-keys/#actions-api-access>).
    pub async fn list_apps(
        &self,
        params: ListAppsOptionalParams,
    ) -> Result<crate::datadogV2::model::ListAppsResponse, datadog::Error<ListAppsError>> {
        match self.list_apps_with_http_info(params).await {
            Ok(response_content) => {
                if let Some(e) = response_content.entity {
                    Ok(e)
                } else {
                    Err(datadog::Error::Serde(serde::de::Error::custom(
                        "response content was None",
                    )))
                }
            }
            Err(err) => Err(err),
        }
    }

    /// List all apps, with optional filters and sorting. This endpoint is paginated. Only basic app information such as the app ID, name, and description is returned by this endpoint. This API requires a [registered application key](<https://docs.datadoghq.com/api/latest/action-connection/#register-a-new-app-key>). Alternatively, you can configure these permissions [in the UI](<https://docs.datadoghq.com/account_management/api-app-keys/#actions-api-access>).
    pub async fn list_apps_with_http_info(
        &self,
        params: ListAppsOptionalParams,
    ) -> Result<
        datadog::ResponseContent<crate::datadogV2::model::ListAppsResponse>,
        datadog::Error<ListAppsError>,
    > {
        let local_configuration = &self.config;
        let operation_id = "v2.list_apps";

        // unbox and build optional parameters
        let limit = params.limit;
        let page = params.page;
        let filter_user_name = params.filter_user_name;
        let filter_user_uuid = params.filter_user_uuid;
        let filter_name = params.filter_name;
        let filter_query = params.filter_query;
        let filter_deployed = params.filter_deployed;
        let filter_tags = params.filter_tags;
        let filter_favorite = params.filter_favorite;
        let filter_self_service = params.filter_self_service;
        let sort = params.sort;

        let local_client = &self.client;

        let local_uri_str = format!(
            "{}/api/v2/app-builder/apps",
            local_configuration.get_operation_host(operation_id)
        );
        let mut local_req_builder =
            local_client.request(reqwest::Method::GET, local_uri_str.as_str());

        if let Some(ref local_query_param) = limit {
            local_req_builder =
                local_req_builder.query(&[("limit", &local_query_param.to_string())]);
        };
        if let Some(ref local_query_param) = page {
            local_req_builder =
                local_req_builder.query(&[("page", &local_query_param.to_string())]);
        };
        if let Some(ref local_query_param) = filter_user_name {
            local_req_builder =
                local_req_builder.query(&[("filter[user_name]", &local_query_param.to_string())]);
        };
        if let Some(ref local_query_param) = filter_user_uuid {
            local_req_builder =
                local_req_builder.query(&[("filter[user_uuid]", &local_query_param.to_string())]);
        };
        if let Some(ref local_query_param) = filter_name {
            local_req_builder =
                local_req_builder.query(&[("filter[name]", &local_query_param.to_string())]);
        };
        if let Some(ref local_query_param) = filter_query {
            local_req_builder =
                local_req_builder.query(&[("filter[query]", &local_query_param.to_string())]);
        };
        if let Some(ref local_query_param) = filter_deployed {
            local_req_builder =
                local_req_builder.query(&[("filter[deployed]", &local_query_param.to_string())]);
        };
        if let Some(ref local_query_param) = filter_tags {
            local_req_builder =
                local_req_builder.query(&[("filter[tags]", &local_query_param.to_string())]);
        };
        if let Some(ref local_query_param) = filter_favorite {
            local_req_builder =
                local_req_builder.query(&[("filter[favorite]", &local_query_param.to_string())]);
        };
        if let Some(ref local_query_param) = filter_self_service {
            local_req_builder = local_req_builder
                .query(&[("filter[self_service]", &local_query_param.to_string())]);
        };
        if let Some(ref local) = sort {
            local_req_builder = local_req_builder.query(&[(
                "sort",
                &local
                    .iter()
                    .map(|p| p.to_string())
                    .collect::<Vec<String>>()
                    .join(",")
                    .to_string(),
            )]);
        };

        // build headers
        let mut headers = HeaderMap::new();
        headers.insert("Accept", HeaderValue::from_static("application/json"));

        // build user agent
        match HeaderValue::from_str(local_configuration.user_agent.as_str()) {
            Ok(user_agent) => headers.insert(reqwest::header::USER_AGENT, user_agent),
            Err(e) => {
                log::warn!("Failed to parse user agent header: {e}, falling back to default");
                headers.insert(
                    reqwest::header::USER_AGENT,
                    HeaderValue::from_static(datadog::DEFAULT_USER_AGENT.as_str()),
                )
            }
        };

        // build auth
        if let Some(local_key) = local_configuration.auth_keys.get("apiKeyAuth") {
            headers.insert(
                "DD-API-KEY",
                HeaderValue::from_str(local_key.key.as_str())
                    .expect("failed to parse DD-API-KEY header"),
            );
        };
        if let Some(local_key) = local_configuration.auth_keys.get("appKeyAuth") {
            headers.insert(
                "DD-APPLICATION-KEY",
                HeaderValue::from_str(local_key.key.as_str())
                    .expect("failed to parse DD-APPLICATION-KEY header"),
            );
        };

        local_req_builder = local_req_builder.headers(headers);
        let local_req = local_req_builder.build()?;
        log::debug!("request content: {:?}", local_req.body());
        let local_resp = local_client.execute(local_req).await?;

        let local_status = local_resp.status();
        let local_content = local_resp.text().await?;
        log::debug!("response content: {}", local_content);

        if !local_status.is_client_error() && !local_status.is_server_error() {
            match serde_json::from_str::<crate::datadogV2::model::ListAppsResponse>(&local_content)
            {
                Ok(e) => {
                    return Ok(datadog::ResponseContent {
                        status: local_status,
                        content: local_content,
                        entity: Some(e),
                    })
                }
                Err(e) => return Err(datadog::Error::Serde(e)),
            };
        } else {
            let local_entity: Option<ListAppsError> = serde_json::from_str(&local_content).ok();
            let local_error = datadog::ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(datadog::Error::ResponseError(local_error))
        }
    }

    /// List available app blueprints.
    pub async fn list_blueprints(
        &self,
        params: ListBlueprintsOptionalParams,
    ) -> Result<crate::datadogV2::model::ListBlueprintsResponse, datadog::Error<ListBlueprintsError>>
    {
        match self.list_blueprints_with_http_info(params).await {
            Ok(response_content) => {
                if let Some(e) = response_content.entity {
                    Ok(e)
                } else {
                    Err(datadog::Error::Serde(serde::de::Error::custom(
                        "response content was None",
                    )))
                }
            }
            Err(err) => Err(err),
        }
    }

    /// List available app blueprints.
    pub async fn list_blueprints_with_http_info(
        &self,
        params: ListBlueprintsOptionalParams,
    ) -> Result<
        datadog::ResponseContent<crate::datadogV2::model::ListBlueprintsResponse>,
        datadog::Error<ListBlueprintsError>,
    > {
        let local_configuration = &self.config;
        let operation_id = "v2.list_blueprints";

        // unbox and build optional parameters
        let limit = params.limit;
        let page = params.page;

        let local_client = &self.client;

        let local_uri_str = format!(
            "{}/api/v2/app-builder/blueprints",
            local_configuration.get_operation_host(operation_id)
        );
        let mut local_req_builder =
            local_client.request(reqwest::Method::GET, local_uri_str.as_str());

        if let Some(ref local_query_param) = limit {
            local_req_builder =
                local_req_builder.query(&[("limit", &local_query_param.to_string())]);
        };
        if let Some(ref local_query_param) = page {
            local_req_builder =
                local_req_builder.query(&[("page", &local_query_param.to_string())]);
        };

        // build headers
        let mut headers = HeaderMap::new();
        headers.insert("Accept", HeaderValue::from_static("application/json"));

        // build user agent
        match HeaderValue::from_str(local_configuration.user_agent.as_str()) {
            Ok(user_agent) => headers.insert(reqwest::header::USER_AGENT, user_agent),
            Err(e) => {
                log::warn!("Failed to parse user agent header: {e}, falling back to default");
                headers.insert(
                    reqwest::header::USER_AGENT,
                    HeaderValue::from_static(datadog::DEFAULT_USER_AGENT.as_str()),
                )
            }
        };

        // build auth
        if let Some(local_key) = local_configuration.auth_keys.get("apiKeyAuth") {
            headers.insert(
                "DD-API-KEY",
                HeaderValue::from_str(local_key.key.as_str())
                    .expect("failed to parse DD-API-KEY header"),
            );
        };
        if let Some(local_key) = local_configuration.auth_keys.get("appKeyAuth") {
            headers.insert(
                "DD-APPLICATION-KEY",
                HeaderValue::from_str(local_key.key.as_str())
                    .expect("failed to parse DD-APPLICATION-KEY header"),
            );
        };

        local_req_builder = local_req_builder.headers(headers);
        let local_req = local_req_builder.build()?;
        log::debug!("request content: {:?}", local_req.body());
        let local_resp = local_client.execute(local_req).await?;

        let local_status = local_resp.status();
        let local_content = local_resp.text().await?;
        log::debug!("response content: {}", local_content);

        if !local_status.is_client_error() && !local_status.is_server_error() {
            match serde_json::from_str::<crate::datadogV2::model::ListBlueprintsResponse>(
                &local_content,
            ) {
                Ok(e) => {
                    return Ok(datadog::ResponseContent {
                        status: local_status,
                        content: local_content,
                        entity: Some(e),
                    })
                }
                Err(e) => return Err(datadog::Error::Serde(e)),
            };
        } else {
            let local_entity: Option<ListBlueprintsError> =
                serde_json::from_str(&local_content).ok();
            let local_error = datadog::ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(datadog::Error::ResponseError(local_error))
        }
    }

    /// List all tags associated with the authenticated user's apps.
    pub async fn list_tags(
        &self,
    ) -> Result<crate::datadogV2::model::AppBuilderListTagsResponse, datadog::Error<ListTagsError>>
    {
        match self.list_tags_with_http_info().await {
            Ok(response_content) => {
                if let Some(e) = response_content.entity {
                    Ok(e)
                } else {
                    Err(datadog::Error::Serde(serde::de::Error::custom(
                        "response content was None",
                    )))
                }
            }
            Err(err) => Err(err),
        }
    }

    /// List all tags associated with the authenticated user's apps.
    pub async fn list_tags_with_http_info(
        &self,
    ) -> Result<
        datadog::ResponseContent<crate::datadogV2::model::AppBuilderListTagsResponse>,
        datadog::Error<ListTagsError>,
    > {
        let local_configuration = &self.config;
        let operation_id = "v2.list_tags";

        let local_client = &self.client;

        let local_uri_str = format!(
            "{}/api/v2/app-builder/tags",
            local_configuration.get_operation_host(operation_id)
        );
        let mut local_req_builder =
            local_client.request(reqwest::Method::GET, local_uri_str.as_str());

        // build headers
        let mut headers = HeaderMap::new();
        headers.insert("Accept", HeaderValue::from_static("application/json"));

        // build user agent
        match HeaderValue::from_str(local_configuration.user_agent.as_str()) {
            Ok(user_agent) => headers.insert(reqwest::header::USER_AGENT, user_agent),
            Err(e) => {
                log::warn!("Failed to parse user agent header: {e}, falling back to default");
                headers.insert(
                    reqwest::header::USER_AGENT,
                    HeaderValue::from_static(datadog::DEFAULT_USER_AGENT.as_str()),
                )
            }
        };

        // build auth
        if let Some(local_key) = local_configuration.auth_keys.get("apiKeyAuth") {
            headers.insert(
                "DD-API-KEY",
                HeaderValue::from_str(local_key.key.as_str())
                    .expect("failed to parse DD-API-KEY header"),
            );
        };
        if let Some(local_key) = local_configuration.auth_keys.get("appKeyAuth") {
            headers.insert(
                "DD-APPLICATION-KEY",
                HeaderValue::from_str(local_key.key.as_str())
                    .expect("failed to parse DD-APPLICATION-KEY header"),
            );
        };

        local_req_builder = local_req_builder.headers(headers);
        let local_req = local_req_builder.build()?;
        log::debug!("request content: {:?}", local_req.body());
        let local_resp = local_client.execute(local_req).await?;

        let local_status = local_resp.status();
        let local_content = local_resp.text().await?;
        log::debug!("response content: {}", local_content);

        if !local_status.is_client_error() && !local_status.is_server_error() {
            match serde_json::from_str::<crate::datadogV2::model::AppBuilderListTagsResponse>(
                &local_content,
            ) {
                Ok(e) => {
                    return Ok(datadog::ResponseContent {
                        status: local_status,
                        content: local_content,
                        entity: Some(e),
                    })
                }
                Err(e) => return Err(datadog::Error::Serde(e)),
            };
        } else {
            let local_entity: Option<ListTagsError> = serde_json::from_str(&local_content).ok();
            let local_error = datadog::ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(datadog::Error::ResponseError(local_error))
        }
    }

    /// Publish an app for use by other users. To ensure the app is accessible to the correct users, you also need to set a [Restriction Policy](<https://docs.datadoghq.com/api/latest/restriction-policies/>) on the app if a policy does not yet exist. This API requires a [registered application key](<https://docs.datadoghq.com/api/latest/action-connection/#register-a-new-app-key>). Alternatively, you can configure these permissions [in the UI](<https://docs.datadoghq.com/account_management/api-app-keys/#actions-api-access>).
    pub async fn publish_app(
        &self,
        app_id: uuid::Uuid,
    ) -> Result<crate::datadogV2::model::PublishAppResponse, datadog::Error<PublishAppError>> {
        match self.publish_app_with_http_info(app_id).await {
            Ok(response_content) => {
                if let Some(e) = response_content.entity {
                    Ok(e)
                } else {
                    Err(datadog::Error::Serde(serde::de::Error::custom(
                        "response content was None",
                    )))
                }
            }
            Err(err) => Err(err),
        }
    }

    /// Publish an app for use by other users. To ensure the app is accessible to the correct users, you also need to set a [Restriction Policy](<https://docs.datadoghq.com/api/latest/restriction-policies/>) on the app if a policy does not yet exist. This API requires a [registered application key](<https://docs.datadoghq.com/api/latest/action-connection/#register-a-new-app-key>). Alternatively, you can configure these permissions [in the UI](<https://docs.datadoghq.com/account_management/api-app-keys/#actions-api-access>).
    pub async fn publish_app_with_http_info(
        &self,
        app_id: uuid::Uuid,
    ) -> Result<
        datadog::ResponseContent<crate::datadogV2::model::PublishAppResponse>,
        datadog::Error<PublishAppError>,
    > {
        let local_configuration = &self.config;
        let operation_id = "v2.publish_app";

        let local_client = &self.client;

        let local_uri_str = format!(
            "{}/api/v2/app-builder/apps/{app_id}/deployment",
            local_configuration.get_operation_host(operation_id),
            app_id = datadog::urlencode(app_id.to_string())
        );
        let mut local_req_builder =
            local_client.request(reqwest::Method::POST, local_uri_str.as_str());

        // build headers
        let mut headers = HeaderMap::new();
        headers.insert("Accept", HeaderValue::from_static("application/json"));

        // build user agent
        match HeaderValue::from_str(local_configuration.user_agent.as_str()) {
            Ok(user_agent) => headers.insert(reqwest::header::USER_AGENT, user_agent),
            Err(e) => {
                log::warn!("Failed to parse user agent header: {e}, falling back to default");
                headers.insert(
                    reqwest::header::USER_AGENT,
                    HeaderValue::from_static(datadog::DEFAULT_USER_AGENT.as_str()),
                )
            }
        };

        // build auth
        if let Some(local_key) = local_configuration.auth_keys.get("apiKeyAuth") {
            headers.insert(
                "DD-API-KEY",
                HeaderValue::from_str(local_key.key.as_str())
                    .expect("failed to parse DD-API-KEY header"),
            );
        };
        if let Some(local_key) = local_configuration.auth_keys.get("appKeyAuth") {
            headers.insert(
                "DD-APPLICATION-KEY",
                HeaderValue::from_str(local_key.key.as_str())
                    .expect("failed to parse DD-APPLICATION-KEY header"),
            );
        };

        local_req_builder = local_req_builder.headers(headers);
        let local_req = local_req_builder.build()?;
        log::debug!("request content: {:?}", local_req.body());
        let local_resp = local_client.execute(local_req).await?;

        let local_status = local_resp.status();
        let local_content = local_resp.text().await?;
        log::debug!("response content: {}", local_content);

        if !local_status.is_client_error() && !local_status.is_server_error() {
            match serde_json::from_str::<crate::datadogV2::model::PublishAppResponse>(
                &local_content,
            ) {
                Ok(e) => {
                    return Ok(datadog::ResponseContent {
                        status: local_status,
                        content: local_content,
                        entity: Some(e),
                    })
                }
                Err(e) => return Err(datadog::Error::Serde(e)),
            };
        } else {
            let local_entity: Option<PublishAppError> = serde_json::from_str(&local_content).ok();
            let local_error = datadog::ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(datadog::Error::ResponseError(local_error))
        }
    }

    /// Revert an app to a previous version. The version to revert to is selected through the `version` query parameter. The reverted version becomes the new latest version of the app.
    pub async fn revert_app(
        &self,
        app_id: uuid::Uuid,
        version: String,
    ) -> Result<crate::datadogV2::model::UpdateAppResponse, datadog::Error<RevertAppError>> {
        match self.revert_app_with_http_info(app_id, version).await {
            Ok(response_content) => {
                if let Some(e) = response_content.entity {
                    Ok(e)
                } else {
                    Err(datadog::Error::Serde(serde::de::Error::custom(
                        "response content was None",
                    )))
                }
            }
            Err(err) => Err(err),
        }
    }

    /// Revert an app to a previous version. The version to revert to is selected through the `version` query parameter. The reverted version becomes the new latest version of the app.
    pub async fn revert_app_with_http_info(
        &self,
        app_id: uuid::Uuid,
        version: String,
    ) -> Result<
        datadog::ResponseContent<crate::datadogV2::model::UpdateAppResponse>,
        datadog::Error<RevertAppError>,
    > {
        let local_configuration = &self.config;
        let operation_id = "v2.revert_app";

        let local_client = &self.client;

        let local_uri_str = format!(
            "{}/api/v2/app-builder/apps/{app_id}/revert",
            local_configuration.get_operation_host(operation_id),
            app_id = datadog::urlencode(app_id.to_string())
        );
        let mut local_req_builder =
            local_client.request(reqwest::Method::POST, local_uri_str.as_str());

        local_req_builder = local_req_builder.query(&[("version", &version.to_string())]);

        // build headers
        let mut headers = HeaderMap::new();
        headers.insert("Accept", HeaderValue::from_static("application/json"));

        // build user agent
        match HeaderValue::from_str(local_configuration.user_agent.as_str()) {
            Ok(user_agent) => headers.insert(reqwest::header::USER_AGENT, user_agent),
            Err(e) => {
                log::warn!("Failed to parse user agent header: {e}, falling back to default");
                headers.insert(
                    reqwest::header::USER_AGENT,
                    HeaderValue::from_static(datadog::DEFAULT_USER_AGENT.as_str()),
                )
            }
        };

        // build auth
        if let Some(local_key) = local_configuration.auth_keys.get("apiKeyAuth") {
            headers.insert(
                "DD-API-KEY",
                HeaderValue::from_str(local_key.key.as_str())
                    .expect("failed to parse DD-API-KEY header"),
            );
        };
        if let Some(local_key) = local_configuration.auth_keys.get("appKeyAuth") {
            headers.insert(
                "DD-APPLICATION-KEY",
                HeaderValue::from_str(local_key.key.as_str())
                    .expect("failed to parse DD-APPLICATION-KEY header"),
            );
        };

        local_req_builder = local_req_builder.headers(headers);
        let local_req = local_req_builder.build()?;
        log::debug!("request content: {:?}", local_req.body());
        let local_resp = local_client.execute(local_req).await?;

        let local_status = local_resp.status();
        let local_content = local_resp.text().await?;
        log::debug!("response content: {}", local_content);

        if !local_status.is_client_error() && !local_status.is_server_error() {
            match serde_json::from_str::<crate::datadogV2::model::UpdateAppResponse>(&local_content)
            {
                Ok(e) => {
                    return Ok(datadog::ResponseContent {
                        status: local_status,
                        content: local_content,
                        entity: Some(e),
                    })
                }
                Err(e) => return Err(datadog::Error::Serde(e)),
            };
        } else {
            let local_entity: Option<RevertAppError> = serde_json::from_str(&local_content).ok();
            let local_error = datadog::ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(datadog::Error::ResponseError(local_error))
        }
    }

    /// Unpublish an app, removing the live version of the app. Unpublishing creates a new instance of a `deployment` object on the app, with a nil `app_version_id` (`00000000-0000-0000-0000-000000000000`). The app can still be updated and published again in the future. This API requires a [registered application key](<https://docs.datadoghq.com/api/latest/action-connection/#register-a-new-app-key>). Alternatively, you can configure these permissions [in the UI](<https://docs.datadoghq.com/account_management/api-app-keys/#actions-api-access>).
    pub async fn unpublish_app(
        &self,
        app_id: uuid::Uuid,
    ) -> Result<crate::datadogV2::model::UnpublishAppResponse, datadog::Error<UnpublishAppError>>
    {
        match self.unpublish_app_with_http_info(app_id).await {
            Ok(response_content) => {
                if let Some(e) = response_content.entity {
                    Ok(e)
                } else {
                    Err(datadog::Error::Serde(serde::de::Error::custom(
                        "response content was None",
                    )))
                }
            }
            Err(err) => Err(err),
        }
    }

    /// Unpublish an app, removing the live version of the app. Unpublishing creates a new instance of a `deployment` object on the app, with a nil `app_version_id` (`00000000-0000-0000-0000-000000000000`). The app can still be updated and published again in the future. This API requires a [registered application key](<https://docs.datadoghq.com/api/latest/action-connection/#register-a-new-app-key>). Alternatively, you can configure these permissions [in the UI](<https://docs.datadoghq.com/account_management/api-app-keys/#actions-api-access>).
    pub async fn unpublish_app_with_http_info(
        &self,
        app_id: uuid::Uuid,
    ) -> Result<
        datadog::ResponseContent<crate::datadogV2::model::UnpublishAppResponse>,
        datadog::Error<UnpublishAppError>,
    > {
        let local_configuration = &self.config;
        let operation_id = "v2.unpublish_app";

        let local_client = &self.client;

        let local_uri_str = format!(
            "{}/api/v2/app-builder/apps/{app_id}/deployment",
            local_configuration.get_operation_host(operation_id),
            app_id = datadog::urlencode(app_id.to_string())
        );
        let mut local_req_builder =
            local_client.request(reqwest::Method::DELETE, local_uri_str.as_str());

        // build headers
        let mut headers = HeaderMap::new();
        headers.insert("Accept", HeaderValue::from_static("application/json"));

        // build user agent
        match HeaderValue::from_str(local_configuration.user_agent.as_str()) {
            Ok(user_agent) => headers.insert(reqwest::header::USER_AGENT, user_agent),
            Err(e) => {
                log::warn!("Failed to parse user agent header: {e}, falling back to default");
                headers.insert(
                    reqwest::header::USER_AGENT,
                    HeaderValue::from_static(datadog::DEFAULT_USER_AGENT.as_str()),
                )
            }
        };

        // build auth
        if let Some(local_key) = local_configuration.auth_keys.get("apiKeyAuth") {
            headers.insert(
                "DD-API-KEY",
                HeaderValue::from_str(local_key.key.as_str())
                    .expect("failed to parse DD-API-KEY header"),
            );
        };
        if let Some(local_key) = local_configuration.auth_keys.get("appKeyAuth") {
            headers.insert(
                "DD-APPLICATION-KEY",
                HeaderValue::from_str(local_key.key.as_str())
                    .expect("failed to parse DD-APPLICATION-KEY header"),
            );
        };

        local_req_builder = local_req_builder.headers(headers);
        let local_req = local_req_builder.build()?;
        log::debug!("request content: {:?}", local_req.body());
        let local_resp = local_client.execute(local_req).await?;

        let local_status = local_resp.status();
        let local_content = local_resp.text().await?;
        log::debug!("response content: {}", local_content);

        if !local_status.is_client_error() && !local_status.is_server_error() {
            match serde_json::from_str::<crate::datadogV2::model::UnpublishAppResponse>(
                &local_content,
            ) {
                Ok(e) => {
                    return Ok(datadog::ResponseContent {
                        status: local_status,
                        content: local_content,
                        entity: Some(e),
                    })
                }
                Err(e) => return Err(datadog::Error::Serde(e)),
            };
        } else {
            let local_entity: Option<UnpublishAppError> = serde_json::from_str(&local_content).ok();
            let local_error = datadog::ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(datadog::Error::ResponseError(local_error))
        }
    }

    /// Update an existing app. This creates a new version of the app. This API requires a [registered application key](<https://docs.datadoghq.com/api/latest/action-connection/#register-a-new-app-key>). Alternatively, you can configure these permissions [in the UI](<https://docs.datadoghq.com/account_management/api-app-keys/#actions-api-access>).
    pub async fn update_app(
        &self,
        app_id: uuid::Uuid,
        body: crate::datadogV2::model::UpdateAppRequest,
    ) -> Result<crate::datadogV2::model::UpdateAppResponse, datadog::Error<UpdateAppError>> {
        match self.update_app_with_http_info(app_id, body).await {
            Ok(response_content) => {
                if let Some(e) = response_content.entity {
                    Ok(e)
                } else {
                    Err(datadog::Error::Serde(serde::de::Error::custom(
                        "response content was None",
                    )))
                }
            }
            Err(err) => Err(err),
        }
    }

    /// Update an existing app. This creates a new version of the app. This API requires a [registered application key](<https://docs.datadoghq.com/api/latest/action-connection/#register-a-new-app-key>). Alternatively, you can configure these permissions [in the UI](<https://docs.datadoghq.com/account_management/api-app-keys/#actions-api-access>).
    pub async fn update_app_with_http_info(
        &self,
        app_id: uuid::Uuid,
        body: crate::datadogV2::model::UpdateAppRequest,
    ) -> Result<
        datadog::ResponseContent<crate::datadogV2::model::UpdateAppResponse>,
        datadog::Error<UpdateAppError>,
    > {
        let local_configuration = &self.config;
        let operation_id = "v2.update_app";

        let local_client = &self.client;

        let local_uri_str = format!(
            "{}/api/v2/app-builder/apps/{app_id}",
            local_configuration.get_operation_host(operation_id),
            app_id = datadog::urlencode(app_id.to_string())
        );
        let mut local_req_builder =
            local_client.request(reqwest::Method::PATCH, local_uri_str.as_str());

        // build headers
        let mut headers = HeaderMap::new();
        headers.insert("Content-Type", HeaderValue::from_static("application/json"));
        headers.insert("Accept", HeaderValue::from_static("application/json"));

        // build user agent
        match HeaderValue::from_str(local_configuration.user_agent.as_str()) {
            Ok(user_agent) => headers.insert(reqwest::header::USER_AGENT, user_agent),
            Err(e) => {
                log::warn!("Failed to parse user agent header: {e}, falling back to default");
                headers.insert(
                    reqwest::header::USER_AGENT,
                    HeaderValue::from_static(datadog::DEFAULT_USER_AGENT.as_str()),
                )
            }
        };

        // build auth
        if let Some(local_key) = local_configuration.auth_keys.get("apiKeyAuth") {
            headers.insert(
                "DD-API-KEY",
                HeaderValue::from_str(local_key.key.as_str())
                    .expect("failed to parse DD-API-KEY header"),
            );
        };
        if let Some(local_key) = local_configuration.auth_keys.get("appKeyAuth") {
            headers.insert(
                "DD-APPLICATION-KEY",
                HeaderValue::from_str(local_key.key.as_str())
                    .expect("failed to parse DD-APPLICATION-KEY header"),
            );
        };

        // build body parameters
        let output = Vec::new();
        let mut ser = serde_json::Serializer::with_formatter(output, datadog::DDFormatter);
        if body.serialize(&mut ser).is_ok() {
            if let Some(content_encoding) = headers.get("Content-Encoding") {
                match content_encoding.to_str().unwrap_or_default() {
                    "gzip" => {
                        let mut enc = GzEncoder::new(Vec::new(), Compression::default());
                        let _ = enc.write_all(ser.into_inner().as_slice());
                        match enc.finish() {
                            Ok(buf) => {
                                local_req_builder = local_req_builder.body(buf);
                            }
                            Err(e) => return Err(datadog::Error::Io(e)),
                        }
                    }
                    "deflate" => {
                        let mut enc = ZlibEncoder::new(Vec::new(), Compression::default());
                        let _ = enc.write_all(ser.into_inner().as_slice());
                        match enc.finish() {
                            Ok(buf) => {
                                local_req_builder = local_req_builder.body(buf);
                            }
                            Err(e) => return Err(datadog::Error::Io(e)),
                        }
                    }
                    #[cfg(feature = "zstd")]
                    "zstd1" => {
                        let mut enc = zstd::stream::Encoder::new(Vec::new(), 0).unwrap();
                        let _ = enc.write_all(ser.into_inner().as_slice());
                        match enc.finish() {
                            Ok(buf) => {
                                local_req_builder = local_req_builder.body(buf);
                            }
                            Err(e) => return Err(datadog::Error::Io(e)),
                        }
                    }
                    _ => {
                        local_req_builder = local_req_builder.body(ser.into_inner());
                    }
                }
            } else {
                local_req_builder = local_req_builder.body(ser.into_inner());
            }
        }

        local_req_builder = local_req_builder.headers(headers);
        let local_req = local_req_builder.build()?;
        log::debug!("request content: {:?}", local_req.body());
        let local_resp = local_client.execute(local_req).await?;

        let local_status = local_resp.status();
        let local_content = local_resp.text().await?;
        log::debug!("response content: {}", local_content);

        if !local_status.is_client_error() && !local_status.is_server_error() {
            match serde_json::from_str::<crate::datadogV2::model::UpdateAppResponse>(&local_content)
            {
                Ok(e) => {
                    return Ok(datadog::ResponseContent {
                        status: local_status,
                        content: local_content,
                        entity: Some(e),
                    })
                }
                Err(e) => return Err(datadog::Error::Serde(e)),
            };
        } else {
            let local_entity: Option<UpdateAppError> = serde_json::from_str(&local_content).ok();
            let local_error = datadog::ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(datadog::Error::ResponseError(local_error))
        }
    }

    /// Add or remove an app from the current user's favorites. Favorited apps can be filtered for using the `filter[favorite]` query parameter on the [List Apps](<https://docs.datadoghq.com/api/latest/app-builder/#list-apps>) endpoint.
    pub async fn update_app_favorite(
        &self,
        app_id: uuid::Uuid,
        body: crate::datadogV2::model::UpdateAppFavoriteRequest,
    ) -> Result<(), datadog::Error<UpdateAppFavoriteError>> {
        match self.update_app_favorite_with_http_info(app_id, body).await {
            Ok(_) => Ok(()),
            Err(err) => Err(err),
        }
    }

    /// Add or remove an app from the current user's favorites. Favorited apps can be filtered for using the `filter[favorite]` query parameter on the [List Apps](<https://docs.datadoghq.com/api/latest/app-builder/#list-apps>) endpoint.
    pub async fn update_app_favorite_with_http_info(
        &self,
        app_id: uuid::Uuid,
        body: crate::datadogV2::model::UpdateAppFavoriteRequest,
    ) -> Result<datadog::ResponseContent<()>, datadog::Error<UpdateAppFavoriteError>> {
        let local_configuration = &self.config;
        let operation_id = "v2.update_app_favorite";

        let local_client = &self.client;

        let local_uri_str = format!(
            "{}/api/v2/app-builder/apps/{app_id}/favorite",
            local_configuration.get_operation_host(operation_id),
            app_id = datadog::urlencode(app_id.to_string())
        );
        let mut local_req_builder =
            local_client.request(reqwest::Method::PATCH, local_uri_str.as_str());

        // build headers
        let mut headers = HeaderMap::new();
        headers.insert("Content-Type", HeaderValue::from_static("application/json"));
        headers.insert("Accept", HeaderValue::from_static("*/*"));

        // build user agent
        match HeaderValue::from_str(local_configuration.user_agent.as_str()) {
            Ok(user_agent) => headers.insert(reqwest::header::USER_AGENT, user_agent),
            Err(e) => {
                log::warn!("Failed to parse user agent header: {e}, falling back to default");
                headers.insert(
                    reqwest::header::USER_AGENT,
                    HeaderValue::from_static(datadog::DEFAULT_USER_AGENT.as_str()),
                )
            }
        };

        // build auth
        if let Some(local_key) = local_configuration.auth_keys.get("apiKeyAuth") {
            headers.insert(
                "DD-API-KEY",
                HeaderValue::from_str(local_key.key.as_str())
                    .expect("failed to parse DD-API-KEY header"),
            );
        };
        if let Some(local_key) = local_configuration.auth_keys.get("appKeyAuth") {
            headers.insert(
                "DD-APPLICATION-KEY",
                HeaderValue::from_str(local_key.key.as_str())
                    .expect("failed to parse DD-APPLICATION-KEY header"),
            );
        };

        // build body parameters
        let output = Vec::new();
        let mut ser = serde_json::Serializer::with_formatter(output, datadog::DDFormatter);
        if body.serialize(&mut ser).is_ok() {
            if let Some(content_encoding) = headers.get("Content-Encoding") {
                match content_encoding.to_str().unwrap_or_default() {
                    "gzip" => {
                        let mut enc = GzEncoder::new(Vec::new(), Compression::default());
                        let _ = enc.write_all(ser.into_inner().as_slice());
                        match enc.finish() {
                            Ok(buf) => {
                                local_req_builder = local_req_builder.body(buf);
                            }
                            Err(e) => return Err(datadog::Error::Io(e)),
                        }
                    }
                    "deflate" => {
                        let mut enc = ZlibEncoder::new(Vec::new(), Compression::default());
                        let _ = enc.write_all(ser.into_inner().as_slice());
                        match enc.finish() {
                            Ok(buf) => {
                                local_req_builder = local_req_builder.body(buf);
                            }
                            Err(e) => return Err(datadog::Error::Io(e)),
                        }
                    }
                    #[cfg(feature = "zstd")]
                    "zstd1" => {
                        let mut enc = zstd::stream::Encoder::new(Vec::new(), 0).unwrap();
                        let _ = enc.write_all(ser.into_inner().as_slice());
                        match enc.finish() {
                            Ok(buf) => {
                                local_req_builder = local_req_builder.body(buf);
                            }
                            Err(e) => return Err(datadog::Error::Io(e)),
                        }
                    }
                    _ => {
                        local_req_builder = local_req_builder.body(ser.into_inner());
                    }
                }
            } else {
                local_req_builder = local_req_builder.body(ser.into_inner());
            }
        }

        local_req_builder = local_req_builder.headers(headers);
        let local_req = local_req_builder.build()?;
        log::debug!("request content: {:?}", local_req.body());
        let local_resp = local_client.execute(local_req).await?;

        let local_status = local_resp.status();
        let local_content = local_resp.text().await?;
        log::debug!("response content: {}", local_content);

        if !local_status.is_client_error() && !local_status.is_server_error() {
            Ok(datadog::ResponseContent {
                status: local_status,
                content: local_content,
                entity: None,
            })
        } else {
            let local_entity: Option<UpdateAppFavoriteError> =
                serde_json::from_str(&local_content).ok();
            let local_error = datadog::ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(datadog::Error::ResponseError(local_error))
        }
    }

    /// Enable or disable self-service for an app. Self-service apps can be discovered and run by users in your organization without explicit access being granted.
    pub async fn update_app_self_service(
        &self,
        app_id: uuid::Uuid,
        body: crate::datadogV2::model::UpdateAppSelfServiceRequest,
    ) -> Result<(), datadog::Error<UpdateAppSelfServiceError>> {
        match self
            .update_app_self_service_with_http_info(app_id, body)
            .await
        {
            Ok(_) => Ok(()),
            Err(err) => Err(err),
        }
    }

    /// Enable or disable self-service for an app. Self-service apps can be discovered and run by users in your organization without explicit access being granted.
    pub async fn update_app_self_service_with_http_info(
        &self,
        app_id: uuid::Uuid,
        body: crate::datadogV2::model::UpdateAppSelfServiceRequest,
    ) -> Result<datadog::ResponseContent<()>, datadog::Error<UpdateAppSelfServiceError>> {
        let local_configuration = &self.config;
        let operation_id = "v2.update_app_self_service";

        let local_client = &self.client;

        let local_uri_str = format!(
            "{}/api/v2/app-builder/apps/{app_id}/self-service",
            local_configuration.get_operation_host(operation_id),
            app_id = datadog::urlencode(app_id.to_string())
        );
        let mut local_req_builder =
            local_client.request(reqwest::Method::PATCH, local_uri_str.as_str());

        // build headers
        let mut headers = HeaderMap::new();
        headers.insert("Content-Type", HeaderValue::from_static("application/json"));
        headers.insert("Accept", HeaderValue::from_static("*/*"));

        // build user agent
        match HeaderValue::from_str(local_configuration.user_agent.as_str()) {
            Ok(user_agent) => headers.insert(reqwest::header::USER_AGENT, user_agent),
            Err(e) => {
                log::warn!("Failed to parse user agent header: {e}, falling back to default");
                headers.insert(
                    reqwest::header::USER_AGENT,
                    HeaderValue::from_static(datadog::DEFAULT_USER_AGENT.as_str()),
                )
            }
        };

        // build auth
        if let Some(local_key) = local_configuration.auth_keys.get("apiKeyAuth") {
            headers.insert(
                "DD-API-KEY",
                HeaderValue::from_str(local_key.key.as_str())
                    .expect("failed to parse DD-API-KEY header"),
            );
        };
        if let Some(local_key) = local_configuration.auth_keys.get("appKeyAuth") {
            headers.insert(
                "DD-APPLICATION-KEY",
                HeaderValue::from_str(local_key.key.as_str())
                    .expect("failed to parse DD-APPLICATION-KEY header"),
            );
        };

        // build body parameters
        let output = Vec::new();
        let mut ser = serde_json::Serializer::with_formatter(output, datadog::DDFormatter);
        if body.serialize(&mut ser).is_ok() {
            if let Some(content_encoding) = headers.get("Content-Encoding") {
                match content_encoding.to_str().unwrap_or_default() {
                    "gzip" => {
                        let mut enc = GzEncoder::new(Vec::new(), Compression::default());
                        let _ = enc.write_all(ser.into_inner().as_slice());
                        match enc.finish() {
                            Ok(buf) => {
                                local_req_builder = local_req_builder.body(buf);
                            }
                            Err(e) => return Err(datadog::Error::Io(e)),
                        }
                    }
                    "deflate" => {
                        let mut enc = ZlibEncoder::new(Vec::new(), Compression::default());
                        let _ = enc.write_all(ser.into_inner().as_slice());
                        match enc.finish() {
                            Ok(buf) => {
                                local_req_builder = local_req_builder.body(buf);
                            }
                            Err(e) => return Err(datadog::Error::Io(e)),
                        }
                    }
                    #[cfg(feature = "zstd")]
                    "zstd1" => {
                        let mut enc = zstd::stream::Encoder::new(Vec::new(), 0).unwrap();
                        let _ = enc.write_all(ser.into_inner().as_slice());
                        match enc.finish() {
                            Ok(buf) => {
                                local_req_builder = local_req_builder.body(buf);
                            }
                            Err(e) => return Err(datadog::Error::Io(e)),
                        }
                    }
                    _ => {
                        local_req_builder = local_req_builder.body(ser.into_inner());
                    }
                }
            } else {
                local_req_builder = local_req_builder.body(ser.into_inner());
            }
        }

        local_req_builder = local_req_builder.headers(headers);
        let local_req = local_req_builder.build()?;
        log::debug!("request content: {:?}", local_req.body());
        let local_resp = local_client.execute(local_req).await?;

        let local_status = local_resp.status();
        let local_content = local_resp.text().await?;
        log::debug!("response content: {}", local_content);

        if !local_status.is_client_error() && !local_status.is_server_error() {
            Ok(datadog::ResponseContent {
                status: local_status,
                content: local_content,
                entity: None,
            })
        } else {
            let local_entity: Option<UpdateAppSelfServiceError> =
                serde_json::from_str(&local_content).ok();
            let local_error = datadog::ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(datadog::Error::ResponseError(local_error))
        }
    }

    /// Replace the tags on an app. The provided list overwrites the existing tags entirely; tags not present in the request body are removed.
    pub async fn update_app_tags(
        &self,
        app_id: uuid::Uuid,
        body: crate::datadogV2::model::UpdateAppTagsRequest,
    ) -> Result<(), datadog::Error<UpdateAppTagsError>> {
        match self.update_app_tags_with_http_info(app_id, body).await {
            Ok(_) => Ok(()),
            Err(err) => Err(err),
        }
    }

    /// Replace the tags on an app. The provided list overwrites the existing tags entirely; tags not present in the request body are removed.
    pub async fn update_app_tags_with_http_info(
        &self,
        app_id: uuid::Uuid,
        body: crate::datadogV2::model::UpdateAppTagsRequest,
    ) -> Result<datadog::ResponseContent<()>, datadog::Error<UpdateAppTagsError>> {
        let local_configuration = &self.config;
        let operation_id = "v2.update_app_tags";

        let local_client = &self.client;

        let local_uri_str = format!(
            "{}/api/v2/app-builder/apps/{app_id}/tags",
            local_configuration.get_operation_host(operation_id),
            app_id = datadog::urlencode(app_id.to_string())
        );
        let mut local_req_builder =
            local_client.request(reqwest::Method::PATCH, local_uri_str.as_str());

        // build headers
        let mut headers = HeaderMap::new();
        headers.insert("Content-Type", HeaderValue::from_static("application/json"));
        headers.insert("Accept", HeaderValue::from_static("*/*"));

        // build user agent
        match HeaderValue::from_str(local_configuration.user_agent.as_str()) {
            Ok(user_agent) => headers.insert(reqwest::header::USER_AGENT, user_agent),
            Err(e) => {
                log::warn!("Failed to parse user agent header: {e}, falling back to default");
                headers.insert(
                    reqwest::header::USER_AGENT,
                    HeaderValue::from_static(datadog::DEFAULT_USER_AGENT.as_str()),
                )
            }
        };

        // build auth
        if let Some(local_key) = local_configuration.auth_keys.get("apiKeyAuth") {
            headers.insert(
                "DD-API-KEY",
                HeaderValue::from_str(local_key.key.as_str())
                    .expect("failed to parse DD-API-KEY header"),
            );
        };
        if let Some(local_key) = local_configuration.auth_keys.get("appKeyAuth") {
            headers.insert(
                "DD-APPLICATION-KEY",
                HeaderValue::from_str(local_key.key.as_str())
                    .expect("failed to parse DD-APPLICATION-KEY header"),
            );
        };

        // build body parameters
        let output = Vec::new();
        let mut ser = serde_json::Serializer::with_formatter(output, datadog::DDFormatter);
        if body.serialize(&mut ser).is_ok() {
            if let Some(content_encoding) = headers.get("Content-Encoding") {
                match content_encoding.to_str().unwrap_or_default() {
                    "gzip" => {
                        let mut enc = GzEncoder::new(Vec::new(), Compression::default());
                        let _ = enc.write_all(ser.into_inner().as_slice());
                        match enc.finish() {
                            Ok(buf) => {
                                local_req_builder = local_req_builder.body(buf);
                            }
                            Err(e) => return Err(datadog::Error::Io(e)),
                        }
                    }
                    "deflate" => {
                        let mut enc = ZlibEncoder::new(Vec::new(), Compression::default());
                        let _ = enc.write_all(ser.into_inner().as_slice());
                        match enc.finish() {
                            Ok(buf) => {
                                local_req_builder = local_req_builder.body(buf);
                            }
                            Err(e) => return Err(datadog::Error::Io(e)),
                        }
                    }
                    #[cfg(feature = "zstd")]
                    "zstd1" => {
                        let mut enc = zstd::stream::Encoder::new(Vec::new(), 0).unwrap();
                        let _ = enc.write_all(ser.into_inner().as_slice());
                        match enc.finish() {
                            Ok(buf) => {
                                local_req_builder = local_req_builder.body(buf);
                            }
                            Err(e) => return Err(datadog::Error::Io(e)),
                        }
                    }
                    _ => {
                        local_req_builder = local_req_builder.body(ser.into_inner());
                    }
                }
            } else {
                local_req_builder = local_req_builder.body(ser.into_inner());
            }
        }

        local_req_builder = local_req_builder.headers(headers);
        let local_req = local_req_builder.build()?;
        log::debug!("request content: {:?}", local_req.body());
        let local_resp = local_client.execute(local_req).await?;

        let local_status = local_resp.status();
        let local_content = local_resp.text().await?;
        log::debug!("response content: {}", local_content);

        if !local_status.is_client_error() && !local_status.is_server_error() {
            Ok(datadog::ResponseContent {
                status: local_status,
                content: local_content,
                entity: None,
            })
        } else {
            let local_entity: Option<UpdateAppTagsError> =
                serde_json::from_str(&local_content).ok();
            let local_error = datadog::ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(datadog::Error::ResponseError(local_error))
        }
    }

    /// Assign a human-readable name to a specific version of an app. The version is selected through the `version` query parameter.
    pub async fn update_app_version_name(
        &self,
        app_id: uuid::Uuid,
        version: String,
        body: crate::datadogV2::model::UpdateAppVersionNameRequest,
    ) -> Result<(), datadog::Error<UpdateAppVersionNameError>> {
        match self
            .update_app_version_name_with_http_info(app_id, version, body)
            .await
        {
            Ok(_) => Ok(()),
            Err(err) => Err(err),
        }
    }

    /// Assign a human-readable name to a specific version of an app. The version is selected through the `version` query parameter.
    pub async fn update_app_version_name_with_http_info(
        &self,
        app_id: uuid::Uuid,
        version: String,
        body: crate::datadogV2::model::UpdateAppVersionNameRequest,
    ) -> Result<datadog::ResponseContent<()>, datadog::Error<UpdateAppVersionNameError>> {
        let local_configuration = &self.config;
        let operation_id = "v2.update_app_version_name";

        let local_client = &self.client;

        let local_uri_str = format!(
            "{}/api/v2/app-builder/apps/{app_id}/version-name",
            local_configuration.get_operation_host(operation_id),
            app_id = datadog::urlencode(app_id.to_string())
        );
        let mut local_req_builder =
            local_client.request(reqwest::Method::PATCH, local_uri_str.as_str());

        local_req_builder = local_req_builder.query(&[("version", &version.to_string())]);

        // build headers
        let mut headers = HeaderMap::new();
        headers.insert("Content-Type", HeaderValue::from_static("application/json"));
        headers.insert("Accept", HeaderValue::from_static("*/*"));

        // build user agent
        match HeaderValue::from_str(local_configuration.user_agent.as_str()) {
            Ok(user_agent) => headers.insert(reqwest::header::USER_AGENT, user_agent),
            Err(e) => {
                log::warn!("Failed to parse user agent header: {e}, falling back to default");
                headers.insert(
                    reqwest::header::USER_AGENT,
                    HeaderValue::from_static(datadog::DEFAULT_USER_AGENT.as_str()),
                )
            }
        };

        // build auth
        if let Some(local_key) = local_configuration.auth_keys.get("apiKeyAuth") {
            headers.insert(
                "DD-API-KEY",
                HeaderValue::from_str(local_key.key.as_str())
                    .expect("failed to parse DD-API-KEY header"),
            );
        };
        if let Some(local_key) = local_configuration.auth_keys.get("appKeyAuth") {
            headers.insert(
                "DD-APPLICATION-KEY",
                HeaderValue::from_str(local_key.key.as_str())
                    .expect("failed to parse DD-APPLICATION-KEY header"),
            );
        };

        // build body parameters
        let output = Vec::new();
        let mut ser = serde_json::Serializer::with_formatter(output, datadog::DDFormatter);
        if body.serialize(&mut ser).is_ok() {
            if let Some(content_encoding) = headers.get("Content-Encoding") {
                match content_encoding.to_str().unwrap_or_default() {
                    "gzip" => {
                        let mut enc = GzEncoder::new(Vec::new(), Compression::default());
                        let _ = enc.write_all(ser.into_inner().as_slice());
                        match enc.finish() {
                            Ok(buf) => {
                                local_req_builder = local_req_builder.body(buf);
                            }
                            Err(e) => return Err(datadog::Error::Io(e)),
                        }
                    }
                    "deflate" => {
                        let mut enc = ZlibEncoder::new(Vec::new(), Compression::default());
                        let _ = enc.write_all(ser.into_inner().as_slice());
                        match enc.finish() {
                            Ok(buf) => {
                                local_req_builder = local_req_builder.body(buf);
                            }
                            Err(e) => return Err(datadog::Error::Io(e)),
                        }
                    }
                    #[cfg(feature = "zstd")]
                    "zstd1" => {
                        let mut enc = zstd::stream::Encoder::new(Vec::new(), 0).unwrap();
                        let _ = enc.write_all(ser.into_inner().as_slice());
                        match enc.finish() {
                            Ok(buf) => {
                                local_req_builder = local_req_builder.body(buf);
                            }
                            Err(e) => return Err(datadog::Error::Io(e)),
                        }
                    }
                    _ => {
                        local_req_builder = local_req_builder.body(ser.into_inner());
                    }
                }
            } else {
                local_req_builder = local_req_builder.body(ser.into_inner());
            }
        }

        local_req_builder = local_req_builder.headers(headers);
        let local_req = local_req_builder.build()?;
        log::debug!("request content: {:?}", local_req.body());
        let local_resp = local_client.execute(local_req).await?;

        let local_status = local_resp.status();
        let local_content = local_resp.text().await?;
        log::debug!("response content: {}", local_content);

        if !local_status.is_client_error() && !local_status.is_server_error() {
            Ok(datadog::ResponseContent {
                status: local_status,
                content: local_content,
                entity: None,
            })
        } else {
            let local_entity: Option<UpdateAppVersionNameError> =
                serde_json::from_str(&local_content).ok();
            let local_error = datadog::ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(datadog::Error::ResponseError(local_error))
        }
    }

    /// Update the publication protection level of an app. When set to `approval_required`, future publishes must go through an approval workflow before going live.
    pub async fn update_protection_level(
        &self,
        app_id: uuid::Uuid,
        body: crate::datadogV2::model::UpdateAppProtectionLevelRequest,
    ) -> Result<
        crate::datadogV2::model::UpdateAppResponse,
        datadog::Error<UpdateProtectionLevelError>,
    > {
        match self
            .update_protection_level_with_http_info(app_id, body)
            .await
        {
            Ok(response_content) => {
                if let Some(e) = response_content.entity {
                    Ok(e)
                } else {
                    Err(datadog::Error::Serde(serde::de::Error::custom(
                        "response content was None",
                    )))
                }
            }
            Err(err) => Err(err),
        }
    }

    /// Update the publication protection level of an app. When set to `approval_required`, future publishes must go through an approval workflow before going live.
    pub async fn update_protection_level_with_http_info(
        &self,
        app_id: uuid::Uuid,
        body: crate::datadogV2::model::UpdateAppProtectionLevelRequest,
    ) -> Result<
        datadog::ResponseContent<crate::datadogV2::model::UpdateAppResponse>,
        datadog::Error<UpdateProtectionLevelError>,
    > {
        let local_configuration = &self.config;
        let operation_id = "v2.update_protection_level";

        let local_client = &self.client;

        let local_uri_str = format!(
            "{}/api/v2/app-builder/apps/{app_id}/protection-level",
            local_configuration.get_operation_host(operation_id),
            app_id = datadog::urlencode(app_id.to_string())
        );
        let mut local_req_builder =
            local_client.request(reqwest::Method::PATCH, local_uri_str.as_str());

        // build headers
        let mut headers = HeaderMap::new();
        headers.insert("Content-Type", HeaderValue::from_static("application/json"));
        headers.insert("Accept", HeaderValue::from_static("application/json"));

        // build user agent
        match HeaderValue::from_str(local_configuration.user_agent.as_str()) {
            Ok(user_agent) => headers.insert(reqwest::header::USER_AGENT, user_agent),
            Err(e) => {
                log::warn!("Failed to parse user agent header: {e}, falling back to default");
                headers.insert(
                    reqwest::header::USER_AGENT,
                    HeaderValue::from_static(datadog::DEFAULT_USER_AGENT.as_str()),
                )
            }
        };

        // build auth
        if let Some(local_key) = local_configuration.auth_keys.get("apiKeyAuth") {
            headers.insert(
                "DD-API-KEY",
                HeaderValue::from_str(local_key.key.as_str())
                    .expect("failed to parse DD-API-KEY header"),
            );
        };
        if let Some(local_key) = local_configuration.auth_keys.get("appKeyAuth") {
            headers.insert(
                "DD-APPLICATION-KEY",
                HeaderValue::from_str(local_key.key.as_str())
                    .expect("failed to parse DD-APPLICATION-KEY header"),
            );
        };

        // build body parameters
        let output = Vec::new();
        let mut ser = serde_json::Serializer::with_formatter(output, datadog::DDFormatter);
        if body.serialize(&mut ser).is_ok() {
            if let Some(content_encoding) = headers.get("Content-Encoding") {
                match content_encoding.to_str().unwrap_or_default() {
                    "gzip" => {
                        let mut enc = GzEncoder::new(Vec::new(), Compression::default());
                        let _ = enc.write_all(ser.into_inner().as_slice());
                        match enc.finish() {
                            Ok(buf) => {
                                local_req_builder = local_req_builder.body(buf);
                            }
                            Err(e) => return Err(datadog::Error::Io(e)),
                        }
                    }
                    "deflate" => {
                        let mut enc = ZlibEncoder::new(Vec::new(), Compression::default());
                        let _ = enc.write_all(ser.into_inner().as_slice());
                        match enc.finish() {
                            Ok(buf) => {
                                local_req_builder = local_req_builder.body(buf);
                            }
                            Err(e) => return Err(datadog::Error::Io(e)),
                        }
                    }
                    #[cfg(feature = "zstd")]
                    "zstd1" => {
                        let mut enc = zstd::stream::Encoder::new(Vec::new(), 0).unwrap();
                        let _ = enc.write_all(ser.into_inner().as_slice());
                        match enc.finish() {
                            Ok(buf) => {
                                local_req_builder = local_req_builder.body(buf);
                            }
                            Err(e) => return Err(datadog::Error::Io(e)),
                        }
                    }
                    _ => {
                        local_req_builder = local_req_builder.body(ser.into_inner());
                    }
                }
            } else {
                local_req_builder = local_req_builder.body(ser.into_inner());
            }
        }

        local_req_builder = local_req_builder.headers(headers);
        let local_req = local_req_builder.build()?;
        log::debug!("request content: {:?}", local_req.body());
        let local_resp = local_client.execute(local_req).await?;

        let local_status = local_resp.status();
        let local_content = local_resp.text().await?;
        log::debug!("response content: {}", local_content);

        if !local_status.is_client_error() && !local_status.is_server_error() {
            match serde_json::from_str::<crate::datadogV2::model::UpdateAppResponse>(&local_content)
            {
                Ok(e) => {
                    return Ok(datadog::ResponseContent {
                        status: local_status,
                        content: local_content,
                        entity: Some(e),
                    })
                }
                Err(e) => return Err(datadog::Error::Serde(e)),
            };
        } else {
            let local_entity: Option<UpdateProtectionLevelError> =
                serde_json::from_str(&local_content).ok();
            let local_error = datadog::ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(datadog::Error::ResponseError(local_error))
        }
    }
}
