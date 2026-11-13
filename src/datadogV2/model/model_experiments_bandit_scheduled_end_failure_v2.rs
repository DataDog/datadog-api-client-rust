// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Why the scheduled end couldn't run. It isn't retried until the end or fallback changes. Absent when there's no failure.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct ExperimentsBanditScheduledEndFailureV2 {
    /// When the scheduled end failed.
    #[serde(rename = "failed_at")]
    pub failed_at: chrono::DateTime<chrono::Utc>,
    /// Why the scheduled end failed.
    #[serde(rename = "reason")]
    pub reason: String,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl ExperimentsBanditScheduledEndFailureV2 {
    pub fn new(
        failed_at: chrono::DateTime<chrono::Utc>,
        reason: String,
    ) -> ExperimentsBanditScheduledEndFailureV2 {
        ExperimentsBanditScheduledEndFailureV2 {
            failed_at,
            reason,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn additional_properties(
        mut self,
        value: std::collections::BTreeMap<String, serde_json::Value>,
    ) -> Self {
        self.additional_properties = value;
        self
    }
}

impl<'de> Deserialize<'de> for ExperimentsBanditScheduledEndFailureV2 {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct ExperimentsBanditScheduledEndFailureV2Visitor;
        impl<'a> Visitor<'a> for ExperimentsBanditScheduledEndFailureV2Visitor {
            type Value = ExperimentsBanditScheduledEndFailureV2;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut failed_at: Option<chrono::DateTime<chrono::Utc>> = None;
                let mut reason: Option<String> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "failed_at" => {
                            failed_at = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "reason" => {
                            reason = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }
                let failed_at = failed_at.ok_or_else(|| M::Error::missing_field("failed_at"))?;
                let reason = reason.ok_or_else(|| M::Error::missing_field("reason"))?;

                let content = ExperimentsBanditScheduledEndFailureV2 {
                    failed_at,
                    reason,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(ExperimentsBanditScheduledEndFailureV2Visitor)
    }
}
