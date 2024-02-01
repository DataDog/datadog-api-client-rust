// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// Hourly usage for a product family for an org.
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct HourlyUsage {
    /// Attributes of hourly usage for a product family for an org for a time period.
    #[serde(rename = "attributes")]
    pub attributes: Option<crate::datadogV2::model::HourlyUsageAttributes>,
    /// Unique ID of the response.
    #[serde(rename = "id")]
    pub id: Option<String>,
    /// Type of usage data.
    #[serde(rename = "type")]
    pub type_: Option<crate::datadogV2::model::UsageTimeSeriesType>,
}

impl HourlyUsage {
    pub fn new() -> HourlyUsage {
        HourlyUsage {
            attributes: None,
            id: None,
            type_: None,
        }
    }

    pub fn attributes(
        &mut self,
        value: crate::datadogV2::model::HourlyUsageAttributes,
    ) -> &mut Self {
        self.attributes = Some(value);
        self
    }

    pub fn id(&mut self, value: String) -> &mut Self {
        self.id = Some(value);
        self
    }

    pub fn type_(&mut self, value: crate::datadogV2::model::UsageTimeSeriesType) -> &mut Self {
        self.type_ = Some(value);
        self
    }
}

impl Default for HourlyUsage {
    fn default() -> Self {
        Self::new()
    }
}
