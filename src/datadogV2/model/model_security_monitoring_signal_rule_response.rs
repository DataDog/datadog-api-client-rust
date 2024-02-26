// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// Rule.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SecurityMonitoringSignalRuleResponse {
    /// Cases for generating signals.
    #[serde(rename = "cases")]
    pub cases: Option<Vec<crate::datadogV2::model::SecurityMonitoringRuleCase>>,
    /// When the rule was created, timestamp in milliseconds.
    #[serde(rename = "createdAt")]
    pub created_at: Option<i64>,
    /// User ID of the user who created the rule.
    #[serde(rename = "creationAuthorId")]
    pub creation_author_id: Option<i64>,
    /// When the rule will be deprecated, timestamp in milliseconds.
    #[serde(rename = "deprecationDate")]
    pub deprecation_date: Option<i64>,
    /// Additional queries to filter matched events before they are processed.
    #[serde(rename = "filters")]
    pub filters: Option<Vec<crate::datadogV2::model::SecurityMonitoringFilter>>,
    /// Whether the notifications include the triggering group-by values in their title.
    #[serde(rename = "hasExtendedTitle")]
    pub has_extended_title: Option<bool>,
    /// The ID of the rule.
    #[serde(rename = "id")]
    pub id: Option<String>,
    /// Whether the rule is included by default.
    #[serde(rename = "isDefault")]
    pub is_default: Option<bool>,
    /// Whether the rule has been deleted.
    #[serde(rename = "isDeleted")]
    pub is_deleted: Option<bool>,
    /// Whether the rule is enabled.
    #[serde(rename = "isEnabled")]
    pub is_enabled: Option<bool>,
    /// Message for generated signals.
    #[serde(rename = "message")]
    pub message: Option<String>,
    /// The name of the rule.
    #[serde(rename = "name")]
    pub name: Option<String>,
    /// Options on rules.
    #[serde(rename = "options")]
    pub options: Option<crate::datadogV2::model::SecurityMonitoringRuleOptions>,
    /// Queries for selecting logs which are part of the rule.
    #[serde(rename = "queries")]
    pub queries: Option<Vec<crate::datadogV2::model::SecurityMonitoringSignalRuleResponseQuery>>,
    /// Tags for generated signals.
    #[serde(rename = "tags")]
    pub tags: Option<Vec<String>>,
    /// The rule type.
    #[serde(rename = "type")]
    pub type_: Option<crate::datadogV2::model::SecurityMonitoringSignalRuleType>,
    /// User ID of the user who updated the rule.
    #[serde(rename = "updateAuthorId")]
    pub update_author_id: Option<i64>,
    /// The version of the rule.
    #[serde(rename = "version")]
    pub version: Option<i64>,
}

impl SecurityMonitoringSignalRuleResponse {
    pub fn new() -> SecurityMonitoringSignalRuleResponse {
        SecurityMonitoringSignalRuleResponse {
            cases: None,
            created_at: None,
            creation_author_id: None,
            deprecation_date: None,
            filters: None,
            has_extended_title: None,
            id: None,
            is_default: None,
            is_deleted: None,
            is_enabled: None,
            message: None,
            name: None,
            options: None,
            queries: None,
            tags: None,
            type_: None,
            update_author_id: None,
            version: None,
        }
    }

    pub fn cases(
        &mut self,
        value: Vec<crate::datadogV2::model::SecurityMonitoringRuleCase>,
    ) -> &mut Self {
        self.cases = Some(value);
        self
    }

    pub fn created_at(&mut self, value: i64) -> &mut Self {
        self.created_at = Some(value);
        self
    }

    pub fn creation_author_id(&mut self, value: i64) -> &mut Self {
        self.creation_author_id = Some(value);
        self
    }

    pub fn deprecation_date(&mut self, value: i64) -> &mut Self {
        self.deprecation_date = Some(value);
        self
    }

    pub fn filters(
        &mut self,
        value: Vec<crate::datadogV2::model::SecurityMonitoringFilter>,
    ) -> &mut Self {
        self.filters = Some(value);
        self
    }

    pub fn has_extended_title(&mut self, value: bool) -> &mut Self {
        self.has_extended_title = Some(value);
        self
    }

    pub fn id(&mut self, value: String) -> &mut Self {
        self.id = Some(value);
        self
    }

    pub fn is_default(&mut self, value: bool) -> &mut Self {
        self.is_default = Some(value);
        self
    }

    pub fn is_deleted(&mut self, value: bool) -> &mut Self {
        self.is_deleted = Some(value);
        self
    }

    pub fn is_enabled(&mut self, value: bool) -> &mut Self {
        self.is_enabled = Some(value);
        self
    }

    pub fn message(&mut self, value: String) -> &mut Self {
        self.message = Some(value);
        self
    }

    pub fn name(&mut self, value: String) -> &mut Self {
        self.name = Some(value);
        self
    }

    pub fn options(
        &mut self,
        value: crate::datadogV2::model::SecurityMonitoringRuleOptions,
    ) -> &mut Self {
        self.options = Some(value);
        self
    }

    pub fn queries(
        &mut self,
        value: Vec<crate::datadogV2::model::SecurityMonitoringSignalRuleResponseQuery>,
    ) -> &mut Self {
        self.queries = Some(value);
        self
    }

    pub fn tags(&mut self, value: Vec<String>) -> &mut Self {
        self.tags = Some(value);
        self
    }

    pub fn type_(
        &mut self,
        value: crate::datadogV2::model::SecurityMonitoringSignalRuleType,
    ) -> &mut Self {
        self.type_ = Some(value);
        self
    }

    pub fn update_author_id(&mut self, value: i64) -> &mut Self {
        self.update_author_id = Some(value);
        self
    }

    pub fn version(&mut self, value: i64) -> &mut Self {
        self.version = Some(value);
        self
    }
}

impl Default for SecurityMonitoringSignalRuleResponse {
    fn default() -> Self {
        Self::new()
    }
}
