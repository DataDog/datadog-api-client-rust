// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Data related to the reordering of scanning groups.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct SensitiveDataScannerReorderConfig {
    /// ID of the configuration.
    #[serde(rename = "id")]
    pub id: Option<String>,
    /// Relationships of the configuration.
    #[serde(rename = "relationships")]
    pub relationships:
        Option<crate::datadogV2::model::SensitiveDataScannerConfigurationRelationships>,
    /// Sensitive Data Scanner configuration type.
    #[serde(rename = "type")]
    pub type_: Option<crate::datadogV2::model::SensitiveDataScannerConfigurationType>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl SensitiveDataScannerReorderConfig {
    pub fn new() -> SensitiveDataScannerReorderConfig {
        SensitiveDataScannerReorderConfig {
            id: None,
            relationships: None,
            type_: None,
            _unparsed: false,
        }
    }

    pub fn id(&mut self, value: String) -> &mut Self {
        self.id = Some(value);
        self
    }

    pub fn relationships(
        &mut self,
        value: crate::datadogV2::model::SensitiveDataScannerConfigurationRelationships,
    ) -> &mut Self {
        self.relationships = Some(value);
        self
    }

    pub fn type_(
        &mut self,
        value: crate::datadogV2::model::SensitiveDataScannerConfigurationType,
    ) -> &mut Self {
        self.type_ = Some(value);
        self
    }
}

impl Default for SensitiveDataScannerReorderConfig {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for SensitiveDataScannerReorderConfig {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct SensitiveDataScannerReorderConfigVisitor;
        impl<'a> Visitor<'a> for SensitiveDataScannerReorderConfigVisitor {
            type Value = SensitiveDataScannerReorderConfig;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut id: Option<String> = None;
                let mut relationships: Option<
                    crate::datadogV2::model::SensitiveDataScannerConfigurationRelationships,
                > = None;
                let mut type_: Option<
                    crate::datadogV2::model::SensitiveDataScannerConfigurationType,
                > = None;
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "id" => {
                            if v.is_null() {
                                continue;
                            }
                            id = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "relationships" => {
                            if v.is_null() {
                                continue;
                            }
                            relationships =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "type" => {
                            if v.is_null() {
                                continue;
                            }
                            type_ = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _type_) = type_ {
                                match _type_ {
                                    crate::datadogV2::model::SensitiveDataScannerConfigurationType::UnparsedObject(_type_) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        }
                        &_ => {}
                    }
                }

                let content = SensitiveDataScannerReorderConfig {
                    id,
                    relationships,
                    type_,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(SensitiveDataScannerReorderConfigVisitor)
    }
}
