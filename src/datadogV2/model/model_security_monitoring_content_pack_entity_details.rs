// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Details for an entity or identity content pack.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct SecurityMonitoringContentPackEntityDetails {
    /// The activation status of a content pack.
    #[serde(rename = "cp_activation")]
    pub cp_activation: crate::datadogV2::model::SecurityMonitoringContentPackActivation,
    /// Type for entity content pack details.
    #[serde(rename = "type")]
    pub type_: crate::datadogV2::model::SecurityMonitoringContentPackEntityDetailsType,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl SecurityMonitoringContentPackEntityDetails {
    pub fn new(
        cp_activation: crate::datadogV2::model::SecurityMonitoringContentPackActivation,
        type_: crate::datadogV2::model::SecurityMonitoringContentPackEntityDetailsType,
    ) -> SecurityMonitoringContentPackEntityDetails {
        SecurityMonitoringContentPackEntityDetails {
            cp_activation,
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

impl<'de> Deserialize<'de> for SecurityMonitoringContentPackEntityDetails {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct SecurityMonitoringContentPackEntityDetailsVisitor;
        impl<'a> Visitor<'a> for SecurityMonitoringContentPackEntityDetailsVisitor {
            type Value = SecurityMonitoringContentPackEntityDetails;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut cp_activation: Option<
                    crate::datadogV2::model::SecurityMonitoringContentPackActivation,
                > = None;
                let mut type_: Option<
                    crate::datadogV2::model::SecurityMonitoringContentPackEntityDetailsType,
                > = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "cp_activation" => {
                            cp_activation =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _cp_activation) = cp_activation {
                                match _cp_activation {
                                    crate::datadogV2::model::SecurityMonitoringContentPackActivation::UnparsedObject(_cp_activation) => {
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
                                    crate::datadogV2::model::SecurityMonitoringContentPackEntityDetailsType::UnparsedObject(_type_) => {
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
                let cp_activation =
                    cp_activation.ok_or_else(|| M::Error::missing_field("cp_activation"))?;
                let type_ = type_.ok_or_else(|| M::Error::missing_field("type_"))?;

                let content = SecurityMonitoringContentPackEntityDetails {
                    cp_activation,
                    type_,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(SecurityMonitoringContentPackEntityDetailsVisitor)
    }
}
