// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// A metric SLI specification.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct SLOCountSpec {
    /// A count-based (metric) SLI specification, composed of three parts: the good events formula, the bad or total events formula, and the underlying queries.
    #[serde(rename = "count")]
    pub count: crate::datadogV1::model::SLOCountDefinition,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl SLOCountSpec {
    pub fn new(count: crate::datadogV1::model::SLOCountDefinition) -> SLOCountSpec {
        SLOCountSpec {
            count,
            _unparsed: false,
        }
    }
}

impl<'de> Deserialize<'de> for SLOCountSpec {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct SLOCountSpecVisitor;
        impl<'a> Visitor<'a> for SLOCountSpecVisitor {
            type Value = SLOCountSpec;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut count: Option<crate::datadogV1::model::SLOCountDefinition> = None;
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "count" => {
                            count = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            return Err(serde::de::Error::custom(
                                "Additional properties not allowed",
                            ));
                        }
                    }
                }
                let count = count.ok_or_else(|| M::Error::missing_field("count"))?;

                let content = SLOCountSpec { count, _unparsed };

                Ok(content)
            }
        }

        deserializer.deserialize_any(SLOCountSpecVisitor)
    }
}
