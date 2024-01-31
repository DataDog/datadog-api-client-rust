// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// The security filters properties to be updated.
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SecurityFilterUpdateAttributes {
    /// Exclusion filters to exclude some logs from the security filter.
    #[serde(rename = "exclusion_filters")]
    pub exclusion_filters: Option<Vec<crate::datadogV2::model::SecurityFilterExclusionFilter>>,
    /// The filtered data type.
    #[serde(rename = "filtered_data_type")]
    pub filtered_data_type: Option<crate::datadogV2::model::SecurityFilterFilteredDataType>,
    /// Whether the security filter is enabled.
    #[serde(rename = "is_enabled")]
    pub is_enabled: Option<bool>,
    /// The name of the security filter.
    #[serde(rename = "name")]
    pub name: Option<String>,
    /// The query of the security filter.
    #[serde(rename = "query")]
    pub query: Option<String>,
    /// The version of the security filter to update.
    #[serde(rename = "version")]
    pub version: Option<i32>,
}

impl SecurityFilterUpdateAttributes {
    pub fn new() -> SecurityFilterUpdateAttributes {
        SecurityFilterUpdateAttributes {
            exclusion_filters: None,
            filtered_data_type: None,
            is_enabled: None,
            name: None,
            query: None,
            version: None,
        }
    }

    pub fn with_exclusion_filters(
        &mut self,
        value: Vec<crate::datadogV2::model::SecurityFilterExclusionFilter>,
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
impl Default for SecurityFilterUpdateAttributes {
    fn default() -> Self {
        Self::new()
    }
}
