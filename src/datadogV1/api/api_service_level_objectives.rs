// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

use crate::datadog::*;
use reqwest;
use serde::{Deserialize, Serialize};

/// CheckCanDeleteSLOParams is a struct for passing parameters to the method [`CheckCanDeleteSLO`]
#[derive(Clone, Debug, Default)]
pub struct CheckCanDeleteSLOParams {
    /// A comma separated list of the IDs of the service level objectives objects.
    pub ids: String,
}

/// CreateSLOParams is a struct for passing parameters to the method [`CreateSLO`]
#[derive(Clone, Debug, Default)]
pub struct CreateSLOParams {
    /// Service level objective request object.
    pub body: crate::datadogV1::model::ServiceLevelObjectiveRequest,
}

/// DeleteSLOParams is a struct for passing parameters to the method [`DeleteSLO`]
#[derive(Clone, Debug, Default)]
pub struct DeleteSLOParams {
    /// The ID of the service level objective.
    pub slo_id: String,
    /// Delete the monitor even if it's referenced by other resources (for example SLO, composite monitor).
    pub force: Option<String>,
}

/// DeleteSLOTimeframeInBulkParams is a struct for passing parameters to the method [`DeleteSLOTimeframeInBulk`]
#[derive(Clone, Debug, Default)]
pub struct DeleteSLOTimeframeInBulkParams {
    /// Delete multiple service level objective objects request body.
    pub body: std::collections::HashMap<String, Vec<crate::datadogV1::model::SLOTimeframe>>,
}

/// GetSLOParams is a struct for passing parameters to the method [`GetSLO`]
#[derive(Clone, Debug, Default)]
pub struct GetSLOParams {
    /// The ID of the service level objective object.
    pub slo_id: String,
    /// Get the IDs of SLO monitors that reference this SLO.
    pub with_configured_alert_ids: Option<bool>,
}

/// GetSLOCorrectionsParams is a struct for passing parameters to the method [`GetSLOCorrections`]
#[derive(Clone, Debug, Default)]
pub struct GetSLOCorrectionsParams {
    /// The ID of the service level objective object.
    pub slo_id: String,
}

/// GetSLOHistoryParams is a struct for passing parameters to the method [`GetSLOHistory`]
#[derive(Clone, Debug, Default)]
pub struct GetSLOHistoryParams {
    /// The ID of the service level objective object.
    pub slo_id: String,
    /// The `from` timestamp for the query window in epoch seconds.
    pub from_ts: i64,
    /// The `to` timestamp for the query window in epoch seconds.
    pub to_ts: i64,
    /// The SLO target. If `target` is passed in, the response will include the remaining error budget and a timeframe value of `custom`.
    pub target: Option<f64>,
    /// Defaults to `true`. If any SLO corrections are applied and this parameter is set to `false`,
    /// then the corrections will not be applied and the SLI values will not be affected.
    pub apply_correction: Option<bool>,
}

/// ListSLOsParams is a struct for passing parameters to the method [`ListSLOs`]
#[derive(Clone, Debug, Default)]
pub struct ListSLOsParams {
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

/// SearchSLOParams is a struct for passing parameters to the method [`SearchSLO`]
#[derive(Clone, Debug, Default)]
pub struct SearchSLOParams {
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

/// UpdateSLOParams is a struct for passing parameters to the method [`UpdateSLO`]
#[derive(Clone, Debug, Default)]
pub struct UpdateSLOParams {
    /// The ID of the service level objective object.
    pub slo_id: String,
    /// The edited service level objective request object.
    pub body: crate::datadogV1::model::ServiceLevelObjective,
}

/// CheckCanDeleteSLOError is a struct for typed errors of method [`CheckCanDeleteSLO`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CheckCanDeleteSLOError {
    Status400(Option<crate::datadogV1::model::APIErrorResponse>),
    Status403(Option<crate::datadogV1::model::APIErrorResponse>),
    Status429(Option<crate::datadogV1::model::APIErrorResponse>),
    Status409(Option<crate::datadogV1::model::CheckCanDeleteSLOResponse>),
    UnknownValue(serde_json::Value),
}

/// CreateSLOError is a struct for typed errors of method [`CreateSLO`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CreateSLOError {
    Status400(Option<crate::datadogV1::model::APIErrorResponse>),
    Status403(Option<crate::datadogV1::model::APIErrorResponse>),
    Status429(Option<crate::datadogV1::model::APIErrorResponse>),
    UnknownValue(serde_json::Value),
}

/// DeleteSLOError is a struct for typed errors of method [`DeleteSLO`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DeleteSLOError {
    Status403(Option<crate::datadogV1::model::APIErrorResponse>),
    Status404(Option<crate::datadogV1::model::APIErrorResponse>),
    Status429(Option<crate::datadogV1::model::APIErrorResponse>),
    Status409(Option<crate::datadogV1::model::SLODeleteResponse>),
    UnknownValue(serde_json::Value),
}

/// DeleteSLOTimeframeInBulkError is a struct for typed errors of method [`DeleteSLOTimeframeInBulk`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DeleteSLOTimeframeInBulkError {
    Status400(Option<crate::datadogV1::model::APIErrorResponse>),
    Status403(Option<crate::datadogV1::model::APIErrorResponse>),
    Status429(Option<crate::datadogV1::model::APIErrorResponse>),
    UnknownValue(serde_json::Value),
}

/// GetSLOError is a struct for typed errors of method [`GetSLO`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetSLOError {
    Status403(Option<crate::datadogV1::model::APIErrorResponse>),
    Status404(Option<crate::datadogV1::model::APIErrorResponse>),
    Status429(Option<crate::datadogV1::model::APIErrorResponse>),
    UnknownValue(serde_json::Value),
}

/// GetSLOCorrectionsError is a struct for typed errors of method [`GetSLOCorrections`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetSLOCorrectionsError {
    Status400(Option<crate::datadogV1::model::APIErrorResponse>),
    Status403(Option<crate::datadogV1::model::APIErrorResponse>),
    Status404(Option<crate::datadogV1::model::APIErrorResponse>),
    Status429(Option<crate::datadogV1::model::APIErrorResponse>),
    UnknownValue(serde_json::Value),
}

/// GetSLOHistoryError is a struct for typed errors of method [`GetSLOHistory`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetSLOHistoryError {
    Status400(Option<crate::datadogV1::model::APIErrorResponse>),
    Status403(Option<crate::datadogV1::model::APIErrorResponse>),
    Status404(Option<crate::datadogV1::model::APIErrorResponse>),
    Status429(Option<crate::datadogV1::model::APIErrorResponse>),
    UnknownValue(serde_json::Value),
}

/// ListSLOsError is a struct for typed errors of method [`ListSLOs`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ListSLOsError {
    Status400(Option<crate::datadogV1::model::APIErrorResponse>),
    Status403(Option<crate::datadogV1::model::APIErrorResponse>),
    Status404(Option<crate::datadogV1::model::APIErrorResponse>),
    Status429(Option<crate::datadogV1::model::APIErrorResponse>),
    UnknownValue(serde_json::Value),
}

/// SearchSLOError is a struct for typed errors of method [`SearchSLO`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum SearchSLOError {
    Status400(Option<crate::datadogV1::model::APIErrorResponse>),
    Status403(Option<crate::datadogV1::model::APIErrorResponse>),
    Status429(Option<crate::datadogV1::model::APIErrorResponse>),
    UnknownValue(serde_json::Value),
}

/// UpdateSLOError is a struct for typed errors of method [`UpdateSLO`]
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
}

impl Default for ServiceLevelObjectivesAPI {
    fn default() -> Self {
        Self {
            config: configuration::Configuration::new(),
        }
    }
}

impl ServiceLevelObjectivesAPI {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn with_config(config: configuration::Configuration) -> Self {
        Self { config }
    }

    /// Check if an SLO can be safely deleted. For example,
    /// assure an SLO can be deleted without disrupting a dashboard.
    pub async fn check_can_delete_slo(
        &self,
        params: CheckCanDeleteSLOParams,
    ) -> Result<
        Option<crate::datadogV1::model::CheckCanDeleteSLOResponse>,
        Error<CheckCanDeleteSLOError>,
    > {
        match self.check_can_delete_slo_with_http_info(params).await {
            Ok(response_content) => Ok(response_content.entity),
            Err(err) => Err(err),
        }
    }

    /// Check if an SLO can be safely deleted. For example,
    /// assure an SLO can be deleted without disrupting a dashboard.
    pub async fn check_can_delete_slo_with_http_info(
        &self,
        params: CheckCanDeleteSLOParams,
    ) -> Result<
        ResponseContent<crate::datadogV1::model::CheckCanDeleteSLOResponse>,
        Error<CheckCanDeleteSLOError>,
    > {
        let local_configuration = &self.config;

        // unbox and build parameters
        let ids = params.ids;

        let local_client = &local_configuration.client;

        let local_uri_str = format!("{}/api/v1/slo/can_delete", local_configuration.base_path);
        let mut local_req_builder =
            local_client.request(reqwest::Method::GET, local_uri_str.as_str());

        local_req_builder = local_req_builder.query(&[("ids", &ids.to_string())]);

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
            let local_entity: Option<crate::datadogV1::model::CheckCanDeleteSLOResponse> =
                serde_json::from_str(&local_content).ok();
            Ok(ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            })
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
        params: CreateSLOParams,
    ) -> Result<Option<crate::datadogV1::model::SLOListResponse>, Error<CreateSLOError>> {
        match self.create_slo_with_http_info(params).await {
            Ok(response_content) => Ok(response_content.entity),
            Err(err) => Err(err),
        }
    }

    /// Create a service level objective object.
    pub async fn create_slo_with_http_info(
        &self,
        params: CreateSLOParams,
    ) -> Result<ResponseContent<crate::datadogV1::model::SLOListResponse>, Error<CreateSLOError>>
    {
        let local_configuration = &self.config;

        // unbox and build parameters
        let body = params.body;

        let local_client = &local_configuration.client;

        let local_uri_str = format!("{}/api/v1/slo", local_configuration.base_path);
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
            let local_entity: Option<crate::datadogV1::model::SLOListResponse> =
                serde_json::from_str(&local_content).ok();
            Ok(ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            })
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
        params: DeleteSLOParams,
    ) -> Result<Option<crate::datadogV1::model::SLODeleteResponse>, Error<DeleteSLOError>> {
        match self.delete_slo_with_http_info(params).await {
            Ok(response_content) => Ok(response_content.entity),
            Err(err) => Err(err),
        }
    }

    /// Permanently delete the specified service level objective object.
    ///
    /// If an SLO is used in a dashboard, the `DELETE /v1/slo/` endpoint returns
    /// a 409 conflict error because the SLO is referenced in a dashboard.
    pub async fn delete_slo_with_http_info(
        &self,
        params: DeleteSLOParams,
    ) -> Result<ResponseContent<crate::datadogV1::model::SLODeleteResponse>, Error<DeleteSLOError>>
    {
        let local_configuration = &self.config;

        // unbox and build parameters
        let slo_id = params.slo_id;
        let force = params.force;

        let local_client = &local_configuration.client;

        let local_uri_str = format!(
            "{}/api/v1/slo/{slo_id}",
            local_configuration.base_path,
            slo_id = urlencode(slo_id)
        );
        let mut local_req_builder =
            local_client.request(reqwest::Method::DELETE, local_uri_str.as_str());

        if let Some(ref local_str) = force {
            local_req_builder = local_req_builder.query(&[("force", &local_str.to_string())]);
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
            let local_entity: Option<crate::datadogV1::model::SLODeleteResponse> =
                serde_json::from_str(&local_content).ok();
            Ok(ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            })
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
        params: DeleteSLOTimeframeInBulkParams,
    ) -> Result<
        Option<crate::datadogV1::model::SLOBulkDeleteResponse>,
        Error<DeleteSLOTimeframeInBulkError>,
    > {
        match self
            .delete_slo_timeframe_in_bulk_with_http_info(params)
            .await
        {
            Ok(response_content) => Ok(response_content.entity),
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
        params: DeleteSLOTimeframeInBulkParams,
    ) -> Result<
        ResponseContent<crate::datadogV1::model::SLOBulkDeleteResponse>,
        Error<DeleteSLOTimeframeInBulkError>,
    > {
        let local_configuration = &self.config;

        // unbox and build parameters
        let body = params.body;

        let local_client = &local_configuration.client;

        let local_uri_str = format!("{}/api/v1/slo/bulk_delete", local_configuration.base_path);
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
            let local_entity: Option<crate::datadogV1::model::SLOBulkDeleteResponse> =
                serde_json::from_str(&local_content).ok();
            Ok(ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            })
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
        params: GetSLOParams,
    ) -> Result<Option<crate::datadogV1::model::SLOResponse>, Error<GetSLOError>> {
        match self.get_slo_with_http_info(params).await {
            Ok(response_content) => Ok(response_content.entity),
            Err(err) => Err(err),
        }
    }

    /// Get a service level objective object.
    pub async fn get_slo_with_http_info(
        &self,
        params: GetSLOParams,
    ) -> Result<ResponseContent<crate::datadogV1::model::SLOResponse>, Error<GetSLOError>> {
        let local_configuration = &self.config;

        // unbox and build parameters
        let slo_id = params.slo_id;
        let with_configured_alert_ids = params.with_configured_alert_ids;

        let local_client = &local_configuration.client;

        let local_uri_str = format!(
            "{}/api/v1/slo/{slo_id}",
            local_configuration.base_path,
            slo_id = urlencode(slo_id)
        );
        let mut local_req_builder =
            local_client.request(reqwest::Method::GET, local_uri_str.as_str());

        if let Some(ref local_str) = with_configured_alert_ids {
            local_req_builder =
                local_req_builder.query(&[("with_configured_alert_ids", &local_str.to_string())]);
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
            let local_entity: Option<crate::datadogV1::model::SLOResponse> =
                serde_json::from_str(&local_content).ok();
            Ok(ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            })
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
        params: GetSLOCorrectionsParams,
    ) -> Result<
        Option<crate::datadogV1::model::SLOCorrectionListResponse>,
        Error<GetSLOCorrectionsError>,
    > {
        match self.get_slo_corrections_with_http_info(params).await {
            Ok(response_content) => Ok(response_content.entity),
            Err(err) => Err(err),
        }
    }

    /// Get corrections applied to an SLO
    pub async fn get_slo_corrections_with_http_info(
        &self,
        params: GetSLOCorrectionsParams,
    ) -> Result<
        ResponseContent<crate::datadogV1::model::SLOCorrectionListResponse>,
        Error<GetSLOCorrectionsError>,
    > {
        let local_configuration = &self.config;

        // unbox and build parameters
        let slo_id = params.slo_id;

        let local_client = &local_configuration.client;

        let local_uri_str = format!(
            "{}/api/v1/slo/{slo_id}/corrections",
            local_configuration.base_path,
            slo_id = urlencode(slo_id)
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
            let local_entity: Option<crate::datadogV1::model::SLOCorrectionListResponse> =
                serde_json::from_str(&local_content).ok();
            Ok(ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            })
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
        params: GetSLOHistoryParams,
    ) -> Result<Option<crate::datadogV1::model::SLOHistoryResponse>, Error<GetSLOHistoryError>>
    {
        match self.get_slo_history_with_http_info(params).await {
            Ok(response_content) => Ok(response_content.entity),
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
        params: GetSLOHistoryParams,
    ) -> Result<
        ResponseContent<crate::datadogV1::model::SLOHistoryResponse>,
        Error<GetSLOHistoryError>,
    > {
        let local_configuration = &self.config;

        // unbox and build parameters
        let slo_id = params.slo_id;
        let from_ts = params.from_ts;
        let to_ts = params.to_ts;
        let target = params.target;
        let apply_correction = params.apply_correction;

        let local_client = &local_configuration.client;

        let local_uri_str = format!(
            "{}/api/v1/slo/{slo_id}/history",
            local_configuration.base_path,
            slo_id = urlencode(slo_id)
        );
        let mut local_req_builder =
            local_client.request(reqwest::Method::GET, local_uri_str.as_str());

        local_req_builder = local_req_builder.query(&[("from_ts", &from_ts.to_string())]);
        local_req_builder = local_req_builder.query(&[("to_ts", &to_ts.to_string())]);
        if let Some(ref local_str) = target {
            local_req_builder = local_req_builder.query(&[("target", &local_str.to_string())]);
        };
        if let Some(ref local_str) = apply_correction {
            local_req_builder =
                local_req_builder.query(&[("apply_correction", &local_str.to_string())]);
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
            let local_entity: Option<crate::datadogV1::model::SLOHistoryResponse> =
                serde_json::from_str(&local_content).ok();
            Ok(ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            })
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
    pub async fn list_sl_os(
        &self,
        params: ListSLOsParams,
    ) -> Result<Option<crate::datadogV1::model::SLOListResponse>, Error<ListSLOsError>> {
        match self.list_sl_os_with_http_info(params).await {
            Ok(response_content) => Ok(response_content.entity),
            Err(err) => Err(err),
        }
    }

    /// Get a list of service level objective objects for your organization.
    pub async fn list_sl_os_with_http_info(
        &self,
        params: ListSLOsParams,
    ) -> Result<ResponseContent<crate::datadogV1::model::SLOListResponse>, Error<ListSLOsError>>
    {
        let local_configuration = &self.config;

        // unbox and build parameters
        let ids = params.ids;
        let query = params.query;
        let tags_query = params.tags_query;
        let metrics_query = params.metrics_query;
        let limit = params.limit;
        let offset = params.offset;

        let local_client = &local_configuration.client;

        let local_uri_str = format!("{}/api/v1/slo", local_configuration.base_path);
        let mut local_req_builder =
            local_client.request(reqwest::Method::GET, local_uri_str.as_str());

        if let Some(ref local_str) = ids {
            local_req_builder = local_req_builder.query(&[("ids", &local_str.to_string())]);
        };
        if let Some(ref local_str) = query {
            local_req_builder = local_req_builder.query(&[("query", &local_str.to_string())]);
        };
        if let Some(ref local_str) = tags_query {
            local_req_builder = local_req_builder.query(&[("tags_query", &local_str.to_string())]);
        };
        if let Some(ref local_str) = metrics_query {
            local_req_builder =
                local_req_builder.query(&[("metrics_query", &local_str.to_string())]);
        };
        if let Some(ref local_str) = limit {
            local_req_builder = local_req_builder.query(&[("limit", &local_str.to_string())]);
        };
        if let Some(ref local_str) = offset {
            local_req_builder = local_req_builder.query(&[("offset", &local_str.to_string())]);
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
            let local_entity: Option<crate::datadogV1::model::SLOListResponse> =
                serde_json::from_str(&local_content).ok();
            Ok(ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            })
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
        params: SearchSLOParams,
    ) -> Result<Option<crate::datadogV1::model::SearchSLOResponse>, Error<SearchSLOError>> {
        match self.search_slo_with_http_info(params).await {
            Ok(response_content) => Ok(response_content.entity),
            Err(err) => Err(err),
        }
    }

    /// Get a list of service level objective objects for your organization.
    pub async fn search_slo_with_http_info(
        &self,
        params: SearchSLOParams,
    ) -> Result<ResponseContent<crate::datadogV1::model::SearchSLOResponse>, Error<SearchSLOError>>
    {
        let local_configuration = &self.config;

        // unbox and build parameters
        let query = params.query;
        let page_size = params.page_size;
        let page_number = params.page_number;
        let include_facets = params.include_facets;

        let local_client = &local_configuration.client;

        let local_uri_str = format!("{}/api/v1/slo/search", local_configuration.base_path);
        let mut local_req_builder =
            local_client.request(reqwest::Method::GET, local_uri_str.as_str());

        if let Some(ref local_str) = query {
            local_req_builder = local_req_builder.query(&[("query", &local_str.to_string())]);
        };
        if let Some(ref local_str) = page_size {
            local_req_builder = local_req_builder.query(&[("page[size]", &local_str.to_string())]);
        };
        if let Some(ref local_str) = page_number {
            local_req_builder =
                local_req_builder.query(&[("page[number]", &local_str.to_string())]);
        };
        if let Some(ref local_str) = include_facets {
            local_req_builder =
                local_req_builder.query(&[("include_facets", &local_str.to_string())]);
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
            let local_entity: Option<crate::datadogV1::model::SearchSLOResponse> =
                serde_json::from_str(&local_content).ok();
            Ok(ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            })
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
        params: UpdateSLOParams,
    ) -> Result<Option<crate::datadogV1::model::SLOListResponse>, Error<UpdateSLOError>> {
        match self.update_slo_with_http_info(params).await {
            Ok(response_content) => Ok(response_content.entity),
            Err(err) => Err(err),
        }
    }

    /// Update the specified service level objective object.
    pub async fn update_slo_with_http_info(
        &self,
        params: UpdateSLOParams,
    ) -> Result<ResponseContent<crate::datadogV1::model::SLOListResponse>, Error<UpdateSLOError>>
    {
        let local_configuration = &self.config;

        // unbox and build parameters
        let slo_id = params.slo_id;
        let body = params.body;

        let local_client = &local_configuration.client;

        let local_uri_str = format!(
            "{}/api/v1/slo/{slo_id}",
            local_configuration.base_path,
            slo_id = urlencode(slo_id)
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
            let local_entity: Option<crate::datadogV1::model::SLOListResponse> =
                serde_json::from_str(&local_content).ok();
            Ok(ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            })
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