// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// A formula and functions Cloud Cost query.
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct FormulaAndFunctionCloudCostQueryDefinition {
    /// Aggregator used for the request.
    #[serde(rename = "aggregator")]
    pub aggregator: Option<crate::datadogV1::model::WidgetAggregator>,
    /// Data source for Cloud Cost queries.
    #[serde(rename = "data_source")]
    pub data_source: crate::datadogV1::model::FormulaAndFunctionCloudCostDataSource,
    /// Name of the query for use in formulas.
    #[serde(rename = "name")]
    pub name: String,
    /// Query for Cloud Cost data.
    #[serde(rename = "query")]
    pub query: String,
}

impl FormulaAndFunctionCloudCostQueryDefinition {
    pub fn new(
        data_source: crate::datadogV1::model::FormulaAndFunctionCloudCostDataSource,
        name: String,
        query: String,
    ) -> FormulaAndFunctionCloudCostQueryDefinition {
        FormulaAndFunctionCloudCostQueryDefinition {
            aggregator: None,
            data_source,
            name,
            query,
        }
    }

    pub fn aggregator(&mut self, value: crate::datadogV1::model::WidgetAggregator) -> &mut Self {
        self.aggregator = Some(value);
        self
    }
}
