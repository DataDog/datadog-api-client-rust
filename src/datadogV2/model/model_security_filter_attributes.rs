// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// The object describing a security filter.
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SecurityFilterAttributes {
    /// The list of exclusion filters applied in this security filter.
    #[serde(rename = "exclusion_filters")]
    pub exclusion_filters:
        Option<Vec<crate::datadogV2::model::SecurityFilterExclusionFilterResponse>>,
    /// The filtered data type.
    #[serde(rename = "filtered_data_type")]
    pub filtered_data_type: Option<crate::datadogV2::model::SecurityFilterFilteredDataType>,
    /// Whether the security filter is the built-in filter.
    #[serde(rename = "is_builtin")]
    pub is_builtin: Option<bool>,
    /// Whether the security filter is enabled.
    #[serde(rename = "is_enabled")]
    pub is_enabled: Option<bool>,
    /// The security filter name.
    #[serde(rename = "name")]
    pub name: Option<String>,
    /// The security filter query. Logs accepted by this query will be accepted by this filter.
    #[serde(rename = "query")]
    pub query: Option<String>,
    /// The version of the security filter.
    #[serde(rename = "version")]
    pub version: Option<i32>,
}

impl SecurityFilterAttributes {
    pub fn new() -> SecurityFilterAttributes {
        SecurityFilterAttributes {
            exclusion_filters: None,
            filtered_data_type: None,
            is_builtin: None,
            is_enabled: None,
            name: None,
            query: None,
            version: None,
        }
    }

    pub fn with_exclusion_filters(
        &mut self,
        value: Vec<crate::datadogV2::model::SecurityFilterExclusionFilterResponse>,
    ) -> &mut Self {
        self.exclusion_filters = Some(value);
        self
    }

    pub fn with_filtered_data_type(
        &mut self,
        value: crate::datadogV2::model::SecurityFilterFilteredDataType,
    ) -> &mut Self {
        self.filtered_data_type = Some(value);
        self
    }

    pub fn with_is_builtin(&mut self, value: bool) -> &mut Self {
        self.is_builtin = Some(value);
        self
    }

    pub fn with_is_enabled(&mut self, value: bool) -> &mut Self {
        self.is_enabled = Some(value);
        self
    }

    pub fn with_name(&mut self, value: String) -> &mut Self {
        self.name = Some(value);
        self
    }

    pub fn with_query(&mut self, value: String) -> &mut Self {
        self.query = Some(value);
        self
    }

    pub fn with_version(&mut self, value: i32) -> &mut Self {
        self.version = Some(value);
        self
    }
}
impl Default for SecurityFilterAttributes {
    fn default() -> Self {
        Self::new()
    }
}
