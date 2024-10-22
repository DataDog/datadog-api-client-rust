// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// AWS Traces config
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct AWSTracesConfig {
    /// AWS X-Ray services to collect traces from
    #[serde(rename = "xray_services")]
    pub xray_services: Option<crate::datadogV2::model::XRayServicesList>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl AWSTracesConfig {
    pub fn new() -> AWSTracesConfig {
        AWSTracesConfig {
            xray_services: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn xray_services(mut self, value: crate::datadogV2::model::XRayServicesList) -> Self {
        self.xray_services = Some(value);
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

impl Default for AWSTracesConfig {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for AWSTracesConfig {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct AWSTracesConfigVisitor;
        impl<'a> Visitor<'a> for AWSTracesConfigVisitor {
            type Value = AWSTracesConfig;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut xray_services: Option<crate::datadogV2::model::XRayServicesList> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "xray_services" => {
                            if v.is_null() {
                                continue;
                            }
                            xray_services =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _xray_services) = xray_services {
                                match _xray_services {
                                    crate::datadogV2::model::XRayServicesList::UnparsedObject(
                                        _xray_services,
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

                let content = AWSTracesConfig {
                    xray_services,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(AWSTracesConfigVisitor)
    }
}
