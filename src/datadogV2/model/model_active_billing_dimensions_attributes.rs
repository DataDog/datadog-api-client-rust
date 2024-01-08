// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// List of active billing dimensions.
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ActiveBillingDimensionsAttributes {
    /// Datetime in ISO-8601 format, UTC, precise to hour: `[YYYY-MM-DDThh]`.
    #[serde(rename = "month")]
    pub month: Option<String>,
    /// List of active billing dimensions. Example: `[infra_host, apm_host, serverless_infra]`.
    #[serde(rename = "values")]
    pub values: Option<Vec<String>>,
}

impl ActiveBillingDimensionsAttributes {
    pub fn new() -> ActiveBillingDimensionsAttributes {
        ActiveBillingDimensionsAttributes {
            month: None,
            values: None,
        }
    }
}
impl Default for ActiveBillingDimensionsAttributes {
    fn default() -> Self {
        Self::new()
    }
}
