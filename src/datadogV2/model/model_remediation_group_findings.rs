// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Counts of the group's open findings. The counts include every finding in the group, including findings that
/// don't match `filter[query]`.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct RemediationGroupFindings {
    /// Number of the group's findings for each value of their `@severity` attribute. The response includes every
    /// level, including levels with a count of zero.
    #[serde(rename = "by_severity")]
    pub by_severity: crate::datadogV2::model::RemediationGroupFindingsBySeverity,
    /// Number of open findings in the group.
    #[serde(rename = "total")]
    pub total: i64,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl RemediationGroupFindings {
    pub fn new(
        by_severity: crate::datadogV2::model::RemediationGroupFindingsBySeverity,
        total: i64,
    ) -> RemediationGroupFindings {
        RemediationGroupFindings {
            by_severity,
            total,
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

impl<'de> Deserialize<'de> for RemediationGroupFindings {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct RemediationGroupFindingsVisitor;
        impl<'a> Visitor<'a> for RemediationGroupFindingsVisitor {
            type Value = RemediationGroupFindings;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut by_severity: Option<
                    crate::datadogV2::model::RemediationGroupFindingsBySeverity,
                > = None;
                let mut total: Option<i64> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "by_severity" => {
                            by_severity =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "total" => {
                            total = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }
                let by_severity =
                    by_severity.ok_or_else(|| M::Error::missing_field("by_severity"))?;
                let total = total.ok_or_else(|| M::Error::missing_field("total"))?;

                let content = RemediationGroupFindings {
                    by_severity,
                    total,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(RemediationGroupFindingsVisitor)
    }
}
