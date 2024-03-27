// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use crate::datadog::*;
use reqwest;
use serde::{Deserialize, Serialize};

/// CreateChildOrgError is a struct for typed errors of method [`OrganizationsAPI::create_child_org`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CreateChildOrgError {
    Status400(Option<crate::datadogV1::model::APIErrorResponse>),
    Status403(Option<crate::datadogV1::model::APIErrorResponse>),
    Status429(Option<crate::datadogV1::model::APIErrorResponse>),
    UnknownValue(serde_json::Value),
}

/// DowngradeOrgError is a struct for typed errors of method [`OrganizationsAPI::downgrade_org`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DowngradeOrgError {
    Status400(Option<crate::datadogV1::model::APIErrorResponse>),
    Status403(Option<crate::datadogV1::model::APIErrorResponse>),
    Status429(Option<crate::datadogV1::model::APIErrorResponse>),
    UnknownValue(serde_json::Value),
}

/// GetOrgError is a struct for typed errors of method [`OrganizationsAPI::get_org`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetOrgError {
    Status400(Option<crate::datadogV1::model::APIErrorResponse>),
    Status403(Option<crate::datadogV1::model::APIErrorResponse>),
    Status429(Option<crate::datadogV1::model::APIErrorResponse>),
    UnknownValue(serde_json::Value),
}

/// ListOrgsError is a struct for typed errors of method [`OrganizationsAPI::list_orgs`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ListOrgsError {
    Status403(Option<crate::datadogV1::model::APIErrorResponse>),
    Status429(Option<crate::datadogV1::model::APIErrorResponse>),
    UnknownValue(serde_json::Value),
}

/// UpdateOrgError is a struct for typed errors of method [`OrganizationsAPI::update_org`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum UpdateOrgError {
    Status400(Option<crate::datadogV1::model::APIErrorResponse>),
    Status403(Option<crate::datadogV1::model::APIErrorResponse>),
    Status429(Option<crate::datadogV1::model::APIErrorResponse>),
    UnknownValue(serde_json::Value),
}

/// UploadIdPForOrgError is a struct for typed errors of method [`OrganizationsAPI::upload_idp_for_org`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum UploadIdPForOrgError {
    Status400(Option<crate::datadogV1::model::APIErrorResponse>),
    Status403(Option<crate::datadogV1::model::APIErrorResponse>),
    Status415(Option<crate::datadogV1::model::APIErrorResponse>),
    Status429(Option<crate::datadogV1::model::APIErrorResponse>),
    UnknownValue(serde_json::Value),
}

#[derive(Debug, Clone)]
pub struct OrganizationsAPI {
    config: configuration::Configuration,
    client: reqwest_middleware::ClientWithMiddleware,
}

impl Default for OrganizationsAPI {
    fn default() -> Self {
        Self::with_config(configuration::Configuration::default())
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
        config: configuration::Configuration,
        client: reqwest_middleware::ClientWithMiddleware,
    ) -> Self {
        Self { config, client }
    }

    /// Create a child organization.
    ///
    /// This endpoint requires the
    /// [multi-organization account](<https://docs.datadoghq.com/account_management/multi_organization/>)
    /// feature and must be enabled by
    /// [contacting support](<https://docs.datadoghq.com/help/>).
    ///
    /// Once a new child organization is created, you can interact with it
    /// by using the `org.public_id`, `api_key.key`, and
    /// `application_key.hash` provided in the response.
    pub async fn create_child_org(
        &self,
        body: crate::datadogV1::model::OrganizationCreateBody,
    ) -> Result<crate::datadogV1::model::OrganizationCreateResponse, Error<CreateChildOrgError>>
    {
        match self.create_child_org_with_http_info(body).await {
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

    /// Create a child organization.
    ///
    /// This endpoint requires the
    /// [multi-organization account](<https://docs.datadoghq.com/account_management/multi_organization/>)
    /// feature and must be enabled by
    /// [contacting support](<https://docs.datadoghq.com/help/>).
    ///
    /// Once a new child organization is created, you can interact with it
    /// by using the `org.public_id`, `api_key.key`, and
    /// `application_key.hash` provided in the response.
    pub async fn create_child_org_with_http_info(
        &self,
        body: crate::datadogV1::model::OrganizationCreateBody,
    ) -> Result<
        ResponseContent<crate::datadogV1::model::OrganizationCreateResponse>,
        Error<CreateChildOrgError>,
    > {
        let local_configuration = &self.config;
        let operation_id = "v1.create_child_org";

        let local_client = &self.client;

        let local_uri_str = format!(
            "{}/api/v1/org",
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
            match serde_json::from_str::<crate::datadogV1::model::OrganizationCreateResponse>(
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
            let local_entity: Option<CreateChildOrgError> =
                serde_json::from_str(&local_content).ok();
            let local_error = ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(Error::ResponseError(local_error))
        }
    }

    /// Only available for MSP customers. Removes a child organization from the hierarchy of the master organization and places the child organization on a 30-day trial.
    pub async fn downgrade_org(
        &self,
        public_id: String,
    ) -> Result<crate::datadogV1::model::OrgDowngradedResponse, Error<DowngradeOrgError>> {
        match self.downgrade_org_with_http_info(public_id).await {
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

    /// Only available for MSP customers. Removes a child organization from the hierarchy of the master organization and places the child organization on a 30-day trial.
    pub async fn downgrade_org_with_http_info(
        &self,
        public_id: String,
    ) -> Result<
        ResponseContent<crate::datadogV1::model::OrgDowngradedResponse>,
        Error<DowngradeOrgError>,
    > {
        let local_configuration = &self.config;
        let operation_id = "v1.downgrade_org";

        let local_client = &self.client;

        let local_uri_str = format!(
            "{}/api/v1/org/{public_id}/downgrade",
            local_configuration.get_operation_host(operation_id),
            public_id = urlencode(public_id)
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

        let local_req = local_req_builder.build()?;
        let local_resp = local_client.execute(local_req).await?;

        let local_status = local_resp.status();
        let local_content = local_resp.text().await?;

        if !local_status.is_client_error() && !local_status.is_server_error() {
            match serde_json::from_str::<crate::datadogV1::model::OrgDowngradedResponse>(
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
            let local_entity: Option<DowngradeOrgError> = serde_json::from_str(&local_content).ok();
            let local_error = ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(Error::ResponseError(local_error))
        }
    }

    /// Get organization information.
    pub async fn get_org(
        &self,
        public_id: String,
    ) -> Result<crate::datadogV1::model::OrganizationResponse, Error<GetOrgError>> {
        match self.get_org_with_http_info(public_id).await {
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

    /// Get organization information.
    pub async fn get_org_with_http_info(
        &self,
        public_id: String,
    ) -> Result<ResponseContent<crate::datadogV1::model::OrganizationResponse>, Error<GetOrgError>>
    {
        let local_configuration = &self.config;
        let operation_id = "v1.get_org";

        let local_client = &self.client;

        let local_uri_str = format!(
            "{}/api/v1/org/{public_id}",
            local_configuration.get_operation_host(operation_id),
            public_id = urlencode(public_id)
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
            match serde_json::from_str::<crate::datadogV1::model::OrganizationResponse>(
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
            let local_entity: Option<GetOrgError> = serde_json::from_str(&local_content).ok();
            let local_error = ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(Error::ResponseError(local_error))
        }
    }

    /// This endpoint returns data on your top-level organization.
    pub async fn list_orgs(
        &self,
    ) -> Result<crate::datadogV1::model::OrganizationListResponse, Error<ListOrgsError>> {
        match self.list_orgs_with_http_info().await {
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

    /// This endpoint returns data on your top-level organization.
    pub async fn list_orgs_with_http_info(
        &self,
    ) -> Result<
        ResponseContent<crate::datadogV1::model::OrganizationListResponse>,
        Error<ListOrgsError>,
    > {
        let local_configuration = &self.config;
        let operation_id = "v1.list_orgs";

        let local_client = &self.client;

        let local_uri_str = format!(
            "{}/api/v1/org",
            local_configuration.get_operation_host(operation_id)
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
            match serde_json::from_str::<crate::datadogV1::model::OrganizationListResponse>(
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
            let local_entity: Option<ListOrgsError> = serde_json::from_str(&local_content).ok();
            let local_error = ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(Error::ResponseError(local_error))
        }
    }

    /// Update your organization.
    pub async fn update_org(
        &self,
        public_id: String,
        body: crate::datadogV1::model::Organization,
    ) -> Result<crate::datadogV1::model::OrganizationResponse, Error<UpdateOrgError>> {
        match self.update_org_with_http_info(public_id, body).await {
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

    /// Update your organization.
    pub async fn update_org_with_http_info(
        &self,
        public_id: String,
        body: crate::datadogV1::model::Organization,
    ) -> Result<ResponseContent<crate::datadogV1::model::OrganizationResponse>, Error<UpdateOrgError>>
    {
        let local_configuration = &self.config;
        let operation_id = "v1.update_org";

        let local_client = &self.client;

        let local_uri_str = format!(
            "{}/api/v1/org/{public_id}",
            local_configuration.get_operation_host(operation_id),
            public_id = urlencode(public_id)
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
            match serde_json::from_str::<crate::datadogV1::model::OrganizationResponse>(
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
            let local_entity: Option<UpdateOrgError> = serde_json::from_str(&local_content).ok();
            let local_error = ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(Error::ResponseError(local_error))
        }
    }

    /// There are a couple of options for updating the Identity Provider (IdP)
    /// metadata from your SAML IdP.
    ///
    /// * **Multipart Form-Data**: Post the IdP metadata file using a form post.
    ///
    /// * **XML Body:** Post the IdP metadata file as the body of the request.
    pub async fn upload_idp_for_org(
        &self,
        public_id: String,
        idp_file: Vec<u8>,
    ) -> Result<crate::datadogV1::model::IdpResponse, Error<UploadIdPForOrgError>> {
        match self
            .upload_idp_for_org_with_http_info(public_id, idp_file)
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

    /// There are a couple of options for updating the Identity Provider (IdP)
    /// metadata from your SAML IdP.
    ///
    /// * **Multipart Form-Data**: Post the IdP metadata file using a form post.
    ///
    /// * **XML Body:** Post the IdP metadata file as the body of the request.
    pub async fn upload_idp_for_org_with_http_info(
        &self,
        public_id: String,
        idp_file: Vec<u8>,
    ) -> Result<ResponseContent<crate::datadogV1::model::IdpResponse>, Error<UploadIdPForOrgError>>
    {
        let local_configuration = &self.config;
        let operation_id = "v1.upload_idp_for_org";

        let local_client = &self.client;

        let local_uri_str = format!(
            "{}/api/v1/org/{public_id}/idp_metadata",
            local_configuration.get_operation_host(operation_id),
            public_id = urlencode(public_id)
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

        // build form parameters
        let mut local_form = reqwest::multipart::Form::new();
        local_form = local_form.part(
            "idp_file",
            reqwest::multipart::Part::bytes(idp_file).file_name("idp_file"),
        );
        local_req_builder = local_req_builder.multipart(local_form);

        let local_req = local_req_builder.build()?;
        let local_resp = local_client.execute(local_req).await?;

        let local_status = local_resp.status();
        let local_content = local_resp.text().await?;

        if !local_status.is_client_error() && !local_status.is_server_error() {
            match serde_json::from_str::<crate::datadogV1::model::IdpResponse>(&local_content) {
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
            let local_entity: Option<UploadIdPForOrgError> =
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
