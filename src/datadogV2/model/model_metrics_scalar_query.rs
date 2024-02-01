// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// An individual scalar metrics query.
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MetricsScalarQuery {
    /// The type of aggregation that can be performed on metrics-based queries.
    #[serde(rename = "aggregator")]
    pub aggregator: crate::datadogV2::model::MetricsAggregator,
    /// A data source that is powered by the Metrics platform.
    #[serde(rename = "data_source")]
    pub data_source: crate::datadogV2::model::MetricsDataSource,
    /// The variable name for use in formulas.
    #[serde(rename = "name")]
    pub name: Option<String>,
    /// A classic metrics query string.
    #[serde(rename = "query")]
    pub query: String,
}

impl MetricsScalarQuery {
    pub fn new(
        aggregator: crate::datadogV2::model::MetricsAggregator,
        data_source: crate::datadogV2::model::MetricsDataSource,
        query: String,
    ) -> MetricsScalarQuery {
        MetricsScalarQuery {
            aggregator,
            data_source,
            name: None,
            query,
        }
    }

    pub fn name(&mut self, value: String) -> &mut Self {
        self.name = Some(value);
        self
    }
}
