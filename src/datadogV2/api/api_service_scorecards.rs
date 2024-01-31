// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use crate::datadog::*;
use reqwest;
use serde::{Deserialize, Serialize};

/// ListScorecardOutcomesOptionalParams is a struct for passing parameters to the method [`ServiceScorecardsAPI::list_scorecard_outcomes`]
#[derive(Clone, Debug)]
pub struct ListScorecardOutcomesOptionalParams {
    /// Size for a given page. The maximum allowed value is 100.
    pub page_size: Option<i64>,
    /// Specific offset to use as the beginning of the returned page.
    pub page_offset: Option<i64>,
    /// Include related rule details in the response.
    pub include: Option<String>,
    /// Return only specified values in the outcome attributes.
    pub fields_outcome: Option<String>,
    /// Return only specified values in the included rule details.
    pub fields_rule: Option<String>,
    /// Filter the outcomes on a specific service name.
    pub filter_outcome_service_name: Option<String>,
    /// Filter the outcomes by a specific state.
    pub filter_outcome_state: Option<String>,
    /// Filter outcomes on whether a rule is enabled/disabled.
    pub filter_rule_enabled: Option<bool>,
    /// Filter outcomes based on rule ID.
    pub filter_rule_id: Option<String>,
    /// Filter outcomes based on rule name.
    pub filter_rule_name: Option<String>,
}

/// ListScorecardRulesOptionalParams is a struct for passing parameters to the method [`ServiceScorecardsAPI::list_scorecard_rules`]
#[derive(Clone, Debug)]
pub struct ListScorecardRulesOptionalParams {
    /// Size for a given page. The maximum allowed value is 100.
    pub page_size: Option<i64>,
    /// Specific offset to use as the beginning of the returned page.
    pub page_offset: Option<i64>,
    /// Include related scorecard details in the response.
    pub include: Option<String>,
    /// Filter the rules on a rule ID.
    pub filter_rule_id: Option<String>,
    /// Filter for enabled rules only.
    pub filter_rule_enabled: Option<bool>,
    /// Filter for custom rules only.
    pub filter_rule_custom: Option<bool>,
    /// Filter rules on the rule name.
    pub filter_rule_name: Option<String>,
    /// Filter rules on the rule description.
    pub filter_rule_description: Option<String>,
    /// Return only specific fields in the response for rule attributes.
    pub fields_rule: Option<String>,
    /// Return only specific fields in the included response for scorecard attributes.
    pub fields_scorecard: Option<String>,
}

/// CreateScorecardOutcomesBatchError is a struct for typed errors of method [`ServiceScorecardsAPI::create_scorecard_outcomes_batch`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CreateScorecardOutcomesBatchError {
    Status400(Option<crate::datadogV2::model::APIErrorResponse>),
    Status403(Option<crate::datadogV2::model::APIErrorResponse>),
    Status429(Option<crate::datadogV2::model::APIErrorResponse>),
    UnknownValue(serde_json::Value),
}

/// CreateScorecardRuleError is a struct for typed errors of method [`ServiceScorecardsAPI::create_scorecard_rule`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CreateScorecardRuleError {
    Status400(Option<crate::datadogV2::model::APIErrorResponse>),
    Status403(Option<crate::datadogV2::model::APIErrorResponse>),
    Status429(Option<crate::datadogV2::model::APIErrorResponse>),
    UnknownValue(serde_json::Value),
}

/// DeleteScorecardRuleError is a struct for typed errors of method [`ServiceScorecardsAPI::delete_scorecard_rule`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DeleteScorecardRuleError {
    Status400(Option<crate::datadogV2::model::APIErrorResponse>),
    Status403(Option<crate::datadogV2::model::APIErrorResponse>),
    Status404(Option<crate::datadogV2::model::APIErrorResponse>),
    Status429(Option<crate::datadogV2::model::APIErrorResponse>),
    UnknownValue(serde_json::Value),
}

/// ListScorecardOutcomesError is a struct for typed errors of method [`ServiceScorecardsAPI::list_scorecard_outcomes`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ListScorecardOutcomesError {
    Status400(Option<crate::datadogV2::model::APIErrorResponse>),
    Status403(Option<crate::datadogV2::model::APIErrorResponse>),
    Status429(Option<crate::datadogV2::model::APIErrorResponse>),
    UnknownValue(serde_json::Value),
}

/// ListScorecardRulesError is a struct for typed errors of method [`ServiceScorecardsAPI::list_scorecard_rules`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ListScorecardRulesError {
    Status400(Option<crate::datadogV2::model::APIErrorResponse>),
    Status403(Option<crate::datadogV2::model::APIErrorResponse>),
    Status429(Option<crate::datadogV2::model::APIErrorResponse>),
    UnknownValue(serde_json::Value),
}

#[derive(Debug, Clone)]
pub struct ServiceScorecardsAPI {
    config: configuration::Configuration,
}

impl Default for ServiceScorecardsAPI {
    fn default() -> Self {
        Self {
            config: configuration::Configuration::new(),
        }
    }
}

impl ServiceScorecardsAPI {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn with_config(config: configuration::Configuration) -> Self {
        Self { config }
    }

    /// Sets multiple service-rule outcomes in a single batched request.
    pub async fn create_scorecard_outcomes_batch(
        &self,
        body: crate::datadogV2::model::OutcomesBatchRequest,
    ) -> Result<
        Option<crate::datadogV2::model::OutcomesBatchResponse>,
        Error<CreateScorecardOutcomesBatchError>,
    > {
        match self
            .create_scorecard_outcomes_batch_with_http_info(body)
            .await
        {
            Ok(response_content) => Ok(response_content.entity),
            Err(err) => Err(err),
        }
    }

    /// Sets multiple service-rule outcomes in a single batched request.
    pub async fn create_scorecard_outcomes_batch_with_http_info(
        &self,
        body: crate::datadogV2::model::OutcomesBatchRequest,
    ) -> Result<
        ResponseContent<crate::datadogV2::model::OutcomesBatchResponse>,
        Error<CreateScorecardOutcomesBatchError>,
    > {
        let local_configuration = &self.config;

        let local_client = &local_configuration.client;

        let local_uri_str = format!(
            "{}/api/v2/scorecard/outcomes/batch",
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
            let local_entity: Option<crate::datadogV2::model::OutcomesBatchResponse> =
                serde_json::from_str(&local_content).ok();
            Ok(ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            })
        } else {
            let local_entity: Option<CreateScorecardOutcomesBatchError> =
                serde_json::from_str(&local_content).ok();
            let local_error = ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(Error::ResponseError(local_error))
        }
    }

    /// Creates a new rule.
    pub async fn create_scorecard_rule(
        &self,
        body: crate::datadogV2::model::CreateRuleRequest,
    ) -> Result<Option<crate::datadogV2::model::CreateRuleResponse>, Error<CreateScorecardRuleError>>
    {
        match self.create_scorecard_rule_with_http_info(body).await {
            Ok(response_content) => Ok(response_content.entity),
            Err(err) => Err(err),
        }
    }

    /// Creates a new rule.
    pub async fn create_scorecard_rule_with_http_info(
        &self,
        body: crate::datadogV2::model::CreateRuleRequest,
    ) -> Result<
        ResponseContent<crate::datadogV2::model::CreateRuleResponse>,
        Error<CreateScorecardRuleError>,
    > {
        let local_configuration = &self.config;

        let local_client = &local_configuration.client;

        let local_uri_str = format!("{}/api/v2/scorecard/rules", local_configuration.base_path);
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
            let local_entity: Option<crate::datadogV2::model::CreateRuleResponse> =
                serde_json::from_str(&local_content).ok();
            Ok(ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            })
        } else {
            let local_entity: Option<CreateScorecardRuleError> =
                serde_json::from_str(&local_content).ok();
            let local_error = ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(Error::ResponseError(local_error))
        }
    }

    /// Deletes a single rule.
    pub async fn delete_scorecard_rule(
        &self,
        rule_id: String,
    ) -> Result<Option<()>, Error<DeleteScorecardRuleError>> {
        match self.delete_scorecard_rule_with_http_info(rule_id).await {
            Ok(response_content) => Ok(response_content.entity),
            Err(err) => Err(err),
        }
    }

    /// Deletes a single rule.
    pub async fn delete_scorecard_rule_with_http_info(
        &self,
        rule_id: String,
    ) -> Result<ResponseContent<()>, Error<DeleteScorecardRuleError>> {
        let local_configuration = &self.config;

        let local_client = &local_configuration.client;

        let local_uri_str = format!(
            "{}/api/v2/scorecard/rules/{rule_id}",
            local_configuration.base_path,
            rule_id = urlencode(rule_id)
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
            let local_entity: Option<DeleteScorecardRuleError> =
                serde_json::from_str(&local_content).ok();
            let local_error = ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(Error::ResponseError(local_error))
        }
    }

    /// Fetches all rule outcomes.
    pub async fn list_scorecard_outcomes(
        &self,
        params: ListScorecardOutcomesOptionalParams,
    ) -> Result<Option<crate::datadogV2::model::OutcomesResponse>, Error<ListScorecardOutcomesError>>
    {
        match self.list_scorecard_outcomes_with_http_info(params).await {
            Ok(response_content) => Ok(response_content.entity),
            Err(err) => Err(err),
        }
    }

    /// Fetches all rule outcomes.
    pub async fn list_scorecard_outcomes_with_http_info(
        &self,
        params: ListScorecardOutcomesOptionalParams,
    ) -> Result<
        ResponseContent<crate::datadogV2::model::OutcomesResponse>,
        Error<ListScorecardOutcomesError>,
    > {
        let local_configuration = &self.config;

        // unbox and build optional parameters
        let page_size = params.page_size;
        let page_offset = params.page_offset;
        let include = params.include;
        let fields_outcome = params.fields_outcome;
        let fields_rule = params.fields_rule;
        let filter_outcome_service_name = params.filter_outcome_service_name;
        let filter_outcome_state = params.filter_outcome_state;
        let filter_rule_enabled = params.filter_rule_enabled;
        let filter_rule_id = params.filter_rule_id;
        let filter_rule_name = params.filter_rule_name;

        let local_client = &local_configuration.client;

        let local_uri_str = format!(
            "{}/api/v2/scorecard/outcomes",
            local_configuration.base_path
        );
        let mut local_req_builder =
            local_client.request(reqwest::Method::GET, local_uri_str.as_str());

        if let Some(ref local_query_param) = page_size {
            local_req_builder =
                local_req_builder.query(&[("page[size]", &local_query_param.to_string())]);
        };
        if let Some(ref local_query_param) = page_offset {
            local_req_builder =
                local_req_builder.query(&[("page[offset]", &local_query_param.to_string())]);
        };
        if let Some(ref local_query_param) = include {
            local_req_builder =
                local_req_builder.query(&[("include", &local_query_param.to_string())]);
        };
        if let Some(ref local_query_param) = fields_outcome {
            local_req_builder =
                local_req_builder.query(&[("fields[outcome]", &local_query_param.to_string())]);
        };
        if let Some(ref local_query_param) = fields_rule {
            local_req_builder =
                local_req_builder.query(&[("fields[rule]", &local_query_param.to_string())]);
        };
        if let Some(ref local_query_param) = filter_outcome_service_name {
            local_req_builder = local_req_builder.query(&[(
                "filter[outcome][service_name]",
                &local_query_param.to_string(),
            )]);
        };
        if let Some(ref local_query_param) = filter_outcome_state {
            local_req_builder = local_req_builder
                .query(&[("filter[outcome][state]", &local_query_param.to_string())]);
        };
        if let Some(ref local_query_param) = filter_rule_enabled {
            local_req_builder = local_req_builder
                .query(&[("filter[rule][enabled]", &local_query_param.to_string())]);
        };
        if let Some(ref local_query_param) = filter_rule_id {
            local_req_builder =
                local_req_builder.query(&[("filter[rule][id]", &local_query_param.to_string())]);
        };
        if let Some(ref local_query_param) = filter_rule_name {
            local_req_builder =
                local_req_builder.query(&[("filter[rule][name]", &local_query_param.to_string())]);
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
            let local_entity: Option<crate::datadogV2::model::OutcomesResponse> =
                serde_json::from_str(&local_content).ok();
            Ok(ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            })
        } else {
            let local_entity: Option<ListScorecardOutcomesError> =
                serde_json::from_str(&local_content).ok();
            let local_error = ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(Error::ResponseError(local_error))
        }
    }

    /// Fetch all rules.
    pub async fn list_scorecard_rules(
        &self,
        params: ListScorecardRulesOptionalParams,
    ) -> Result<Option<crate::datadogV2::model::ListRulesResponse>, Error<ListScorecardRulesError>>
    {
        match self.list_scorecard_rules_with_http_info(params).await {
            Ok(response_content) => Ok(response_content.entity),
            Err(err) => Err(err),
        }
    }

    /// Fetch all rules.
    pub async fn list_scorecard_rules_with_http_info(
        &self,
        params: ListScorecardRulesOptionalParams,
    ) -> Result<
        ResponseContent<crate::datadogV2::model::ListRulesResponse>,
        Error<ListScorecardRulesError>,
    > {
        let local_configuration = &self.config;

        // unbox and build optional parameters
        let page_size = params.page_size;
        let page_offset = params.page_offset;
        let include = params.include;
        let filter_rule_id = params.filter_rule_id;
        let filter_rule_enabled = params.filter_rule_enabled;
        let filter_rule_custom = params.filter_rule_custom;
        let filter_rule_name = params.filter_rule_name;
        let filter_rule_description = params.filter_rule_description;
        let fields_rule = params.fields_rule;
        let fields_scorecard = params.fields_scorecard;

        let local_client = &local_configuration.client;

        let local_uri_str = format!("{}/api/v2/scorecard/rules", local_configuration.base_path);
        let mut local_req_builder =
            local_client.request(reqwest::Method::GET, local_uri_str.as_str());

        if let Some(ref local_query_param) = page_size {
            local_req_builder =
                local_req_builder.query(&[("page[size]", &local_query_param.to_string())]);
        };
        if let Some(ref local_query_param) = page_offset {
            local_req_builder =
                local_req_builder.query(&[("page[offset]", &local_query_param.to_string())]);
        };
        if let Some(ref local_query_param) = include {
            local_req_builder =
                local_req_builder.query(&[("include", &local_query_param.to_string())]);
        };
        if let Some(ref local_query_param) = filter_rule_id {
            local_req_builder =
                local_req_builder.query(&[("filter[rule][id]", &local_query_param.to_string())]);
        };
        if let Some(ref local_query_param) = filter_rule_enabled {
            local_req_builder = local_req_builder
                .query(&[("filter[rule][enabled]", &local_query_param.to_string())]);
        };
        if let Some(ref local_query_param) = filter_rule_custom {
            local_req_builder = local_req_builder
                .query(&[("filter[rule][custom]", &local_query_param.to_string())]);
        };
        if let Some(ref local_query_param) = filter_rule_name {
            local_req_builder =
                local_req_builder.query(&[("filter[rule][name]", &local_query_param.to_string())]);
        };
        if let Some(ref local_query_param) = filter_rule_description {
            local_req_builder = local_req_builder
                .query(&[("filter[rule][description]", &local_query_param.to_string())]);
        };
        if let Some(ref local_query_param) = fields_rule {
            local_req_builder =
                local_req_builder.query(&[("fields[rule]", &local_query_param.to_string())]);
        };
        if let Some(ref local_query_param) = fields_scorecard {
            local_req_builder =
                local_req_builder.query(&[("fields[scorecard]", &local_query_param.to_string())]);
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
            let local_entity: Option<crate::datadogV2::model::ListRulesResponse> =
                serde_json::from_str(&local_content).ok();
            Ok(ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            })
        } else {
            let local_entity: Option<ListScorecardRulesError> =
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
