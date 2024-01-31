// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// Defined items in the group.
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct LogQueryDefinitionGroupBy {
    /// Facet name.
    #[serde(rename = "facet")]
    pub facet: String,
    /// Maximum number of items in the group.
    #[serde(rename = "limit")]
    pub limit: Option<i64>,
    /// Define a sorting method.
    #[serde(rename = "sort")]
    pub sort: Option<crate::datadogV1::model::LogQueryDefinitionGroupBySort>,
}

impl LogQueryDefinitionGroupBy {
    pub fn new(facet: String) -> LogQueryDefinitionGroupBy {
        LogQueryDefinitionGroupBy {
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
        value: crate::datadogV1::model::LogQueryDefinitionGroupBySort,
    ) -> &mut Self {
        self.sort = Some(value);
        self
    }
}
