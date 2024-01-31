// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// A formula and functions events query.
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MonitorFormulaAndFunctionEventQueryDefinition {
    /// Compute options.
    #[serde(rename = "compute")]
    pub compute: crate::datadogV1::model::MonitorFormulaAndFunctionEventQueryDefinitionCompute,
    /// Data source for event platform-based queries.
    #[serde(rename = "data_source")]
    pub data_source: crate::datadogV1::model::MonitorFormulaAndFunctionEventsDataSource,
    /// Group by options.
    #[serde(rename = "group_by")]
    pub group_by: Option<Vec<crate::datadogV1::model::MonitorFormulaAndFunctionEventQueryGroupBy>>,
    /// An array of index names to query in the stream. Omit or use `[]` to query all indexes at once.
    #[serde(rename = "indexes")]
    pub indexes: Option<Vec<String>>,
    /// Name of the query for use in formulas.
    #[serde(rename = "name")]
    pub name: String,
    /// Search options.
    #[serde(rename = "search")]
    pub search:
        Option<crate::datadogV1::model::MonitorFormulaAndFunctionEventQueryDefinitionSearch>,
}

impl MonitorFormulaAndFunctionEventQueryDefinition {
    pub fn new(
        compute: crate::datadogV1::model::MonitorFormulaAndFunctionEventQueryDefinitionCompute,
        data_source: crate::datadogV1::model::MonitorFormulaAndFunctionEventsDataSource,
        name: String,
    ) -> MonitorFormulaAndFunctionEventQueryDefinition {
        MonitorFormulaAndFunctionEventQueryDefinition {
            compute,
            data_source,
            group_by: None,
            indexes: None,
            name,
            search: None,
        }
    }

    pub fn with_group_by(
        &mut self,
        value: Vec<crate::datadogV1::model::MonitorFormulaAndFunctionEventQueryGroupBy>,
    ) -> &mut Self {
        self.group_by = Some(value);
        self
    }

    pub fn with_indexes(&mut self, value: Vec<String>) -> &mut Self {
        self.indexes = Some(value);
        self
    }

    pub fn with_search(
        &mut self,
        value: crate::datadogV1::model::MonitorFormulaAndFunctionEventQueryDefinitionSearch,
    ) -> &mut Self {
        self.search = Some(value);
        self
    }
}
