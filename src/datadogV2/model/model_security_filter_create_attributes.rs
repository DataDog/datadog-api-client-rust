// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// Object containing the attributes of the security filter to be created.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SecurityFilterCreateAttributes {
    /// Exclusion filters to exclude some logs from the security filter.
    #[serde(rename = "exclusion_filters")]
    pub exclusion_filters: Vec<crate::datadogV2::model::SecurityFilterExclusionFilter>,
    /// The filtered data type.
    #[serde(rename = "filtered_data_type")]
    pub filtered_data_type: crate::datadogV2::model::SecurityFilterFilteredDataType,
    /// Whether the security filter is enabled.
    #[serde(rename = "is_enabled")]
    pub is_enabled: bool,
    /// The name of the security filter.
    #[serde(rename = "name")]
    pub name: String,
    /// The query of the security filter.
    #[serde(rename = "query")]
    pub query: String,
}

impl SecurityFilterCreateAttributes {
    pub fn new(
        exclusion_filters: Vec<crate::datadogV2::model::SecurityFilterExclusionFilter>,
        filtered_data_type: crate::datadogV2::model::SecurityFilterFilteredDataType,
        is_enabled: bool,
        name: String,
        query: String,
    ) -> SecurityFilterCreateAttributes {
        SecurityFilterCreateAttributes {
            exclusion_filters,
            filtered_data_type,
            is_enabled,
            name,
            query,
        }
    }
}
