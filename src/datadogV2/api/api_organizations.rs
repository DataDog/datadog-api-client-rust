// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use crate::datadog::*;
use reqwest;
use reqwest::header::{HeaderMap, HeaderValue};
use serde::{Deserialize, Serialize};

/// UploadIdPMetadataOptionalParams is a struct for passing parameters to the method [`OrganizationsAPI::upload_idp_metadata`]
#[non_exhaustive]
#[derive(Clone, Default, Debug)]
pub struct UploadIdPMetadataOptionalParams {
    /// The IdP metadata XML file
    pub idp_file: Option<Vec<u8>>,
}

impl UploadIdPMetadataOptionalParams {
    /// The IdP metadata XML file
    pub fn idp_file(mut self, value: Vec<u8>) -> Self {
        self.idp_file = Some(value);
        self
    }
}

/// UploadIdPMetadataError is a struct for typed errors of method [`OrganizationsAPI::upload_idp_metadata`]
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
    client: reqwest_middleware::ClientWithMiddleware,
}

impl Default for OrganizationsAPI {
    fn default() -> Self {
        Self {
            config: configuration::Configuration::new(),
            client: reqwest_middleware::ClientBuilder::new(reqwest::Client::new()).build(),
        }
    }
}

impl OrganizationsAPI {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn with_config(config: configuration::Configuration) -> Self {
        let mut reqwest_client_builder = reqwest::Client::builder();

        if let Some(proxy_url) = &config.proxy_url {
            let proxy = reqwest::Proxy::all(proxy_url).expect("Failed to parse proxy URL");
            reqwest_client_builder = reqwest_client_builder.proxy(proxy);
        }

        let middleware_client_builder =
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

    /// Endpoint for uploading IdP metadata for SAML setup.
    ///
    /// Use this endpoint to upload or replace IdP metadata for SAML login configuration.
    pub async fn upload_idp_metadata(
        &self,
        params: UploadIdPMetadataOptionalParams,
    ) -> Result<(), Error<UploadIdPMetadataError>> {
        match self.upload_idp_metadata_with_http_info(params).await {
            Ok(_) => Ok(()),
            Err(err) => Err(err),
        }
    }

    /// Endpoint for uploading IdP metadata for SAML setup.
    ///
    /// Use this endpoint to upload or replace IdP metadata for SAML login configuration.
    pub async fn upload_idp_metadata_with_http_info(
        &self,
        params: UploadIdPMetadataOptionalParams,
    ) -> Result<ResponseContent<()>, Error<UploadIdPMetadataError>> {
        let local_configuration = &self.config;
        let operation_id = "v2.upload_idp_metadata";

        // unbox and build optional parameters
        let idp_file = params.idp_file;

        let local_client = &self.client;

        let local_uri_str = format!(
            "{}/api/v2/saml_configurations/idp_metadata",
            local_configuration.get_operation_host(operation_id)
        );
        let mut local_req_builder =
            local_client.request(reqwest::Method::POST, local_uri_str.as_str());

        // build headers
        let mut headers = HeaderMap::new();
        headers.insert(
            "Content-Type",
            HeaderValue::from_static("multipart/form-data"),
        );
        headers.insert("Accept", HeaderValue::from_static("*/*"));

        // build user agent
        match HeaderValue::from_str(local_configuration.user_agent.clone().as_str()) {
            Ok(user_agent) => headers.insert(reqwest::header::USER_AGENT, user_agent),
            Err(e) => {
                log::warn!("Failed to parse user agent header: {e}, falling back to default");
                headers.insert(
                    reqwest::header::USER_AGENT,
                    HeaderValue::from_static(configuration::DEFAULT_USER_AGENT.as_str()),
                )
            }
        };

        // build auth
        if let Some(local_key) = local_configuration.auth_keys.get("apiKeyAuth") {
            headers.insert(
                "DD-API-KEY",
                local_key
                    .key
                    .clone()
                    .parse()
                    .expect("failed to parse DD-API-KEY header"),
            );
        };
        if let Some(local_key) = local_configuration.auth_keys.get("appKeyAuth") {
            headers.insert(
                "DD-APPLICATION-KEY",
                local_key
                    .key
                    .clone()
                    .parse()
                    .expect("failed to parse DD-APPLICATION-KEY header"),
            );
        };

        // build form parameters
        if let Some(idp_file) = idp_file {
            let mut local_form = reqwest::multipart::Form::new();
            local_form = local_form.part(
                "idp_file",
                reqwest::multipart::Part::bytes(idp_file).file_name("idp_file"),
            );
            headers.insert(
                "Content-Type",
                format!("multipart/form-data; boundary={}", local_form.boundary())
                    .parse()
                    .unwrap(),
            );
            local_req_builder = local_req_builder.multipart(local_form);
        };

        local_req_builder = local_req_builder.headers(headers);
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
