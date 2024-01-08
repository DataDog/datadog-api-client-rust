// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// Number of RUM Units used for each hour for a given organization (data available as of November 1, 2021).
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct UsageRumUnitsHour {
    /// The number of browser RUM units.
    #[serde(
        rename = "browser_rum_units",
        default,
        with = "::serde_with::rust::double_option"
    )]
    pub browser_rum_units: Option<Option<i64>>,
    /// The number of mobile RUM units.
    #[serde(
        rename = "mobile_rum_units",
        default,
        with = "::serde_with::rust::double_option"
    )]
    pub mobile_rum_units: Option<Option<i64>>,
    /// The organization name.
    #[serde(rename = "org_name")]
    pub org_name: Option<String>,
    /// The organization public ID.
    #[serde(rename = "public_id")]
    pub public_id: Option<String>,
    /// Total RUM units across mobile and browser RUM.
    #[serde(
        rename = "rum_units",
        default,
        with = "::serde_with::rust::double_option"
    )]
    pub rum_units: Option<Option<i64>>,
}

impl UsageRumUnitsHour {
    pub fn new() -> UsageRumUnitsHour {
        UsageRumUnitsHour {
            browser_rum_units: None,
            mobile_rum_units: None,
            org_name: None,
            public_id: None,
            rum_units: None,
        }
    }
}
impl Default for UsageRumUnitsHour {
    fn default() -> Self {
        Self::new()
    }
}
