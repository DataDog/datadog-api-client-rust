// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Links attributes.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct ContainerImageGroupRelationshipsLinks {
    /// Link to related Container Images.
    #[serde(rename = "related")]
    pub related: Option<String>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl ContainerImageGroupRelationshipsLinks {
    pub fn new() -> ContainerImageGroupRelationshipsLinks {
        ContainerImageGroupRelationshipsLinks {
            related: None,
            _unparsed: false,
        }
    }

    pub fn related(mut self, value: String) -> Self {
        self.related = Some(value);
        self
    }
}

impl Default for ContainerImageGroupRelationshipsLinks {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for ContainerImageGroupRelationshipsLinks {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct ContainerImageGroupRelationshipsLinksVisitor;
        impl<'a> Visitor<'a> for ContainerImageGroupRelationshipsLinksVisitor {
            type Value = ContainerImageGroupRelationshipsLinks;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut related: Option<String> = None;
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "related" => {
                            if v.is_null() {
                                continue;
                            }
                            related = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {}
                    }
                }

                let content = ContainerImageGroupRelationshipsLinks { related, _unparsed };

                Ok(content)
            }
        }

        deserializer.deserialize_any(ContainerImageGroupRelationshipsLinksVisitor)
    }
}
