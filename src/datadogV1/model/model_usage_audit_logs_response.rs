// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Response containing the audit logs usage for each hour for a given organization.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct UsageAuditLogsResponse {
    /// Get hourly usage for audit logs.
    #[serde(rename = "usage")]
    pub usage: Option<Vec<crate::datadogV1::model::UsageAuditLogsHour>>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl UsageAuditLogsResponse {
    pub fn new() -> UsageAuditLogsResponse {
        UsageAuditLogsResponse {
            usage: None,
            _unparsed: false,
        }
    }

    pub fn usage(mut self, value: Vec<crate::datadogV1::model::UsageAuditLogsHour>) -> Self {
        self.usage = Some(value);
        self
    }
}

impl Default for UsageAuditLogsResponse {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for UsageAuditLogsResponse {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct UsageAuditLogsResponseVisitor;
        impl<'a> Visitor<'a> for UsageAuditLogsResponseVisitor {
            type Value = UsageAuditLogsResponse;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut usage: Option<Vec<crate::datadogV1::model::UsageAuditLogsHour>> = None;
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "usage" => {
                            if v.is_null() {
                                continue;
                            }
                            usage = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {}
                    }
                }

                let content = UsageAuditLogsResponse { usage, _unparsed };

                Ok(content)
            }
        }

        deserializer.deserialize_any(UsageAuditLogsResponseVisitor)
    }
}
