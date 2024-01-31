// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// Ingested spans usage for a given organization for a given hour.
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct UsageIngestedSpansHour {
    /// The hour for the usage.
    #[serde(rename = "hour")]
    pub hour: Option<String>,
    /// Contains the total number of bytes ingested for APM spans during a given hour.
    #[serde(
        rename = "ingested_events_bytes",
        default,
        with = "::serde_with::rust::double_option"
    )]
    pub ingested_events_bytes: Option<Option<i64>>,
    /// The organization name.
    #[serde(rename = "org_name")]
    pub org_name: Option<String>,
    /// The organization public ID.
    #[serde(rename = "public_id")]
    pub public_id: Option<String>,
}

impl UsageIngestedSpansHour {
    pub fn new() -> UsageIngestedSpansHour {
        UsageIngestedSpansHour {
            hour: None,
            ingested_events_bytes: None,
            org_name: None,
            public_id: None,
        }
    }

    pub fn with_hour(&mut self, value: String) -> &mut Self {
        self.hour = Some(value);
        self
    }

    pub fn with_ingested_events_bytes(&mut self, value: Option<i64>) -> &mut Self {
        self.ingested_events_bytes = Some(value);
        self
    }

    pub fn with_org_name(&mut self, value: String) -> &mut Self {
        self.org_name = Some(value);
        self
    }

    pub fn with_public_id(&mut self, value: String) -> &mut Self {
        self.public_id = Some(value);
        self
    }
}
impl Default for UsageIngestedSpansHour {
    fn default() -> Self {
        Self::new()
    }
}
