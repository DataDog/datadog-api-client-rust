// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// A STIX 2.1 bundle containing threat intelligence indicator objects.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct STIXBundleRequest {
    /// The STIX bundle identifier.
    #[serde(rename = "id")]
    pub id: String,
    /// The STIX objects included in the bundle. Indicator objects are processed and ingested; all other STIX object types are ignored and are not included in the response counters.
    #[serde(rename = "objects")]
    pub objects: Vec<crate::datadogV2::model::STIXObject>,
    /// The supported STIX specification version.
    #[serde(rename = "spec_version")]
    pub spec_version: Option<crate::datadogV2::model::STIXSpecVersion>,
    /// The STIX object type for a bundle.
    #[serde(rename = "type")]
    pub type_: crate::datadogV2::model::STIXBundleType,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl STIXBundleRequest {
    pub fn new(
        id: String,
        objects: Vec<crate::datadogV2::model::STIXObject>,
        type_: crate::datadogV2::model::STIXBundleType,
    ) -> STIXBundleRequest {
        STIXBundleRequest {
            id,
            objects,
            spec_version: None,
            type_,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn spec_version(mut self, value: crate::datadogV2::model::STIXSpecVersion) -> Self {
        self.spec_version = Some(value);
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

impl<'de> Deserialize<'de> for STIXBundleRequest {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct STIXBundleRequestVisitor;
        impl<'a> Visitor<'a> for STIXBundleRequestVisitor {
            type Value = STIXBundleRequest;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut id: Option<String> = None;
                let mut objects: Option<Vec<crate::datadogV2::model::STIXObject>> = None;
                let mut spec_version: Option<crate::datadogV2::model::STIXSpecVersion> = None;
                let mut type_: Option<crate::datadogV2::model::STIXBundleType> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "id" => {
                            id = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "objects" => {
                            objects = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "spec_version" => {
                            if v.is_null() {
                                continue;
                            }
                            spec_version =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _spec_version) = spec_version {
                                match _spec_version {
                                    crate::datadogV2::model::STIXSpecVersion::UnparsedObject(
                                        _spec_version,
                                    ) => {
                                        _unparsed = true;
                                    }
                                    _ => {}
                                }
                            }
                        }
                        "type" => {
                            type_ = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _type_) = type_ {
                                match _type_ {
                                    crate::datadogV2::model::STIXBundleType::UnparsedObject(
                                        _type_,
                                    ) => {
                                        _unparsed = true;
                                    }
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
                let id = id.ok_or_else(|| M::Error::missing_field("id"))?;
                let objects = objects.ok_or_else(|| M::Error::missing_field("objects"))?;
                let type_ = type_.ok_or_else(|| M::Error::missing_field("type_"))?;

                let content = STIXBundleRequest {
                    id,
                    objects,
                    spec_version,
                    type_,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(STIXBundleRequestVisitor)
    }
}
