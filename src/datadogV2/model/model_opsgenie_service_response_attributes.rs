// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// The attributes from an Opsgenie service response.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct OpsgenieServiceResponseAttributes {
    /// The custom URL for a custom region.
    #[serde(
        rename = "custom_url",
        default,
        with = "::serde_with::rust::double_option"
    )]
    pub custom_url: Option<Option<String>>,
    /// The name for the Opsgenie service.
    #[serde(rename = "name")]
    pub name: Option<String>,
    /// The region for the Opsgenie service.
    #[serde(rename = "region")]
    pub region: Option<crate::datadogV2::model::OpsgenieServiceRegionType>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl OpsgenieServiceResponseAttributes {
    pub fn new() -> OpsgenieServiceResponseAttributes {
        OpsgenieServiceResponseAttributes {
            custom_url: None,
            name: None,
            region: None,
            _unparsed: false,
        }
    }

    pub fn custom_url(mut self, value: Option<String>) -> Self {
        self.custom_url = Some(value);
        self
    }

    pub fn name(mut self, value: String) -> Self {
        self.name = Some(value);
        self
    }

    pub fn region(mut self, value: crate::datadogV2::model::OpsgenieServiceRegionType) -> Self {
        self.region = Some(value);
        self
    }
}

impl Default for OpsgenieServiceResponseAttributes {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for OpsgenieServiceResponseAttributes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct OpsgenieServiceResponseAttributesVisitor;
        impl<'a> Visitor<'a> for OpsgenieServiceResponseAttributesVisitor {
            type Value = OpsgenieServiceResponseAttributes;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut custom_url: Option<Option<String>> = None;
                let mut name: Option<String> = None;
                let mut region: Option<crate::datadogV2::model::OpsgenieServiceRegionType> = None;
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "custom_url" => {
                            custom_url = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "name" => {
                            if v.is_null() {
                                continue;
                            }
                            name = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "region" => {
                            if v.is_null() {
                                continue;
                            }
                            region = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _region) = region {
                                match _region {
                                    crate::datadogV2::model::OpsgenieServiceRegionType::UnparsedObject(_region) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        }
                        &_ => {}
                    }
                }

                let content = OpsgenieServiceResponseAttributes {
                    custom_url,
                    name,
                    region,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(OpsgenieServiceResponseAttributesVisitor)
    }
}
