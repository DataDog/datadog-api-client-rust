// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

use crate::datadog::*;
use reqwest;
use serde::{Deserialize, Serialize};

/// CreateChildOrgParams is a struct for passing parameters to the method [`CreateChildOrg`]
#[derive(Clone, Debug, Default)]
pub struct CreateChildOrgParams {
    /// Organization object that needs to be created
    pub body: crate::datadogV1::model::OrganizationCreateBody,
}

/// DowngradeOrgParams is a struct for passing parameters to the method [`DowngradeOrg`]
#[derive(Clone, Debug, Default)]
pub struct DowngradeOrgParams {
    /// The `public_id` of the organization you are operating within.
    pub public_id: String,
}

/// GetOrgParams is a struct for passing parameters to the method [`GetOrg`]
#[derive(Clone, Debug, Default)]
pub struct GetOrgParams {
    /// The `public_id` of the organization you are operating within.
    pub public_id: String,
}

/// UpdateOrgParams is a struct for passing parameters to the method [`UpdateOrg`]
#[derive(Clone, Debug, Default)]
pub struct UpdateOrgParams {
    /// The `public_id` of the organization you are operating within.
    pub public_id: String,
    pub body: crate::datadogV1::model::Organization,
}

/// UploadIdPForOrgParams is a struct for passing parameters to the method [`UploadIdPForOrg`]
#[derive(Clone, Debug, Default)]
pub struct UploadIdPForOrgParams {
    /// The `public_id` of the organization you are operating with
    pub public_id: String,
    /// The path to the XML metadata file you wish to upload.
    pub idp_file: Vec<u8>,
}

/// CreateChildOrgError is a struct for typed errors of method [`CreateChildOrg`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CreateChildOrgError {
    Status400(Option<crate::datadogV1::model::APIErrorResponse>),
    Status403(Option<crate::datadogV1::model::APIErrorResponse>),
    Status429(Option<crate::datadogV1::model::APIErrorResponse>),
    UnknownValue(serde_json::Value),
}

/// DowngradeOrgError is a struct for typed errors of method [`DowngradeOrg`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DowngradeOrgError {
    Status400(Option<crate::datadogV1::model::APIErrorResponse>),
    Status403(Option<crate::datadogV1::model::APIErrorResponse>),
    Status429(Option<crate::datadogV1::model::APIErrorResponse>),
    UnknownValue(serde_json::Value),
}

/// GetOrgError is a struct for typed errors of method [`GetOrg`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetOrgError {
    Status400(Option<crate::datadogV1::model::APIErrorResponse>),
    Status403(Option<crate::datadogV1::model::APIErrorResponse>),
    Status429(Option<crate::datadogV1::model::APIErrorResponse>),
    UnknownValue(serde_json::Value),
}

/// ListOrgsError is a struct for typed errors of method [`ListOrgs`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ListOrgsError {
    Status403(Option<crate::datadogV1::model::APIErrorResponse>),
    Status429(Option<crate::datadogV1::model::APIErrorResponse>),
    UnknownValue(serde_json::Value),
}

/// UpdateOrgError is a struct for typed errors of method [`UpdateOrg`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum UpdateOrgError {
    Status400(Option<crate::datadogV1::model::APIErrorResponse>),
    Status403(Option<crate::datadogV1::model::APIErrorResponse>),
    Status429(Option<crate::datadogV1::model::APIErrorResponse>),
    UnknownValue(serde_json::Value),
}

/// UploadIdPForOrgError is a struct for typed errors of method [`UploadIdPForOrg`]
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

    /// Create a child organization.
    ///
    /// This endpoint requires the
    /// [multi-organization account](https://docs.datadoghq.com/account_management/multi_organization/)
    /// feature and must be enabled by
    /// [contacting support](https://docs.datadoghq.com/help/).
    ///
    /// Once a new child organization is created, you can interact with it
    /// by using the `org.public_id`, `api_key.key`, and
    /// `application_key.hash` provided in the response.
    pub async fn create_child_org(
        &self,
        params: CreateChildOrgParams,
    ) -> Result<Option<crate::datadogV1::model::OrganizationCreateResponse>, Error<CreateChildOrgError>> {
        match self.create_child_org_with_http_info(params).await {
            Ok(response_content) => Ok(response_content.entity),
            Err(err) => Err(err),
        }
    }

    /// Create a child organization.
    ///
    /// This endpoint requires the
    /// [multi-organization account](https://docs.datadoghq.com/account_management/multi_organization/)
    /// feature and must be enabled by
    /// [contacting support](https://docs.datadoghq.com/help/).
    ///
    /// Once a new child organization is created, you can interact with it
    /// by using the `org.public_id`, `api_key.key`, and
    /// `application_key.hash` provided in the response.
    pub async fn create_child_org_with_http_info(
        &self,
        params: CreateChildOrgParams,
    ) -> Result<ResponseContent<crate::datadogV1::model::OrganizationCreateResponse>, Error<CreateChildOrgError>> {
        let local_configuration = &self.config;

        // unbox the parameters
        let body = params.body;

        let local_client = &local_configuration.client;

        let local_uri_str = format!("{}/api/v1/org", local_configuration.base_path);
        let mut local_req_builder = local_client.request(reqwest::Method::POST, local_uri_str.as_str());

        if let Some(ref local_user_agent) = local_configuration.user_agent {
            local_req_builder = local_req_builder.header(reqwest::header::USER_AGENT, local_user_agent.clone());
        }

        if let Some(ref local_apikey) = local_configuration.api_key_auth {
            local_req_builder = local_req_builder.header("DD-API-KEY", local_apikey);
        };
        if let Some(ref local_apikey) = local_configuration.app_key_auth {
            local_req_builder = local_req_builder.header("DD-APPLICATION-KEY", local_apikey);
        };

        // body params
        local_req_builder = local_req_builder.json(&body);

        let local_req = local_req_builder.build()?;
        let local_resp = local_client.execute(local_req).await?;

        let local_status = local_resp.status();
        let local_content = local_resp.text().await?;

        if !local_status.is_client_error() && !local_status.is_server_error() {
            let local_entity: Option<crate::datadogV1::model::OrganizationCreateResponse> =
                serde_json::from_str(&local_content).ok();
            Ok(ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            })
        } else {
            let local_entity: Option<CreateChildOrgError> = serde_json::from_str(&local_content).ok();
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
        params: DowngradeOrgParams,
    ) -> Result<Option<crate::datadogV1::model::OrgDowngradedResponse>, Error<DowngradeOrgError>> {
        match self.downgrade_org_with_http_info(params).await {
            Ok(response_content) => Ok(response_content.entity),
            Err(err) => Err(err),
        }
    }

    /// Only available for MSP customers. Removes a child organization from the hierarchy of the master organization and places the child organization on a 30-day trial.
    pub async fn downgrade_org_with_http_info(
        &self,
        params: DowngradeOrgParams,
    ) -> Result<ResponseContent<crate::datadogV1::model::OrgDowngradedResponse>, Error<DowngradeOrgError>> {
        let local_configuration = &self.config;

        // unbox the parameters
        let public_id = params.public_id;

        let local_client = &local_configuration.client;

        let local_uri_str = format!(
            "{}/api/v1/org/{public_id}/downgrade",
            local_configuration.base_path,
            public_id = urlencode(public_id)
        );
        let mut local_req_builder = local_client.request(reqwest::Method::POST, local_uri_str.as_str());

        if let Some(ref local_user_agent) = local_configuration.user_agent {
            local_req_builder = local_req_builder.header(reqwest::header::USER_AGENT, local_user_agent.clone());
        }

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
            let local_entity: Option<crate::datadogV1::model::OrgDowngradedResponse> =
                serde_json::from_str(&local_content).ok();
            Ok(ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            })
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
        params: GetOrgParams,
    ) -> Result<Option<crate::datadogV1::model::OrganizationResponse>, Error<GetOrgError>> {
        match self.get_org_with_http_info(params).await {
            Ok(response_content) => Ok(response_content.entity),
            Err(err) => Err(err),
        }
    }

    /// Get organization information.
    pub async fn get_org_with_http_info(
        &self,
        params: GetOrgParams,
    ) -> Result<ResponseContent<crate::datadogV1::model::OrganizationResponse>, Error<GetOrgError>> {
        let local_configuration = &self.config;

        // unbox the parameters
        let public_id = params.public_id;

        let local_client = &local_configuration.client;

        let local_uri_str = format!(
            "{}/api/v1/org/{public_id}",
            local_configuration.base_path,
            public_id = urlencode(public_id)
        );
        let mut local_req_builder = local_client.request(reqwest::Method::GET, local_uri_str.as_str());

        if let Some(ref local_user_agent) = local_configuration.user_agent {
            local_req_builder = local_req_builder.header(reqwest::header::USER_AGENT, local_user_agent.clone());
        }

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
            let local_entity: Option<crate::datadogV1::model::OrganizationResponse> =
                serde_json::from_str(&local_content).ok();
            Ok(ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            })
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
    ) -> Result<Option<crate::datadogV1::model::OrganizationListResponse>, Error<ListOrgsError>> {
        match self.list_orgs_with_http_info().await {
            Ok(response_content) => Ok(response_content.entity),
            Err(err) => Err(err),
        }
    }

    /// This endpoint returns data on your top-level organization.
    pub async fn list_orgs_with_http_info(
        &self,
    ) -> Result<ResponseContent<crate::datadogV1::model::OrganizationListResponse>, Error<ListOrgsError>> {
        let local_configuration = &self.config;

        // unbox the parameters

        let local_client = &local_configuration.client;

        let local_uri_str = format!("{}/api/v1/org", local_configuration.base_path);
        let mut local_req_builder = local_client.request(reqwest::Method::GET, local_uri_str.as_str());

        if let Some(ref local_user_agent) = local_configuration.user_agent {
            local_req_builder = local_req_builder.header(reqwest::header::USER_AGENT, local_user_agent.clone());
        }

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
            let local_entity: Option<crate::datadogV1::model::OrganizationListResponse> =
                serde_json::from_str(&local_content).ok();
            Ok(ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            })
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
        params: UpdateOrgParams,
    ) -> Result<Option<crate::datadogV1::model::OrganizationResponse>, Error<UpdateOrgError>> {
        match self.update_org_with_http_info(params).await {
            Ok(response_content) => Ok(response_content.entity),
            Err(err) => Err(err),
        }
    }

    /// Update your organization.
    pub async fn update_org_with_http_info(
        &self,
        params: UpdateOrgParams,
    ) -> Result<ResponseContent<crate::datadogV1::model::OrganizationResponse>, Error<UpdateOrgError>> {
        let local_configuration = &self.config;

        // unbox the parameters
        let public_id = params.public_id;
        let body = params.body;

        let local_client = &local_configuration.client;

        let local_uri_str = format!(
            "{}/api/v1/org/{public_id}",
            local_configuration.base_path,
            public_id = urlencode(public_id)
        );
        let mut local_req_builder = local_client.request(reqwest::Method::PUT, local_uri_str.as_str());

        if let Some(ref local_user_agent) = local_configuration.user_agent {
            local_req_builder = local_req_builder.header(reqwest::header::USER_AGENT, local_user_agent.clone());
        }

        if let Some(ref local_apikey) = local_configuration.api_key_auth {
            local_req_builder = local_req_builder.header("DD-API-KEY", local_apikey);
        };
        if let Some(ref local_apikey) = local_configuration.app_key_auth {
            local_req_builder = local_req_builder.header("DD-APPLICATION-KEY", local_apikey);
        };

        // body params
        local_req_builder = local_req_builder.json(&body);

        let local_req = local_req_builder.build()?;
        let local_resp = local_client.execute(local_req).await?;

        let local_status = local_resp.status();
        let local_content = local_resp.text().await?;

        if !local_status.is_client_error() && !local_status.is_server_error() {
            let local_entity: Option<crate::datadogV1::model::OrganizationResponse> =
                serde_json::from_str(&local_content).ok();
            Ok(ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            })
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
    pub async fn upload_id_p_for_org(
        &self,
        params: UploadIdPForOrgParams,
    ) -> Result<Option<crate::datadogV1::model::IdpResponse>, Error<UploadIdPForOrgError>> {
        match self.upload_id_p_for_org_with_http_info(params).await {
            Ok(response_content) => Ok(response_content.entity),
            Err(err) => Err(err),
        }
    }

    /// There are a couple of options for updating the Identity Provider (IdP)
    /// metadata from your SAML IdP.
    ///
    /// * **Multipart Form-Data**: Post the IdP metadata file using a form post.
    ///
    /// * **XML Body:** Post the IdP metadata file as the body of the request.
    pub async fn upload_id_p_for_org_with_http_info(
        &self,
        params: UploadIdPForOrgParams,
    ) -> Result<ResponseContent<crate::datadogV1::model::IdpResponse>, Error<UploadIdPForOrgError>> {
        let local_configuration = &self.config;

        // unbox the parameters
        let public_id = params.public_id;
        let idp_file = params.idp_file;

        let local_client = &local_configuration.client;

        let local_uri_str = format!(
            "{}/api/v1/org/{public_id}/idp_metadata",
            local_configuration.base_path,
            public_id = urlencode(public_id)
        );
        let mut local_req_builder = local_client.request(reqwest::Method::POST, local_uri_str.as_str());

        if let Some(ref local_user_agent) = local_configuration.user_agent {
            local_req_builder = local_req_builder.header(reqwest::header::USER_AGENT, local_user_agent.clone());
        }

        if let Some(ref local_apikey) = local_configuration.api_key_auth {
            local_req_builder = local_req_builder.header("DD-API-KEY", local_apikey);
        };
        if let Some(ref local_apikey) = local_configuration.app_key_auth {
            local_req_builder = local_req_builder.header("DD-APPLICATION-KEY", local_apikey);
        };

        // body params
        local_req_builder = local_req_builder.json(&body);

        let local_req = local_req_builder.build()?;
        let local_resp = local_client.execute(local_req).await?;

        let local_status = local_resp.status();
        let local_content = local_resp.text().await?;

        if !local_status.is_client_error() && !local_status.is_server_error() {
            let local_entity: Option<crate::datadogV1::model::IdpResponse> = serde_json::from_str(&local_content).ok();
            Ok(ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            })
        } else {
            let local_entity: Option<UploadIdPForOrgError> = serde_json::from_str(&local_content).ok();
            let local_error = ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(Error::ResponseError(local_error))
        }
    }
}
