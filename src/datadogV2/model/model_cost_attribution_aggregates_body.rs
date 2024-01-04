// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// The object containing the aggregates.
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CostAttributionAggregatesBody {
    /// The aggregate type.
    #[serde(rename = "agg_type")]
    pub agg_type: Option<String>,
    /// The field.
    #[serde(rename = "field")]
    pub field: Option<String>,
    /// The value for a given field.
    #[serde(rename = "value")]
    pub value: Option<f64>,
}

impl CostAttributionAggregatesBody {
    pub fn new() -> CostAttributionAggregatesBody {
        CostAttributionAggregatesBody {
            agg_type: None,
            field: None,
            value: None,
        }
    }
}
