// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// The Opsgenie service attributes for a create request.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct OpsgenieServiceCreateAttributes {
    /// The custom URL for a custom region.
    #[serde(rename = "custom_url")]
    pub custom_url: Option<String>,
    /// The name for the Opsgenie service.
    #[serde(rename = "name")]
    pub name: String,
    /// The Opsgenie API key for your Opsgenie service.
    #[serde(rename = "opsgenie_api_key")]
    pub opsgenie_api_key: String,
    /// The region for the Opsgenie service.
    #[serde(rename = "region")]
    pub region: crate::datadogV2::model::OpsgenieServiceRegionType,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl OpsgenieServiceCreateAttributes {
    pub fn new(
        name: String,
        opsgenie_api_key: String,
        region: crate::datadogV2::model::OpsgenieServiceRegionType,
    ) -> OpsgenieServiceCreateAttributes {
        OpsgenieServiceCreateAttributes {
            custom_url: None,
            name,
            opsgenie_api_key,
            region,
            _unparsed: false,
        }
    }

    pub fn custom_url(&mut self, value: String) -> &mut Self {
        self.custom_url = Some(value);
        self
    }
}

impl<'de> Deserialize<'de> for OpsgenieServiceCreateAttributes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct OpsgenieServiceCreateAttributesVisitor;
        impl<'a> Visitor<'a> for OpsgenieServiceCreateAttributesVisitor {
            type Value = OpsgenieServiceCreateAttributes;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut custom_url: Option<String> = None;
                let mut name: Option<String> = None;
                let mut opsgenie_api_key: Option<String> = None;
                let mut region: Option<crate::datadogV2::model::OpsgenieServiceRegionType> = None;
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "custom_url" => {
                            if v.is_null() {
                                continue;
                            }
                            custom_url = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "name" => {
                            name = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "opsgenie_api_key" => {
                            opsgenie_api_key =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "region" => {
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
                let name = name.ok_or_else(|| M::Error::missing_field("name"))?;
                let opsgenie_api_key =
                    opsgenie_api_key.ok_or_else(|| M::Error::missing_field("opsgenie_api_key"))?;
                let region = region.ok_or_else(|| M::Error::missing_field("region"))?;

                let content = OpsgenieServiceCreateAttributes {
                    custom_url,
                    name,
                    opsgenie_api_key,
                    region,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(OpsgenieServiceCreateAttributesVisitor)
    }
}
