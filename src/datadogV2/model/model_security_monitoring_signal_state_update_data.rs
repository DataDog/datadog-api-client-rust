// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Data containing the patch for changing the state of a signal.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct SecurityMonitoringSignalStateUpdateData {
    /// Attributes describing the change of state of a security signal.
    #[serde(rename = "attributes")]
    pub attributes: crate::datadogV2::model::SecurityMonitoringSignalStateUpdateAttributes,
    /// The unique ID of the security signal.
    #[serde(rename = "id")]
    pub id: Option<serde_json::Value>,
    /// The type of event.
    #[serde(rename = "type")]
    pub type_: Option<crate::datadogV2::model::SecurityMonitoringSignalMetadataType>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl SecurityMonitoringSignalStateUpdateData {
    pub fn new(
        attributes: crate::datadogV2::model::SecurityMonitoringSignalStateUpdateAttributes,
    ) -> SecurityMonitoringSignalStateUpdateData {
        SecurityMonitoringSignalStateUpdateData {
            attributes,
            id: None,
            type_: None,
            _unparsed: false,
        }
    }

    pub fn id(&mut self, value: serde_json::Value) -> &mut Self {
        self.id = Some(value);
        self
    }

    pub fn type_(
        &mut self,
        value: crate::datadogV2::model::SecurityMonitoringSignalMetadataType,
    ) -> &mut Self {
        self.type_ = Some(value);
        self
    }
}

impl<'de> Deserialize<'de> for SecurityMonitoringSignalStateUpdateData {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct SecurityMonitoringSignalStateUpdateDataVisitor;
        impl<'a> Visitor<'a> for SecurityMonitoringSignalStateUpdateDataVisitor {
            type Value = SecurityMonitoringSignalStateUpdateData;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut attributes: Option<
                    crate::datadogV2::model::SecurityMonitoringSignalStateUpdateAttributes,
                > = None;
                let mut id: Option<serde_json::Value> = None;
                let mut type_: Option<
                    crate::datadogV2::model::SecurityMonitoringSignalMetadataType,
                > = None;
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "attributes" => {
                            attributes = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "id" => {
                            if v.is_null() {
                                continue;
                            }
                            id = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "type" => {
                            if v.is_null() {
                                continue;
                            }
                            type_ = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _type_) = type_ {
                                match _type_ {
                                    crate::datadogV2::model::SecurityMonitoringSignalMetadataType::UnparsedObject(_type_) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        }
                        &_ => {}
                    }
                }
                let attributes = attributes.ok_or_else(|| M::Error::missing_field("attributes"))?;

                let content = SecurityMonitoringSignalStateUpdateData {
                    attributes,
                    id,
                    type_,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(SecurityMonitoringSignalStateUpdateDataVisitor)
    }
}
