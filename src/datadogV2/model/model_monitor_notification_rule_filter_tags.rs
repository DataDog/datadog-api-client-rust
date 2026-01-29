// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Filters monitor notifications by a list of tag key:value pairs.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct MonitorNotificationRuleFilterTags {
    /// A list of tag key:value pairs (e.g. `team:product`). All tags must match (AND semantics).
    #[serde(rename = "tags")]
    pub tags: Vec<String>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl MonitorNotificationRuleFilterTags {
    pub fn new(tags: Vec<String>) -> MonitorNotificationRuleFilterTags {
        MonitorNotificationRuleFilterTags {
            tags,
            _unparsed: false,
        }
    }
}

impl<'de> Deserialize<'de> for MonitorNotificationRuleFilterTags {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct MonitorNotificationRuleFilterTagsVisitor;
        impl<'a> Visitor<'a> for MonitorNotificationRuleFilterTagsVisitor {
            type Value = MonitorNotificationRuleFilterTags;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut tags: Option<Vec<String>> = None;
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "tags" => {
                            tags = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            return Err(serde::de::Error::custom(
                                "Additional properties not allowed",
                            ));
                        }
                    }
                }
                let tags = tags.ok_or_else(|| M::Error::missing_field("tags"))?;

                let content = MonitorNotificationRuleFilterTags { tags, _unparsed };

                Ok(content)
            }
        }

        deserializer.deserialize_any(MonitorNotificationRuleFilterTagsVisitor)
    }
}
