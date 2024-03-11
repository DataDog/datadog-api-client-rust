// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Metadata for the resulting numerical values.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct ScalarMeta {
    /// Detailed information about the unit.
    /// First element describes the "primary unit" (for example, `bytes` in `bytes per second`).
    /// The second element describes the "per unit" (for example, `second` in `bytes per second`).
    /// If the second element is not present, the API returns null.
    #[serde(rename = "unit", default, with = "::serde_with::rust::double_option")]
    pub unit: Option<Option<Vec<Option<crate::datadogV2::model::Unit>>>>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl ScalarMeta {
    pub fn new() -> ScalarMeta {
        ScalarMeta {
            unit: None,
            _unparsed: false,
        }
    }

    pub fn unit(&mut self, value: Option<Vec<Option<crate::datadogV2::model::Unit>>>) -> &mut Self {
        self.unit = Some(value);
        self
    }
}

impl Default for ScalarMeta {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for ScalarMeta {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct ScalarMetaVisitor;
        impl<'a> Visitor<'a> for ScalarMetaVisitor {
            type Value = ScalarMeta;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut unit: Option<Option<Vec<Option<crate::datadogV2::model::Unit>>>> = None;
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "unit" => {
                            unit = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {}
                    }
                }

                let content = ScalarMeta { unit, _unparsed };

                Ok(content)
            }
        }

        deserializer.deserialize_any(ScalarMetaVisitor)
    }
}
