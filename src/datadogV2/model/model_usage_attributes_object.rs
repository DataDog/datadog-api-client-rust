// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// Usage attributes data.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct UsageAttributesObject {
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
    /// List of usage data reported for each requested hour.
    #[serde(rename = "timeseries")]
    pub timeseries: Option<Vec<crate::datadogV2::model::UsageTimeSeriesObject>>,
    /// Usage type that is being measured.
    #[serde(rename = "usage_type")]
    pub usage_type: Option<crate::datadogV2::model::HourlyUsageType>,
}

impl UsageAttributesObject {
    pub fn new() -> UsageAttributesObject {
        UsageAttributesObject {
            org_name: None,
            product_family: None,
            public_id: None,
            region: None,
            timeseries: None,
            usage_type: None,
        }
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

    pub fn timeseries(
        &mut self,
        value: Vec<crate::datadogV2::model::UsageTimeSeriesObject>,
    ) -> &mut Self {
        self.timeseries = Some(value);
        self
    }

    pub fn usage_type(&mut self, value: crate::datadogV2::model::HourlyUsageType) -> &mut Self {
        self.usage_type = Some(value);
        self
    }
}

impl Default for UsageAttributesObject {
    fn default() -> Self {
        Self::new()
    }
}
