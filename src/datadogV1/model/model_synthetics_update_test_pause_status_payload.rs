// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Object to start or pause an existing Synthetic test.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct SyntheticsUpdateTestPauseStatusPayload {
    /// Define whether you want to start (`live`) or pause (`paused`) a
    /// Synthetic test.
    #[serde(rename = "new_status")]
    pub new_status: Option<crate::datadogV1::model::SyntheticsTestPauseStatus>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl SyntheticsUpdateTestPauseStatusPayload {
    pub fn new() -> SyntheticsUpdateTestPauseStatusPayload {
        SyntheticsUpdateTestPauseStatusPayload {
            new_status: None,
            _unparsed: false,
        }
    }

    pub fn new_status(
        &mut self,
        value: crate::datadogV1::model::SyntheticsTestPauseStatus,
    ) -> &mut Self {
        self.new_status = Some(value);
        self
    }
}

impl Default for SyntheticsUpdateTestPauseStatusPayload {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for SyntheticsUpdateTestPauseStatusPayload {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct SyntheticsUpdateTestPauseStatusPayloadVisitor;
        impl<'a> Visitor<'a> for SyntheticsUpdateTestPauseStatusPayloadVisitor {
            type Value = SyntheticsUpdateTestPauseStatusPayload;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut new_status: Option<crate::datadogV1::model::SyntheticsTestPauseStatus> =
                    None;
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "new_status" => {
                            if v.is_null() {
                                continue;
                            }
                            new_status = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _new_status) = new_status {
                                match _new_status {
                                    crate::datadogV1::model::SyntheticsTestPauseStatus::UnparsedObject(_new_status) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        }
                        &_ => {}
                    }
                }

                let content = SyntheticsUpdateTestPauseStatusPayload {
                    new_status,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(SyntheticsUpdateTestPauseStatusPayloadVisitor)
    }
}
