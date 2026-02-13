// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// The definition of `DataAttributesRulesItemsMapping` object.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct DataAttributesRulesItemsMapping {
    /// The `mapping` `destination_key`.
    #[serde(rename = "destination_key")]
    pub destination_key: String,
    /// Deprecated. Use `if_tag_exists` instead. The `mapping` `if_not_exists`.
    #[deprecated]
    #[serde(rename = "if_not_exists")]
    pub if_not_exists: Option<bool>,
    /// The behavior when the tag already exists.
    #[serde(rename = "if_tag_exists")]
    pub if_tag_exists: Option<crate::datadogV2::model::DataAttributesRulesItemsIfTagExists>,
    /// The `mapping` `source_keys`.
    #[serde(rename = "source_keys")]
    pub source_keys: Vec<String>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl DataAttributesRulesItemsMapping {
    pub fn new(
        destination_key: String,
        source_keys: Vec<String>,
    ) -> DataAttributesRulesItemsMapping {
        #[allow(deprecated)]
        DataAttributesRulesItemsMapping {
            destination_key,
            if_not_exists: None,
            if_tag_exists: None,
            source_keys,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    #[allow(deprecated)]
    pub fn if_not_exists(mut self, value: bool) -> Self {
        self.if_not_exists = Some(value);
        self
    }

    #[allow(deprecated)]
    pub fn if_tag_exists(
        mut self,
        value: crate::datadogV2::model::DataAttributesRulesItemsIfTagExists,
    ) -> Self {
        self.if_tag_exists = Some(value);
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

impl<'de> Deserialize<'de> for DataAttributesRulesItemsMapping {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct DataAttributesRulesItemsMappingVisitor;
        impl<'a> Visitor<'a> for DataAttributesRulesItemsMappingVisitor {
            type Value = DataAttributesRulesItemsMapping;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut destination_key: Option<String> = None;
                let mut if_not_exists: Option<bool> = None;
                let mut if_tag_exists: Option<
                    crate::datadogV2::model::DataAttributesRulesItemsIfTagExists,
                > = None;
                let mut source_keys: Option<Vec<String>> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "destination_key" => {
                            destination_key =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "if_not_exists" => {
                            if v.is_null() {
                                continue;
                            }
                            if_not_exists =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "if_tag_exists" => {
                            if v.is_null() {
                                continue;
                            }
                            if_tag_exists =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _if_tag_exists) = if_tag_exists {
                                match _if_tag_exists {
                                    crate::datadogV2::model::DataAttributesRulesItemsIfTagExists::UnparsedObject(_if_tag_exists) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        }
                        "source_keys" => {
                            source_keys =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }
                let destination_key =
                    destination_key.ok_or_else(|| M::Error::missing_field("destination_key"))?;
                let source_keys =
                    source_keys.ok_or_else(|| M::Error::missing_field("source_keys"))?;

                #[allow(deprecated)]
                let content = DataAttributesRulesItemsMapping {
                    destination_key,
                    if_not_exists,
                    if_tag_exists,
                    source_keys,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(DataAttributesRulesItemsMappingVisitor)
    }
}
