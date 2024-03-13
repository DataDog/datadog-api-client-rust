// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use crate::datadog::*;
use async_stream::try_stream;
use futures_core::stream::Stream;
use reqwest;
use serde::{Deserialize, Serialize};

/// DeleteSLOOptionalParams is a struct for passing parameters to the method [`ServiceLevelObjectivesAPI::delete_slo`]
#[non_exhaustive]
#[derive(Clone, Default, Debug)]
pub struct DeleteSLOOptionalParams {
    /// Delete the monitor even if it's referenced by other resources (for example SLO, composite monitor).
    pub force: Option<String>,
}

impl DeleteSLOOptionalParams {
    /// Delete the monitor even if it's referenced by other resources (for example SLO, composite monitor).
    pub fn force(mut self, value: String) -> Self {
        self.force = Some(value);
        self
    }
}

/// GetSLOOptionalParams is a struct for passing parameters to the method [`ServiceLevelObjectivesAPI::get_slo`]
#[non_exhaustive]
#[derive(Clone, Default, Debug)]
pub struct GetSLOOptionalParams {
    /// Get the IDs of SLO monitors that reference this SLO.
    pub with_configured_alert_ids: Option<bool>,
}

impl GetSLOOptionalParams {
    /// Get the IDs of SLO monitors that reference this SLO.
    pub fn with_configured_alert_ids(mut self, value: bool) -> Self {
        self.with_configured_alert_ids = Some(value);
        self
    }
}

/// GetSLOHistoryOptionalParams is a struct for passing parameters to the method [`ServiceLevelObjectivesAPI::get_slo_history`]
#[non_exhaustive]
#[derive(Clone, Default, Debug)]
pub struct GetSLOHistoryOptionalParams {
    /// The SLO target. If `target` is passed in, the response will include the remaining error budget and a timeframe value of `custom`.
    pub target: Option<f64>,
    /// Defaults to `true`. If any SLO corrections are applied and this parameter is set to `false`,
    /// then the corrections will not be applied and the SLI values will not be affected.
    pub apply_correction: Option<bool>,
}

impl GetSLOHistoryOptionalParams {
    /// The SLO target. If `target` is passed in, the response will include the remaining error budget and a timeframe value of `custom`.
    pub fn target(mut self, value: f64) -> Self {
        self.target = Some(value);
        self
    }
    /// Defaults to `true`. If any SLO corrections are applied and this parameter is set to `false`,
    /// then the corrections will not be applied and the SLI values will not be affected.
    pub fn apply_correction(mut self, value: bool) -> Self {
        self.apply_correction = Some(value);
        self
    }
}

/// ListSLOsOptionalParams is a struct for passing parameters to the method [`ServiceLevelObjectivesAPI::list_slos`]
#[non_exhaustive]
#[derive(Clone, Default, Debug)]
pub struct ListSLOsOptionalParams {
    /// A comma separated list of the IDs of the service level objectives objects.
    pub ids: Option<String>,
    /// The query string to filter results based on SLO names.
    pub query: Option<String>,
    /// The query string to filter results based on a single SLO tag.
    pub tags_query: Option<String>,
    /// The query string to filter results based on SLO numerator and denominator.
    pub metrics_query: Option<String>,
    /// The number of SLOs to return in the response.
    pub limit: Option<i64>,
    /// The specific offset to use as the beginning of the returned response.
    pub offset: Option<i64>,
}

impl ListSLOsOptionalParams {
    /// A comma separated list of the IDs of the service level objectives objects.
    pub fn ids(mut self, value: String) -> Self {
        self.ids = Some(value);
        self
    }
    /// The query string to filter results based on SLO names.
    pub fn query(mut self, value: String) -> Self {
        self.query = Some(value);
        self
    }
    /// The query string to filter results based on a single SLO tag.
    pub fn tags_query(mut self, value: String) -> Self {
        self.tags_query = Some(value);
        self
    }
    /// The query string to filter results based on SLO numerator and denominator.
    pub fn metrics_query(mut self, value: String) -> Self {
        self.metrics_query = Some(value);
        self
    }
    /// The number of SLOs to return in the response.
    pub fn limit(mut self, value: i64) -> Self {
        self.limit = Some(value);
        self
    }
    /// The specific offset to use as the beginning of the returned response.
    pub fn offset(mut self, value: i64) -> Self {
        self.offset = Some(value);
        self
    }
}

/// SearchSLOOptionalParams is a struct for passing parameters to the method [`ServiceLevelObjectivesAPI::search_slo`]
#[non_exhaustive]
#[derive(Clone, Default, Debug)]
pub struct SearchSLOOptionalParams {
    /// The query string to filter results based on SLO names.
    /// Some examples of queries include `service:<service-name>`
    /// and `<slo-name>`.
    pub query: Option<String>,
    /// The number of files to return in the response `[default=10]`.
    pub page_size: Option<i64>,
    /// The identifier of the first page to return. This parameter is used for the pagination feature `[default=0]`.
    pub page_number: Option<i64>,
    /// Whether or not to return facet information in the response `[default=false]`.
    pub include_facets: Option<bool>,
}

impl SearchSLOOptionalParams {
    /// The query string to filter results based on SLO names.
    /// Some examples of queries include `service:<service-name>`
    /// and `<slo-name>`.
    pub fn query(mut self, value: String) -> Self {
        self.query = Some(value);
        self
    }
    /// The number of files to return in the response `[default=10]`.
    pub fn page_size(mut self, value: i64) -> Self {
        self.page_size = Some(value);
        self
    }
    /// The identifier of the first page to return. This parameter is used for the pagination feature `[default=0]`.
    pub fn page_number(mut self, value: i64) -> Self {
        self.page_number = Some(value);
        self
    }
    /// Whether or not to return facet information in the response `[default=false]`.
    pub fn include_facets(mut self, value: bool) -> Self {
        self.include_facets = Some(value);
        self
    }
}

/// CheckCanDeleteSLOError is a struct for typed errors of method [`ServiceLevelObjectivesAPI::check_can_delete_slo`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CheckCanDeleteSLOError {
    Status400(Option<crate::datadogV1::model::APIErrorResponse>),
    Status403(Option<crate::datadogV1::model::APIErrorResponse>),
    Status429(Option<crate::datadogV1::model::APIErrorResponse>),
    Status409(Option<crate::datadogV1::model::CheckCanDeleteSLOResponse>),
    UnknownValue(serde_json::Value),
}

/// CreateSLOError is a struct for typed errors of method [`ServiceLevelObjectivesAPI::create_slo`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CreateSLOError {
    Status400(Option<crate::datadogV1::model::APIErrorResponse>),
    Status403(Option<crate::datadogV1::model::APIErrorResponse>),
    Status429(Option<crate::datadogV1::model::APIErrorResponse>),
    UnknownValue(serde_json::Value),
}

/// DeleteSLOError is a struct for typed errors of method [`ServiceLevelObjectivesAPI::delete_slo`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DeleteSLOError {
    Status403(Option<crate::datadogV1::model::APIErrorResponse>),
    Status404(Option<crate::datadogV1::model::APIErrorResponse>),
    Status429(Option<crate::datadogV1::model::APIErrorResponse>),
    Status409(Option<crate::datadogV1::model::SLODeleteResponse>),
    UnknownValue(serde_json::Value),
}

/// DeleteSLOTimeframeInBulkError is a struct for typed errors of method [`ServiceLevelObjectivesAPI::delete_slo_timeframe_in_bulk`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DeleteSLOTimeframeInBulkError {
    Status400(Option<crate::datadogV1::model::APIErrorResponse>),
    Status403(Option<crate::datadogV1::model::APIErrorResponse>),
    Status429(Option<crate::datadogV1::model::APIErrorResponse>),
    UnknownValue(serde_json::Value),
}

/// GetSLOError is a struct for typed errors of method [`ServiceLevelObjectivesAPI::get_slo`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetSLOError {
    Status403(Option<crate::datadogV1::model::APIErrorResponse>),
    Status404(Option<crate::datadogV1::model::APIErrorResponse>),
    Status429(Option<crate::datadogV1::model::APIErrorResponse>),
    UnknownValue(serde_json::Value),
}

/// GetSLOCorrectionsError is a struct for typed errors of method [`ServiceLevelObjectivesAPI::get_slo_corrections`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetSLOCorrectionsError {
    Status400(Option<crate::datadogV1::model::APIErrorResponse>),
    Status403(Option<crate::datadogV1::model::APIErrorResponse>),
    Status404(Option<crate::datadogV1::model::APIErrorResponse>),
    Status429(Option<crate::datadogV1::model::APIErrorResponse>),
    UnknownValue(serde_json::Value),
}

/// GetSLOHistoryError is a struct for typed errors of method [`ServiceLevelObjectivesAPI::get_slo_history`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetSLOHistoryError {
    Status400(Option<crate::datadogV1::model::APIErrorResponse>),
    Status403(Option<crate::datadogV1::model::APIErrorResponse>),
    Status404(Option<crate::datadogV1::model::APIErrorResponse>),
    Status429(Option<crate::datadogV1::model::APIErrorResponse>),
    UnknownValue(serde_json::Value),
}

/// ListSLOsError is a struct for typed errors of method [`ServiceLevelObjectivesAPI::list_slos`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ListSLOsError {
    Status400(Option<crate::datadogV1::model::APIErrorResponse>),
    Status403(Option<crate::datadogV1::model::APIErrorResponse>),
    Status404(Option<crate::datadogV1::model::APIErrorResponse>),
    Status429(Option<crate::datadogV1::model::APIErrorResponse>),
    UnknownValue(serde_json::Value),
}

/// SearchSLOError is a struct for typed errors of method [`ServiceLevelObjectivesAPI::search_slo`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum SearchSLOError {
    Status400(Option<crate::datadogV1::model::APIErrorResponse>),
    Status403(Option<crate::datadogV1::model::APIErrorResponse>),
    Status429(Option<crate::datadogV1::model::APIErrorResponse>),
    UnknownValue(serde_json::Value),
}

/// UpdateSLOError is a struct for typed errors of method [`ServiceLevelObjectivesAPI::update_slo`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum UpdateSLOError {
    Status400(Option<crate::datadogV1::model::APIErrorResponse>),
    Status403(Option<crate::datadogV1::model::APIErrorResponse>),
    Status404(Option<crate::datadogV1::model::APIErrorResponse>),
    Status429(Option<crate::datadogV1::model::APIErrorResponse>),
    UnknownValue(serde_json::Value),
}

#[derive(Debug, Clone)]
pub struct ServiceLevelObjectivesAPI {
    config: configuration::Configuration,
    client: reqwest_middleware::ClientWithMiddleware,
}

impl Default for ServiceLevelObjectivesAPI {
    fn default() -> Self {
        Self {
            config: configuration::Configuration::new(),
            client: reqwest_middleware::ClientBuilder::new(reqwest::Client::new()).build(),
        }
    }
}

impl ServiceLevelObjectivesAPI {
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

    /// Check if an SLO can be safely deleted. For example,
    /// assure an SLO can be deleted without disrupting a dashboard.
    pub async fn check_can_delete_slo(
        &self,
        ids: String,
    ) -> Result<crate::datadogV1::model::CheckCanDeleteSLOResponse, Error<CheckCanDeleteSLOError>>
    {
        match self.check_can_delete_slo_with_http_info(ids).await {
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

    /// Check if an SLO can be safely deleted. For example,
    /// assure an SLO can be deleted without disrupting a dashboard.
    pub async fn check_can_delete_slo_with_http_info(
        &self,
        ids: String,
    ) -> Result<
        ResponseContent<crate::datadogV1::model::CheckCanDeleteSLOResponse>,
        Error<CheckCanDeleteSLOError>,
    > {
        let local_configuration = &self.config;
        let operation_id = "v1.check_can_delete_slo";

        let local_client = &self.client;

        let local_uri_str = format!(
            "{}/api/v1/slo/can_delete",
            local_configuration.get_operation_host(operation_id)
        );
        let mut local_req_builder =
            local_client.request(reqwest::Method::GET, local_uri_str.as_str());

        local_req_builder = local_req_builder.query(&[("ids", &ids.to_string())]);

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
            match serde_json::from_str::<crate::datadogV1::model::CheckCanDeleteSLOResponse>(
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
            let local_entity: Option<CheckCanDeleteSLOError> =
                serde_json::from_str(&local_content).ok();
            let local_error = ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(Error::ResponseError(local_error))
        }
    }

    /// Create a service level objective object.
    pub async fn create_slo(
        &self,
        body: crate::datadogV1::model::ServiceLevelObjectiveRequest,
    ) -> Result<crate::datadogV1::model::SLOListResponse, Error<CreateSLOError>> {
        match self.create_slo_with_http_info(body).await {
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

    /// Create a service level objective object.
    pub async fn create_slo_with_http_info(
        &self,
        body: crate::datadogV1::model::ServiceLevelObjectiveRequest,
    ) -> Result<ResponseContent<crate::datadogV1::model::SLOListResponse>, Error<CreateSLOError>>
    {
        let local_configuration = &self.config;
        let operation_id = "v1.create_slo";

        let local_client = &self.client;

        let local_uri_str = format!(
            "{}/api/v1/slo",
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
            match serde_json::from_str::<crate::datadogV1::model::SLOListResponse>(&local_content) {
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
            let local_entity: Option<CreateSLOError> = serde_json::from_str(&local_content).ok();
            let local_error = ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(Error::ResponseError(local_error))
        }
    }

    /// Permanently delete the specified service level objective object.
    ///
    /// If an SLO is used in a dashboard, the `DELETE /v1/slo/` endpoint returns
    /// a 409 conflict error because the SLO is referenced in a dashboard.
    pub async fn delete_slo(
        &self,
        slo_id: String,
        params: DeleteSLOOptionalParams,
    ) -> Result<crate::datadogV1::model::SLODeleteResponse, Error<DeleteSLOError>> {
        match self.delete_slo_with_http_info(slo_id, params).await {
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

    /// Permanently delete the specified service level objective object.
    ///
    /// If an SLO is used in a dashboard, the `DELETE /v1/slo/` endpoint returns
    /// a 409 conflict error because the SLO is referenced in a dashboard.
    pub async fn delete_slo_with_http_info(
        &self,
        slo_id: String,
        params: DeleteSLOOptionalParams,
    ) -> Result<ResponseContent<crate::datadogV1::model::SLODeleteResponse>, Error<DeleteSLOError>>
    {
        let local_configuration = &self.config;
        let operation_id = "v1.delete_slo";

        // unbox and build optional parameters
        let force = params.force;

        let local_client = &self.client;

        let local_uri_str = format!(
            "{}/api/v1/slo/{slo_id}",
            local_configuration.get_operation_host(operation_id),
            slo_id = urlencode(slo_id)
        );
        let mut local_req_builder =
            local_client.request(reqwest::Method::DELETE, local_uri_str.as_str());

        if let Some(ref local_query_param) = force {
            local_req_builder =
                local_req_builder.query(&[("force", &local_query_param.to_string())]);
        };

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
            match serde_json::from_str::<crate::datadogV1::model::SLODeleteResponse>(&local_content)
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
            let local_entity: Option<DeleteSLOError> = serde_json::from_str(&local_content).ok();
            let local_error = ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(Error::ResponseError(local_error))
        }
    }

    /// Delete (or partially delete) multiple service level objective objects.
    ///
    /// This endpoint facilitates deletion of one or more thresholds for one or more
    /// service level objective objects. If all thresholds are deleted, the service level
    /// objective object is deleted as well.
    pub async fn delete_slo_timeframe_in_bulk(
        &self,
        body: std::collections::BTreeMap<String, Vec<crate::datadogV1::model::SLOTimeframe>>,
    ) -> Result<crate::datadogV1::model::SLOBulkDeleteResponse, Error<DeleteSLOTimeframeInBulkError>>
    {
        match self.delete_slo_timeframe_in_bulk_with_http_info(body).await {
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

    /// Delete (or partially delete) multiple service level objective objects.
    ///
    /// This endpoint facilitates deletion of one or more thresholds for one or more
    /// service level objective objects. If all thresholds are deleted, the service level
    /// objective object is deleted as well.
    pub async fn delete_slo_timeframe_in_bulk_with_http_info(
        &self,
        body: std::collections::BTreeMap<String, Vec<crate::datadogV1::model::SLOTimeframe>>,
    ) -> Result<
        ResponseContent<crate::datadogV1::model::SLOBulkDeleteResponse>,
        Error<DeleteSLOTimeframeInBulkError>,
    > {
        let local_configuration = &self.config;
        let operation_id = "v1.delete_slo_timeframe_in_bulk";

        let local_client = &self.client;

        let local_uri_str = format!(
            "{}/api/v1/slo/bulk_delete",
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
            match serde_json::from_str::<crate::datadogV1::model::SLOBulkDeleteResponse>(
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
            let local_entity: Option<DeleteSLOTimeframeInBulkError> =
                serde_json::from_str(&local_content).ok();
            let local_error = ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(Error::ResponseError(local_error))
        }
    }

    /// Get a service level objective object.
    pub async fn get_slo(
        &self,
        slo_id: String,
        params: GetSLOOptionalParams,
    ) -> Result<crate::datadogV1::model::SLOResponse, Error<GetSLOError>> {
        match self.get_slo_with_http_info(slo_id, params).await {
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

    /// Get a service level objective object.
    pub async fn get_slo_with_http_info(
        &self,
        slo_id: String,
        params: GetSLOOptionalParams,
    ) -> Result<ResponseContent<crate::datadogV1::model::SLOResponse>, Error<GetSLOError>> {
        let local_configuration = &self.config;
        let operation_id = "v1.get_slo";

        // unbox and build optional parameters
        let with_configured_alert_ids = params.with_configured_alert_ids;

        let local_client = &self.client;

        let local_uri_str = format!(
            "{}/api/v1/slo/{slo_id}",
            local_configuration.get_operation_host(operation_id),
            slo_id = urlencode(slo_id)
        );
        let mut local_req_builder =
            local_client.request(reqwest::Method::GET, local_uri_str.as_str());

        if let Some(ref local_query_param) = with_configured_alert_ids {
            local_req_builder = local_req_builder
                .query(&[("with_configured_alert_ids", &local_query_param.to_string())]);
        };

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
            match serde_json::from_str::<crate::datadogV1::model::SLOResponse>(&local_content) {
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
            let local_entity: Option<GetSLOError> = serde_json::from_str(&local_content).ok();
            let local_error = ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(Error::ResponseError(local_error))
        }
    }

    /// Get corrections applied to an SLO
    pub async fn get_slo_corrections(
        &self,
        slo_id: String,
    ) -> Result<crate::datadogV1::model::SLOCorrectionListResponse, Error<GetSLOCorrectionsError>>
    {
        match self.get_slo_corrections_with_http_info(slo_id).await {
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

    /// Get corrections applied to an SLO
    pub async fn get_slo_corrections_with_http_info(
        &self,
        slo_id: String,
    ) -> Result<
        ResponseContent<crate::datadogV1::model::SLOCorrectionListResponse>,
        Error<GetSLOCorrectionsError>,
    > {
        let local_configuration = &self.config;
        let operation_id = "v1.get_slo_corrections";

        let local_client = &self.client;

        let local_uri_str = format!(
            "{}/api/v1/slo/{slo_id}/corrections",
            local_configuration.get_operation_host(operation_id),
            slo_id = urlencode(slo_id)
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
            match serde_json::from_str::<crate::datadogV1::model::SLOCorrectionListResponse>(
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
            let local_entity: Option<GetSLOCorrectionsError> =
                serde_json::from_str(&local_content).ok();
            let local_error = ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(Error::ResponseError(local_error))
        }
    }

    /// Get a specific SLO’s history, regardless of its SLO type.
    ///
    /// The detailed history data is structured according to the source data type.
    /// For example, metric data is included for event SLOs that use
    /// the metric source, and monitor SLO types include the monitor transition history.
    ///
    /// **Note:** There are different response formats for event based and time based SLOs.
    /// Examples of both are shown.
    pub async fn get_slo_history(
        &self,
        slo_id: String,
        from_ts: i64,
        to_ts: i64,
        params: GetSLOHistoryOptionalParams,
    ) -> Result<crate::datadogV1::model::SLOHistoryResponse, Error<GetSLOHistoryError>> {
        match self
            .get_slo_history_with_http_info(slo_id, from_ts, to_ts, params)
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

    /// Get a specific SLO’s history, regardless of its SLO type.
    ///
    /// The detailed history data is structured according to the source data type.
    /// For example, metric data is included for event SLOs that use
    /// the metric source, and monitor SLO types include the monitor transition history.
    ///
    /// **Note:** There are different response formats for event based and time based SLOs.
    /// Examples of both are shown.
    pub async fn get_slo_history_with_http_info(
        &self,
        slo_id: String,
        from_ts: i64,
        to_ts: i64,
        params: GetSLOHistoryOptionalParams,
    ) -> Result<
        ResponseContent<crate::datadogV1::model::SLOHistoryResponse>,
        Error<GetSLOHistoryError>,
    > {
        let local_configuration = &self.config;
        let operation_id = "v1.get_slo_history";

        // unbox and build optional parameters
        let target = params.target;
        let apply_correction = params.apply_correction;

        let local_client = &self.client;

        let local_uri_str = format!(
            "{}/api/v1/slo/{slo_id}/history",
            local_configuration.get_operation_host(operation_id),
            slo_id = urlencode(slo_id)
        );
        let mut local_req_builder =
            local_client.request(reqwest::Method::GET, local_uri_str.as_str());

        local_req_builder = local_req_builder.query(&[("from_ts", &from_ts.to_string())]);
        local_req_builder = local_req_builder.query(&[("to_ts", &to_ts.to_string())]);
        if let Some(ref local_query_param) = target {
            local_req_builder =
                local_req_builder.query(&[("target", &local_query_param.to_string())]);
        };
        if let Some(ref local_query_param) = apply_correction {
            local_req_builder =
                local_req_builder.query(&[("apply_correction", &local_query_param.to_string())]);
        };

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
            match serde_json::from_str::<crate::datadogV1::model::SLOHistoryResponse>(
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
            let local_entity: Option<GetSLOHistoryError> =
                serde_json::from_str(&local_content).ok();
            let local_error = ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(Error::ResponseError(local_error))
        }
    }

    /// Get a list of service level objective objects for your organization.
    pub async fn list_slos(
        &self,
        params: ListSLOsOptionalParams,
    ) -> Result<crate::datadogV1::model::SLOListResponse, Error<ListSLOsError>> {
        match self.list_slos_with_http_info(params).await {
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

    pub fn list_slos_with_pagination(
        &self,
        mut params: ListSLOsOptionalParams,
    ) -> impl Stream<
        Item = Result<crate::datadogV1::model::ServiceLevelObjective, Error<ListSLOsError>>,
    > + '_ {
        try_stream! {
            let mut page_size: i64 = 1000;
            if params.limit.is_none() {
                params.limit = Some(page_size);
            } else {
                page_size = params.limit.unwrap().clone();
            }
            loop {
                let resp = self.list_slos(params.clone()).await?;
                let Some(data) = resp.data else { break };

                let r = data;
                let count = r.len();
                for team in r {
                    yield team;
                }

                if count < page_size as usize {
                    break;
                }
                if params.offset.is_none() {
                    params.offset = Some(page_size.clone());
                } else {
                    params.offset = Some(params.offset.unwrap() + page_size.clone());
                }
            }
        }
    }

    /// Get a list of service level objective objects for your organization.
    pub async fn list_slos_with_http_info(
        &self,
        params: ListSLOsOptionalParams,
    ) -> Result<ResponseContent<crate::datadogV1::model::SLOListResponse>, Error<ListSLOsError>>
    {
        let local_configuration = &self.config;
        let operation_id = "v1.list_slos";

        // unbox and build optional parameters
        let ids = params.ids;
        let query = params.query;
        let tags_query = params.tags_query;
        let metrics_query = params.metrics_query;
        let limit = params.limit;
        let offset = params.offset;

        let local_client = &self.client;

        let local_uri_str = format!(
            "{}/api/v1/slo",
            local_configuration.get_operation_host(operation_id)
        );
        let mut local_req_builder =
            local_client.request(reqwest::Method::GET, local_uri_str.as_str());

        if let Some(ref local_query_param) = ids {
            local_req_builder = local_req_builder.query(&[("ids", &local_query_param.to_string())]);
        };
        if let Some(ref local_query_param) = query {
            local_req_builder =
                local_req_builder.query(&[("query", &local_query_param.to_string())]);
        };
        if let Some(ref local_query_param) = tags_query {
            local_req_builder =
                local_req_builder.query(&[("tags_query", &local_query_param.to_string())]);
        };
        if let Some(ref local_query_param) = metrics_query {
            local_req_builder =
                local_req_builder.query(&[("metrics_query", &local_query_param.to_string())]);
        };
        if let Some(ref local_query_param) = limit {
            local_req_builder =
                local_req_builder.query(&[("limit", &local_query_param.to_string())]);
        };
        if let Some(ref local_query_param) = offset {
            local_req_builder =
                local_req_builder.query(&[("offset", &local_query_param.to_string())]);
        };

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
            match serde_json::from_str::<crate::datadogV1::model::SLOListResponse>(&local_content) {
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
            let local_entity: Option<ListSLOsError> = serde_json::from_str(&local_content).ok();
            let local_error = ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(Error::ResponseError(local_error))
        }
    }

    /// Get a list of service level objective objects for your organization.
    pub async fn search_slo(
        &self,
        params: SearchSLOOptionalParams,
    ) -> Result<crate::datadogV1::model::SearchSLOResponse, Error<SearchSLOError>> {
        match self.search_slo_with_http_info(params).await {
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

    /// Get a list of service level objective objects for your organization.
    pub async fn search_slo_with_http_info(
        &self,
        params: SearchSLOOptionalParams,
    ) -> Result<ResponseContent<crate::datadogV1::model::SearchSLOResponse>, Error<SearchSLOError>>
    {
        let local_configuration = &self.config;
        let operation_id = "v1.search_slo";

        // unbox and build optional parameters
        let query = params.query;
        let page_size = params.page_size;
        let page_number = params.page_number;
        let include_facets = params.include_facets;

        let local_client = &self.client;

        let local_uri_str = format!(
            "{}/api/v1/slo/search",
            local_configuration.get_operation_host(operation_id)
        );
        let mut local_req_builder =
            local_client.request(reqwest::Method::GET, local_uri_str.as_str());

        if let Some(ref local_query_param) = query {
            local_req_builder =
                local_req_builder.query(&[("query", &local_query_param.to_string())]);
        };
        if let Some(ref local_query_param) = page_size {
            local_req_builder =
                local_req_builder.query(&[("page[size]", &local_query_param.to_string())]);
        };
        if let Some(ref local_query_param) = page_number {
            local_req_builder =
                local_req_builder.query(&[("page[number]", &local_query_param.to_string())]);
        };
        if let Some(ref local_query_param) = include_facets {
            local_req_builder =
                local_req_builder.query(&[("include_facets", &local_query_param.to_string())]);
        };

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
            match serde_json::from_str::<crate::datadogV1::model::SearchSLOResponse>(&local_content)
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
            let local_entity: Option<SearchSLOError> = serde_json::from_str(&local_content).ok();
            let local_error = ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(Error::ResponseError(local_error))
        }
    }

    /// Update the specified service level objective object.
    pub async fn update_slo(
        &self,
        slo_id: String,
        body: crate::datadogV1::model::ServiceLevelObjective,
    ) -> Result<crate::datadogV1::model::SLOListResponse, Error<UpdateSLOError>> {
        match self.update_slo_with_http_info(slo_id, body).await {
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

    /// Update the specified service level objective object.
    pub async fn update_slo_with_http_info(
        &self,
        slo_id: String,
        body: crate::datadogV1::model::ServiceLevelObjective,
    ) -> Result<ResponseContent<crate::datadogV1::model::SLOListResponse>, Error<UpdateSLOError>>
    {
        let local_configuration = &self.config;
        let operation_id = "v1.update_slo";

        let local_client = &self.client;

        let local_uri_str = format!(
            "{}/api/v1/slo/{slo_id}",
            local_configuration.get_operation_host(operation_id),
            slo_id = urlencode(slo_id)
        );
        let mut local_req_builder =
            local_client.request(reqwest::Method::PUT, local_uri_str.as_str());

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
            match serde_json::from_str::<crate::datadogV1::model::SLOListResponse>(&local_content) {
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
            let local_entity: Option<UpdateSLOError> = serde_json::from_str(&local_content).ok();
            let local_error = ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(Error::ResponseError(local_error))
        }
    }
}
