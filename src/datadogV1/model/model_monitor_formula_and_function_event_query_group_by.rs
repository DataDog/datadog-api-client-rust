// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// List of objects used to group by.
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MonitorFormulaAndFunctionEventQueryGroupBy {
    /// Event facet.
    #[serde(rename = "facet")]
    pub facet: String,
    /// Number of groups to return.
    #[serde(rename = "limit")]
    pub limit: Option<i64>,
    /// Options for sorting group by results.
    #[serde(rename = "sort")]
    pub sort: Option<crate::datadogV1::model::MonitorFormulaAndFunctionEventQueryGroupBySort>,
}

impl MonitorFormulaAndFunctionEventQueryGroupBy {
    pub fn new(facet: String) -> MonitorFormulaAndFunctionEventQueryGroupBy {
        MonitorFormulaAndFunctionEventQueryGroupBy {
            facet,
            limit: None,
            sort: None,
        }
    }

    pub fn with_limit(&mut self, value: i64) -> &mut Self {
        self.limit = Some(value);
        self
    }

    pub fn with_sort(
        &mut self,
        value: crate::datadogV1::model::MonitorFormulaAndFunctionEventQueryGroupBySort,
    ) -> &mut Self {
        self.sort = Some(value);
        self
    }
}
