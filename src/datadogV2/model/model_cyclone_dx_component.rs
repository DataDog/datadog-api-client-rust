// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// A component (library, application, or operating system) in the BOM.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct CycloneDXComponent {
    /// Unique reference identifier for this component.
    #[serde(rename = "bom-ref")]
    pub bom_ref: String,
    /// The name of the component.
    #[serde(rename = "name")]
    pub name: String,
    /// Package URL for the component. Required for library components.
    #[serde(rename = "purl")]
    pub purl: Option<String>,
    /// The type of the component. Supported types are library, application, and operating-system.
    #[serde(rename = "type")]
    pub type_: crate::datadogV2::model::CycloneDXComponentType,
    /// The version of the component.
    #[serde(rename = "version")]
    pub version: String,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl CycloneDXComponent {
    pub fn new(
        bom_ref: String,
        name: String,
        type_: crate::datadogV2::model::CycloneDXComponentType,
        version: String,
    ) -> CycloneDXComponent {
        CycloneDXComponent {
            bom_ref,
            name,
            purl: None,
            type_,
            version,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn purl(mut self, value: String) -> Self {
        self.purl = Some(value);
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

impl<'de> Deserialize<'de> for CycloneDXComponent {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct CycloneDXComponentVisitor;
        impl<'a> Visitor<'a> for CycloneDXComponentVisitor {
            type Value = CycloneDXComponent;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut bom_ref: Option<String> = None;
                let mut name: Option<String> = None;
                let mut purl: Option<String> = None;
                let mut type_: Option<crate::datadogV2::model::CycloneDXComponentType> = None;
                let mut version: Option<String> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "bom-ref" => {
                            bom_ref = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "name" => {
                            name = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "purl" => {
                            if v.is_null() {
                                continue;
                            }
                            purl = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "type" => {
                            type_ = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _type_) = type_ {
                                match _type_ {
                                    crate::datadogV2::model::CycloneDXComponentType::UnparsedObject(_type_) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        }
                        "version" => {
                            version = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }
                let bom_ref = bom_ref.ok_or_else(|| M::Error::missing_field("bom_ref"))?;
                let name = name.ok_or_else(|| M::Error::missing_field("name"))?;
                let type_ = type_.ok_or_else(|| M::Error::missing_field("type_"))?;
                let version = version.ok_or_else(|| M::Error::missing_field("version"))?;

                let content = CycloneDXComponent {
                    bom_ref,
                    name,
                    purl,
                    type_,
                    version,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(CycloneDXComponentVisitor)
    }
}
