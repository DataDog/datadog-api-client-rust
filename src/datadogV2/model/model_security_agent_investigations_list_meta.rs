// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Metadata of a page of investigations.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct SecurityAgentInvestigationsListMeta {
    /// Why `filter[signal_id]` returned no investigation. Set only on the first page of an empty result. The reason is
    /// based on the organization's current settings. Poll again only on `investigation_not_started`, after the number
    /// of seconds in the `Retry-After` header; treat the others as final for this signal.
    /// - `investigation_not_started`: no investigation has started yet, but one may still start.
    /// - `not_investigated`: no investigation started within an hour of the signal, so none is expected. The signal
    ///   was skipped for a reason this API doesn't report, such as its severity or a rate limit.
    /// - `rule_not_enabled`: the signal's detection rule doesn't have Bits Security Analyst enabled, so no
    ///   investigation starts automatically.
    /// - `signal_not_found`: the ID isn't a signal ID, or no signal with this ID exists in the organization.
    /// - `org_not_activated`: the organization hasn't activated Bits Security Analyst.
    #[serde(rename = "reason")]
    pub reason: Option<crate::datadogV2::model::SecurityAgentInvestigationNoRunReason>,
    /// Without `filter[signal_id]`, the number of signals that match the filters. With `filter[signal_id]`, the
    /// number of investigations of that signal. Pages past the last page return 0.
    #[serde(rename = "total_count")]
    pub total_count: i64,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl SecurityAgentInvestigationsListMeta {
    pub fn new(total_count: i64) -> SecurityAgentInvestigationsListMeta {
        SecurityAgentInvestigationsListMeta {
            reason: None,
            total_count,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn reason(
        mut self,
        value: crate::datadogV2::model::SecurityAgentInvestigationNoRunReason,
    ) -> Self {
        self.reason = Some(value);
        self
    }

    pub fn additional_properties(
        mut self,
        value: std::collections::BTreeMap<String, serde_json::Value>,
    ) -> Self {
        self.additional_properties = value;
        self
    }
}

impl<'de> Deserialize<'de> for SecurityAgentInvestigationsListMeta {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct SecurityAgentInvestigationsListMetaVisitor;
        impl<'a> Visitor<'a> for SecurityAgentInvestigationsListMetaVisitor {
            type Value = SecurityAgentInvestigationsListMeta;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut reason: Option<
                    crate::datadogV2::model::SecurityAgentInvestigationNoRunReason,
                > = None;
                let mut total_count: Option<i64> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "reason" => {
                            if v.is_null() {
                                continue;
                            }
                            reason = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _reason) = reason {
                                match _reason {
                                    crate::datadogV2::model::SecurityAgentInvestigationNoRunReason::UnparsedObject(_reason) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        }
                        "total_count" => {
                            total_count =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }
                let total_count =
                    total_count.ok_or_else(|| M::Error::missing_field("total_count"))?;

                let content = SecurityAgentInvestigationsListMeta {
                    reason,
                    total_count,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(SecurityAgentInvestigationsListMetaVisitor)
    }
}
