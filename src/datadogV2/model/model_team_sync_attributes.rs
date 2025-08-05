// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Team sync attributes.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct TeamSyncAttributes {
    /// The external source platform for team synchronization. Only "github" is supported.
    #[serde(rename = "source")]
    pub source: crate::datadogV2::model::TeamSyncAttributesSource,
    /// The type of synchronization operation. Only "link" is supported, which links existing teams by matching names.
    #[serde(rename = "type")]
    pub type_: crate::datadogV2::model::TeamSyncAttributesType,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl TeamSyncAttributes {
    pub fn new(
        source: crate::datadogV2::model::TeamSyncAttributesSource,
        type_: crate::datadogV2::model::TeamSyncAttributesType,
    ) -> TeamSyncAttributes {
        TeamSyncAttributes {
            source,
            type_,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn additional_properties(
        mut self,
        value: std::collections::BTreeMap<String, serde_json::Value>,
    ) -> Self {
        self.additional_properties = value;
        self
    }
}

impl<'de> Deserialize<'de> for TeamSyncAttributes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct TeamSyncAttributesVisitor;
        impl<'a> Visitor<'a> for TeamSyncAttributesVisitor {
            type Value = TeamSyncAttributes;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut source: Option<crate::datadogV2::model::TeamSyncAttributesSource> = None;
                let mut type_: Option<crate::datadogV2::model::TeamSyncAttributesType> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "source" => {
                            source = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _source) = source {
                                match _source {
                                    crate::datadogV2::model::TeamSyncAttributesSource::UnparsedObject(_source) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        }
                        "type" => {
                            type_ = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _type_) = type_ {
                                match _type_ {
                                    crate::datadogV2::model::TeamSyncAttributesType::UnparsedObject(_type_) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }
                let source = source.ok_or_else(|| M::Error::missing_field("source"))?;
                let type_ = type_.ok_or_else(|| M::Error::missing_field("type_"))?;

                let content = TeamSyncAttributes {
                    source,
                    type_,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(TeamSyncAttributesVisitor)
    }
}
