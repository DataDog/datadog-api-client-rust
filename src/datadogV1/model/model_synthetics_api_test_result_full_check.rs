// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Object describing the API test configuration.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct SyntheticsAPITestResultFullCheck {
    /// Configuration object for a Synthetic test.
    #[serde(rename = "config")]
    pub config: crate::datadogV1::model::SyntheticsTestConfig,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl SyntheticsAPITestResultFullCheck {
    pub fn new(
        config: crate::datadogV1::model::SyntheticsTestConfig,
    ) -> SyntheticsAPITestResultFullCheck {
        SyntheticsAPITestResultFullCheck {
            config,
            _unparsed: false,
        }
    }
}

impl<'de> Deserialize<'de> for SyntheticsAPITestResultFullCheck {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct SyntheticsAPITestResultFullCheckVisitor;
        impl<'a> Visitor<'a> for SyntheticsAPITestResultFullCheckVisitor {
            type Value = SyntheticsAPITestResultFullCheck;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut config: Option<crate::datadogV1::model::SyntheticsTestConfig> = None;
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "config" => {
                            config = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {}
                    }
                }
                let config = config.ok_or_else(|| M::Error::missing_field("config"))?;

                let content = SyntheticsAPITestResultFullCheck { config, _unparsed };

                Ok(content)
            }
        }

        deserializer.deserialize_any(SyntheticsAPITestResultFullCheckVisitor)
    }
}
