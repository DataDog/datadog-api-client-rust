// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Object of the monitor tags.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct DowntimeMonitorIdentifierTags {
    /// A list of monitor tags. For example, tags that are applied directly to monitors,
    /// not tags that are used in monitor queries (which are filtered by the scope parameter), to which the downtime applies.
    /// The resulting downtime applies to monitors that match **all** provided monitor tags. Setting `monitor_tags`
    /// to `[*]` configures the downtime to mute all monitors for the given scope.
    #[serde(rename = "monitor_tags")]
    pub monitor_tags: Vec<String>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl DowntimeMonitorIdentifierTags {
    pub fn new(monitor_tags: Vec<String>) -> DowntimeMonitorIdentifierTags {
        DowntimeMonitorIdentifierTags {
            monitor_tags,
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

impl<'de> Deserialize<'de> for DowntimeMonitorIdentifierTags {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct DowntimeMonitorIdentifierTagsVisitor;
        impl<'a> Visitor<'a> for DowntimeMonitorIdentifierTagsVisitor {
            type Value = DowntimeMonitorIdentifierTags;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut monitor_tags: Option<Vec<String>> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "monitor_tags" => {
                            monitor_tags =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }
                let monitor_tags =
                    monitor_tags.ok_or_else(|| M::Error::missing_field("monitor_tags"))?;

                let content = DowntimeMonitorIdentifierTags {
                    monitor_tags,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(DowntimeMonitorIdentifierTagsVisitor)
    }
}
