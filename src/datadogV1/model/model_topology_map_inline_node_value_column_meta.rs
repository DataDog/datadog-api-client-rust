// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Display options for a value column.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct TopologyMapInlineNodeValueColumnMeta {
    /// Number format unit.
    #[serde(rename = "unit")]
    pub unit: Option<crate::datadogV1::model::NumberFormatUnit>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl TopologyMapInlineNodeValueColumnMeta {
    pub fn new() -> TopologyMapInlineNodeValueColumnMeta {
        TopologyMapInlineNodeValueColumnMeta {
            unit: None,
            _unparsed: false,
        }
    }

    pub fn unit(mut self, value: crate::datadogV1::model::NumberFormatUnit) -> Self {
        self.unit = Some(value);
        self
    }
}

impl Default for TopologyMapInlineNodeValueColumnMeta {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for TopologyMapInlineNodeValueColumnMeta {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct TopologyMapInlineNodeValueColumnMetaVisitor;
        impl<'a> Visitor<'a> for TopologyMapInlineNodeValueColumnMetaVisitor {
            type Value = TopologyMapInlineNodeValueColumnMeta;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut unit: Option<crate::datadogV1::model::NumberFormatUnit> = None;
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "unit" => {
                            if v.is_null() {
                                continue;
                            }
                            unit = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _unit) = unit {
                                match _unit {
                                    crate::datadogV1::model::NumberFormatUnit::UnparsedObject(
                                        _unit,
                                    ) => {
                                        _unparsed = true;
                                    }
                                    _ => {}
                                }
                            }
                        }
                        &_ => {
                            return Err(serde::de::Error::custom(
                                "Additional properties not allowed",
                            ));
                        }
                    }
                }

                let content = TopologyMapInlineNodeValueColumnMeta { unit, _unparsed };

                Ok(content)
            }
        }

        deserializer.deserialize_any(TopologyMapInlineNodeValueColumnMetaVisitor)
    }
}
