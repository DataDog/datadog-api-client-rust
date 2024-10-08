// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// CI Pipelines association.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct EntityV3DatadogPipelines {
    /// A list of CI Fingerprints that associate CI Pipelines with the entity.
    #[serde(rename = "fingerprints")]
    pub fingerprints: Option<Vec<String>>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl EntityV3DatadogPipelines {
    pub fn new() -> EntityV3DatadogPipelines {
        EntityV3DatadogPipelines {
            fingerprints: None,
            _unparsed: false,
        }
    }

    pub fn fingerprints(mut self, value: Vec<String>) -> Self {
        self.fingerprints = Some(value);
        self
    }
}

impl Default for EntityV3DatadogPipelines {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for EntityV3DatadogPipelines {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct EntityV3DatadogPipelinesVisitor;
        impl<'a> Visitor<'a> for EntityV3DatadogPipelinesVisitor {
            type Value = EntityV3DatadogPipelines;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut fingerprints: Option<Vec<String>> = None;
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "fingerprints" => {
                            if v.is_null() {
                                continue;
                            }
                            fingerprints =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            return Err(serde::de::Error::custom(
                                "Additional properties not allowed",
                            ));
                        }
                    }
                }

                let content = EntityV3DatadogPipelines {
                    fingerprints,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(EntityV3DatadogPipelinesVisitor)
    }
}
