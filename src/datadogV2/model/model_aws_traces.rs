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
pub struct AWSTraces {
    /// AWS X-Ray services to collect traces from
    #[serde(rename = "xray_services")]
    pub xray_services: Option<crate::datadogV2::model::AWSXRayServicesList>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl AWSTraces {
    pub fn new() -> AWSTraces {
        AWSTraces {
            xray_services: None,
            _unparsed: false,
        }
    }

    pub fn xray_services(mut self, value: crate::datadogV2::model::AWSXRayServicesList) -> Self {
        self.xray_services = Some(value);
        self
    }
}

impl Default for AWSTraces {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for AWSTraces {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct AWSTracesVisitor;
        impl<'a> Visitor<'a> for AWSTracesVisitor {
            type Value = AWSTraces;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut xray_services: Option<crate::datadogV2::model::AWSXRayServicesList> = None;
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "xray_services" => {
                            if v.is_null() {
                                continue;
                            }
                            xray_services =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {}
                    }
                }

                let content = AWSTraces {
                    xray_services,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(AWSTracesVisitor)
    }
}
