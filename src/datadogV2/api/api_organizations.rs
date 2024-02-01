// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use crate::datadog::*;
use reqwest;
use serde::{Deserialize, Serialize};

/// UploadIdPMetadataOptionalParams is a struct for passing parameters to the method [`OrganizationsAPI::upload_id_p_metadata`]
#[derive(Clone, Default, Debug)]
pub struct UploadIdPMetadataOptionalParams {
    /// The IdP metadata XML file
    pub idp_file: Option<Vec<u8>>,
}

impl UploadIdPMetadataOptionalParams {
    /// The IdP metadata XML file
    pub fn idp_file(&mut self, value: Vec<u8>) -> &mut Self {
        self.idp_file = Some(value);
        self
    }
}

/// UploadIdPMetadataError is a struct for typed errors of method [`OrganizationsAPI::upload_id_p_metadata`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum UploadIdPMetadataError {
    Status400(Option<crate::datadogV2::model::APIErrorResponse>),
    Status403(Option<crate::datadogV2::model::APIErrorResponse>),
    Status429(Option<crate::datadogV2::model::APIErrorResponse>),
    UnknownValue(serde_json::Value),
}

#[derive(Debug, Clone)]
pub struct OrganizationsAPI {
    config: configuration::Configuration,
}

impl Default for OrganizationsAPI {
    fn default() -> Self {
        Self {
            config: configuration::Configuration::new(),
        }
    }
}

impl OrganizationsAPI {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn with_config(config: configuration::Configuration) -> Self {
        Self { config }
    }

    /// Endpoint for uploading IdP metadata for SAML setup.
    ///
    /// Use this endpoint to upload or replace IdP metadata for SAML login configuration.
    pub async fn upload_id_p_metadata(
        &self,
        params: UploadIdPMetadataOptionalParams,
    ) -> Result<Option<()>, Error<UploadIdPMetadataError>> {
        match self.upload_id_p_metadata_with_http_info(params).await {
            Ok(response_content) => Ok(response_content.entity),
            Err(err) => Err(err),
        }
    }

    /// Endpoint for uploading IdP metadata for SAML setup.
    ///
    /// Use this endpoint to upload or replace IdP metadata for SAML login configuration.
    pub async fn upload_id_p_metadata_with_http_info(
        &self,
        params: UploadIdPMetadataOptionalParams,
    ) -> Result<ResponseContent<()>, Error<UploadIdPMetadataError>> {
        let local_configuration = &self.config;

        // unbox and build optional parameters
        let idp_file = params.idp_file;

        let local_client = &local_configuration.client;

        let local_uri_str = format!(
            "{}/api/v2/saml_configurations/idp_metadata",
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

        // build form parameters
        if let Some(idp_file) = idp_file {
            let mut local_form = reqwest::multipart::Form::new();
            local_form = local_form.part(
                "idp_file",
                reqwest::multipart::Part::bytes(idp_file).file_name("idp_file"),
            );
            local_req_builder = local_req_builder.multipart(local_form);
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
            let local_entity: Option<UploadIdPMetadataError> =
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
