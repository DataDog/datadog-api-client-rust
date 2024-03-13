// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// In this object, the key is the tag, the value is a list of host names that are reporting that tag.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct TagToHosts {
    /// A list of tags to apply to the host.
    #[serde(rename = "tags")]
    pub tags: Option<std::collections::BTreeMap<String, Vec<String>>>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl TagToHosts {
    pub fn new() -> TagToHosts {
        TagToHosts {
            tags: None,
            _unparsed: false,
        }
    }

    pub fn tags(mut self, value: std::collections::BTreeMap<String, Vec<String>>) -> Self {
        self.tags = Some(value);
        self
    }
}

impl Default for TagToHosts {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for TagToHosts {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct TagToHostsVisitor;
        impl<'a> Visitor<'a> for TagToHostsVisitor {
            type Value = TagToHosts;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut tags: Option<std::collections::BTreeMap<String, Vec<String>>> = None;
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "tags" => {
                            if v.is_null() {
                                continue;
                            }
                            tags = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {}
                    }
                }

                let content = TagToHosts { tags, _unparsed };

                Ok(content)
            }
        }

        deserializer.deserialize_any(TagToHostsVisitor)
    }
}
