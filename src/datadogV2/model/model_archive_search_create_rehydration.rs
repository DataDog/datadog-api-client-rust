// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Rehydration settings. Include this object to index the matched logs into a retained historical view.
/// Omit it to run an Archive Search that only scans the archive.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct ArchiveSearchCreateRehydration {
    /// Maximum number of events to rehydrate. The organization's reindexing limit is configured
    /// in millions, so this value is at least 1,000,000.
    #[serde(rename = "max_rehydrated_events")]
    pub max_rehydrated_events: i64,
    /// Number of days the rehydrated logs are retained for.
    #[serde(rename = "retention_days")]
    pub retention_days: i64,
    /// Storage tier the matched logs are rehydrated into.
    #[serde(rename = "tier")]
    pub tier: crate::datadogV2::model::ArchiveSearchRehydrationTier,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl ArchiveSearchCreateRehydration {
    pub fn new(
        max_rehydrated_events: i64,
        retention_days: i64,
        tier: crate::datadogV2::model::ArchiveSearchRehydrationTier,
    ) -> ArchiveSearchCreateRehydration {
        ArchiveSearchCreateRehydration {
            max_rehydrated_events,
            retention_days,
            tier,
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

impl<'de> Deserialize<'de> for ArchiveSearchCreateRehydration {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct ArchiveSearchCreateRehydrationVisitor;
        impl<'a> Visitor<'a> for ArchiveSearchCreateRehydrationVisitor {
            type Value = ArchiveSearchCreateRehydration;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut max_rehydrated_events: Option<i64> = None;
                let mut retention_days: Option<i64> = None;
                let mut tier: Option<crate::datadogV2::model::ArchiveSearchRehydrationTier> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "max_rehydrated_events" => {
                            max_rehydrated_events =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "retention_days" => {
                            retention_days =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "tier" => {
                            tier = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _tier) = tier {
                                match _tier {
                                    crate::datadogV2::model::ArchiveSearchRehydrationTier::UnparsedObject(_tier) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }
                let max_rehydrated_events = max_rehydrated_events
                    .ok_or_else(|| M::Error::missing_field("max_rehydrated_events"))?;
                let retention_days =
                    retention_days.ok_or_else(|| M::Error::missing_field("retention_days"))?;
                let tier = tier.ok_or_else(|| M::Error::missing_field("tier"))?;

                let content = ArchiveSearchCreateRehydration {
                    max_rehydrated_events,
                    retention_days,
                    tier,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(ArchiveSearchCreateRehydrationVisitor)
    }
}
