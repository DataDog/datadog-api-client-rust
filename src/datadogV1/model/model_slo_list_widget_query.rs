// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// Updated SLO List widget.
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SLOListWidgetQuery {
    /// Maximum number of results to display in the table.
    #[serde(rename = "limit")]
    pub limit: Option<i64>,
    /// Widget query.
    #[serde(rename = "query_string")]
    pub query_string: String,
    /// Options for sorting results.
    #[serde(rename = "sort")]
    pub sort: Option<Vec<crate::datadogV1::model::WidgetFieldSort>>,
}

impl SLOListWidgetQuery {
    pub fn new(query_string: String) -> SLOListWidgetQuery {
        SLOListWidgetQuery {
            limit: None,
            query_string,
            sort: None,
        }
    }

    pub fn with_limit(&mut self, value: i64) -> &mut Self {
        self.limit = Some(value);
        self
    }

    pub fn with_sort(&mut self, value: Vec<crate::datadogV1::model::WidgetFieldSort>) -> &mut Self {
        self.sort = Some(value);
        self
    }
}
