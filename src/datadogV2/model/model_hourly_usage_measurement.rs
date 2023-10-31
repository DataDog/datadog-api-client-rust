// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// Usage amount for a given usage type.
#[skip_serializing_none]
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct HourlyUsageMeasurement {
    /// Type of usage.
    #[serde(rename = "usage_type")]
    pub usage_type: Option<String>,
    /// Contains the number measured for the given usage_type during the hour.
    #[serde(rename = "value", default, with = "::serde_with::rust::double_option")]
    pub value: Option<Option<i64>>,
}

impl HourlyUsageMeasurement {
    pub fn new() -> HourlyUsageMeasurement {
        HourlyUsageMeasurement {
            usage_type: None,
            value: None,
        }
    }
}
