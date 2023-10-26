// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct HourlyUsage {
    /// Attributes of hourly usage for a product family for an org for a time period.
    #[serde(rename = "attributes", skip_serializing_if = "Option::is_none")]
    pub attributes: Option<Box<crate::datadogV2::model::HourlyUsageAttributes>>,
    /// Unique ID of the response.
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// Type of usage data.
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub type_: Option<crate::datadogV2::model::UsageTimeSeriesType>,
}

impl HourlyUsage {
    /// Hourly usage for a product family for an org.
    pub fn new() -> HourlyUsage {
        HourlyUsage {
            attributes: None,
            id: None,
            type_: None,
        }
    }
}
