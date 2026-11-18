// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Attributes of a deleted segment.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct SegmentDeleteAttributes {
    /// Time the segment was deleted.
    #[serde(rename = "disabled_at")]
    pub disabled_at: chrono::DateTime<chrono::Utc>,
    /// A Datadog user who created, modified, or disabled a segment.
    #[serde(rename = "disabled_by")]
    pub disabled_by: Option<crate::datadogV2::model::SegmentUser>,
    /// Name of the segment.
    #[serde(rename = "name")]
    pub name: String,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl SegmentDeleteAttributes {
    pub fn new(
        disabled_at: chrono::DateTime<chrono::Utc>,
        name: String,
    ) -> SegmentDeleteAttributes {
        SegmentDeleteAttributes {
            disabled_at,
            disabled_by: None,
            name,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn disabled_by(mut self, value: crate::datadogV2::model::SegmentUser) -> Self {
        self.disabled_by = Some(value);
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

impl<'de> Deserialize<'de> for SegmentDeleteAttributes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct SegmentDeleteAttributesVisitor;
        impl<'a> Visitor<'a> for SegmentDeleteAttributesVisitor {
            type Value = SegmentDeleteAttributes;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut disabled_at: Option<chrono::DateTime<chrono::Utc>> = None;
                let mut disabled_by: Option<crate::datadogV2::model::SegmentUser> = None;
                let mut name: Option<String> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "disabled_at" => {
                            disabled_at =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "disabled_by" => {
                            if v.is_null() {
                                continue;
                            }
                            disabled_by =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "name" => {
                            name = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }
                let disabled_at =
                    disabled_at.ok_or_else(|| M::Error::missing_field("disabled_at"))?;
                let name = name.ok_or_else(|| M::Error::missing_field("name"))?;

                let content = SegmentDeleteAttributes {
                    disabled_at,
                    disabled_by,
                    name,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(SegmentDeleteAttributesVisitor)
    }
}
