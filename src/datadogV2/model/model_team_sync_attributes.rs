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
    /// How often the sync process should be run. Defaults to `once` when not provided.
    #[serde(rename = "frequency")]
    pub frequency: Option<crate::datadogV2::model::TeamSyncAttributesFrequency>,
    /// The external source platform for team synchronization. Only "github" is supported.
    #[serde(rename = "source")]
    pub source: crate::datadogV2::model::TeamSyncAttributesSource,
    /// Whether to sync members from the external team to the Datadog team. Defaults to `false` when not provided.
    #[serde(rename = "sync_membership")]
    pub sync_membership: Option<bool>,
    /// The type of synchronization operation. "link" connects teams by matching names. "provision" creates new teams when no match is found.
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
            frequency: None,
            source,
            sync_membership: None,
            type_,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn frequency(
        mut self,
        value: crate::datadogV2::model::TeamSyncAttributesFrequency,
    ) -> Self {
        self.frequency = Some(value);
        self
    }

    pub fn sync_membership(mut self, value: bool) -> Self {
        self.sync_membership = Some(value);
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
                let mut frequency: Option<crate::datadogV2::model::TeamSyncAttributesFrequency> =
                    None;
                let mut source: Option<crate::datadogV2::model::TeamSyncAttributesSource> = None;
                let mut sync_membership: Option<bool> = None;
                let mut type_: Option<crate::datadogV2::model::TeamSyncAttributesType> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "frequency" => {
                            if v.is_null() {
                                continue;
                            }
                            frequency = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _frequency) = frequency {
                                match _frequency {
                                    crate::datadogV2::model::TeamSyncAttributesFrequency::UnparsedObject(_frequency) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        }
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
                        "sync_membership" => {
                            if v.is_null() {
                                continue;
                            }
                            sync_membership =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
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
                    frequency,
                    source,
                    sync_membership,
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
