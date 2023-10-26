// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct HourlyUsageAttributes {
    /// List of the measured usage values for the product family for the org for the time period.
    #[serde(rename = "measurements", skip_serializing_if = "Option::is_none")]
    pub measurements: Option<Vec<crate::datadogV2::model::HourlyUsageMeasurement>>,
    /// The organization name.
    #[serde(rename = "org_name", skip_serializing_if = "Option::is_none")]
    pub org_name: Option<String>,
    /// The product for which usage is being reported.
    #[serde(rename = "product_family", skip_serializing_if = "Option::is_none")]
    pub product_family: Option<String>,
    /// The organization public ID.
    #[serde(rename = "public_id", skip_serializing_if = "Option::is_none")]
    pub public_id: Option<String>,
    /// The region of the Datadog instance that the organization belongs to.
    #[serde(rename = "region", skip_serializing_if = "Option::is_none")]
    pub region: Option<String>,
    /// Datetime in ISO-8601 format, UTC. The hour for the usage.
    #[serde(rename = "timestamp", skip_serializing_if = "Option::is_none")]
    pub timestamp: Option<String>,
}

impl HourlyUsageAttributes {
    /// Attributes of hourly usage for a product family for an org for a time period.
    pub fn new() -> HourlyUsageAttributes {
        HourlyUsageAttributes {
            measurements: None,
            org_name: None,
            product_family: None,
            public_id: None,
            region: None,
            timestamp: None,
        }
    }
}
