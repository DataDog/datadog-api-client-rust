// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use crate::datadog::*;
use async_stream::try_stream;
use futures_core::stream::Stream;
use reqwest;
use serde::{Deserialize, Serialize};

/// GetServiceDefinitionOptionalParams is a struct for passing parameters to the method [`ServiceDefinitionAPI::get_service_definition`]
#[derive(Clone, Default, Debug)]
pub struct GetServiceDefinitionOptionalParams {
    /// The schema version desired in the response.
    pub schema_version: Option<crate::datadogV2::model::ServiceDefinitionSchemaVersions>,
}

impl GetServiceDefinitionOptionalParams {
    /// The schema version desired in the response.
    pub fn schema_version(
        &mut self,
        value: crate::datadogV2::model::ServiceDefinitionSchemaVersions,
    ) -> &mut Self {
        self.schema_version = Some(value);
        self
    }
}

/// ListServiceDefinitionsOptionalParams is a struct for passing parameters to the method [`ServiceDefinitionAPI::list_service_definitions`]
#[derive(Clone, Default, Debug)]
pub struct ListServiceDefinitionsOptionalParams {
    /// Size for a given page. The maximum allowed value is 100.
    pub page_size: Option<i64>,
    /// Specific page number to return.
    pub page_number: Option<i64>,
    /// The schema version desired in the response.
    pub schema_version: Option<crate::datadogV2::model::ServiceDefinitionSchemaVersions>,
}

impl ListServiceDefinitionsOptionalParams {
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
    /// The schema version desired in the response.
    pub fn schema_version(
        &mut self,
        value: crate::datadogV2::model::ServiceDefinitionSchemaVersions,
    ) -> &mut Self {
        self.schema_version = Some(value);
        self
    }
}

/// CreateOrUpdateServiceDefinitionsError is a struct for typed errors of method [`ServiceDefinitionAPI::create_or_update_service_definitions`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CreateOrUpdateServiceDefinitionsError {
    Status400(Option<crate::datadogV2::model::APIErrorResponse>),
    Status403(Option<crate::datadogV2::model::APIErrorResponse>),
    Status409(Option<crate::datadogV2::model::APIErrorResponse>),
    Status429(Option<crate::datadogV2::model::APIErrorResponse>),
    UnknownValue(serde_json::Value),
}

/// DeleteServiceDefinitionError is a struct for typed errors of method [`ServiceDefinitionAPI::delete_service_definition`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DeleteServiceDefinitionError {
    Status400(Option<crate::datadogV2::model::APIErrorResponse>),
    Status403(Option<crate::datadogV2::model::APIErrorResponse>),
    Status404(Option<crate::datadogV2::model::APIErrorResponse>),
    Status429(Option<crate::datadogV2::model::APIErrorResponse>),
    UnknownValue(serde_json::Value),
}

/// GetServiceDefinitionError is a struct for typed errors of method [`ServiceDefinitionAPI::get_service_definition`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetServiceDefinitionError {
    Status400(Option<crate::datadogV2::model::APIErrorResponse>),
    Status403(Option<crate::datadogV2::model::APIErrorResponse>),
    Status404(Option<crate::datadogV2::model::APIErrorResponse>),
    Status409(Option<crate::datadogV2::model::APIErrorResponse>),
    Status429(Option<crate::datadogV2::model::APIErrorResponse>),
    UnknownValue(serde_json::Value),
}

/// ListServiceDefinitionsError is a struct for typed errors of method [`ServiceDefinitionAPI::list_service_definitions`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ListServiceDefinitionsError {
    Status403(Option<crate::datadogV2::model::APIErrorResponse>),
    Status429(Option<crate::datadogV2::model::APIErrorResponse>),
    UnknownValue(serde_json::Value),
}

#[derive(Debug, Clone)]
pub struct ServiceDefinitionAPI {
    config: configuration::Configuration,
}

impl Default for ServiceDefinitionAPI {
    fn default() -> Self {
        Self {
            config: configuration::Configuration::new(),
        }
    }
}

impl ServiceDefinitionAPI {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn with_config(config: configuration::Configuration) -> Self {
        Self { config }
    }

    /// Create or update service definition in the Datadog Service Catalog.
    pub async fn create_or_update_service_definitions(
        &self,
        body: crate::datadogV2::model::ServiceDefinitionsCreateRequest,
    ) -> Result<
        Option<crate::datadogV2::model::ServiceDefinitionCreateResponse>,
        Error<CreateOrUpdateServiceDefinitionsError>,
    > {
        match self
            .create_or_update_service_definitions_with_http_info(body)
            .await
        {
            Ok(response_content) => Ok(response_content.entity),
            Err(err) => Err(err),
        }
    }

    /// Create or update service definition in the Datadog Service Catalog.
    pub async fn create_or_update_service_definitions_with_http_info(
        &self,
        body: crate::datadogV2::model::ServiceDefinitionsCreateRequest,
    ) -> Result<
        ResponseContent<crate::datadogV2::model::ServiceDefinitionCreateResponse>,
        Error<CreateOrUpdateServiceDefinitionsError>,
    > {
        let local_configuration = &self.config;

        let local_client = &local_configuration.client;

        let local_uri_str = format!(
            "{}/api/v2/services/definitions",
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
            let local_entity: Option<crate::datadogV2::model::ServiceDefinitionCreateResponse> =
                serde_json::from_str(&local_content).ok();
            Ok(ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            })
        } else {
            let local_entity: Option<CreateOrUpdateServiceDefinitionsError> =
                serde_json::from_str(&local_content).ok();
            let local_error = ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(Error::ResponseError(local_error))
        }
    }

    /// Delete a single service definition in the Datadog Service Catalog.
    pub async fn delete_service_definition(
        &self,
        service_name: String,
    ) -> Result<Option<()>, Error<DeleteServiceDefinitionError>> {
        match self
            .delete_service_definition_with_http_info(service_name)
            .await
        {
            Ok(response_content) => Ok(response_content.entity),
            Err(err) => Err(err),
        }
    }

    /// Delete a single service definition in the Datadog Service Catalog.
    pub async fn delete_service_definition_with_http_info(
        &self,
        service_name: String,
    ) -> Result<ResponseContent<()>, Error<DeleteServiceDefinitionError>> {
        let local_configuration = &self.config;

        let local_client = &local_configuration.client;

        let local_uri_str = format!(
            "{}/api/v2/services/definitions/{service_name}",
            local_configuration.base_path,
            service_name = urlencode(service_name)
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
            let local_entity: Option<DeleteServiceDefinitionError> =
                serde_json::from_str(&local_content).ok();
            let local_error = ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(Error::ResponseError(local_error))
        }
    }

    /// Get a single service definition from the Datadog Service Catalog.
    pub async fn get_service_definition(
        &self,
        service_name: String,
        params: GetServiceDefinitionOptionalParams,
    ) -> Result<
        Option<crate::datadogV2::model::ServiceDefinitionGetResponse>,
        Error<GetServiceDefinitionError>,
    > {
        match self
            .get_service_definition_with_http_info(service_name, params)
            .await
        {
            Ok(response_content) => Ok(response_content.entity),
            Err(err) => Err(err),
        }
    }

    /// Get a single service definition from the Datadog Service Catalog.
    pub async fn get_service_definition_with_http_info(
        &self,
        service_name: String,
        params: GetServiceDefinitionOptionalParams,
    ) -> Result<
        ResponseContent<crate::datadogV2::model::ServiceDefinitionGetResponse>,
        Error<GetServiceDefinitionError>,
    > {
        let local_configuration = &self.config;

        // unbox and build optional parameters
        let schema_version = params.schema_version;

        let local_client = &local_configuration.client;

        let local_uri_str = format!(
            "{}/api/v2/services/definitions/{service_name}",
            local_configuration.base_path,
            service_name = urlencode(service_name)
        );
        let mut local_req_builder =
            local_client.request(reqwest::Method::GET, local_uri_str.as_str());

        if let Some(ref local_query_param) = schema_version {
            local_req_builder =
                local_req_builder.query(&[("schema_version", &local_query_param.to_string())]);
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
            let local_entity: Option<crate::datadogV2::model::ServiceDefinitionGetResponse> =
                serde_json::from_str(&local_content).ok();
            Ok(ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            })
        } else {
            let local_entity: Option<GetServiceDefinitionError> =
                serde_json::from_str(&local_content).ok();
            let local_error = ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(Error::ResponseError(local_error))
        }
    }

    /// Get a list of all service definitions from the Datadog Service Catalog.
    pub async fn list_service_definitions(
        &self,
        params: ListServiceDefinitionsOptionalParams,
    ) -> Result<
        Option<crate::datadogV2::model::ServiceDefinitionsListResponse>,
        Error<ListServiceDefinitionsError>,
    > {
        match self.list_service_definitions_with_http_info(params).await {
            Ok(response_content) => Ok(response_content.entity),
            Err(err) => Err(err),
        }
    }

    pub fn list_service_definitions_with_pagination(
        &self,
        mut params: ListServiceDefinitionsOptionalParams,
    ) -> impl Stream<
        Item = Result<
            crate::datadogV2::model::ServiceDefinitionData,
            Error<ListServiceDefinitionsError>,
        >,
    > + '_ {
        try_stream! {
            let mut page_size: i64 = 10;
            if params.page_size.is_none() {
                params.page_size = Some(page_size);
            } else {
                page_size = params.page_size.unwrap().clone();
            }
            loop {
                let resp = self.list_service_definitions(params.clone()).await?;

                let Some(resp) = resp else { break };
                let Some(data) = resp.data else { break };

                let r = data;
                let count = r.len();
                for team in r {
                    yield team;
                }

                if count < page_size as usize {
                    break;
                }
                if params.page_number.is_none() {
                    params.page_number = Some(page_size.clone());
                } else {
                    params.page_number = Some(params.page_number.unwrap() + page_size.clone());
                }
            }
        }
    }

    /// Get a list of all service definitions from the Datadog Service Catalog.
    pub async fn list_service_definitions_with_http_info(
        &self,
        params: ListServiceDefinitionsOptionalParams,
    ) -> Result<
        ResponseContent<crate::datadogV2::model::ServiceDefinitionsListResponse>,
        Error<ListServiceDefinitionsError>,
    > {
        let local_configuration = &self.config;

        // unbox and build optional parameters
        let page_size = params.page_size;
        let page_number = params.page_number;
        let schema_version = params.schema_version;

        let local_client = &local_configuration.client;

        let local_uri_str = format!(
            "{}/api/v2/services/definitions",
            local_configuration.base_path
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
        if let Some(ref local_query_param) = schema_version {
            local_req_builder =
                local_req_builder.query(&[("schema_version", &local_query_param.to_string())]);
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
            let local_entity: Option<crate::datadogV2::model::ServiceDefinitionsListResponse> =
                serde_json::from_str(&local_content).ok();
            Ok(ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            })
        } else {
            let local_entity: Option<ListServiceDefinitionsError> =
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
