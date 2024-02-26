// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// Attributes of hourly usage for a product family for an org for a time period.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct HourlyUsageAttributes {
    /// List of the measured usage values for the product family for the org for the time period.
    #[serde(rename = "measurements")]
    pub measurements: Option<Vec<crate::datadogV2::model::HourlyUsageMeasurement>>,
    /// The organization name.
    #[serde(rename = "org_name")]
    pub org_name: Option<String>,
    /// The product for which usage is being reported.
    #[serde(rename = "product_family")]
    pub product_family: Option<String>,
    /// The organization public ID.
    #[serde(rename = "public_id")]
    pub public_id: Option<String>,
    /// The region of the Datadog instance that the organization belongs to.
    #[serde(rename = "region")]
    pub region: Option<String>,
    /// Datetime in ISO-8601 format, UTC. The hour for the usage.
    #[serde(rename = "timestamp")]
    pub timestamp: Option<String>,
}

impl HourlyUsageAttributes {
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

    pub fn measurements(
        &mut self,
        value: Vec<crate::datadogV2::model::HourlyUsageMeasurement>,
    ) -> &mut Self {
        self.measurements = Some(value);
        self
    }

    pub fn org_name(&mut self, value: String) -> &mut Self {
        self.org_name = Some(value);
        self
    }

    pub fn product_family(&mut self, value: String) -> &mut Self {
        self.product_family = Some(value);
        self
    }

    pub fn public_id(&mut self, value: String) -> &mut Self {
        self.public_id = Some(value);
        self
    }

    pub fn region(&mut self, value: String) -> &mut Self {
        self.region = Some(value);
        self
    }

    pub fn timestamp(&mut self, value: String) -> &mut Self {
        self.timestamp = Some(value);
        self
    }
}

impl Default for HourlyUsageAttributes {
    fn default() -> Self {
        Self::new()
    }
}
