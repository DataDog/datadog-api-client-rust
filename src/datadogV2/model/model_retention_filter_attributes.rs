// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct RetentionFilterAttributes {
    /// The creation timestamp of the retention filter.
    #[serde(rename = "created_at", skip_serializing_if = "Option::is_none")]
    pub created_at: Option<i64>,
    /// The creator of the retention filter.
    #[serde(rename = "created_by", skip_serializing_if = "Option::is_none")]
    pub created_by: Option<String>,
    /// Shows whether the filter can be edited.
    #[serde(rename = "editable", skip_serializing_if = "Option::is_none")]
    pub editable: Option<bool>,
    /// The status of the retention filter (Enabled/Disabled).
    #[serde(rename = "enabled", skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    /// The execution order of the retention filter.
    #[serde(rename = "execution_order", skip_serializing_if = "Option::is_none")]
    pub execution_order: Option<i64>,
    /// The spans filter used to index spans.
    #[serde(rename = "filter", skip_serializing_if = "Option::is_none")]
    pub filter: Option<Box<crate::datadogV2::model::SpansFilter>>,
    /// The type of retention filter. The value should always be spans-sampling-processor.
    #[serde(rename = "filter_type", skip_serializing_if = "Option::is_none")]
    pub filter_type: Option<crate::datadogV2::model::RetentionFilterType>,
    /// The modification timestamp of the retention filter.
    #[serde(rename = "modified_at", skip_serializing_if = "Option::is_none")]
    pub modified_at: Option<i64>,
    /// The modifier of the retention filter.
    #[serde(rename = "modified_by", skip_serializing_if = "Option::is_none")]
    pub modified_by: Option<String>,
    /// The name of the retention filter.
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Sample rate to apply to spans going through this retention filter,
    /// a value of 1.0 keeps all spans matching the query.
    #[serde(rename = "rate", skip_serializing_if = "Option::is_none")]
    pub rate: Option<f64>,
}

impl RetentionFilterAttributes {
    /// The attributes of the retention filter.
    pub fn new() -> RetentionFilterAttributes {
        RetentionFilterAttributes {
            created_at: None,
            created_by: None,
            editable: None,
            enabled: None,
            execution_order: None,
            filter: None,
            filter_type: None,
            modified_at: None,
            modified_by: None,
            name: None,
            rate: None,
        }
    }
}
