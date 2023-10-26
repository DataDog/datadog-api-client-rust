// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct UsageAttributesObject {
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
    /// List of usage data reported for each requested hour.
    #[serde(rename = "timeseries", skip_serializing_if = "Option::is_none")]
    pub timeseries: Option<Vec<crate::datadogV2::model::UsageTimeSeriesObject>>,
    /// Usage type that is being measured.
    #[serde(rename = "usage_type", skip_serializing_if = "Option::is_none")]
    pub usage_type: Option<crate::datadogV2::model::HourlyUsageType>,
}

impl UsageAttributesObject {
    /// Usage attributes data.
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
}
