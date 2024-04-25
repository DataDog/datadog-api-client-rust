// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use crate::datadog;
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
    APIErrorResponse(crate::datadogV2::model::APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// Create, edit, and manage your organizations. Read more about [multi-org accounts](<https://docs.datadoghq.com/account_management/multi_organization>).
#[derive(Debug, Clone)]
pub struct OrganizationsAPI {
    config: datadog::Configuration,
    client: reqwest_middleware::ClientWithMiddleware,
}

impl Default for OrganizationsAPI {
    fn default() -> Self {
        Self::with_config(datadog::Configuration::default())
    }
}

impl OrganizationsAPI {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn with_config(config: datadog::Configuration) -> Self {
        let mut reqwest_client_builder = reqwest::Client::builder();

        if let Some(proxy_url) = &config.proxy_url {
            let proxy = reqwest::Proxy::all(proxy_url).expect("Failed to parse proxy URL");
            reqwest_client_builder = reqwest_client_builder.proxy(proxy);
        }

        let mut middleware_client_builder =
            reqwest_middleware::ClientBuilder::new(reqwest_client_builder.build().unwrap());

        if config.enable_retry {
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

            middleware_client_builder = middleware_client_builder.with(retry_middleware);
        }

        let client = middleware_client_builder.build();

        Self { config, client }
    }

    pub fn with_client_and_config(
        config: datadog::Configuration,
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
    ) -> Result<(), datadog::Error<UploadIdPMetadataError>> {
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
    ) -> Result<datadog::ResponseContent<()>, datadog::Error<UploadIdPMetadataError>> {
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

        // build form parameters
        if let Some(idp_file) = idp_file {
            let mut local_form = form_data_builder::FormData::new(Vec::new());
            let cursor = std::io::Cursor::new(idp_file);
            if let Err(e) = local_form.write_file(
                "idp_file",
                cursor,
                Some("idp_file".as_ref()),
                "application/octet-stream",
            ) {
                return Err(crate::datadog::Error::Io(e));
            };
            headers.insert(
                "Content-Type",
                local_form.content_type_header().parse().unwrap(),
            );
            let form_result = local_form.finish();
            match form_result {
                Ok(form) => local_req_builder = local_req_builder.body(form),
                Err(e) => return Err(crate::datadog::Error::Io(e)),
            };
        };

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
            let local_entity: Option<UploadIdPMetadataError> =
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
