// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Number of errors reported by each source. Several sources can report the same error, such as an APM span and
/// a log, so the counts overlap: don't add them together.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct IssueImpactErrorCounts {
    /// Number of errors reported by APM.
    #[serde(rename = "apm")]
    pub apm: i64,
    /// Number of errors reported by Agent Observability.
    #[serde(rename = "llmobs")]
    pub llmobs: i64,
    /// Number of errors reported by Log Management.
    #[serde(rename = "logs")]
    pub logs: i64,
    /// Number of errors reported by RUM.
    #[serde(rename = "rum")]
    pub rum: i64,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl IssueImpactErrorCounts {
    pub fn new(apm: i64, llmobs: i64, logs: i64, rum: i64) -> IssueImpactErrorCounts {
        IssueImpactErrorCounts {
            apm,
            llmobs,
            logs,
            rum,
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

impl<'de> Deserialize<'de> for IssueImpactErrorCounts {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct IssueImpactErrorCountsVisitor;
        impl<'a> Visitor<'a> for IssueImpactErrorCountsVisitor {
            type Value = IssueImpactErrorCounts;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut apm: Option<i64> = None;
                let mut llmobs: Option<i64> = None;
                let mut logs: Option<i64> = None;
                let mut rum: Option<i64> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "apm" => {
                            apm = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "llmobs" => {
                            llmobs = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "logs" => {
                            logs = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "rum" => {
                            rum = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }
                let apm = apm.ok_or_else(|| M::Error::missing_field("apm"))?;
                let llmobs = llmobs.ok_or_else(|| M::Error::missing_field("llmobs"))?;
                let logs = logs.ok_or_else(|| M::Error::missing_field("logs"))?;
                let rum = rum.ok_or_else(|| M::Error::missing_field("rum"))?;

                let content = IssueImpactErrorCounts {
                    apm,
                    llmobs,
                    logs,
                    rum,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(IssueImpactErrorCountsVisitor)
    }
}
