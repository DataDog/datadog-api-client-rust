// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Only supported for `proportional_timeseries` and `even_timeseries` rules. Applies tag key renames and tag value transforms to timeseries input rows after `mapping_transform`.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct ArbitraryRuleTagTransforms {
    /// Map of source tag key to renamed tag key.
    #[serde(rename = "tag_key_rename")]
    pub tag_key_rename: Option<std::collections::BTreeMap<String, String>>,
    /// Map of tag key to the transform applied to its values.
    #[serde(rename = "tag_value_transforms")]
    pub tag_value_transforms: Option<
        std::collections::BTreeMap<String, crate::datadogV2::model::ArbitraryRuleTagValueTransform>,
    >,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl ArbitraryRuleTagTransforms {
    pub fn new() -> ArbitraryRuleTagTransforms {
        ArbitraryRuleTagTransforms {
            tag_key_rename: None,
            tag_value_transforms: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn tag_key_rename(mut self, value: std::collections::BTreeMap<String, String>) -> Self {
        self.tag_key_rename = Some(value);
        self
    }

    pub fn tag_value_transforms(
        mut self,
        value: std::collections::BTreeMap<
            String,
            crate::datadogV2::model::ArbitraryRuleTagValueTransform,
        >,
    ) -> Self {
        self.tag_value_transforms = Some(value);
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

impl Default for ArbitraryRuleTagTransforms {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for ArbitraryRuleTagTransforms {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct ArbitraryRuleTagTransformsVisitor;
        impl<'a> Visitor<'a> for ArbitraryRuleTagTransformsVisitor {
            type Value = ArbitraryRuleTagTransforms;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut tag_key_rename: Option<std::collections::BTreeMap<String, String>> = None;
                let mut tag_value_transforms: Option<
                    std::collections::BTreeMap<
                        String,
                        crate::datadogV2::model::ArbitraryRuleTagValueTransform,
                    >,
                > = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "tag_key_rename" => {
                            if v.is_null() {
                                continue;
                            }
                            tag_key_rename =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "tag_value_transforms" => {
                            if v.is_null() {
                                continue;
                            }
                            tag_value_transforms =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }

                let content = ArbitraryRuleTagTransforms {
                    tag_key_rename,
                    tag_value_transforms,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(ArbitraryRuleTagTransformsVisitor)
    }
}
