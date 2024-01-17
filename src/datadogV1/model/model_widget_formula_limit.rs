// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// Options for limiting results returned.
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct WidgetFormulaLimit {
    /// Number of results to return.
    #[serde(rename = "count")]
    pub count: Option<i64>,
    /// Direction of sort.
    #[serde(rename = "order")]
    pub order: Option<crate::datadogV1::model::QuerySortOrder>,
}

impl WidgetFormulaLimit {
    pub fn new() -> WidgetFormulaLimit {
        WidgetFormulaLimit {
            count: None,
            order: None,
        }
    }
}
impl Default for WidgetFormulaLimit {
    fn default() -> Self {
        Self::new()
    }
}
