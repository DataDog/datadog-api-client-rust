// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use crate::datadog;
use flate2::{
    write::{GzEncoder, ZlibEncoder},
    Compression,
};
use log::warn;
use reqwest::header::{HeaderMap, HeaderValue};
use serde::{Deserialize, Serialize};
use std::io::Write;

/// DeleteAiGatewayPolicyOptionalParams is a struct for passing parameters to the method [`AIGatewayAPI::delete_ai_gateway_policy`]
#[non_exhaustive]
#[derive(Clone, Default, Debug)]
pub struct DeleteAiGatewayPolicyOptionalParams {
    /// Filter or scope policies to a specific user. Omit for organization-wide policies.
    pub user_id: Option<uuid::Uuid>,
}

impl DeleteAiGatewayPolicyOptionalParams {
    /// Filter or scope policies to a specific user. Omit for organization-wide policies.
    pub fn user_id(mut self, value: uuid::Uuid) -> Self {
        self.user_id = Some(value);
        self
    }
}

/// DeleteAiGatewayProviderOptionalParams is a struct for passing parameters to the method [`AIGatewayAPI::delete_ai_gateway_provider`]
#[non_exhaustive]
#[derive(Clone, Default, Debug)]
pub struct DeleteAiGatewayProviderOptionalParams {
    /// Apply the routing rule changes returned by provider_in_use. Defaults to false.
    pub force: Option<bool>,
}

impl DeleteAiGatewayProviderOptionalParams {
    /// Apply the routing rule changes returned by provider_in_use. Defaults to false.
    pub fn force(mut self, value: bool) -> Self {
        self.force = Some(value);
        self
    }
}

/// DeleteAiGatewayRoutingRuleOptionalParams is a struct for passing parameters to the method [`AIGatewayAPI::delete_ai_gateway_routing_rule`]
#[non_exhaustive]
#[derive(Clone, Default, Debug)]
pub struct DeleteAiGatewayRoutingRuleOptionalParams {
    /// Delete referencing tiers and recursively delete parent rules left without tiers.
    pub force: Option<bool>,
}

impl DeleteAiGatewayRoutingRuleOptionalParams {
    /// Delete referencing tiers and recursively delete parent rules left without tiers.
    pub fn force(mut self, value: bool) -> Self {
        self.force = Some(value);
        self
    }
}

/// GetAiGatewayBudgetInstancesSummaryOptionalParams is a struct for passing parameters to the method [`AIGatewayAPI::get_ai_gateway_budget_instances_summary`]
#[non_exhaustive]
#[derive(Clone, Default, Debug)]
pub struct GetAiGatewayBudgetInstancesSummaryOptionalParams {
    /// RFC3339 timestamp inside the desired accounting window. Omit to use the current window.
    pub window: Option<chrono::DateTime<chrono::Utc>>,
    /// Keep only instance counters whose split dimension equals a value, written as `<dim>=<value>`, such as `split_dim=user_id=<uuid>&split_dim=tag:env=prod`. Dimensions use the `split_dims` names `user_id`, `team_id`, `model`, `provider`, and `tag:<key>`. Repeat the parameter to filter on several dimensions; a counter must match all of them. Each dimension may appear once and must be a per-value dimension of the budget.
    pub split_dim: Option<Vec<String>>,
}

impl GetAiGatewayBudgetInstancesSummaryOptionalParams {
    /// RFC3339 timestamp inside the desired accounting window. Omit to use the current window.
    pub fn window(mut self, value: chrono::DateTime<chrono::Utc>) -> Self {
        self.window = Some(value);
        self
    }
    /// Keep only instance counters whose split dimension equals a value, written as `<dim>=<value>`, such as `split_dim=user_id=<uuid>&split_dim=tag:env=prod`. Dimensions use the `split_dims` names `user_id`, `team_id`, `model`, `provider`, and `tag:<key>`. Repeat the parameter to filter on several dimensions; a counter must match all of them. Each dimension may appear once and must be a per-value dimension of the budget.
    pub fn split_dim(mut self, value: Vec<String>) -> Self {
        self.split_dim = Some(value);
        self
    }
}

/// GetAiGatewayPolicyOptionalParams is a struct for passing parameters to the method [`AIGatewayAPI::get_ai_gateway_policy`]
#[non_exhaustive]
#[derive(Clone, Default, Debug)]
pub struct GetAiGatewayPolicyOptionalParams {
    /// Filter or scope policies to a specific user. Omit for organization-wide policies.
    pub user_id: Option<uuid::Uuid>,
}

impl GetAiGatewayPolicyOptionalParams {
    /// Filter or scope policies to a specific user. Omit for organization-wide policies.
    pub fn user_id(mut self, value: uuid::Uuid) -> Self {
        self.user_id = Some(value);
        self
    }
}

/// ListAiGatewayBudgetInstanceDimensionsOptionalParams is a struct for passing parameters to the method [`AIGatewayAPI::list_ai_gateway_budget_instance_dimensions`]
#[non_exhaustive]
#[derive(Clone, Default, Debug)]
pub struct ListAiGatewayBudgetInstanceDimensionsOptionalParams {
    /// RFC3339 timestamp inside the desired accounting window. Omit to use the current window.
    pub window: Option<chrono::DateTime<chrono::Utc>>,
    /// Keep only instance counters whose split dimension equals a value, written as `<dim>=<value>`, such as `split_dim=user_id=<uuid>&split_dim=tag:env=prod`. Dimensions use the `split_dims` names `user_id`, `team_id`, `model`, `provider`, and `tag:<key>`. Repeat the parameter to filter on several dimensions; a counter must match all of them. Each dimension may appear once and must be a per-value dimension of the budget.
    pub split_dim: Option<Vec<String>>,
    /// Return only these split dimensions, such as `dim=user_id`. Repeat the parameter for several dimensions. Omit it to return every split dimension of the budget.
    pub dim: Option<Vec<String>>,
    /// Keep only values that contain this text, case-insensitively. It matches raw values, so User and Team values match their UUIDs; search their names elsewhere and pass the IDs as `value`.
    pub query: Option<String>,
    /// Keep only these values. Repeat the parameter to check several candidates, such as User IDs found by name, and learn which of them have instance counters.
    pub value: Option<Vec<String>>,
    /// Maximum number of values to return per dimension.
    pub limit: Option<i64>,
}

impl ListAiGatewayBudgetInstanceDimensionsOptionalParams {
    /// RFC3339 timestamp inside the desired accounting window. Omit to use the current window.
    pub fn window(mut self, value: chrono::DateTime<chrono::Utc>) -> Self {
        self.window = Some(value);
        self
    }
    /// Keep only instance counters whose split dimension equals a value, written as `<dim>=<value>`, such as `split_dim=user_id=<uuid>&split_dim=tag:env=prod`. Dimensions use the `split_dims` names `user_id`, `team_id`, `model`, `provider`, and `tag:<key>`. Repeat the parameter to filter on several dimensions; a counter must match all of them. Each dimension may appear once and must be a per-value dimension of the budget.
    pub fn split_dim(mut self, value: Vec<String>) -> Self {
        self.split_dim = Some(value);
        self
    }
    /// Return only these split dimensions, such as `dim=user_id`. Repeat the parameter for several dimensions. Omit it to return every split dimension of the budget.
    pub fn dim(mut self, value: Vec<String>) -> Self {
        self.dim = Some(value);
        self
    }
    /// Keep only values that contain this text, case-insensitively. It matches raw values, so User and Team values match their UUIDs; search their names elsewhere and pass the IDs as `value`.
    pub fn query(mut self, value: String) -> Self {
        self.query = Some(value);
        self
    }
    /// Keep only these values. Repeat the parameter to check several candidates, such as User IDs found by name, and learn which of them have instance counters.
    pub fn value(mut self, value: Vec<String>) -> Self {
        self.value = Some(value);
        self
    }
    /// Maximum number of values to return per dimension.
    pub fn limit(mut self, value: i64) -> Self {
        self.limit = Some(value);
        self
    }
}

/// ListAiGatewayBudgetInstancesOptionalParams is a struct for passing parameters to the method [`AIGatewayAPI::list_ai_gateway_budget_instances`]
#[non_exhaustive]
#[derive(Clone, Default, Debug)]
pub struct ListAiGatewayBudgetInstancesOptionalParams {
    /// RFC3339 timestamp inside the desired accounting window. Omit to use the current window.
    pub window: Option<chrono::DateTime<chrono::Utc>>,
    /// Keep only instance counters whose split dimension equals a value, written as `<dim>=<value>`, such as `split_dim=user_id=<uuid>&split_dim=tag:env=prod`. Dimensions use the `split_dims` names `user_id`, `team_id`, `model`, `provider`, and `tag:<key>`. Repeat the parameter to filter on several dimensions; a counter must match all of them. Each dimension may appear once and must be a per-value dimension of the budget.
    pub split_dim: Option<Vec<String>>,
    /// Sort order for budget instance counters.
    pub sort: Option<crate::datadogV2::model::AiGatewayBudgetInstanceSort>,
    /// Maximum number of budget instance counters to return.
    pub limit: Option<i64>,
    /// Number of budget instance counters to skip.
    pub offset: Option<i64>,
}

impl ListAiGatewayBudgetInstancesOptionalParams {
    /// RFC3339 timestamp inside the desired accounting window. Omit to use the current window.
    pub fn window(mut self, value: chrono::DateTime<chrono::Utc>) -> Self {
        self.window = Some(value);
        self
    }
    /// Keep only instance counters whose split dimension equals a value, written as `<dim>=<value>`, such as `split_dim=user_id=<uuid>&split_dim=tag:env=prod`. Dimensions use the `split_dims` names `user_id`, `team_id`, `model`, `provider`, and `tag:<key>`. Repeat the parameter to filter on several dimensions; a counter must match all of them. Each dimension may appear once and must be a per-value dimension of the budget.
    pub fn split_dim(mut self, value: Vec<String>) -> Self {
        self.split_dim = Some(value);
        self
    }
    /// Sort order for budget instance counters.
    pub fn sort(mut self, value: crate::datadogV2::model::AiGatewayBudgetInstanceSort) -> Self {
        self.sort = Some(value);
        self
    }
    /// Maximum number of budget instance counters to return.
    pub fn limit(mut self, value: i64) -> Self {
        self.limit = Some(value);
        self
    }
    /// Number of budget instance counters to skip.
    pub fn offset(mut self, value: i64) -> Self {
        self.offset = Some(value);
        self
    }
}

/// ListAiGatewayPoliciesOptionalParams is a struct for passing parameters to the method [`AIGatewayAPI::list_ai_gateway_policies`]
#[non_exhaustive]
#[derive(Clone, Default, Debug)]
pub struct ListAiGatewayPoliciesOptionalParams {
    /// Filter or scope policies to a specific user. Omit for organization-wide policies.
    pub user_id: Option<uuid::Uuid>,
}

impl ListAiGatewayPoliciesOptionalParams {
    /// Filter or scope policies to a specific user. Omit for organization-wide policies.
    pub fn user_id(mut self, value: uuid::Uuid) -> Self {
        self.user_id = Some(value);
        self
    }
}

/// ListAiGatewayProviderModelsOptionalParams is a struct for passing parameters to the method [`AIGatewayAPI::list_ai_gateway_provider_models`]
#[non_exhaustive]
#[derive(Clone, Default, Debug)]
pub struct ListAiGatewayProviderModelsOptionalParams {
    /// GCP project ID used for Gemini model discovery. Required for Gemini integrations.
    pub project_id: Option<String>,
    /// Optional GCP location used for Gemini model discovery. Defaults to global.
    pub location: Option<String>,
    /// AWS region used for Bedrock model discovery. Required for Amazon Bedrock integrations.
    pub bedrock_region: Option<String>,
    /// ARM resource ID of the Azure AI Foundry resource used for model discovery. Required for Azure AI Foundry integrations.
    pub foundry_resource_id: Option<String>,
}

impl ListAiGatewayProviderModelsOptionalParams {
    /// GCP project ID used for Gemini model discovery. Required for Gemini integrations.
    pub fn project_id(mut self, value: String) -> Self {
        self.project_id = Some(value);
        self
    }
    /// Optional GCP location used for Gemini model discovery. Defaults to global.
    pub fn location(mut self, value: String) -> Self {
        self.location = Some(value);
        self
    }
    /// AWS region used for Bedrock model discovery. Required for Amazon Bedrock integrations.
    pub fn bedrock_region(mut self, value: String) -> Self {
        self.bedrock_region = Some(value);
        self
    }
    /// ARM resource ID of the Azure AI Foundry resource used for model discovery. Required for Azure AI Foundry integrations.
    pub fn foundry_resource_id(mut self, value: String) -> Self {
        self.foundry_resource_id = Some(value);
        self
    }
}

/// UpsertAiGatewayPolicyOptionalParams is a struct for passing parameters to the method [`AIGatewayAPI::upsert_ai_gateway_policy`]
#[non_exhaustive]
#[derive(Clone, Default, Debug)]
pub struct UpsertAiGatewayPolicyOptionalParams {
    /// Filter or scope policies to a specific user. Omit for organization-wide policies.
    pub user_id: Option<uuid::Uuid>,
}

impl UpsertAiGatewayPolicyOptionalParams {
    /// Filter or scope policies to a specific user. Omit for organization-wide policies.
    pub fn user_id(mut self, value: uuid::Uuid) -> Self {
        self.user_id = Some(value);
        self
    }
}

/// BulkCreateAiGatewayModelsError is a struct for typed errors of method [`AIGatewayAPI::bulk_create_ai_gateway_models`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum BulkCreateAiGatewayModelsError {
    APIErrorResponse(crate::datadogV2::model::APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// CreateAiGatewayBudgetError is a struct for typed errors of method [`AIGatewayAPI::create_ai_gateway_budget`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CreateAiGatewayBudgetError {
    APIErrorResponse(crate::datadogV2::model::APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// CreateAiGatewayDataCaptureOverrideError is a struct for typed errors of method [`AIGatewayAPI::create_ai_gateway_data_capture_override`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CreateAiGatewayDataCaptureOverrideError {
    APIErrorResponse(crate::datadogV2::model::APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// CreateAiGatewayModelError is a struct for typed errors of method [`AIGatewayAPI::create_ai_gateway_model`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CreateAiGatewayModelError {
    APIErrorResponse(crate::datadogV2::model::APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// CreateAiGatewayModelGroupError is a struct for typed errors of method [`AIGatewayAPI::create_ai_gateway_model_group`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CreateAiGatewayModelGroupError {
    APIErrorResponse(crate::datadogV2::model::APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// CreateAiGatewayProviderError is a struct for typed errors of method [`AIGatewayAPI::create_ai_gateway_provider`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CreateAiGatewayProviderError {
    APIErrorResponse(crate::datadogV2::model::APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// CreateAiGatewayRateLimitError is a struct for typed errors of method [`AIGatewayAPI::create_ai_gateway_rate_limit`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CreateAiGatewayRateLimitError {
    APIErrorResponse(crate::datadogV2::model::APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// CreateAiGatewayRoutingRuleError is a struct for typed errors of method [`AIGatewayAPI::create_ai_gateway_routing_rule`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CreateAiGatewayRoutingRuleError {
    APIErrorResponse(crate::datadogV2::model::APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// DeleteAiGatewayBudgetError is a struct for typed errors of method [`AIGatewayAPI::delete_ai_gateway_budget`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DeleteAiGatewayBudgetError {
    APIErrorResponse(crate::datadogV2::model::APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// DeleteAiGatewayDataCaptureOverrideError is a struct for typed errors of method [`AIGatewayAPI::delete_ai_gateway_data_capture_override`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DeleteAiGatewayDataCaptureOverrideError {
    APIErrorResponse(crate::datadogV2::model::APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// DeleteAiGatewayModelError is a struct for typed errors of method [`AIGatewayAPI::delete_ai_gateway_model`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DeleteAiGatewayModelError {
    APIErrorResponse(crate::datadogV2::model::APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// DeleteAiGatewayModelGroupError is a struct for typed errors of method [`AIGatewayAPI::delete_ai_gateway_model_group`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DeleteAiGatewayModelGroupError {
    APIErrorResponse(crate::datadogV2::model::APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// DeleteAiGatewayModelPricingError is a struct for typed errors of method [`AIGatewayAPI::delete_ai_gateway_model_pricing`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DeleteAiGatewayModelPricingError {
    APIErrorResponse(crate::datadogV2::model::APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// DeleteAiGatewayPolicyError is a struct for typed errors of method [`AIGatewayAPI::delete_ai_gateway_policy`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DeleteAiGatewayPolicyError {
    APIErrorResponse(crate::datadogV2::model::APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// DeleteAiGatewayProviderError is a struct for typed errors of method [`AIGatewayAPI::delete_ai_gateway_provider`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DeleteAiGatewayProviderError {
    APIErrorResponse(crate::datadogV2::model::APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// DeleteAiGatewayRateLimitError is a struct for typed errors of method [`AIGatewayAPI::delete_ai_gateway_rate_limit`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DeleteAiGatewayRateLimitError {
    APIErrorResponse(crate::datadogV2::model::APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// DeleteAiGatewayRoutingRuleError is a struct for typed errors of method [`AIGatewayAPI::delete_ai_gateway_routing_rule`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DeleteAiGatewayRoutingRuleError {
    APIErrorResponse(crate::datadogV2::model::APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// GetAiGatewayBudgetError is a struct for typed errors of method [`AIGatewayAPI::get_ai_gateway_budget`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetAiGatewayBudgetError {
    APIErrorResponse(crate::datadogV2::model::APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// GetAiGatewayBudgetCurrentPeriodError is a struct for typed errors of method [`AIGatewayAPI::get_ai_gateway_budget_current_period`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetAiGatewayBudgetCurrentPeriodError {
    APIErrorResponse(crate::datadogV2::model::APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// GetAiGatewayBudgetInstancesSummaryError is a struct for typed errors of method [`AIGatewayAPI::get_ai_gateway_budget_instances_summary`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetAiGatewayBudgetInstancesSummaryError {
    APIErrorResponse(crate::datadogV2::model::APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// GetAiGatewayDataCaptureConfigurationError is a struct for typed errors of method [`AIGatewayAPI::get_ai_gateway_data_capture_configuration`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetAiGatewayDataCaptureConfigurationError {
    APIErrorResponse(crate::datadogV2::model::APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// GetAiGatewayGovernanceError is a struct for typed errors of method [`AIGatewayAPI::get_ai_gateway_governance`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetAiGatewayGovernanceError {
    APIErrorResponse(crate::datadogV2::model::APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// GetAiGatewayModelGroupError is a struct for typed errors of method [`AIGatewayAPI::get_ai_gateway_model_group`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetAiGatewayModelGroupError {
    APIErrorResponse(crate::datadogV2::model::APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// GetAiGatewayModelPricingError is a struct for typed errors of method [`AIGatewayAPI::get_ai_gateway_model_pricing`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetAiGatewayModelPricingError {
    APIErrorResponse(crate::datadogV2::model::APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// GetAiGatewayPolicyError is a struct for typed errors of method [`AIGatewayAPI::get_ai_gateway_policy`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetAiGatewayPolicyError {
    APIErrorResponse(crate::datadogV2::model::APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// GetAiGatewayProviderError is a struct for typed errors of method [`AIGatewayAPI::get_ai_gateway_provider`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetAiGatewayProviderError {
    APIErrorResponse(crate::datadogV2::model::APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// GetAiGatewayRateLimitError is a struct for typed errors of method [`AIGatewayAPI::get_ai_gateway_rate_limit`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetAiGatewayRateLimitError {
    APIErrorResponse(crate::datadogV2::model::APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// GetAiGatewayRateLimitUsageError is a struct for typed errors of method [`AIGatewayAPI::get_ai_gateway_rate_limit_usage`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetAiGatewayRateLimitUsageError {
    APIErrorResponse(crate::datadogV2::model::APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// GetAiGatewayRoutingRuleError is a struct for typed errors of method [`AIGatewayAPI::get_ai_gateway_routing_rule`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetAiGatewayRoutingRuleError {
    APIErrorResponse(crate::datadogV2::model::APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// ListAiGatewayBudgetInstanceDimensionsError is a struct for typed errors of method [`AIGatewayAPI::list_ai_gateway_budget_instance_dimensions`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ListAiGatewayBudgetInstanceDimensionsError {
    APIErrorResponse(crate::datadogV2::model::APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// ListAiGatewayBudgetInstancesError is a struct for typed errors of method [`AIGatewayAPI::list_ai_gateway_budget_instances`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ListAiGatewayBudgetInstancesError {
    APIErrorResponse(crate::datadogV2::model::APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// ListAiGatewayBudgetsError is a struct for typed errors of method [`AIGatewayAPI::list_ai_gateway_budgets`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ListAiGatewayBudgetsError {
    APIErrorResponse(crate::datadogV2::model::APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// ListAiGatewayModelGroupsError is a struct for typed errors of method [`AIGatewayAPI::list_ai_gateway_model_groups`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ListAiGatewayModelGroupsError {
    APIErrorResponse(crate::datadogV2::model::APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// ListAiGatewayModelsError is a struct for typed errors of method [`AIGatewayAPI::list_ai_gateway_models`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ListAiGatewayModelsError {
    APIErrorResponse(crate::datadogV2::model::APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// ListAiGatewayModelsByCanonicalError is a struct for typed errors of method [`AIGatewayAPI::list_ai_gateway_models_by_canonical`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ListAiGatewayModelsByCanonicalError {
    APIErrorResponse(crate::datadogV2::model::APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// ListAiGatewayPoliciesError is a struct for typed errors of method [`AIGatewayAPI::list_ai_gateway_policies`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ListAiGatewayPoliciesError {
    APIErrorResponse(crate::datadogV2::model::APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// ListAiGatewayProviderModelsError is a struct for typed errors of method [`AIGatewayAPI::list_ai_gateway_provider_models`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ListAiGatewayProviderModelsError {
    APIErrorResponse(crate::datadogV2::model::APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// ListAiGatewayProvidersError is a struct for typed errors of method [`AIGatewayAPI::list_ai_gateway_providers`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ListAiGatewayProvidersError {
    APIErrorResponse(crate::datadogV2::model::APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// ListAiGatewayRateLimitsError is a struct for typed errors of method [`AIGatewayAPI::list_ai_gateway_rate_limits`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ListAiGatewayRateLimitsError {
    APIErrorResponse(crate::datadogV2::model::APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// ListAiGatewayRoutingRulesError is a struct for typed errors of method [`AIGatewayAPI::list_ai_gateway_routing_rules`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ListAiGatewayRoutingRulesError {
    APIErrorResponse(crate::datadogV2::model::APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// OverrideAiGatewayBudgetInstanceCapacityError is a struct for typed errors of method [`AIGatewayAPI::override_ai_gateway_budget_instance_capacity`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum OverrideAiGatewayBudgetInstanceCapacityError {
    APIErrorResponse(crate::datadogV2::model::APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// ResetAiGatewayBudgetInstanceSpendError is a struct for typed errors of method [`AIGatewayAPI::reset_ai_gateway_budget_instance_spend`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ResetAiGatewayBudgetInstanceSpendError {
    APIErrorResponse(crate::datadogV2::model::APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// UpdateAiGatewayBudgetError is a struct for typed errors of method [`AIGatewayAPI::update_ai_gateway_budget`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum UpdateAiGatewayBudgetError {
    APIErrorResponse(crate::datadogV2::model::APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// UpdateAiGatewayDataCaptureOverrideError is a struct for typed errors of method [`AIGatewayAPI::update_ai_gateway_data_capture_override`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum UpdateAiGatewayDataCaptureOverrideError {
    APIErrorResponse(crate::datadogV2::model::APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// UpdateAiGatewayGovernanceError is a struct for typed errors of method [`AIGatewayAPI::update_ai_gateway_governance`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum UpdateAiGatewayGovernanceError {
    APIErrorResponse(crate::datadogV2::model::APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// UpdateAiGatewayModelError is a struct for typed errors of method [`AIGatewayAPI::update_ai_gateway_model`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum UpdateAiGatewayModelError {
    APIErrorResponse(crate::datadogV2::model::APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// UpdateAiGatewayModelGroupError is a struct for typed errors of method [`AIGatewayAPI::update_ai_gateway_model_group`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum UpdateAiGatewayModelGroupError {
    APIErrorResponse(crate::datadogV2::model::APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// UpdateAiGatewayModelPricingError is a struct for typed errors of method [`AIGatewayAPI::update_ai_gateway_model_pricing`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum UpdateAiGatewayModelPricingError {
    APIErrorResponse(crate::datadogV2::model::APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// UpdateAiGatewayProviderError is a struct for typed errors of method [`AIGatewayAPI::update_ai_gateway_provider`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum UpdateAiGatewayProviderError {
    APIErrorResponse(crate::datadogV2::model::APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// UpdateAiGatewayRateLimitError is a struct for typed errors of method [`AIGatewayAPI::update_ai_gateway_rate_limit`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum UpdateAiGatewayRateLimitError {
    APIErrorResponse(crate::datadogV2::model::APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// UpdateAiGatewayRoutingRuleError is a struct for typed errors of method [`AIGatewayAPI::update_ai_gateway_routing_rule`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum UpdateAiGatewayRoutingRuleError {
    APIErrorResponse(crate::datadogV2::model::APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// UpsertAiGatewayPolicyError is a struct for typed errors of method [`AIGatewayAPI::upsert_ai_gateway_policy`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum UpsertAiGatewayPolicyError {
    APIErrorResponse(crate::datadogV2::model::APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// AI Gateway routes your applications' LLM requests to the model providers you connect, and
/// applies your organization's routing, access, spending, rate limit, and data capture rules.
///
/// These endpoints allow you to configure:
///
/// - Providers: the model providers your organization connects, and the models and pricing each offers.
/// - Routing rules: the named routes your applications call, and the models that serve each one.
/// - Access: inbound routing policies and model groups that decide who can use which models.
/// - Budgets and rate limits: spending caps and request limits, and their current usage.
/// - Data capture and governance: what request data is recorded, and organization-wide controls.
#[derive(Debug, Clone)]
pub struct AIGatewayAPI {
    config: datadog::Configuration,
    client: reqwest_middleware::ClientWithMiddleware,
}

impl Default for AIGatewayAPI {
    fn default() -> Self {
        Self::with_config(datadog::Configuration::default())
    }
}

impl AIGatewayAPI {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn with_config(config: datadog::Configuration) -> Self {
        let reqwest_client_builder = {
            let builder = config.apply_headers(reqwest::Client::builder());
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

    /// Requires `ai_gateway_config_write` and provider Edit. Models and pricing inherit provider access.
    /// Add multiple manually managed model catalog entries to a provider in a single request. Manually managed
    /// entries are not modified by automatic catalog synchronization.
    pub async fn bulk_create_ai_gateway_models(
        &self,
        provider_id: i64,
        body: crate::datadogV2::model::AiGatewayModelsBulkCreateRequest,
    ) -> Result<
        crate::datadogV2::model::AiGatewayModelCatalogEntryListResponse,
        datadog::Error<BulkCreateAiGatewayModelsError>,
    > {
        match self
            .bulk_create_ai_gateway_models_with_http_info(provider_id, body)
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

    /// Requires `ai_gateway_config_write` and provider Edit. Models and pricing inherit provider access.
    /// Add multiple manually managed model catalog entries to a provider in a single request. Manually managed
    /// entries are not modified by automatic catalog synchronization.
    pub async fn bulk_create_ai_gateway_models_with_http_info(
        &self,
        provider_id: i64,
        body: crate::datadogV2::model::AiGatewayModelsBulkCreateRequest,
    ) -> Result<
        datadog::ResponseContent<crate::datadogV2::model::AiGatewayModelCatalogEntryListResponse>,
        datadog::Error<BulkCreateAiGatewayModelsError>,
    > {
        let local_configuration = &self.config;
        let local_operation_id = "v2.bulk_create_ai_gateway_models";
        if local_configuration.is_unstable_operation_enabled(local_operation_id) {
            warn!("Using unstable operation {local_operation_id}");
        } else {
            let local_error = datadog::UnstableOperationDisabledError {
                msg: "Operation 'v2.bulk_create_ai_gateway_models' is not enabled".to_string(),
            };
            return Err(datadog::Error::UnstableOperationDisabledError(local_error));
        }

        let local_client = &self.client;

        let local_uri_str = format!(
            "{}/api/v2/providers/{provider_id}/models/bulk",
            local_configuration.get_operation_host(local_operation_id),
            provider_id = provider_id
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
            match serde_json::from_str::<
                crate::datadogV2::model::AiGatewayModelCatalogEntryListResponse,
            >(&local_content)
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
            let local_entity: Option<BulkCreateAiGatewayModelsError> =
                serde_json::from_str(&local_content).ok();
            let local_error = datadog::ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(datadog::Error::ResponseError(local_error))
        }
    }

    /// Create a spend budget. Scope tags may include provider, model, and custom dimensions.
    /// Reserved metadata keys are rejected.
    pub async fn create_ai_gateway_budget(
        &self,
        body: crate::datadogV2::model::AiGatewayBudgetCreateRequest,
    ) -> Result<
        crate::datadogV2::model::AiGatewayBudgetResponse,
        datadog::Error<CreateAiGatewayBudgetError>,
    > {
        match self.create_ai_gateway_budget_with_http_info(body).await {
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

    /// Create a spend budget. Scope tags may include provider, model, and custom dimensions.
    /// Reserved metadata keys are rejected.
    pub async fn create_ai_gateway_budget_with_http_info(
        &self,
        body: crate::datadogV2::model::AiGatewayBudgetCreateRequest,
    ) -> Result<
        datadog::ResponseContent<crate::datadogV2::model::AiGatewayBudgetResponse>,
        datadog::Error<CreateAiGatewayBudgetError>,
    > {
        let local_configuration = &self.config;
        let local_operation_id = "v2.create_ai_gateway_budget";
        if local_configuration.is_unstable_operation_enabled(local_operation_id) {
            warn!("Using unstable operation {local_operation_id}");
        } else {
            let local_error = datadog::UnstableOperationDisabledError {
                msg: "Operation 'v2.create_ai_gateway_budget' is not enabled".to_string(),
            };
            return Err(datadog::Error::UnstableOperationDisabledError(local_error));
        }

        let local_client = &self.client;

        let local_uri_str = format!(
            "{}/api/v2/budgets",
            local_configuration.get_operation_host(local_operation_id)
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
            match serde_json::from_str::<crate::datadogV2::model::AiGatewayBudgetResponse>(
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
            let local_entity: Option<CreateAiGatewayBudgetError> =
                serde_json::from_str(&local_content).ok();
            let local_error = datadog::ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(datadog::Error::ResponseError(local_error))
        }
    }

    /// Create a scoped override with a client-generated UUID and the current `configuration_revision`. If the override can enable full capture for more traffic, set `confirm_full_capture` to `true`. Returns HTTP 409 when `configuration_revision` is stale, the UUID already exists, or the organization has 100 overrides. Missing `configuration_revision` or required full-capture confirmation returns HTTP 400. Retrying an accepted create with the same UUID returns HTTP 409. To undo a create, read the current configuration revision and delete the override using its returned override revision.
    pub async fn create_ai_gateway_data_capture_override(
        &self,
        body: crate::datadogV2::model::AiGatewayCaptureCreateRequest,
    ) -> Result<
        crate::datadogV2::model::AiGatewayCaptureOverrideResponse,
        datadog::Error<CreateAiGatewayDataCaptureOverrideError>,
    > {
        match self
            .create_ai_gateway_data_capture_override_with_http_info(body)
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

    /// Create a scoped override with a client-generated UUID and the current `configuration_revision`. If the override can enable full capture for more traffic, set `confirm_full_capture` to `true`. Returns HTTP 409 when `configuration_revision` is stale, the UUID already exists, or the organization has 100 overrides. Missing `configuration_revision` or required full-capture confirmation returns HTTP 400. Retrying an accepted create with the same UUID returns HTTP 409. To undo a create, read the current configuration revision and delete the override using its returned override revision.
    pub async fn create_ai_gateway_data_capture_override_with_http_info(
        &self,
        body: crate::datadogV2::model::AiGatewayCaptureCreateRequest,
    ) -> Result<
        datadog::ResponseContent<crate::datadogV2::model::AiGatewayCaptureOverrideResponse>,
        datadog::Error<CreateAiGatewayDataCaptureOverrideError>,
    > {
        let local_configuration = &self.config;
        let local_operation_id = "v2.create_ai_gateway_data_capture_override";
        if local_configuration.is_unstable_operation_enabled(local_operation_id) {
            warn!("Using unstable operation {local_operation_id}");
        } else {
            let local_error = datadog::UnstableOperationDisabledError {
                msg: "Operation 'v2.create_ai_gateway_data_capture_override' is not enabled"
                    .to_string(),
            };
            return Err(datadog::Error::UnstableOperationDisabledError(local_error));
        }

        let local_client = &self.client;

        let local_uri_str = format!(
            "{}/api/v2/data-capture/overrides",
            local_configuration.get_operation_host(local_operation_id)
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
            match serde_json::from_str::<crate::datadogV2::model::AiGatewayCaptureOverrideResponse>(
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
            let local_entity: Option<CreateAiGatewayDataCaptureOverrideError> =
                serde_json::from_str(&local_content).ok();
            let local_error = datadog::ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(datadog::Error::ResponseError(local_error))
        }
    }

    /// Requires `ai_gateway_config_write` and provider Edit. Models and pricing inherit provider access.
    /// Add a manually managed model catalog entry to a provider. Manually managed entries are not modified by
    /// automatic catalog synchronization.
    pub async fn create_ai_gateway_model(
        &self,
        provider_id: i64,
        body: crate::datadogV2::model::AiGatewayModelCreateRequest,
    ) -> Result<
        crate::datadogV2::model::AiGatewayModelCatalogEntryResponse,
        datadog::Error<CreateAiGatewayModelError>,
    > {
        match self
            .create_ai_gateway_model_with_http_info(provider_id, body)
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

    /// Requires `ai_gateway_config_write` and provider Edit. Models and pricing inherit provider access.
    /// Add a manually managed model catalog entry to a provider. Manually managed entries are not modified by
    /// automatic catalog synchronization.
    pub async fn create_ai_gateway_model_with_http_info(
        &self,
        provider_id: i64,
        body: crate::datadogV2::model::AiGatewayModelCreateRequest,
    ) -> Result<
        datadog::ResponseContent<crate::datadogV2::model::AiGatewayModelCatalogEntryResponse>,
        datadog::Error<CreateAiGatewayModelError>,
    > {
        let local_configuration = &self.config;
        let local_operation_id = "v2.create_ai_gateway_model";
        if local_configuration.is_unstable_operation_enabled(local_operation_id) {
            warn!("Using unstable operation {local_operation_id}");
        } else {
            let local_error = datadog::UnstableOperationDisabledError {
                msg: "Operation 'v2.create_ai_gateway_model' is not enabled".to_string(),
            };
            return Err(datadog::Error::UnstableOperationDisabledError(local_error));
        }

        let local_client = &self.client;

        let local_uri_str = format!(
            "{}/api/v2/providers/{provider_id}/models",
            local_configuration.get_operation_host(local_operation_id),
            provider_id = provider_id
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
            match serde_json::from_str::<crate::datadogV2::model::AiGatewayModelCatalogEntryResponse>(
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
            let local_entity: Option<CreateAiGatewayModelError> =
                serde_json::from_str(&local_content).ok();
            let local_error = datadog::ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(datadog::Error::ResponseError(local_error))
        }
    }

    /// Create an AI Gateway model group from a name and any combination of model catalog entries and routing rules, with optional fast-mode and reasoning limits and initial access bindings.
    pub async fn create_ai_gateway_model_group(
        &self,
        body: crate::datadogV2::model::AiGatewayModelGroupCreateRequest,
    ) -> Result<
        crate::datadogV2::model::AiGatewayModelGroupResponse,
        datadog::Error<CreateAiGatewayModelGroupError>,
    > {
        match self
            .create_ai_gateway_model_group_with_http_info(body)
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

    /// Create an AI Gateway model group from a name and any combination of model catalog entries and routing rules, with optional fast-mode and reasoning limits and initial access bindings.
    pub async fn create_ai_gateway_model_group_with_http_info(
        &self,
        body: crate::datadogV2::model::AiGatewayModelGroupCreateRequest,
    ) -> Result<
        datadog::ResponseContent<crate::datadogV2::model::AiGatewayModelGroupResponse>,
        datadog::Error<CreateAiGatewayModelGroupError>,
    > {
        let local_configuration = &self.config;
        let local_operation_id = "v2.create_ai_gateway_model_group";
        if local_configuration.is_unstable_operation_enabled(local_operation_id) {
            warn!("Using unstable operation {local_operation_id}");
        } else {
            let local_error = datadog::UnstableOperationDisabledError {
                msg: "Operation 'v2.create_ai_gateway_model_group' is not enabled".to_string(),
            };
            return Err(datadog::Error::UnstableOperationDisabledError(local_error));
        }

        let local_client = &self.client;

        let local_uri_str = format!(
            "{}/api/v2/model-groups",
            local_configuration.get_operation_host(local_operation_id)
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
            match serde_json::from_str::<crate::datadogV2::model::AiGatewayModelGroupResponse>(
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
            let local_entity: Option<CreateAiGatewayModelGroupError> =
                serde_json::from_str(&local_content).ok();
            let local_error = datadog::ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(datadog::Error::ResponseError(local_error))
        }
    }

    /// Creating a provider requires `ai_gateway_config_write`. No provider View check applies before creation.
    /// Create a provider and its initial model catalog. All discovered models are enabled; optional models are
    /// added as manually managed entries. The slug is the immutable routing identifier.
    /// Provider and initial catalog rows are saved in one transaction. Initial sharing is unavailable:
    /// supplying initial_bindings returns 503 with stage initial_permissions and outcome not_saved.
    /// A failed database commit can return 503 with stage database_commit and outcome unknown; check the
    /// provider list before retrying. Creation without initial_bindings keeps normal unrestricted defaults.
    pub async fn create_ai_gateway_provider(
        &self,
        body: crate::datadogV2::model::AiGatewayProviderCreateRequest,
    ) -> Result<
        crate::datadogV2::model::AiGatewayProviderResponse,
        datadog::Error<CreateAiGatewayProviderError>,
    > {
        match self.create_ai_gateway_provider_with_http_info(body).await {
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

    /// Creating a provider requires `ai_gateway_config_write`. No provider View check applies before creation.
    /// Create a provider and its initial model catalog. All discovered models are enabled; optional models are
    /// added as manually managed entries. The slug is the immutable routing identifier.
    /// Provider and initial catalog rows are saved in one transaction. Initial sharing is unavailable:
    /// supplying initial_bindings returns 503 with stage initial_permissions and outcome not_saved.
    /// A failed database commit can return 503 with stage database_commit and outcome unknown; check the
    /// provider list before retrying. Creation without initial_bindings keeps normal unrestricted defaults.
    pub async fn create_ai_gateway_provider_with_http_info(
        &self,
        body: crate::datadogV2::model::AiGatewayProviderCreateRequest,
    ) -> Result<
        datadog::ResponseContent<crate::datadogV2::model::AiGatewayProviderResponse>,
        datadog::Error<CreateAiGatewayProviderError>,
    > {
        let local_configuration = &self.config;
        let local_operation_id = "v2.create_ai_gateway_provider";
        if local_configuration.is_unstable_operation_enabled(local_operation_id) {
            warn!("Using unstable operation {local_operation_id}");
        } else {
            let local_error = datadog::UnstableOperationDisabledError {
                msg: "Operation 'v2.create_ai_gateway_provider' is not enabled".to_string(),
            };
            return Err(datadog::Error::UnstableOperationDisabledError(local_error));
        }

        let local_client = &self.client;

        let local_uri_str = format!(
            "{}/api/v2/providers",
            local_configuration.get_operation_host(local_operation_id)
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
            match serde_json::from_str::<crate::datadogV2::model::AiGatewayProviderResponse>(
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
            let local_entity: Option<CreateAiGatewayProviderError> =
                serde_json::from_str(&local_content).ok();
            let local_error = datadog::ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(datadog::Error::ResponseError(local_error))
        }
    }

    /// Create a rate limit. All requests matching its scope share one allowance, and every matching rate limit applies. Returns 409 when another active rate limit has the same user, Team, and tags, or when the organization already has the maximum number of rate limits.
    pub async fn create_ai_gateway_rate_limit(
        &self,
        body: crate::datadogV2::model::AiGatewayRateLimitCreateRequest,
    ) -> Result<
        crate::datadogV2::model::AiGatewayRateLimitResponse,
        datadog::Error<CreateAiGatewayRateLimitError>,
    > {
        match self.create_ai_gateway_rate_limit_with_http_info(body).await {
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

    /// Create a rate limit. All requests matching its scope share one allowance, and every matching rate limit applies. Returns 409 when another active rate limit has the same user, Team, and tags, or when the organization already has the maximum number of rate limits.
    pub async fn create_ai_gateway_rate_limit_with_http_info(
        &self,
        body: crate::datadogV2::model::AiGatewayRateLimitCreateRequest,
    ) -> Result<
        datadog::ResponseContent<crate::datadogV2::model::AiGatewayRateLimitResponse>,
        datadog::Error<CreateAiGatewayRateLimitError>,
    > {
        let local_configuration = &self.config;
        let local_operation_id = "v2.create_ai_gateway_rate_limit";
        if local_configuration.is_unstable_operation_enabled(local_operation_id) {
            warn!("Using unstable operation {local_operation_id}");
        } else {
            let local_error = datadog::UnstableOperationDisabledError {
                msg: "Operation 'v2.create_ai_gateway_rate_limit' is not enabled".to_string(),
            };
            return Err(datadog::Error::UnstableOperationDisabledError(local_error));
        }

        let local_client = &self.client;

        let local_uri_str = format!(
            "{}/api/v2/rate-limits",
            local_configuration.get_operation_host(local_operation_id)
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
            match serde_json::from_str::<crate::datadogV2::model::AiGatewayRateLimitResponse>(
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
            let local_entity: Option<CreateAiGatewayRateLimitError> =
                serde_json::from_str(&local_content).ok();
            let local_error = datadog::ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(datadog::Error::ResponseError(local_error))
        }
    }

    /// Create an organization-owned routing rule. Data capture overrides are managed independently. Each tier has an ordering, which defaults to configured. A weighted_affinity tier distributes first-choice assignments by relative target weights (1 to 10000, default 1) and accepts only distinct model targets with an explicit provider. Weights need not total 100. Configured tiers must omit weights or set them to null. In a classified tier, the decision model set in classifier picks one destination for each request by its rubric. Only the first tier can be classified. It accepts only model targets with an explicit provider: one default destination without a rubric, and 1 to 9 other destinations, each with a different rubric.
    ///
    /// **Note**: Classified routing is in Preview and may not be available for all organizations. Saving a classified tier returns 403 Forbidden unless the feature is enabled for your organization.
    pub async fn create_ai_gateway_routing_rule(
        &self,
        body: crate::datadogV2::model::AiGatewayRoutingRuleCreateRequest,
    ) -> Result<
        crate::datadogV2::model::AiGatewayRoutingRuleResponse,
        datadog::Error<CreateAiGatewayRoutingRuleError>,
    > {
        match self
            .create_ai_gateway_routing_rule_with_http_info(body)
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

    /// Create an organization-owned routing rule. Data capture overrides are managed independently. Each tier has an ordering, which defaults to configured. A weighted_affinity tier distributes first-choice assignments by relative target weights (1 to 10000, default 1) and accepts only distinct model targets with an explicit provider. Weights need not total 100. Configured tiers must omit weights or set them to null. In a classified tier, the decision model set in classifier picks one destination for each request by its rubric. Only the first tier can be classified. It accepts only model targets with an explicit provider: one default destination without a rubric, and 1 to 9 other destinations, each with a different rubric.
    ///
    /// **Note**: Classified routing is in Preview and may not be available for all organizations. Saving a classified tier returns 403 Forbidden unless the feature is enabled for your organization.
    pub async fn create_ai_gateway_routing_rule_with_http_info(
        &self,
        body: crate::datadogV2::model::AiGatewayRoutingRuleCreateRequest,
    ) -> Result<
        datadog::ResponseContent<crate::datadogV2::model::AiGatewayRoutingRuleResponse>,
        datadog::Error<CreateAiGatewayRoutingRuleError>,
    > {
        let local_configuration = &self.config;
        let local_operation_id = "v2.create_ai_gateway_routing_rule";
        if local_configuration.is_unstable_operation_enabled(local_operation_id) {
            warn!("Using unstable operation {local_operation_id}");
        } else {
            let local_error = datadog::UnstableOperationDisabledError {
                msg: "Operation 'v2.create_ai_gateway_routing_rule' is not enabled".to_string(),
            };
            return Err(datadog::Error::UnstableOperationDisabledError(local_error));
        }

        let local_client = &self.client;

        let local_uri_str = format!(
            "{}/api/v2/routing-rules",
            local_configuration.get_operation_host(local_operation_id)
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
            match serde_json::from_str::<crate::datadogV2::model::AiGatewayRoutingRuleResponse>(
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
            let local_entity: Option<CreateAiGatewayRoutingRuleError> =
                serde_json::from_str(&local_content).ok();
            let local_error = datadog::ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(datadog::Error::ResponseError(local_error))
        }
    }

    /// Archive a spend budget by ID while retaining its accounting history.
    pub async fn delete_ai_gateway_budget(
        &self,
        id: uuid::Uuid,
    ) -> Result<(), datadog::Error<DeleteAiGatewayBudgetError>> {
        match self.delete_ai_gateway_budget_with_http_info(id).await {
            Ok(_) => Ok(()),
            Err(err) => Err(err),
        }
    }

    /// Archive a spend budget by ID while retaining its accounting history.
    pub async fn delete_ai_gateway_budget_with_http_info(
        &self,
        id: uuid::Uuid,
    ) -> Result<datadog::ResponseContent<()>, datadog::Error<DeleteAiGatewayBudgetError>> {
        let local_configuration = &self.config;
        let local_operation_id = "v2.delete_ai_gateway_budget";
        if local_configuration.is_unstable_operation_enabled(local_operation_id) {
            warn!("Using unstable operation {local_operation_id}");
        } else {
            let local_error = datadog::UnstableOperationDisabledError {
                msg: "Operation 'v2.delete_ai_gateway_budget' is not enabled".to_string(),
            };
            return Err(datadog::Error::UnstableOperationDisabledError(local_error));
        }

        let local_client = &self.client;

        let local_uri_str = format!(
            "{}/api/v2/budgets/{id}",
            local_configuration.get_operation_host(local_operation_id),
            id = datadog::urlencode(id.to_string())
        );
        let mut local_req_builder =
            local_client.request(reqwest::Method::DELETE, local_uri_str.as_str());

        // build headers
        let mut headers = HeaderMap::new();
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
            let local_entity: Option<DeleteAiGatewayBudgetError> =
                serde_json::from_str(&local_content).ok();
            let local_error = datadog::ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(datadog::Error::ResponseError(local_error))
        }
    }

    /// Delete an override using its `revision` and the current `configuration_revision`. If deleting the override can enable full capture for affected traffic, set `confirm_full_capture` to `true`. Returns HTTP 409 when `configuration_revision` or the override `revision` is stale, including a concurrent change during deletion. Missing revisions or required full-capture confirmation returns HTTP 400.
    pub async fn delete_ai_gateway_data_capture_override(
        &self,
        id: uuid::Uuid,
        body: crate::datadogV2::model::AiGatewayCaptureDeleteRequest,
    ) -> Result<(), datadog::Error<DeleteAiGatewayDataCaptureOverrideError>> {
        match self
            .delete_ai_gateway_data_capture_override_with_http_info(id, body)
            .await
        {
            Ok(_) => Ok(()),
            Err(err) => Err(err),
        }
    }

    /// Delete an override using its `revision` and the current `configuration_revision`. If deleting the override can enable full capture for affected traffic, set `confirm_full_capture` to `true`. Returns HTTP 409 when `configuration_revision` or the override `revision` is stale, including a concurrent change during deletion. Missing revisions or required full-capture confirmation returns HTTP 400.
    pub async fn delete_ai_gateway_data_capture_override_with_http_info(
        &self,
        id: uuid::Uuid,
        body: crate::datadogV2::model::AiGatewayCaptureDeleteRequest,
    ) -> Result<datadog::ResponseContent<()>, datadog::Error<DeleteAiGatewayDataCaptureOverrideError>>
    {
        let local_configuration = &self.config;
        let local_operation_id = "v2.delete_ai_gateway_data_capture_override";
        if local_configuration.is_unstable_operation_enabled(local_operation_id) {
            warn!("Using unstable operation {local_operation_id}");
        } else {
            let local_error = datadog::UnstableOperationDisabledError {
                msg: "Operation 'v2.delete_ai_gateway_data_capture_override' is not enabled"
                    .to_string(),
            };
            return Err(datadog::Error::UnstableOperationDisabledError(local_error));
        }

        let local_client = &self.client;

        let local_uri_str = format!(
            "{}/api/v2/data-capture/overrides/{id}",
            local_configuration.get_operation_host(local_operation_id),
            id = datadog::urlencode(id.to_string())
        );
        let mut local_req_builder =
            local_client.request(reqwest::Method::DELETE, local_uri_str.as_str());

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
            let local_entity: Option<DeleteAiGatewayDataCaptureOverrideError> =
                serde_json::from_str(&local_content).ok();
            let local_error = datadog::ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(datadog::Error::ResponseError(local_error))
        }
    }

    /// Requires `ai_gateway_config_write` and provider Edit. Models and pricing inherit provider access.
    /// Delete a manually managed model catalog entry by its canonical model ID. Discovered entries are read-only.
    pub async fn delete_ai_gateway_model(
        &self,
        provider_id: i64,
        canonical: String,
    ) -> Result<(), datadog::Error<DeleteAiGatewayModelError>> {
        match self
            .delete_ai_gateway_model_with_http_info(provider_id, canonical)
            .await
        {
            Ok(_) => Ok(()),
            Err(err) => Err(err),
        }
    }

    /// Requires `ai_gateway_config_write` and provider Edit. Models and pricing inherit provider access.
    /// Delete a manually managed model catalog entry by its canonical model ID. Discovered entries are read-only.
    pub async fn delete_ai_gateway_model_with_http_info(
        &self,
        provider_id: i64,
        canonical: String,
    ) -> Result<datadog::ResponseContent<()>, datadog::Error<DeleteAiGatewayModelError>> {
        let local_configuration = &self.config;
        let local_operation_id = "v2.delete_ai_gateway_model";
        if local_configuration.is_unstable_operation_enabled(local_operation_id) {
            warn!("Using unstable operation {local_operation_id}");
        } else {
            let local_error = datadog::UnstableOperationDisabledError {
                msg: "Operation 'v2.delete_ai_gateway_model' is not enabled".to_string(),
            };
            return Err(datadog::Error::UnstableOperationDisabledError(local_error));
        }

        let local_client = &self.client;

        let local_uri_str = format!(
            "{}/api/v2/providers/{provider_id}/models/{canonical}",
            local_configuration.get_operation_host(local_operation_id),
            provider_id = provider_id,
            canonical = datadog::urlencode(canonical)
        );
        let mut local_req_builder =
            local_client.request(reqwest::Method::DELETE, local_uri_str.as_str());

        // build headers
        let mut headers = HeaderMap::new();
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
            let local_entity: Option<DeleteAiGatewayModelError> =
                serde_json::from_str(&local_content).ok();
            let local_error = datadog::ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(datadog::Error::ResponseError(local_error))
        }
    }

    /// Delete a named model group. The organization-wide policy cannot be deleted. Requires configuration write permission and edit access to the group.
    pub async fn delete_ai_gateway_model_group(
        &self,
        id: String,
    ) -> Result<(), datadog::Error<DeleteAiGatewayModelGroupError>> {
        match self.delete_ai_gateway_model_group_with_http_info(id).await {
            Ok(_) => Ok(()),
            Err(err) => Err(err),
        }
    }

    /// Delete a named model group. The organization-wide policy cannot be deleted. Requires configuration write permission and edit access to the group.
    pub async fn delete_ai_gateway_model_group_with_http_info(
        &self,
        id: String,
    ) -> Result<datadog::ResponseContent<()>, datadog::Error<DeleteAiGatewayModelGroupError>> {
        let local_configuration = &self.config;
        let local_operation_id = "v2.delete_ai_gateway_model_group";
        if local_configuration.is_unstable_operation_enabled(local_operation_id) {
            warn!("Using unstable operation {local_operation_id}");
        } else {
            let local_error = datadog::UnstableOperationDisabledError {
                msg: "Operation 'v2.delete_ai_gateway_model_group' is not enabled".to_string(),
            };
            return Err(datadog::Error::UnstableOperationDisabledError(local_error));
        }

        let local_client = &self.client;

        let local_uri_str = format!(
            "{}/api/v2/model-groups/{id}",
            local_configuration.get_operation_host(local_operation_id),
            id = datadog::urlencode(id)
        );
        let mut local_req_builder =
            local_client.request(reqwest::Method::DELETE, local_uri_str.as_str());

        // build headers
        let mut headers = HeaderMap::new();
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
            let local_entity: Option<DeleteAiGatewayModelGroupError> =
                serde_json::from_str(&local_content).ok();
            let local_error = datadog::ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(datadog::Error::ResponseError(local_error))
        }
    }

    /// Clear a custom provider model's pricing override. Requires `ai_gateway_config_write` and provider Edit. Models and pricing inherit provider access.
    pub async fn delete_ai_gateway_model_pricing(
        &self,
        provider_id: i64,
        canonical: String,
    ) -> Result<(), datadog::Error<DeleteAiGatewayModelPricingError>> {
        match self
            .delete_ai_gateway_model_pricing_with_http_info(provider_id, canonical)
            .await
        {
            Ok(_) => Ok(()),
            Err(err) => Err(err),
        }
    }

    /// Clear a custom provider model's pricing override. Requires `ai_gateway_config_write` and provider Edit. Models and pricing inherit provider access.
    pub async fn delete_ai_gateway_model_pricing_with_http_info(
        &self,
        provider_id: i64,
        canonical: String,
    ) -> Result<datadog::ResponseContent<()>, datadog::Error<DeleteAiGatewayModelPricingError>>
    {
        let local_configuration = &self.config;
        let local_operation_id = "v2.delete_ai_gateway_model_pricing";
        if local_configuration.is_unstable_operation_enabled(local_operation_id) {
            warn!("Using unstable operation {local_operation_id}");
        } else {
            let local_error = datadog::UnstableOperationDisabledError {
                msg: "Operation 'v2.delete_ai_gateway_model_pricing' is not enabled".to_string(),
            };
            return Err(datadog::Error::UnstableOperationDisabledError(local_error));
        }

        let local_client = &self.client;

        let local_uri_str = format!(
            "{}/api/v2/providers/{provider_id}/models/{canonical}/pricing",
            local_configuration.get_operation_host(local_operation_id),
            provider_id = provider_id,
            canonical = datadog::urlencode(canonical)
        );
        let mut local_req_builder =
            local_client.request(reqwest::Method::DELETE, local_uri_str.as_str());

        // build headers
        let mut headers = HeaderMap::new();
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
            let local_entity: Option<DeleteAiGatewayModelPricingError> =
                serde_json::from_str(&local_content).ok();
            let local_error = datadog::ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(datadog::Error::ResponseError(local_error))
        }
    }

    /// Delete the routing policy document for a specific inbound pattern.
    pub async fn delete_ai_gateway_policy(
        &self,
        inbound: String,
        params: DeleteAiGatewayPolicyOptionalParams,
    ) -> Result<(), datadog::Error<DeleteAiGatewayPolicyError>> {
        match self
            .delete_ai_gateway_policy_with_http_info(inbound, params)
            .await
        {
            Ok(_) => Ok(()),
            Err(err) => Err(err),
        }
    }

    /// Delete the routing policy document for a specific inbound pattern.
    pub async fn delete_ai_gateway_policy_with_http_info(
        &self,
        inbound: String,
        params: DeleteAiGatewayPolicyOptionalParams,
    ) -> Result<datadog::ResponseContent<()>, datadog::Error<DeleteAiGatewayPolicyError>> {
        let local_configuration = &self.config;
        let local_operation_id = "v2.delete_ai_gateway_policy";
        if local_configuration.is_unstable_operation_enabled(local_operation_id) {
            warn!("Using unstable operation {local_operation_id}");
        } else {
            let local_error = datadog::UnstableOperationDisabledError {
                msg: "Operation 'v2.delete_ai_gateway_policy' is not enabled".to_string(),
            };
            return Err(datadog::Error::UnstableOperationDisabledError(local_error));
        }

        // unbox and build optional parameters
        let user_id = params.user_id;

        let local_client = &self.client;

        let local_uri_str = format!(
            "{}/api/v2/policies/{inbound}",
            local_configuration.get_operation_host(local_operation_id),
            inbound = datadog::urlencode(inbound)
        );
        let mut local_req_builder =
            local_client.request(reqwest::Method::DELETE, local_uri_str.as_str());

        if let Some(ref local_query_param) = user_id {
            local_req_builder =
                local_req_builder.query(&[("user_id", &local_query_param.to_string())]);
        };

        // build headers
        let mut headers = HeaderMap::new();
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
            let local_entity: Option<DeleteAiGatewayPolicyError> =
                serde_json::from_str(&local_content).ok();
            let local_error = datadog::ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(datadog::Error::ResponseError(local_error))
        }
    }

    /// Requires `ai_gateway_config_write` and provider Edit. Models and pricing inherit provider access. This also applies when force=true. Existing cascades remain and can affect policies the caller cannot edit. Delete a provider by ID. Providers used by routing rules return provider_in_use with HTTP 409 and the affected rules in error metadata. Retry with force=true to remove provider targets and delete rules left without targets. Surviving tiers keep their ordering and weights. A deletion that would rewrite a surviving tier with an unknown ordering_issue fails with HTTP 409 and code routing_rule_unknown_ordering until that rule is replaced or deleted.
    pub async fn delete_ai_gateway_provider(
        &self,
        id: i64,
        params: DeleteAiGatewayProviderOptionalParams,
    ) -> Result<(), datadog::Error<DeleteAiGatewayProviderError>> {
        match self
            .delete_ai_gateway_provider_with_http_info(id, params)
            .await
        {
            Ok(_) => Ok(()),
            Err(err) => Err(err),
        }
    }

    /// Requires `ai_gateway_config_write` and provider Edit. Models and pricing inherit provider access. This also applies when force=true. Existing cascades remain and can affect policies the caller cannot edit. Delete a provider by ID. Providers used by routing rules return provider_in_use with HTTP 409 and the affected rules in error metadata. Retry with force=true to remove provider targets and delete rules left without targets. Surviving tiers keep their ordering and weights. A deletion that would rewrite a surviving tier with an unknown ordering_issue fails with HTTP 409 and code routing_rule_unknown_ordering until that rule is replaced or deleted.
    pub async fn delete_ai_gateway_provider_with_http_info(
        &self,
        id: i64,
        params: DeleteAiGatewayProviderOptionalParams,
    ) -> Result<datadog::ResponseContent<()>, datadog::Error<DeleteAiGatewayProviderError>> {
        let local_configuration = &self.config;
        let local_operation_id = "v2.delete_ai_gateway_provider";
        if local_configuration.is_unstable_operation_enabled(local_operation_id) {
            warn!("Using unstable operation {local_operation_id}");
        } else {
            let local_error = datadog::UnstableOperationDisabledError {
                msg: "Operation 'v2.delete_ai_gateway_provider' is not enabled".to_string(),
            };
            return Err(datadog::Error::UnstableOperationDisabledError(local_error));
        }

        // unbox and build optional parameters
        let force = params.force;

        let local_client = &self.client;

        let local_uri_str = format!(
            "{}/api/v2/providers/{id}",
            local_configuration.get_operation_host(local_operation_id),
            id = id
        );
        let mut local_req_builder =
            local_client.request(reqwest::Method::DELETE, local_uri_str.as_str());

        if let Some(ref local_query_param) = force {
            local_req_builder =
                local_req_builder.query(&[("force", &local_query_param.to_string())]);
        };

        // build headers
        let mut headers = HeaderMap::new();
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
            let local_entity: Option<DeleteAiGatewayProviderError> =
                serde_json::from_str(&local_content).ok();
            let local_error = datadog::ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(datadog::Error::ResponseError(local_error))
        }
    }

    /// Archive a rate limit by ID. An archived rate limit no longer appears in lists or lookups, and its alert monitor is deleted.
    pub async fn delete_ai_gateway_rate_limit(
        &self,
        id: uuid::Uuid,
    ) -> Result<(), datadog::Error<DeleteAiGatewayRateLimitError>> {
        match self.delete_ai_gateway_rate_limit_with_http_info(id).await {
            Ok(_) => Ok(()),
            Err(err) => Err(err),
        }
    }

    /// Archive a rate limit by ID. An archived rate limit no longer appears in lists or lookups, and its alert monitor is deleted.
    pub async fn delete_ai_gateway_rate_limit_with_http_info(
        &self,
        id: uuid::Uuid,
    ) -> Result<datadog::ResponseContent<()>, datadog::Error<DeleteAiGatewayRateLimitError>> {
        let local_configuration = &self.config;
        let local_operation_id = "v2.delete_ai_gateway_rate_limit";
        if local_configuration.is_unstable_operation_enabled(local_operation_id) {
            warn!("Using unstable operation {local_operation_id}");
        } else {
            let local_error = datadog::UnstableOperationDisabledError {
                msg: "Operation 'v2.delete_ai_gateway_rate_limit' is not enabled".to_string(),
            };
            return Err(datadog::Error::UnstableOperationDisabledError(local_error));
        }

        let local_client = &self.client;

        let local_uri_str = format!(
            "{}/api/v2/rate-limits/{id}",
            local_configuration.get_operation_host(local_operation_id),
            id = datadog::urlencode(id.to_string())
        );
        let mut local_req_builder =
            local_client.request(reqwest::Method::DELETE, local_uri_str.as_str());

        // build headers
        let mut headers = HeaderMap::new();
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
            let local_entity: Option<DeleteAiGatewayRateLimitError> =
                serde_json::from_str(&local_content).ok();
            let local_error = datadog::ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(datadog::Error::ResponseError(local_error))
        }
    }

    /// Delete a routing rule. Referenced rules require force=true. Surviving tiers keep their ordering and weights. A deletion that would rewrite a surviving tier with an unknown ordering_issue fails with HTTP 409 and code routing_rule_unknown_ordering until that rule is replaced or deleted.
    pub async fn delete_ai_gateway_routing_rule(
        &self,
        slug: String,
        params: DeleteAiGatewayRoutingRuleOptionalParams,
    ) -> Result<(), datadog::Error<DeleteAiGatewayRoutingRuleError>> {
        match self
            .delete_ai_gateway_routing_rule_with_http_info(slug, params)
            .await
        {
            Ok(_) => Ok(()),
            Err(err) => Err(err),
        }
    }

    /// Delete a routing rule. Referenced rules require force=true. Surviving tiers keep their ordering and weights. A deletion that would rewrite a surviving tier with an unknown ordering_issue fails with HTTP 409 and code routing_rule_unknown_ordering until that rule is replaced or deleted.
    pub async fn delete_ai_gateway_routing_rule_with_http_info(
        &self,
        slug: String,
        params: DeleteAiGatewayRoutingRuleOptionalParams,
    ) -> Result<datadog::ResponseContent<()>, datadog::Error<DeleteAiGatewayRoutingRuleError>> {
        let local_configuration = &self.config;
        let local_operation_id = "v2.delete_ai_gateway_routing_rule";
        if local_configuration.is_unstable_operation_enabled(local_operation_id) {
            warn!("Using unstable operation {local_operation_id}");
        } else {
            let local_error = datadog::UnstableOperationDisabledError {
                msg: "Operation 'v2.delete_ai_gateway_routing_rule' is not enabled".to_string(),
            };
            return Err(datadog::Error::UnstableOperationDisabledError(local_error));
        }

        // unbox and build optional parameters
        let force = params.force;

        let local_client = &self.client;

        let local_uri_str = format!(
            "{}/api/v2/routing-rules",
            local_configuration.get_operation_host(local_operation_id)
        );
        let mut local_req_builder =
            local_client.request(reqwest::Method::DELETE, local_uri_str.as_str());

        local_req_builder = local_req_builder.query(&[("slug", &slug.to_string())]);
        if let Some(ref local_query_param) = force {
            local_req_builder =
                local_req_builder.query(&[("force", &local_query_param.to_string())]);
        };

        // build headers
        let mut headers = HeaderMap::new();
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
            let local_entity: Option<DeleteAiGatewayRoutingRuleError> =
                serde_json::from_str(&local_content).ok();
            let local_error = datadog::ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(datadog::Error::ResponseError(local_error))
        }
    }

    /// Get a spend budget by ID, including its capacity, scope, reset window, warning threshold, and enforcement mode.
    pub async fn get_ai_gateway_budget(
        &self,
        id: uuid::Uuid,
    ) -> Result<
        crate::datadogV2::model::AiGatewayBudgetResponse,
        datadog::Error<GetAiGatewayBudgetError>,
    > {
        match self.get_ai_gateway_budget_with_http_info(id).await {
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

    /// Get a spend budget by ID, including its capacity, scope, reset window, warning threshold, and enforcement mode.
    pub async fn get_ai_gateway_budget_with_http_info(
        &self,
        id: uuid::Uuid,
    ) -> Result<
        datadog::ResponseContent<crate::datadogV2::model::AiGatewayBudgetResponse>,
        datadog::Error<GetAiGatewayBudgetError>,
    > {
        let local_configuration = &self.config;
        let local_operation_id = "v2.get_ai_gateway_budget";
        if local_configuration.is_unstable_operation_enabled(local_operation_id) {
            warn!("Using unstable operation {local_operation_id}");
        } else {
            let local_error = datadog::UnstableOperationDisabledError {
                msg: "Operation 'v2.get_ai_gateway_budget' is not enabled".to_string(),
            };
            return Err(datadog::Error::UnstableOperationDisabledError(local_error));
        }

        let local_client = &self.client;

        let local_uri_str = format!(
            "{}/api/v2/budgets/{id}",
            local_configuration.get_operation_host(local_operation_id),
            id = datadog::urlencode(id.to_string())
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
            match serde_json::from_str::<crate::datadogV2::model::AiGatewayBudgetResponse>(
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
            let local_entity: Option<GetAiGatewayBudgetError> =
                serde_json::from_str(&local_content).ok();
            let local_error = datadog::ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(datadog::Error::ResponseError(local_error))
        }
    }

    /// Get the active accounting window, enforced capacity, and current spend.
    pub async fn get_ai_gateway_budget_current_period(
        &self,
        id: uuid::Uuid,
    ) -> Result<
        crate::datadogV2::model::AiGatewayBudgetPeriodResponse,
        datadog::Error<GetAiGatewayBudgetCurrentPeriodError>,
    > {
        match self
            .get_ai_gateway_budget_current_period_with_http_info(id)
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

    /// Get the active accounting window, enforced capacity, and current spend.
    pub async fn get_ai_gateway_budget_current_period_with_http_info(
        &self,
        id: uuid::Uuid,
    ) -> Result<
        datadog::ResponseContent<crate::datadogV2::model::AiGatewayBudgetPeriodResponse>,
        datadog::Error<GetAiGatewayBudgetCurrentPeriodError>,
    > {
        let local_configuration = &self.config;
        let local_operation_id = "v2.get_ai_gateway_budget_current_period";
        if local_configuration.is_unstable_operation_enabled(local_operation_id) {
            warn!("Using unstable operation {local_operation_id}");
        } else {
            let local_error = datadog::UnstableOperationDisabledError {
                msg: "Operation 'v2.get_ai_gateway_budget_current_period' is not enabled"
                    .to_string(),
            };
            return Err(datadog::Error::UnstableOperationDisabledError(local_error));
        }

        let local_client = &self.client;

        let local_uri_str = format!(
            "{}/api/v2/budgets/{id}/current-period",
            local_configuration.get_operation_host(local_operation_id),
            id = datadog::urlencode(id.to_string())
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
            match serde_json::from_str::<crate::datadogV2::model::AiGatewayBudgetPeriodResponse>(
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
            let local_entity: Option<GetAiGatewayBudgetCurrentPeriodError> =
                serde_json::from_str(&local_content).ok();
            let local_error = datadog::ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(datadog::Error::ResponseError(local_error))
        }
    }

    /// Aggregate counters for an accounting window, including total spend and the numbers of instances at or above the warning threshold and capacity. Use split_dim to aggregate only counters that match every given dimension value.
    pub async fn get_ai_gateway_budget_instances_summary(
        &self,
        id: uuid::Uuid,
        params: GetAiGatewayBudgetInstancesSummaryOptionalParams,
    ) -> Result<
        crate::datadogV2::model::AiGatewayBudgetInstancesSummaryResponse,
        datadog::Error<GetAiGatewayBudgetInstancesSummaryError>,
    > {
        match self
            .get_ai_gateway_budget_instances_summary_with_http_info(id, params)
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

    /// Aggregate counters for an accounting window, including total spend and the numbers of instances at or above the warning threshold and capacity. Use split_dim to aggregate only counters that match every given dimension value.
    pub async fn get_ai_gateway_budget_instances_summary_with_http_info(
        &self,
        id: uuid::Uuid,
        params: GetAiGatewayBudgetInstancesSummaryOptionalParams,
    ) -> Result<
        datadog::ResponseContent<crate::datadogV2::model::AiGatewayBudgetInstancesSummaryResponse>,
        datadog::Error<GetAiGatewayBudgetInstancesSummaryError>,
    > {
        let local_configuration = &self.config;
        let local_operation_id = "v2.get_ai_gateway_budget_instances_summary";
        if local_configuration.is_unstable_operation_enabled(local_operation_id) {
            warn!("Using unstable operation {local_operation_id}");
        } else {
            let local_error = datadog::UnstableOperationDisabledError {
                msg: "Operation 'v2.get_ai_gateway_budget_instances_summary' is not enabled"
                    .to_string(),
            };
            return Err(datadog::Error::UnstableOperationDisabledError(local_error));
        }

        // unbox and build optional parameters
        let window = params.window;
        let split_dim = params.split_dim;

        let local_client = &self.client;

        let local_uri_str = format!(
            "{}/api/v2/budgets/{id}/instances/summary",
            local_configuration.get_operation_host(local_operation_id),
            id = datadog::urlencode(id.to_string())
        );
        let mut local_req_builder =
            local_client.request(reqwest::Method::GET, local_uri_str.as_str());

        if let Some(ref local_query_param) = window {
            local_req_builder = local_req_builder.query(&[(
                "window",
                &local_query_param.to_rfc3339_opts(chrono::SecondsFormat::Millis, true),
            )]);
        };
        if let Some(ref local) = split_dim {
            for param in local {
                local_req_builder = local_req_builder.query(&[("split_dim", &param.to_string())]);
            }
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
            match serde_json::from_str::<
                crate::datadogV2::model::AiGatewayBudgetInstancesSummaryResponse,
            >(&local_content)
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
            let local_entity: Option<GetAiGatewayBudgetInstancesSummaryError> =
                serde_json::from_str(&local_content).ok();
            let local_error = datadog::ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(datadog::Error::ResponseError(local_error))
        }
    }

    /// Get the organization default mode, scoped capture overrides, and configuration revision.
    pub async fn get_ai_gateway_data_capture_configuration(
        &self,
    ) -> Result<
        crate::datadogV2::model::AiGatewayCaptureConfigurationResponse,
        datadog::Error<GetAiGatewayDataCaptureConfigurationError>,
    > {
        match self
            .get_ai_gateway_data_capture_configuration_with_http_info()
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

    /// Get the organization default mode, scoped capture overrides, and configuration revision.
    pub async fn get_ai_gateway_data_capture_configuration_with_http_info(
        &self,
    ) -> Result<
        datadog::ResponseContent<crate::datadogV2::model::AiGatewayCaptureConfigurationResponse>,
        datadog::Error<GetAiGatewayDataCaptureConfigurationError>,
    > {
        let local_configuration = &self.config;
        let local_operation_id = "v2.get_ai_gateway_data_capture_configuration";
        if local_configuration.is_unstable_operation_enabled(local_operation_id) {
            warn!("Using unstable operation {local_operation_id}");
        } else {
            let local_error = datadog::UnstableOperationDisabledError {
                msg: "Operation 'v2.get_ai_gateway_data_capture_configuration' is not enabled"
                    .to_string(),
            };
            return Err(datadog::Error::UnstableOperationDisabledError(local_error));
        }

        let local_client = &self.client;

        let local_uri_str = format!(
            "{}/api/v2/data-capture/configuration",
            local_configuration.get_operation_host(local_operation_id)
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
            match serde_json::from_str::<
                crate::datadogV2::model::AiGatewayCaptureConfigurationResponse,
            >(&local_content)
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
            let local_entity: Option<GetAiGatewayDataCaptureConfigurationError> =
                serde_json::from_str(&local_content).ok();
            let local_error = datadog::ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(datadog::Error::ResponseError(local_error))
        }
    }

    /// Get the organization-wide AI Gateway governance settings, which are the default data capture mode and whether requests must reach providers covered by a zero data retention agreement.
    pub async fn get_ai_gateway_governance(
        &self,
    ) -> Result<
        crate::datadogV2::model::AiGatewayGovernanceResponse,
        datadog::Error<GetAiGatewayGovernanceError>,
    > {
        match self.get_ai_gateway_governance_with_http_info().await {
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

    /// Get the organization-wide AI Gateway governance settings, which are the default data capture mode and whether requests must reach providers covered by a zero data retention agreement.
    pub async fn get_ai_gateway_governance_with_http_info(
        &self,
    ) -> Result<
        datadog::ResponseContent<crate::datadogV2::model::AiGatewayGovernanceResponse>,
        datadog::Error<GetAiGatewayGovernanceError>,
    > {
        let local_configuration = &self.config;
        let local_operation_id = "v2.get_ai_gateway_governance";
        if local_configuration.is_unstable_operation_enabled(local_operation_id) {
            warn!("Using unstable operation {local_operation_id}");
        } else {
            let local_error = datadog::UnstableOperationDisabledError {
                msg: "Operation 'v2.get_ai_gateway_governance' is not enabled".to_string(),
            };
            return Err(datadog::Error::UnstableOperationDisabledError(local_error));
        }

        let local_client = &self.client;

        let local_uri_str = format!(
            "{}/api/v2/governance",
            local_configuration.get_operation_host(local_operation_id)
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
            match serde_json::from_str::<crate::datadogV2::model::AiGatewayGovernanceResponse>(
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
            let local_entity: Option<GetAiGatewayGovernanceError> =
                serde_json::from_str(&local_content).ok();
            let local_error = datadog::ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(datadog::Error::ResponseError(local_error))
        }
    }

    /// Get a named policy or the explicit organization-wide policy by id "default". Requires configuration read permission; named policies also require view access. Provider selections cover current and future models and are returned separately from individual model selections.
    pub async fn get_ai_gateway_model_group(
        &self,
        id: String,
    ) -> Result<
        crate::datadogV2::model::AiGatewayModelGroupResponse,
        datadog::Error<GetAiGatewayModelGroupError>,
    > {
        match self.get_ai_gateway_model_group_with_http_info(id).await {
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

    /// Get a named policy or the explicit organization-wide policy by id "default". Requires configuration read permission; named policies also require view access. Provider selections cover current and future models and are returned separately from individual model selections.
    pub async fn get_ai_gateway_model_group_with_http_info(
        &self,
        id: String,
    ) -> Result<
        datadog::ResponseContent<crate::datadogV2::model::AiGatewayModelGroupResponse>,
        datadog::Error<GetAiGatewayModelGroupError>,
    > {
        let local_configuration = &self.config;
        let local_operation_id = "v2.get_ai_gateway_model_group";
        if local_configuration.is_unstable_operation_enabled(local_operation_id) {
            warn!("Using unstable operation {local_operation_id}");
        } else {
            let local_error = datadog::UnstableOperationDisabledError {
                msg: "Operation 'v2.get_ai_gateway_model_group' is not enabled".to_string(),
            };
            return Err(datadog::Error::UnstableOperationDisabledError(local_error));
        }

        let local_client = &self.client;

        let local_uri_str = format!(
            "{}/api/v2/model-groups/{id}",
            local_configuration.get_operation_host(local_operation_id),
            id = datadog::urlencode(id)
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
            match serde_json::from_str::<crate::datadogV2::model::AiGatewayModelGroupResponse>(
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
            let local_entity: Option<GetAiGatewayModelGroupError> =
                serde_json::from_str(&local_content).ok();
            let local_error = datadog::ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(datadog::Error::ResponseError(local_error))
        }
    }

    /// Get a custom provider model's pricing override and effective pricing source. Requires `ai_gateway_config_read` and provider View. Catalog and pricing metadata inherit provider access. Lists omit hidden entries; direct reads of hidden resources return 404.
    pub async fn get_ai_gateway_model_pricing(
        &self,
        provider_id: i64,
        canonical: String,
    ) -> Result<
        crate::datadogV2::model::AiGatewayModelPricingResponse,
        datadog::Error<GetAiGatewayModelPricingError>,
    > {
        match self
            .get_ai_gateway_model_pricing_with_http_info(provider_id, canonical)
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

    /// Get a custom provider model's pricing override and effective pricing source. Requires `ai_gateway_config_read` and provider View. Catalog and pricing metadata inherit provider access. Lists omit hidden entries; direct reads of hidden resources return 404.
    pub async fn get_ai_gateway_model_pricing_with_http_info(
        &self,
        provider_id: i64,
        canonical: String,
    ) -> Result<
        datadog::ResponseContent<crate::datadogV2::model::AiGatewayModelPricingResponse>,
        datadog::Error<GetAiGatewayModelPricingError>,
    > {
        let local_configuration = &self.config;
        let local_operation_id = "v2.get_ai_gateway_model_pricing";
        if local_configuration.is_unstable_operation_enabled(local_operation_id) {
            warn!("Using unstable operation {local_operation_id}");
        } else {
            let local_error = datadog::UnstableOperationDisabledError {
                msg: "Operation 'v2.get_ai_gateway_model_pricing' is not enabled".to_string(),
            };
            return Err(datadog::Error::UnstableOperationDisabledError(local_error));
        }

        let local_client = &self.client;

        let local_uri_str = format!(
            "{}/api/v2/providers/{provider_id}/models/{canonical}/pricing",
            local_configuration.get_operation_host(local_operation_id),
            provider_id = provider_id,
            canonical = datadog::urlencode(canonical)
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
            match serde_json::from_str::<crate::datadogV2::model::AiGatewayModelPricingResponse>(
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
            let local_entity: Option<GetAiGatewayModelPricingError> =
                serde_json::from_str(&local_content).ok();
            let local_error = datadog::ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(datadog::Error::ResponseError(local_error))
        }
    }

    /// Get the routing policy document for a specific inbound pattern.
    pub async fn get_ai_gateway_policy(
        &self,
        inbound: String,
        params: GetAiGatewayPolicyOptionalParams,
    ) -> Result<
        crate::datadogV2::model::AiGatewayPolicyResponse,
        datadog::Error<GetAiGatewayPolicyError>,
    > {
        match self
            .get_ai_gateway_policy_with_http_info(inbound, params)
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

    /// Get the routing policy document for a specific inbound pattern.
    pub async fn get_ai_gateway_policy_with_http_info(
        &self,
        inbound: String,
        params: GetAiGatewayPolicyOptionalParams,
    ) -> Result<
        datadog::ResponseContent<crate::datadogV2::model::AiGatewayPolicyResponse>,
        datadog::Error<GetAiGatewayPolicyError>,
    > {
        let local_configuration = &self.config;
        let local_operation_id = "v2.get_ai_gateway_policy";
        if local_configuration.is_unstable_operation_enabled(local_operation_id) {
            warn!("Using unstable operation {local_operation_id}");
        } else {
            let local_error = datadog::UnstableOperationDisabledError {
                msg: "Operation 'v2.get_ai_gateway_policy' is not enabled".to_string(),
            };
            return Err(datadog::Error::UnstableOperationDisabledError(local_error));
        }

        // unbox and build optional parameters
        let user_id = params.user_id;

        let local_client = &self.client;

        let local_uri_str = format!(
            "{}/api/v2/policies/{inbound}",
            local_configuration.get_operation_host(local_operation_id),
            inbound = datadog::urlencode(inbound)
        );
        let mut local_req_builder =
            local_client.request(reqwest::Method::GET, local_uri_str.as_str());

        if let Some(ref local_query_param) = user_id {
            local_req_builder =
                local_req_builder.query(&[("user_id", &local_query_param.to_string())]);
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
            match serde_json::from_str::<crate::datadogV2::model::AiGatewayPolicyResponse>(
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
            let local_entity: Option<GetAiGatewayPolicyError> =
                serde_json::from_str(&local_content).ok();
            let local_error = datadog::ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(datadog::Error::ResponseError(local_error))
        }
    }

    /// Get a provider by ID, including its routing slug, integration account, and cost multiplier. Credentials stay in the integration account and are never returned. Requires `ai_gateway_config_read` and provider View. Catalog and pricing metadata inherit provider access. Lists omit hidden entries; direct reads of hidden resources return 404.
    pub async fn get_ai_gateway_provider(
        &self,
        id: i64,
    ) -> Result<
        crate::datadogV2::model::AiGatewayProviderResponse,
        datadog::Error<GetAiGatewayProviderError>,
    > {
        match self.get_ai_gateway_provider_with_http_info(id).await {
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

    /// Get a provider by ID, including its routing slug, integration account, and cost multiplier. Credentials stay in the integration account and are never returned. Requires `ai_gateway_config_read` and provider View. Catalog and pricing metadata inherit provider access. Lists omit hidden entries; direct reads of hidden resources return 404.
    pub async fn get_ai_gateway_provider_with_http_info(
        &self,
        id: i64,
    ) -> Result<
        datadog::ResponseContent<crate::datadogV2::model::AiGatewayProviderResponse>,
        datadog::Error<GetAiGatewayProviderError>,
    > {
        let local_configuration = &self.config;
        let local_operation_id = "v2.get_ai_gateway_provider";
        if local_configuration.is_unstable_operation_enabled(local_operation_id) {
            warn!("Using unstable operation {local_operation_id}");
        } else {
            let local_error = datadog::UnstableOperationDisabledError {
                msg: "Operation 'v2.get_ai_gateway_provider' is not enabled".to_string(),
            };
            return Err(datadog::Error::UnstableOperationDisabledError(local_error));
        }

        let local_client = &self.client;

        let local_uri_str = format!(
            "{}/api/v2/providers/{id}",
            local_configuration.get_operation_host(local_operation_id),
            id = id
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
            match serde_json::from_str::<crate::datadogV2::model::AiGatewayProviderResponse>(
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
            let local_entity: Option<GetAiGatewayProviderError> =
                serde_json::from_str(&local_content).ok();
            let local_error = datadog::ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(datadog::Error::ResponseError(local_error))
        }
    }

    /// Get a rate limit by ID, including its scope, per-minute request and token limits, and whether it is enabled.
    pub async fn get_ai_gateway_rate_limit(
        &self,
        id: uuid::Uuid,
    ) -> Result<
        crate::datadogV2::model::AiGatewayRateLimitResponse,
        datadog::Error<GetAiGatewayRateLimitError>,
    > {
        match self.get_ai_gateway_rate_limit_with_http_info(id).await {
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

    /// Get a rate limit by ID, including its scope, per-minute request and token limits, and whether it is enabled.
    pub async fn get_ai_gateway_rate_limit_with_http_info(
        &self,
        id: uuid::Uuid,
    ) -> Result<
        datadog::ResponseContent<crate::datadogV2::model::AiGatewayRateLimitResponse>,
        datadog::Error<GetAiGatewayRateLimitError>,
    > {
        let local_configuration = &self.config;
        let local_operation_id = "v2.get_ai_gateway_rate_limit";
        if local_configuration.is_unstable_operation_enabled(local_operation_id) {
            warn!("Using unstable operation {local_operation_id}");
        } else {
            let local_error = datadog::UnstableOperationDisabledError {
                msg: "Operation 'v2.get_ai_gateway_rate_limit' is not enabled".to_string(),
            };
            return Err(datadog::Error::UnstableOperationDisabledError(local_error));
        }

        let local_client = &self.client;

        let local_uri_str = format!(
            "{}/api/v2/rate-limits/{id}",
            local_configuration.get_operation_host(local_operation_id),
            id = datadog::urlencode(id.to_string())
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
            match serde_json::from_str::<crate::datadogV2::model::AiGatewayRateLimitResponse>(
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
            let local_entity: Option<GetAiGatewayRateLimitError> =
                serde_json::from_str(&local_content).ok();
            let local_error = datadog::ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(datadog::Error::ResponseError(local_error))
        }
    }

    /// Get live usage for the organization's rate limits. Returns requests and tokens counted in the rolling one-minute window and requests limited in the last 15 minutes for each enabled rate limit, plus `untracked` (traffic no enforced rate limit covers) and `total` (all gateway traffic).
    pub async fn get_ai_gateway_rate_limit_usage(
        &self,
    ) -> Result<
        crate::datadogV2::model::AiGatewayRateLimitUsageListResponse,
        datadog::Error<GetAiGatewayRateLimitUsageError>,
    > {
        match self.get_ai_gateway_rate_limit_usage_with_http_info().await {
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

    /// Get live usage for the organization's rate limits. Returns requests and tokens counted in the rolling one-minute window and requests limited in the last 15 minutes for each enabled rate limit, plus `untracked` (traffic no enforced rate limit covers) and `total` (all gateway traffic).
    pub async fn get_ai_gateway_rate_limit_usage_with_http_info(
        &self,
    ) -> Result<
        datadog::ResponseContent<crate::datadogV2::model::AiGatewayRateLimitUsageListResponse>,
        datadog::Error<GetAiGatewayRateLimitUsageError>,
    > {
        let local_configuration = &self.config;
        let local_operation_id = "v2.get_ai_gateway_rate_limit_usage";
        if local_configuration.is_unstable_operation_enabled(local_operation_id) {
            warn!("Using unstable operation {local_operation_id}");
        } else {
            let local_error = datadog::UnstableOperationDisabledError {
                msg: "Operation 'v2.get_ai_gateway_rate_limit_usage' is not enabled".to_string(),
            };
            return Err(datadog::Error::UnstableOperationDisabledError(local_error));
        }

        let local_client = &self.client;

        let local_uri_str = format!(
            "{}/api/v2/rate-limits/usage",
            local_configuration.get_operation_host(local_operation_id)
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
            match serde_json::from_str::<crate::datadogV2::model::AiGatewayRateLimitUsageListResponse>(
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
            let local_entity: Option<GetAiGatewayRateLimitUsageError> =
                serde_json::from_str(&local_content).ok();
            let local_error = datadog::ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(datadog::Error::ResponseError(local_error))
        }
    }

    /// Get a routing rule by its organization-unique slug.
    pub async fn get_ai_gateway_routing_rule(
        &self,
        slug: String,
    ) -> Result<
        crate::datadogV2::model::AiGatewayRoutingRuleResponse,
        datadog::Error<GetAiGatewayRoutingRuleError>,
    > {
        match self.get_ai_gateway_routing_rule_with_http_info(slug).await {
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

    /// Get a routing rule by its organization-unique slug.
    pub async fn get_ai_gateway_routing_rule_with_http_info(
        &self,
        slug: String,
    ) -> Result<
        datadog::ResponseContent<crate::datadogV2::model::AiGatewayRoutingRuleResponse>,
        datadog::Error<GetAiGatewayRoutingRuleError>,
    > {
        let local_configuration = &self.config;
        let local_operation_id = "v2.get_ai_gateway_routing_rule";
        if local_configuration.is_unstable_operation_enabled(local_operation_id) {
            warn!("Using unstable operation {local_operation_id}");
        } else {
            let local_error = datadog::UnstableOperationDisabledError {
                msg: "Operation 'v2.get_ai_gateway_routing_rule' is not enabled".to_string(),
            };
            return Err(datadog::Error::UnstableOperationDisabledError(local_error));
        }

        let local_client = &self.client;

        let local_uri_str = format!(
            "{}/api/v2/routing-rules/get",
            local_configuration.get_operation_host(local_operation_id)
        );
        let mut local_req_builder =
            local_client.request(reqwest::Method::GET, local_uri_str.as_str());

        local_req_builder = local_req_builder.query(&[("slug", &slug.to_string())]);

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
            match serde_json::from_str::<crate::datadogV2::model::AiGatewayRoutingRuleResponse>(
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
            let local_entity: Option<GetAiGatewayRoutingRuleError> =
                serde_json::from_str(&local_content).ok();
            let local_error = datadog::ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(datadog::Error::ResponseError(local_error))
        }
    }

    /// Search the values that a template budget's split dimensions take across its instance counters in an accounting window. Each dimension returns its top values by spend, with counter counts. Use query or value to narrow results and limit to cap them. The truncated field reports whether more values matched. Each dimension's values are counted under the other dimensions' split_dim filters, so a dimension's own filter does not hide its alternatives. Legacy fixed-scope budgets return an empty list.
    pub async fn list_ai_gateway_budget_instance_dimensions(
        &self,
        id: uuid::Uuid,
        params: ListAiGatewayBudgetInstanceDimensionsOptionalParams,
    ) -> Result<
        crate::datadogV2::model::AiGatewayBudgetInstanceDimensionListResponse,
        datadog::Error<ListAiGatewayBudgetInstanceDimensionsError>,
    > {
        match self
            .list_ai_gateway_budget_instance_dimensions_with_http_info(id, params)
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

    /// Search the values that a template budget's split dimensions take across its instance counters in an accounting window. Each dimension returns its top values by spend, with counter counts. Use query or value to narrow results and limit to cap them. The truncated field reports whether more values matched. Each dimension's values are counted under the other dimensions' split_dim filters, so a dimension's own filter does not hide its alternatives. Legacy fixed-scope budgets return an empty list.
    pub async fn list_ai_gateway_budget_instance_dimensions_with_http_info(
        &self,
        id: uuid::Uuid,
        params: ListAiGatewayBudgetInstanceDimensionsOptionalParams,
    ) -> Result<
        datadog::ResponseContent<
            crate::datadogV2::model::AiGatewayBudgetInstanceDimensionListResponse,
        >,
        datadog::Error<ListAiGatewayBudgetInstanceDimensionsError>,
    > {
        let local_configuration = &self.config;
        let local_operation_id = "v2.list_ai_gateway_budget_instance_dimensions";
        if local_configuration.is_unstable_operation_enabled(local_operation_id) {
            warn!("Using unstable operation {local_operation_id}");
        } else {
            let local_error = datadog::UnstableOperationDisabledError {
                msg: "Operation 'v2.list_ai_gateway_budget_instance_dimensions' is not enabled"
                    .to_string(),
            };
            return Err(datadog::Error::UnstableOperationDisabledError(local_error));
        }

        // unbox and build optional parameters
        let window = params.window;
        let split_dim = params.split_dim;
        let dim = params.dim;
        let query = params.query;
        let value = params.value;
        let limit = params.limit;

        let local_client = &self.client;

        let local_uri_str = format!(
            "{}/api/v2/budgets/{id}/instances/dimensions",
            local_configuration.get_operation_host(local_operation_id),
            id = datadog::urlencode(id.to_string())
        );
        let mut local_req_builder =
            local_client.request(reqwest::Method::GET, local_uri_str.as_str());

        if let Some(ref local_query_param) = window {
            local_req_builder = local_req_builder.query(&[(
                "window",
                &local_query_param.to_rfc3339_opts(chrono::SecondsFormat::Millis, true),
            )]);
        };
        if let Some(ref local) = split_dim {
            for param in local {
                local_req_builder = local_req_builder.query(&[("split_dim", &param.to_string())]);
            }
        };
        if let Some(ref local) = dim {
            for param in local {
                local_req_builder = local_req_builder.query(&[("dim", &param.to_string())]);
            }
        };
        if let Some(ref local_query_param) = query {
            local_req_builder =
                local_req_builder.query(&[("query", &local_query_param.to_string())]);
        };
        if let Some(ref local) = value {
            for param in local {
                local_req_builder = local_req_builder.query(&[("value", &param.to_string())]);
            }
        };
        if let Some(ref local_query_param) = limit {
            local_req_builder =
                local_req_builder.query(&[("limit", &local_query_param.to_string())]);
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
            match serde_json::from_str::<
                crate::datadogV2::model::AiGatewayBudgetInstanceDimensionListResponse,
            >(&local_content)
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
            let local_entity: Option<ListAiGatewayBudgetInstanceDimensionsError> =
                serde_json::from_str(&local_content).ok();
            let local_error = datadog::ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(datadog::Error::ResponseError(local_error))
        }
    }

    /// List instance counters for an accounting window. Legacy fixed-scope budgets return one pooled row with an empty instance_key. Omit window to use the current window. Use split_dim to keep only counters that match every given dimension value.
    pub async fn list_ai_gateway_budget_instances(
        &self,
        id: uuid::Uuid,
        params: ListAiGatewayBudgetInstancesOptionalParams,
    ) -> Result<
        crate::datadogV2::model::AiGatewayBudgetInstanceListResponse,
        datadog::Error<ListAiGatewayBudgetInstancesError>,
    > {
        match self
            .list_ai_gateway_budget_instances_with_http_info(id, params)
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

    /// List instance counters for an accounting window. Legacy fixed-scope budgets return one pooled row with an empty instance_key. Omit window to use the current window. Use split_dim to keep only counters that match every given dimension value.
    pub async fn list_ai_gateway_budget_instances_with_http_info(
        &self,
        id: uuid::Uuid,
        params: ListAiGatewayBudgetInstancesOptionalParams,
    ) -> Result<
        datadog::ResponseContent<crate::datadogV2::model::AiGatewayBudgetInstanceListResponse>,
        datadog::Error<ListAiGatewayBudgetInstancesError>,
    > {
        let local_configuration = &self.config;
        let local_operation_id = "v2.list_ai_gateway_budget_instances";
        if local_configuration.is_unstable_operation_enabled(local_operation_id) {
            warn!("Using unstable operation {local_operation_id}");
        } else {
            let local_error = datadog::UnstableOperationDisabledError {
                msg: "Operation 'v2.list_ai_gateway_budget_instances' is not enabled".to_string(),
            };
            return Err(datadog::Error::UnstableOperationDisabledError(local_error));
        }

        // unbox and build optional parameters
        let window = params.window;
        let split_dim = params.split_dim;
        let sort = params.sort;
        let limit = params.limit;
        let offset = params.offset;

        let local_client = &self.client;

        let local_uri_str = format!(
            "{}/api/v2/budgets/{id}/instances",
            local_configuration.get_operation_host(local_operation_id),
            id = datadog::urlencode(id.to_string())
        );
        let mut local_req_builder =
            local_client.request(reqwest::Method::GET, local_uri_str.as_str());

        if let Some(ref local_query_param) = window {
            local_req_builder = local_req_builder.query(&[(
                "window",
                &local_query_param.to_rfc3339_opts(chrono::SecondsFormat::Millis, true),
            )]);
        };
        if let Some(ref local) = split_dim {
            for param in local {
                local_req_builder = local_req_builder.query(&[("split_dim", &param.to_string())]);
            }
        };
        if let Some(ref local_query_param) = sort {
            local_req_builder =
                local_req_builder.query(&[("sort", &local_query_param.to_string())]);
        };
        if let Some(ref local_query_param) = limit {
            local_req_builder =
                local_req_builder.query(&[("limit", &local_query_param.to_string())]);
        };
        if let Some(ref local_query_param) = offset {
            local_req_builder =
                local_req_builder.query(&[("offset", &local_query_param.to_string())]);
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
            match serde_json::from_str::<crate::datadogV2::model::AiGatewayBudgetInstanceListResponse>(
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
            let local_entity: Option<ListAiGatewayBudgetInstancesError> =
                serde_json::from_str(&local_content).ok();
            let local_error = datadog::ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(datadog::Error::ResponseError(local_error))
        }
    }

    /// List spend budgets for the organization, including active-window spend and effective capacity.
    pub async fn list_ai_gateway_budgets(
        &self,
    ) -> Result<
        crate::datadogV2::model::AiGatewayBudgetListResponse,
        datadog::Error<ListAiGatewayBudgetsError>,
    > {
        match self.list_ai_gateway_budgets_with_http_info().await {
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

    /// List spend budgets for the organization, including active-window spend and effective capacity.
    pub async fn list_ai_gateway_budgets_with_http_info(
        &self,
    ) -> Result<
        datadog::ResponseContent<crate::datadogV2::model::AiGatewayBudgetListResponse>,
        datadog::Error<ListAiGatewayBudgetsError>,
    > {
        let local_configuration = &self.config;
        let local_operation_id = "v2.list_ai_gateway_budgets";
        if local_configuration.is_unstable_operation_enabled(local_operation_id) {
            warn!("Using unstable operation {local_operation_id}");
        } else {
            let local_error = datadog::UnstableOperationDisabledError {
                msg: "Operation 'v2.list_ai_gateway_budgets' is not enabled".to_string(),
            };
            return Err(datadog::Error::UnstableOperationDisabledError(local_error));
        }

        let local_client = &self.client;

        let local_uri_str = format!(
            "{}/api/v2/budgets",
            local_configuration.get_operation_host(local_operation_id)
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
            match serde_json::from_str::<crate::datadogV2::model::AiGatewayBudgetListResponse>(
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
            let local_entity: Option<ListAiGatewayBudgetsError> =
                serde_json::from_str(&local_content).ok();
            let local_error = datadog::ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(datadog::Error::ResponseError(local_error))
        }
    }

    /// List model groups the user can view. The organization-wide policy has id "default" and explicit model, provider, and route selections. Missing or empty selections grant no organization-wide access. Its audience is fixed to the organization; it cannot be renamed or deleted. Named policies retain their access bindings. Requires configuration read permission; named policies also require view access.
    pub async fn list_ai_gateway_model_groups(
        &self,
    ) -> Result<
        crate::datadogV2::model::AiGatewayModelGroupListResponse,
        datadog::Error<ListAiGatewayModelGroupsError>,
    > {
        match self.list_ai_gateway_model_groups_with_http_info().await {
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

    /// List model groups the user can view. The organization-wide policy has id "default" and explicit model, provider, and route selections. Missing or empty selections grant no organization-wide access. Its audience is fixed to the organization; it cannot be renamed or deleted. Named policies retain their access bindings. Requires configuration read permission; named policies also require view access.
    pub async fn list_ai_gateway_model_groups_with_http_info(
        &self,
    ) -> Result<
        datadog::ResponseContent<crate::datadogV2::model::AiGatewayModelGroupListResponse>,
        datadog::Error<ListAiGatewayModelGroupsError>,
    > {
        let local_configuration = &self.config;
        let local_operation_id = "v2.list_ai_gateway_model_groups";
        if local_configuration.is_unstable_operation_enabled(local_operation_id) {
            warn!("Using unstable operation {local_operation_id}");
        } else {
            let local_error = datadog::UnstableOperationDisabledError {
                msg: "Operation 'v2.list_ai_gateway_model_groups' is not enabled".to_string(),
            };
            return Err(datadog::Error::UnstableOperationDisabledError(local_error));
        }

        let local_client = &self.client;

        let local_uri_str = format!(
            "{}/api/v2/model-groups",
            local_configuration.get_operation_host(local_operation_id)
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
            match serde_json::from_str::<crate::datadogV2::model::AiGatewayModelGroupListResponse>(
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
            let local_entity: Option<ListAiGatewayModelGroupsError> =
                serde_json::from_str(&local_content).ok();
            let local_error = datadog::ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(datadog::Error::ResponseError(local_error))
        }
    }

    /// List all model catalog entries for a provider. Requires `ai_gateway_config_read` and provider View. Catalog and pricing metadata inherit provider access. Lists omit hidden entries; direct reads of hidden resources return 404.
    pub async fn list_ai_gateway_models(
        &self,
        provider_id: i64,
    ) -> Result<
        crate::datadogV2::model::AiGatewayModelCatalogEntryListResponse,
        datadog::Error<ListAiGatewayModelsError>,
    > {
        match self
            .list_ai_gateway_models_with_http_info(provider_id)
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

    /// List all model catalog entries for a provider. Requires `ai_gateway_config_read` and provider View. Catalog and pricing metadata inherit provider access. Lists omit hidden entries; direct reads of hidden resources return 404.
    pub async fn list_ai_gateway_models_with_http_info(
        &self,
        provider_id: i64,
    ) -> Result<
        datadog::ResponseContent<crate::datadogV2::model::AiGatewayModelCatalogEntryListResponse>,
        datadog::Error<ListAiGatewayModelsError>,
    > {
        let local_configuration = &self.config;
        let local_operation_id = "v2.list_ai_gateway_models";
        if local_configuration.is_unstable_operation_enabled(local_operation_id) {
            warn!("Using unstable operation {local_operation_id}");
        } else {
            let local_error = datadog::UnstableOperationDisabledError {
                msg: "Operation 'v2.list_ai_gateway_models' is not enabled".to_string(),
            };
            return Err(datadog::Error::UnstableOperationDisabledError(local_error));
        }

        let local_client = &self.client;

        let local_uri_str = format!(
            "{}/api/v2/providers/{provider_id}/models",
            local_configuration.get_operation_host(local_operation_id),
            provider_id = provider_id
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
            match serde_json::from_str::<
                crate::datadogV2::model::AiGatewayModelCatalogEntryListResponse,
            >(&local_content)
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
            let local_entity: Option<ListAiGatewayModelsError> =
                serde_json::from_str(&local_content).ok();
            let local_error = datadog::ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(datadog::Error::ResponseError(local_error))
        }
    }

    /// List all model catalog entries grouped by canonical model ID. Requires `ai_gateway_config_read` and provider View. Catalog and pricing metadata inherit provider access. Lists omit hidden entries; direct reads of hidden resources return 404.
    pub async fn list_ai_gateway_models_by_canonical(
        &self,
    ) -> Result<
        crate::datadogV2::model::AiGatewayModelsByCanonicalListResponse,
        datadog::Error<ListAiGatewayModelsByCanonicalError>,
    > {
        match self
            .list_ai_gateway_models_by_canonical_with_http_info()
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

    /// List all model catalog entries grouped by canonical model ID. Requires `ai_gateway_config_read` and provider View. Catalog and pricing metadata inherit provider access. Lists omit hidden entries; direct reads of hidden resources return 404.
    pub async fn list_ai_gateway_models_by_canonical_with_http_info(
        &self,
    ) -> Result<
        datadog::ResponseContent<crate::datadogV2::model::AiGatewayModelsByCanonicalListResponse>,
        datadog::Error<ListAiGatewayModelsByCanonicalError>,
    > {
        let local_configuration = &self.config;
        let local_operation_id = "v2.list_ai_gateway_models_by_canonical";
        if local_configuration.is_unstable_operation_enabled(local_operation_id) {
            warn!("Using unstable operation {local_operation_id}");
        } else {
            let local_error = datadog::UnstableOperationDisabledError {
                msg: "Operation 'v2.list_ai_gateway_models_by_canonical' is not enabled"
                    .to_string(),
            };
            return Err(datadog::Error::UnstableOperationDisabledError(local_error));
        }

        let local_client = &self.client;

        let local_uri_str = format!(
            "{}/api/v2/models",
            local_configuration.get_operation_host(local_operation_id)
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
            match serde_json::from_str::<
                crate::datadogV2::model::AiGatewayModelsByCanonicalListResponse,
            >(&local_content)
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
            let local_entity: Option<ListAiGatewayModelsByCanonicalError> =
                serde_json::from_str(&local_content).ok();
            let local_error = datadog::ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(datadog::Error::ResponseError(local_error))
        }
    }

    /// List all routing policy documents for the organization. Pass user_id to filter by user scope.
    pub async fn list_ai_gateway_policies(
        &self,
        params: ListAiGatewayPoliciesOptionalParams,
    ) -> Result<
        crate::datadogV2::model::AiGatewayPolicyListResponse,
        datadog::Error<ListAiGatewayPoliciesError>,
    > {
        match self.list_ai_gateway_policies_with_http_info(params).await {
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

    /// List all routing policy documents for the organization. Pass user_id to filter by user scope.
    pub async fn list_ai_gateway_policies_with_http_info(
        &self,
        params: ListAiGatewayPoliciesOptionalParams,
    ) -> Result<
        datadog::ResponseContent<crate::datadogV2::model::AiGatewayPolicyListResponse>,
        datadog::Error<ListAiGatewayPoliciesError>,
    > {
        let local_configuration = &self.config;
        let local_operation_id = "v2.list_ai_gateway_policies";
        if local_configuration.is_unstable_operation_enabled(local_operation_id) {
            warn!("Using unstable operation {local_operation_id}");
        } else {
            let local_error = datadog::UnstableOperationDisabledError {
                msg: "Operation 'v2.list_ai_gateway_policies' is not enabled".to_string(),
            };
            return Err(datadog::Error::UnstableOperationDisabledError(local_error));
        }

        // unbox and build optional parameters
        let user_id = params.user_id;

        let local_client = &self.client;

        let local_uri_str = format!(
            "{}/api/v2/policies",
            local_configuration.get_operation_host(local_operation_id)
        );
        let mut local_req_builder =
            local_client.request(reqwest::Method::GET, local_uri_str.as_str());

        if let Some(ref local_query_param) = user_id {
            local_req_builder =
                local_req_builder.query(&[("user_id", &local_query_param.to_string())]);
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
            match serde_json::from_str::<crate::datadogV2::model::AiGatewayPolicyListResponse>(
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
            let local_entity: Option<ListAiGatewayPoliciesError> =
                serde_json::from_str(&local_content).ok();
            let local_error = datadog::ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(datadog::Error::ResponseError(local_error))
        }
    }

    /// List live models available to a provider integration account that supports model discovery.
    pub async fn list_ai_gateway_provider_models(
        &self,
        integration_type: String,
        integration_account_id: String,
        params: ListAiGatewayProviderModelsOptionalParams,
    ) -> Result<
        crate::datadogV2::model::AiGatewayDiscoveredModelListResponse,
        datadog::Error<ListAiGatewayProviderModelsError>,
    > {
        match self
            .list_ai_gateway_provider_models_with_http_info(
                integration_type,
                integration_account_id,
                params,
            )
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

    /// List live models available to a provider integration account that supports model discovery.
    pub async fn list_ai_gateway_provider_models_with_http_info(
        &self,
        integration_type: String,
        integration_account_id: String,
        params: ListAiGatewayProviderModelsOptionalParams,
    ) -> Result<
        datadog::ResponseContent<crate::datadogV2::model::AiGatewayDiscoveredModelListResponse>,
        datadog::Error<ListAiGatewayProviderModelsError>,
    > {
        let local_configuration = &self.config;
        let local_operation_id = "v2.list_ai_gateway_provider_models";
        if local_configuration.is_unstable_operation_enabled(local_operation_id) {
            warn!("Using unstable operation {local_operation_id}");
        } else {
            let local_error = datadog::UnstableOperationDisabledError {
                msg: "Operation 'v2.list_ai_gateway_provider_models' is not enabled".to_string(),
            };
            return Err(datadog::Error::UnstableOperationDisabledError(local_error));
        }

        // unbox and build optional parameters
        let project_id = params.project_id;
        let location = params.location;
        let bedrock_region = params.bedrock_region;
        let foundry_resource_id = params.foundry_resource_id;

        let local_client = &self.client;

        let local_uri_str = format!(
            "{}/api/v2/provider-models",
            local_configuration.get_operation_host(local_operation_id)
        );
        let mut local_req_builder =
            local_client.request(reqwest::Method::GET, local_uri_str.as_str());

        local_req_builder =
            local_req_builder.query(&[("integration_type", &integration_type.to_string())]);
        local_req_builder = local_req_builder.query(&[(
            "integration_account_id",
            &integration_account_id.to_string(),
        )]);
        if let Some(ref local_query_param) = project_id {
            local_req_builder =
                local_req_builder.query(&[("project_id", &local_query_param.to_string())]);
        };
        if let Some(ref local_query_param) = location {
            local_req_builder =
                local_req_builder.query(&[("location", &local_query_param.to_string())]);
        };
        if let Some(ref local_query_param) = bedrock_region {
            local_req_builder =
                local_req_builder.query(&[("bedrock_region", &local_query_param.to_string())]);
        };
        if let Some(ref local_query_param) = foundry_resource_id {
            local_req_builder =
                local_req_builder.query(&[("foundry_resource_id", &local_query_param.to_string())]);
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
            match serde_json::from_str::<
                crate::datadogV2::model::AiGatewayDiscoveredModelListResponse,
            >(&local_content)
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
            let local_entity: Option<ListAiGatewayProviderModelsError> =
                serde_json::from_str(&local_content).ok();
            let local_error = datadog::ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(datadog::Error::ResponseError(local_error))
        }
    }

    /// List the model providers connected to the organization. Credentials stay in each provider's integration account and are never returned. Requires `ai_gateway_config_read` and provider View. Catalog and pricing metadata inherit provider access. Lists omit hidden entries; direct reads of hidden resources return 404.
    pub async fn list_ai_gateway_providers(
        &self,
    ) -> Result<
        crate::datadogV2::model::AiGatewayProviderListResponse,
        datadog::Error<ListAiGatewayProvidersError>,
    > {
        match self.list_ai_gateway_providers_with_http_info().await {
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

    /// List the model providers connected to the organization. Credentials stay in each provider's integration account and are never returned. Requires `ai_gateway_config_read` and provider View. Catalog and pricing metadata inherit provider access. Lists omit hidden entries; direct reads of hidden resources return 404.
    pub async fn list_ai_gateway_providers_with_http_info(
        &self,
    ) -> Result<
        datadog::ResponseContent<crate::datadogV2::model::AiGatewayProviderListResponse>,
        datadog::Error<ListAiGatewayProvidersError>,
    > {
        let local_configuration = &self.config;
        let local_operation_id = "v2.list_ai_gateway_providers";
        if local_configuration.is_unstable_operation_enabled(local_operation_id) {
            warn!("Using unstable operation {local_operation_id}");
        } else {
            let local_error = datadog::UnstableOperationDisabledError {
                msg: "Operation 'v2.list_ai_gateway_providers' is not enabled".to_string(),
            };
            return Err(datadog::Error::UnstableOperationDisabledError(local_error));
        }

        let local_client = &self.client;

        let local_uri_str = format!(
            "{}/api/v2/providers",
            local_configuration.get_operation_host(local_operation_id)
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
            match serde_json::from_str::<crate::datadogV2::model::AiGatewayProviderListResponse>(
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
            let local_entity: Option<ListAiGatewayProvidersError> =
                serde_json::from_str(&local_content).ok();
            let local_error = datadog::ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(datadog::Error::ResponseError(local_error))
        }
    }

    /// List the organization's AI Gateway rate limits.
    pub async fn list_ai_gateway_rate_limits(
        &self,
    ) -> Result<
        crate::datadogV2::model::AiGatewayRateLimitListResponse,
        datadog::Error<ListAiGatewayRateLimitsError>,
    > {
        match self.list_ai_gateway_rate_limits_with_http_info().await {
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

    /// List the organization's AI Gateway rate limits.
    pub async fn list_ai_gateway_rate_limits_with_http_info(
        &self,
    ) -> Result<
        datadog::ResponseContent<crate::datadogV2::model::AiGatewayRateLimitListResponse>,
        datadog::Error<ListAiGatewayRateLimitsError>,
    > {
        let local_configuration = &self.config;
        let local_operation_id = "v2.list_ai_gateway_rate_limits";
        if local_configuration.is_unstable_operation_enabled(local_operation_id) {
            warn!("Using unstable operation {local_operation_id}");
        } else {
            let local_error = datadog::UnstableOperationDisabledError {
                msg: "Operation 'v2.list_ai_gateway_rate_limits' is not enabled".to_string(),
            };
            return Err(datadog::Error::UnstableOperationDisabledError(local_error));
        }

        let local_client = &self.client;

        let local_uri_str = format!(
            "{}/api/v2/rate-limits",
            local_configuration.get_operation_host(local_operation_id)
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
            match serde_json::from_str::<crate::datadogV2::model::AiGatewayRateLimitListResponse>(
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
            let local_entity: Option<ListAiGatewayRateLimitsError> =
                serde_json::from_str(&local_content).ok();
            let local_error = datadog::ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(datadog::Error::ResponseError(local_error))
        }
    }

    /// List all routing rules for the organization.
    pub async fn list_ai_gateway_routing_rules(
        &self,
    ) -> Result<
        crate::datadogV2::model::AiGatewayRoutingRuleListResponse,
        datadog::Error<ListAiGatewayRoutingRulesError>,
    > {
        match self.list_ai_gateway_routing_rules_with_http_info().await {
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

    /// List all routing rules for the organization.
    pub async fn list_ai_gateway_routing_rules_with_http_info(
        &self,
    ) -> Result<
        datadog::ResponseContent<crate::datadogV2::model::AiGatewayRoutingRuleListResponse>,
        datadog::Error<ListAiGatewayRoutingRulesError>,
    > {
        let local_configuration = &self.config;
        let local_operation_id = "v2.list_ai_gateway_routing_rules";
        if local_configuration.is_unstable_operation_enabled(local_operation_id) {
            warn!("Using unstable operation {local_operation_id}");
        } else {
            let local_error = datadog::UnstableOperationDisabledError {
                msg: "Operation 'v2.list_ai_gateway_routing_rules' is not enabled".to_string(),
            };
            return Err(datadog::Error::UnstableOperationDisabledError(local_error));
        }

        let local_client = &self.client;

        let local_uri_str = format!(
            "{}/api/v2/routing-rules",
            local_configuration.get_operation_host(local_operation_id)
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
            match serde_json::from_str::<crate::datadogV2::model::AiGatewayRoutingRuleListResponse>(
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
            let local_entity: Option<ListAiGatewayRoutingRulesError> =
                serde_json::from_str(&local_content).ok();
            let local_error = datadog::ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(datadog::Error::ResponseError(local_error))
        }
    }

    /// Override active-period capacity for one instance of a template budget, identified by its instance_key. Other instances and the budget's default capacity are unchanged. The override is retained in the budget adjustment audit history. Returns 404 when the instance has no counter in the active period. Not supported for legacy fixed-scope budgets.
    pub async fn override_ai_gateway_budget_instance_capacity(
        &self,
        id: uuid::Uuid,
        body: crate::datadogV2::model::AiGatewayBudgetInstanceCapacityOverrideRequest,
    ) -> Result<
        crate::datadogV2::model::AiGatewayBudgetInstanceResponse,
        datadog::Error<OverrideAiGatewayBudgetInstanceCapacityError>,
    > {
        match self
            .override_ai_gateway_budget_instance_capacity_with_http_info(id, body)
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

    /// Override active-period capacity for one instance of a template budget, identified by its instance_key. Other instances and the budget's default capacity are unchanged. The override is retained in the budget adjustment audit history. Returns 404 when the instance has no counter in the active period. Not supported for legacy fixed-scope budgets.
    pub async fn override_ai_gateway_budget_instance_capacity_with_http_info(
        &self,
        id: uuid::Uuid,
        body: crate::datadogV2::model::AiGatewayBudgetInstanceCapacityOverrideRequest,
    ) -> Result<
        datadog::ResponseContent<crate::datadogV2::model::AiGatewayBudgetInstanceResponse>,
        datadog::Error<OverrideAiGatewayBudgetInstanceCapacityError>,
    > {
        let local_configuration = &self.config;
        let local_operation_id = "v2.override_ai_gateway_budget_instance_capacity";
        if local_configuration.is_unstable_operation_enabled(local_operation_id) {
            warn!("Using unstable operation {local_operation_id}");
        } else {
            let local_error = datadog::UnstableOperationDisabledError {
                msg: "Operation 'v2.override_ai_gateway_budget_instance_capacity' is not enabled"
                    .to_string(),
            };
            return Err(datadog::Error::UnstableOperationDisabledError(local_error));
        }

        let local_client = &self.client;

        let local_uri_str = format!(
            "{}/api/v2/budgets/{id}/instances/capacity",
            local_configuration.get_operation_host(local_operation_id),
            id = datadog::urlencode(id.to_string())
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
            match serde_json::from_str::<crate::datadogV2::model::AiGatewayBudgetInstanceResponse>(
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
            let local_entity: Option<OverrideAiGatewayBudgetInstanceCapacityError> =
                serde_json::from_str(&local_content).ok();
            let local_error = datadog::ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(datadog::Error::ResponseError(local_error))
        }
    }

    /// Reset active-period spend for one instance of a template budget, identified by its instance_key. The reset is retained in the budget adjustment audit history. Returns 404 when the instance has no counter in the active period. Not supported for legacy fixed-scope budgets.
    pub async fn reset_ai_gateway_budget_instance_spend(
        &self,
        id: uuid::Uuid,
        body: crate::datadogV2::model::AiGatewayBudgetInstanceResetRequest,
    ) -> Result<
        crate::datadogV2::model::AiGatewayBudgetInstanceResponse,
        datadog::Error<ResetAiGatewayBudgetInstanceSpendError>,
    > {
        match self
            .reset_ai_gateway_budget_instance_spend_with_http_info(id, body)
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

    /// Reset active-period spend for one instance of a template budget, identified by its instance_key. The reset is retained in the budget adjustment audit history. Returns 404 when the instance has no counter in the active period. Not supported for legacy fixed-scope budgets.
    pub async fn reset_ai_gateway_budget_instance_spend_with_http_info(
        &self,
        id: uuid::Uuid,
        body: crate::datadogV2::model::AiGatewayBudgetInstanceResetRequest,
    ) -> Result<
        datadog::ResponseContent<crate::datadogV2::model::AiGatewayBudgetInstanceResponse>,
        datadog::Error<ResetAiGatewayBudgetInstanceSpendError>,
    > {
        let local_configuration = &self.config;
        let local_operation_id = "v2.reset_ai_gateway_budget_instance_spend";
        if local_configuration.is_unstable_operation_enabled(local_operation_id) {
            warn!("Using unstable operation {local_operation_id}");
        } else {
            let local_error = datadog::UnstableOperationDisabledError {
                msg: "Operation 'v2.reset_ai_gateway_budget_instance_spend' is not enabled"
                    .to_string(),
            };
            return Err(datadog::Error::UnstableOperationDisabledError(local_error));
        }

        let local_client = &self.client;

        let local_uri_str = format!(
            "{}/api/v2/budgets/{id}/instances/reset",
            local_configuration.get_operation_host(local_operation_id),
            id = datadog::urlencode(id.to_string())
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
            match serde_json::from_str::<crate::datadogV2::model::AiGatewayBudgetInstanceResponse>(
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
            let local_entity: Option<ResetAiGatewayBudgetInstanceSpendError> =
                serde_json::from_str(&local_content).ok();
            let local_error = datadog::ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(datadog::Error::ResponseError(local_error))
        }
    }

    /// Update a budget's mutable definition. Capacity changes apply to future accounting periods
    /// unless `apply_to_current_period` is true.
    pub async fn update_ai_gateway_budget(
        &self,
        id: uuid::Uuid,
        body: crate::datadogV2::model::AiGatewayBudgetUpdateRequest,
    ) -> Result<
        crate::datadogV2::model::AiGatewayBudgetResponse,
        datadog::Error<UpdateAiGatewayBudgetError>,
    > {
        match self.update_ai_gateway_budget_with_http_info(id, body).await {
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

    /// Update a budget's mutable definition. Capacity changes apply to future accounting periods
    /// unless `apply_to_current_period` is true.
    pub async fn update_ai_gateway_budget_with_http_info(
        &self,
        id: uuid::Uuid,
        body: crate::datadogV2::model::AiGatewayBudgetUpdateRequest,
    ) -> Result<
        datadog::ResponseContent<crate::datadogV2::model::AiGatewayBudgetResponse>,
        datadog::Error<UpdateAiGatewayBudgetError>,
    > {
        let local_configuration = &self.config;
        let local_operation_id = "v2.update_ai_gateway_budget";
        if local_configuration.is_unstable_operation_enabled(local_operation_id) {
            warn!("Using unstable operation {local_operation_id}");
        } else {
            let local_error = datadog::UnstableOperationDisabledError {
                msg: "Operation 'v2.update_ai_gateway_budget' is not enabled".to_string(),
            };
            return Err(datadog::Error::UnstableOperationDisabledError(local_error));
        }

        let local_client = &self.client;

        let local_uri_str = format!(
            "{}/api/v2/budgets/{id}",
            local_configuration.get_operation_host(local_operation_id),
            id = datadog::urlencode(id.to_string())
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
            match serde_json::from_str::<crate::datadogV2::model::AiGatewayBudgetResponse>(
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
            let local_entity: Option<UpdateAiGatewayBudgetError> =
                serde_json::from_str(&local_content).ok();
            let local_error = datadog::ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(datadog::Error::ResponseError(local_error))
        }
    }

    /// Replace an override using its `revision` and the current `configuration_revision`. Changes to capture overrides don't affect routing rules. Returns HTTP 409 when `configuration_revision` or the override `revision` is stale, including a concurrent change during replacement. Missing revisions or required full-capture confirmation returns HTTP 400.
    pub async fn update_ai_gateway_data_capture_override(
        &self,
        id: uuid::Uuid,
        body: crate::datadogV2::model::AiGatewayCaptureUpdateRequest,
    ) -> Result<
        crate::datadogV2::model::AiGatewayCaptureOverrideResponse,
        datadog::Error<UpdateAiGatewayDataCaptureOverrideError>,
    > {
        match self
            .update_ai_gateway_data_capture_override_with_http_info(id, body)
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

    /// Replace an override using its `revision` and the current `configuration_revision`. Changes to capture overrides don't affect routing rules. Returns HTTP 409 when `configuration_revision` or the override `revision` is stale, including a concurrent change during replacement. Missing revisions or required full-capture confirmation returns HTTP 400.
    pub async fn update_ai_gateway_data_capture_override_with_http_info(
        &self,
        id: uuid::Uuid,
        body: crate::datadogV2::model::AiGatewayCaptureUpdateRequest,
    ) -> Result<
        datadog::ResponseContent<crate::datadogV2::model::AiGatewayCaptureOverrideResponse>,
        datadog::Error<UpdateAiGatewayDataCaptureOverrideError>,
    > {
        let local_configuration = &self.config;
        let local_operation_id = "v2.update_ai_gateway_data_capture_override";
        if local_configuration.is_unstable_operation_enabled(local_operation_id) {
            warn!("Using unstable operation {local_operation_id}");
        } else {
            let local_error = datadog::UnstableOperationDisabledError {
                msg: "Operation 'v2.update_ai_gateway_data_capture_override' is not enabled"
                    .to_string(),
            };
            return Err(datadog::Error::UnstableOperationDisabledError(local_error));
        }

        let local_client = &self.client;

        let local_uri_str = format!(
            "{}/api/v2/data-capture/overrides/{id}",
            local_configuration.get_operation_host(local_operation_id),
            id = datadog::urlencode(id.to_string())
        );
        let mut local_req_builder =
            local_client.request(reqwest::Method::PUT, local_uri_str.as_str());

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
            match serde_json::from_str::<crate::datadogV2::model::AiGatewayCaptureOverrideResponse>(
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
            let local_entity: Option<UpdateAiGatewayDataCaptureOverrideError> =
                serde_json::from_str(&local_content).ok();
            let local_error = datadog::ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(datadog::Error::ResponseError(local_error))
        }
    }

    /// Replace the organization-wide AI Gateway governance settings, which are the default data capture mode and whether requests must reach providers covered by a zero data retention agreement.
    pub async fn update_ai_gateway_governance(
        &self,
        body: crate::datadogV2::model::AiGatewayGovernanceReplaceRequest,
    ) -> Result<
        crate::datadogV2::model::AiGatewayGovernanceResponse,
        datadog::Error<UpdateAiGatewayGovernanceError>,
    > {
        match self.update_ai_gateway_governance_with_http_info(body).await {
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

    /// Replace the organization-wide AI Gateway governance settings, which are the default data capture mode and whether requests must reach providers covered by a zero data retention agreement.
    pub async fn update_ai_gateway_governance_with_http_info(
        &self,
        body: crate::datadogV2::model::AiGatewayGovernanceReplaceRequest,
    ) -> Result<
        datadog::ResponseContent<crate::datadogV2::model::AiGatewayGovernanceResponse>,
        datadog::Error<UpdateAiGatewayGovernanceError>,
    > {
        let local_configuration = &self.config;
        let local_operation_id = "v2.update_ai_gateway_governance";
        if local_configuration.is_unstable_operation_enabled(local_operation_id) {
            warn!("Using unstable operation {local_operation_id}");
        } else {
            let local_error = datadog::UnstableOperationDisabledError {
                msg: "Operation 'v2.update_ai_gateway_governance' is not enabled".to_string(),
            };
            return Err(datadog::Error::UnstableOperationDisabledError(local_error));
        }

        let local_client = &self.client;

        let local_uri_str = format!(
            "{}/api/v2/governance",
            local_configuration.get_operation_host(local_operation_id)
        );
        let mut local_req_builder =
            local_client.request(reqwest::Method::PUT, local_uri_str.as_str());

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
            match serde_json::from_str::<crate::datadogV2::model::AiGatewayGovernanceResponse>(
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
            let local_entity: Option<UpdateAiGatewayGovernanceError> =
                serde_json::from_str(&local_content).ok();
            let local_error = datadog::ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(datadog::Error::ResponseError(local_error))
        }
    }

    /// Update a manually managed model catalog entry. Discovered entries are read-only. Requires `ai_gateway_config_write` and provider Edit. Models and pricing inherit provider access.
    pub async fn update_ai_gateway_model(
        &self,
        provider_id: i64,
        canonical: String,
        body: crate::datadogV2::model::AiGatewayModelUpdateRequest,
    ) -> Result<
        crate::datadogV2::model::AiGatewayModelCatalogEntryResponse,
        datadog::Error<UpdateAiGatewayModelError>,
    > {
        match self
            .update_ai_gateway_model_with_http_info(provider_id, canonical, body)
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

    /// Update a manually managed model catalog entry. Discovered entries are read-only. Requires `ai_gateway_config_write` and provider Edit. Models and pricing inherit provider access.
    pub async fn update_ai_gateway_model_with_http_info(
        &self,
        provider_id: i64,
        canonical: String,
        body: crate::datadogV2::model::AiGatewayModelUpdateRequest,
    ) -> Result<
        datadog::ResponseContent<crate::datadogV2::model::AiGatewayModelCatalogEntryResponse>,
        datadog::Error<UpdateAiGatewayModelError>,
    > {
        let local_configuration = &self.config;
        let local_operation_id = "v2.update_ai_gateway_model";
        if local_configuration.is_unstable_operation_enabled(local_operation_id) {
            warn!("Using unstable operation {local_operation_id}");
        } else {
            let local_error = datadog::UnstableOperationDisabledError {
                msg: "Operation 'v2.update_ai_gateway_model' is not enabled".to_string(),
            };
            return Err(datadog::Error::UnstableOperationDisabledError(local_error));
        }

        let local_client = &self.client;

        let local_uri_str = format!(
            "{}/api/v2/providers/{provider_id}/models/{canonical}",
            local_configuration.get_operation_host(local_operation_id),
            provider_id = provider_id,
            canonical = datadog::urlencode(canonical)
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
            match serde_json::from_str::<crate::datadogV2::model::AiGatewayModelCatalogEntryResponse>(
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
            let local_entity: Option<UpdateAiGatewayModelError> =
                serde_json::from_str(&local_content).ok();
            let local_error = datadog::ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(datadog::Error::ResponseError(local_error))
        }
    }

    /// Update a model group. For id "default", save membership and request options atomically with expected_revision. Empty selections grant no organization-wide access. Applicable authorized named policies override organization-wide request options; a rejected named-policy request cannot fall back to organization-wide options. The organization-wide audience is fixed and grants resource use only. Requires configuration write permission; named policies also require edit access.
    pub async fn update_ai_gateway_model_group(
        &self,
        id: String,
        body: crate::datadogV2::model::AiGatewayModelGroupUpdateRequest,
    ) -> Result<
        crate::datadogV2::model::AiGatewayModelGroupResponse,
        datadog::Error<UpdateAiGatewayModelGroupError>,
    > {
        match self
            .update_ai_gateway_model_group_with_http_info(id, body)
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

    /// Update a model group. For id "default", save membership and request options atomically with expected_revision. Empty selections grant no organization-wide access. Applicable authorized named policies override organization-wide request options; a rejected named-policy request cannot fall back to organization-wide options. The organization-wide audience is fixed and grants resource use only. Requires configuration write permission; named policies also require edit access.
    pub async fn update_ai_gateway_model_group_with_http_info(
        &self,
        id: String,
        body: crate::datadogV2::model::AiGatewayModelGroupUpdateRequest,
    ) -> Result<
        datadog::ResponseContent<crate::datadogV2::model::AiGatewayModelGroupResponse>,
        datadog::Error<UpdateAiGatewayModelGroupError>,
    > {
        let local_configuration = &self.config;
        let local_operation_id = "v2.update_ai_gateway_model_group";
        if local_configuration.is_unstable_operation_enabled(local_operation_id) {
            warn!("Using unstable operation {local_operation_id}");
        } else {
            let local_error = datadog::UnstableOperationDisabledError {
                msg: "Operation 'v2.update_ai_gateway_model_group' is not enabled".to_string(),
            };
            return Err(datadog::Error::UnstableOperationDisabledError(local_error));
        }

        let local_client = &self.client;

        let local_uri_str = format!(
            "{}/api/v2/model-groups/{id}",
            local_configuration.get_operation_host(local_operation_id),
            id = datadog::urlencode(id)
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
            match serde_json::from_str::<crate::datadogV2::model::AiGatewayModelGroupResponse>(
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
            let local_entity: Option<UpdateAiGatewayModelGroupError> =
                serde_json::from_str(&local_content).ok();
            let local_error = datadog::ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(datadog::Error::ResponseError(local_error))
        }
    }

    /// Set a custom provider model's pricing override. Requires `ai_gateway_config_write` and provider Edit. Models and pricing inherit provider access.
    pub async fn update_ai_gateway_model_pricing(
        &self,
        provider_id: i64,
        canonical: String,
        body: crate::datadogV2::model::AiGatewayModelPricingRequest,
    ) -> Result<
        crate::datadogV2::model::AiGatewayModelPricingResponse,
        datadog::Error<UpdateAiGatewayModelPricingError>,
    > {
        match self
            .update_ai_gateway_model_pricing_with_http_info(provider_id, canonical, body)
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

    /// Set a custom provider model's pricing override. Requires `ai_gateway_config_write` and provider Edit. Models and pricing inherit provider access.
    pub async fn update_ai_gateway_model_pricing_with_http_info(
        &self,
        provider_id: i64,
        canonical: String,
        body: crate::datadogV2::model::AiGatewayModelPricingRequest,
    ) -> Result<
        datadog::ResponseContent<crate::datadogV2::model::AiGatewayModelPricingResponse>,
        datadog::Error<UpdateAiGatewayModelPricingError>,
    > {
        let local_configuration = &self.config;
        let local_operation_id = "v2.update_ai_gateway_model_pricing";
        if local_configuration.is_unstable_operation_enabled(local_operation_id) {
            warn!("Using unstable operation {local_operation_id}");
        } else {
            let local_error = datadog::UnstableOperationDisabledError {
                msg: "Operation 'v2.update_ai_gateway_model_pricing' is not enabled".to_string(),
            };
            return Err(datadog::Error::UnstableOperationDisabledError(local_error));
        }

        let local_client = &self.client;

        let local_uri_str = format!(
            "{}/api/v2/providers/{provider_id}/models/{canonical}/pricing",
            local_configuration.get_operation_host(local_operation_id),
            provider_id = provider_id,
            canonical = datadog::urlencode(canonical)
        );
        let mut local_req_builder =
            local_client.request(reqwest::Method::PUT, local_uri_str.as_str());

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
            match serde_json::from_str::<crate::datadogV2::model::AiGatewayModelPricingResponse>(
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
            let local_entity: Option<UpdateAiGatewayModelPricingError> =
                serde_json::from_str(&local_content).ok();
            let local_error = datadog::ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(datadog::Error::ResponseError(local_error))
        }
    }

    /// Update a provider's display name or cost multiplier. Provider type, integration settings, and slug are immutable. Requires `ai_gateway_config_write` and provider Edit. Models and pricing inherit provider access.
    pub async fn update_ai_gateway_provider(
        &self,
        id: i64,
        body: crate::datadogV2::model::AiGatewayProviderUpdateRequest,
    ) -> Result<
        crate::datadogV2::model::AiGatewayProviderResponse,
        datadog::Error<UpdateAiGatewayProviderError>,
    > {
        match self
            .update_ai_gateway_provider_with_http_info(id, body)
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

    /// Update a provider's display name or cost multiplier. Provider type, integration settings, and slug are immutable. Requires `ai_gateway_config_write` and provider Edit. Models and pricing inherit provider access.
    pub async fn update_ai_gateway_provider_with_http_info(
        &self,
        id: i64,
        body: crate::datadogV2::model::AiGatewayProviderUpdateRequest,
    ) -> Result<
        datadog::ResponseContent<crate::datadogV2::model::AiGatewayProviderResponse>,
        datadog::Error<UpdateAiGatewayProviderError>,
    > {
        let local_configuration = &self.config;
        let local_operation_id = "v2.update_ai_gateway_provider";
        if local_configuration.is_unstable_operation_enabled(local_operation_id) {
            warn!("Using unstable operation {local_operation_id}");
        } else {
            let local_error = datadog::UnstableOperationDisabledError {
                msg: "Operation 'v2.update_ai_gateway_provider' is not enabled".to_string(),
            };
            return Err(datadog::Error::UnstableOperationDisabledError(local_error));
        }

        let local_client = &self.client;

        let local_uri_str = format!(
            "{}/api/v2/providers/{id}",
            local_configuration.get_operation_host(local_operation_id),
            id = id
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
            match serde_json::from_str::<crate::datadogV2::model::AiGatewayProviderResponse>(
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
            let local_entity: Option<UpdateAiGatewayProviderError> =
                serde_json::from_str(&local_content).ok();
            let local_error = datadog::ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(datadog::Error::ResponseError(local_error))
        }
    }

    /// Update a rate limit. Only the attributes sent are changed. Scope stays editable, and changes never reset current usage. Returns 409 when the rule was changed concurrently or the new scope duplicates another rule.
    pub async fn update_ai_gateway_rate_limit(
        &self,
        id: uuid::Uuid,
        body: crate::datadogV2::model::AiGatewayRateLimitUpdateRequest,
    ) -> Result<
        crate::datadogV2::model::AiGatewayRateLimitResponse,
        datadog::Error<UpdateAiGatewayRateLimitError>,
    > {
        match self
            .update_ai_gateway_rate_limit_with_http_info(id, body)
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

    /// Update a rate limit. Only the attributes sent are changed. Scope stays editable, and changes never reset current usage. Returns 409 when the rule was changed concurrently or the new scope duplicates another rule.
    pub async fn update_ai_gateway_rate_limit_with_http_info(
        &self,
        id: uuid::Uuid,
        body: crate::datadogV2::model::AiGatewayRateLimitUpdateRequest,
    ) -> Result<
        datadog::ResponseContent<crate::datadogV2::model::AiGatewayRateLimitResponse>,
        datadog::Error<UpdateAiGatewayRateLimitError>,
    > {
        let local_configuration = &self.config;
        let local_operation_id = "v2.update_ai_gateway_rate_limit";
        if local_configuration.is_unstable_operation_enabled(local_operation_id) {
            warn!("Using unstable operation {local_operation_id}");
        } else {
            let local_error = datadog::UnstableOperationDisabledError {
                msg: "Operation 'v2.update_ai_gateway_rate_limit' is not enabled".to_string(),
            };
            return Err(datadog::Error::UnstableOperationDisabledError(local_error));
        }

        let local_client = &self.client;

        let local_uri_str = format!(
            "{}/api/v2/rate-limits/{id}",
            local_configuration.get_operation_host(local_operation_id),
            id = datadog::urlencode(id.to_string())
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
            match serde_json::from_str::<crate::datadogV2::model::AiGatewayRateLimitResponse>(
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
            let local_entity: Option<UpdateAiGatewayRateLimitError> =
                serde_json::from_str(&local_content).ok();
            let local_error = datadog::ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(datadog::Error::ResponseError(local_error))
        }
    }

    /// Replace a routing rule's tiers. The slug is immutable; data capture is managed independently. This is a full replacement: omitted or null ordering resets the tier to configured, and omitted or null weights in a weighted_affinity tier reset to 1. Omitted or null reasoning_level clears a model destination's reasoning override. In a classified tier, omit both the classifier's provider and model to keep the saved decision model. An omitted context clears it, and an omitted or null confidence_threshold resets to 0.5. Replacing a rule repairs tiers reported with an ordering_issue.
    ///
    /// **Note**: Classified routing is in Preview and may not be available for all organizations. Saving a classified tier returns 403 Forbidden unless the feature is enabled for your organization.
    pub async fn update_ai_gateway_routing_rule(
        &self,
        slug: String,
        body: crate::datadogV2::model::AiGatewayRoutingRuleReplaceRequest,
    ) -> Result<
        crate::datadogV2::model::AiGatewayRoutingRuleResponse,
        datadog::Error<UpdateAiGatewayRoutingRuleError>,
    > {
        match self
            .update_ai_gateway_routing_rule_with_http_info(slug, body)
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

    /// Replace a routing rule's tiers. The slug is immutable; data capture is managed independently. This is a full replacement: omitted or null ordering resets the tier to configured, and omitted or null weights in a weighted_affinity tier reset to 1. Omitted or null reasoning_level clears a model destination's reasoning override. In a classified tier, omit both the classifier's provider and model to keep the saved decision model. An omitted context clears it, and an omitted or null confidence_threshold resets to 0.5. Replacing a rule repairs tiers reported with an ordering_issue.
    ///
    /// **Note**: Classified routing is in Preview and may not be available for all organizations. Saving a classified tier returns 403 Forbidden unless the feature is enabled for your organization.
    pub async fn update_ai_gateway_routing_rule_with_http_info(
        &self,
        slug: String,
        body: crate::datadogV2::model::AiGatewayRoutingRuleReplaceRequest,
    ) -> Result<
        datadog::ResponseContent<crate::datadogV2::model::AiGatewayRoutingRuleResponse>,
        datadog::Error<UpdateAiGatewayRoutingRuleError>,
    > {
        let local_configuration = &self.config;
        let local_operation_id = "v2.update_ai_gateway_routing_rule";
        if local_configuration.is_unstable_operation_enabled(local_operation_id) {
            warn!("Using unstable operation {local_operation_id}");
        } else {
            let local_error = datadog::UnstableOperationDisabledError {
                msg: "Operation 'v2.update_ai_gateway_routing_rule' is not enabled".to_string(),
            };
            return Err(datadog::Error::UnstableOperationDisabledError(local_error));
        }

        let local_client = &self.client;

        let local_uri_str = format!(
            "{}/api/v2/routing-rules",
            local_configuration.get_operation_host(local_operation_id)
        );
        let mut local_req_builder =
            local_client.request(reqwest::Method::PUT, local_uri_str.as_str());

        local_req_builder = local_req_builder.query(&[("slug", &slug.to_string())]);

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
            match serde_json::from_str::<crate::datadogV2::model::AiGatewayRoutingRuleResponse>(
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
            let local_entity: Option<UpdateAiGatewayRoutingRuleError> =
                serde_json::from_str(&local_content).ok();
            let local_error = datadog::ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(datadog::Error::ResponseError(local_error))
        }
    }

    /// Create or replace the routing policy document for an inbound pattern. Pass user_id to scope the policy to a user.
    pub async fn upsert_ai_gateway_policy(
        &self,
        body: crate::datadogV2::model::AiGatewayPolicyRequest,
        params: UpsertAiGatewayPolicyOptionalParams,
    ) -> Result<
        crate::datadogV2::model::AiGatewayPolicyResponse,
        datadog::Error<UpsertAiGatewayPolicyError>,
    > {
        match self
            .upsert_ai_gateway_policy_with_http_info(body, params)
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

    /// Create or replace the routing policy document for an inbound pattern. Pass user_id to scope the policy to a user.
    pub async fn upsert_ai_gateway_policy_with_http_info(
        &self,
        body: crate::datadogV2::model::AiGatewayPolicyRequest,
        params: UpsertAiGatewayPolicyOptionalParams,
    ) -> Result<
        datadog::ResponseContent<crate::datadogV2::model::AiGatewayPolicyResponse>,
        datadog::Error<UpsertAiGatewayPolicyError>,
    > {
        let local_configuration = &self.config;
        let local_operation_id = "v2.upsert_ai_gateway_policy";
        if local_configuration.is_unstable_operation_enabled(local_operation_id) {
            warn!("Using unstable operation {local_operation_id}");
        } else {
            let local_error = datadog::UnstableOperationDisabledError {
                msg: "Operation 'v2.upsert_ai_gateway_policy' is not enabled".to_string(),
            };
            return Err(datadog::Error::UnstableOperationDisabledError(local_error));
        }

        // unbox and build optional parameters
        let user_id = params.user_id;

        let local_client = &self.client;

        let local_uri_str = format!(
            "{}/api/v2/policies",
            local_configuration.get_operation_host(local_operation_id)
        );
        let mut local_req_builder =
            local_client.request(reqwest::Method::POST, local_uri_str.as_str());

        if let Some(ref local_query_param) = user_id {
            local_req_builder =
                local_req_builder.query(&[("user_id", &local_query_param.to_string())]);
        };

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
            match serde_json::from_str::<crate::datadogV2::model::AiGatewayPolicyResponse>(
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
            let local_entity: Option<UpsertAiGatewayPolicyError> =
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
