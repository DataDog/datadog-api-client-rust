// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// The Wait step used in a Synthetic multi-step API test.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct SyntheticsAPIWaitStep {
    /// The name of the step.
    #[serde(rename = "name")]
    pub name: String,
    /// The subtype of the Synthetic multi-step API wait step.
    #[serde(rename = "subtype")]
    pub subtype: crate::datadogV1::model::SyntheticsAPIWaitStepSubtype,
    /// The time to wait in seconds. Minimum value: 0. Maximum value: 180.
    #[serde(rename = "value")]
    pub value: i32,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl SyntheticsAPIWaitStep {
    pub fn new(
        name: String,
        subtype: crate::datadogV1::model::SyntheticsAPIWaitStepSubtype,
        value: i32,
    ) -> SyntheticsAPIWaitStep {
        SyntheticsAPIWaitStep {
            name,
            subtype,
            value,
            _unparsed: false,
        }
    }
}

impl<'de> Deserialize<'de> for SyntheticsAPIWaitStep {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct SyntheticsAPIWaitStepVisitor;
        impl<'a> Visitor<'a> for SyntheticsAPIWaitStepVisitor {
            type Value = SyntheticsAPIWaitStep;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut name: Option<String> = None;
                let mut subtype: Option<crate::datadogV1::model::SyntheticsAPIWaitStepSubtype> =
                    None;
                let mut value: Option<i32> = None;
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "name" => {
                            name = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "subtype" => {
                            subtype = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _subtype) = subtype {
                                match _subtype {
                                    crate::datadogV1::model::SyntheticsAPIWaitStepSubtype::UnparsedObject(_subtype) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        }
                        "value" => {
                            value = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {}
                    }
                }
                let name = name.ok_or_else(|| M::Error::missing_field("name"))?;
                let subtype = subtype.ok_or_else(|| M::Error::missing_field("subtype"))?;
                let value = value.ok_or_else(|| M::Error::missing_field("value"))?;

                let content = SyntheticsAPIWaitStep {
                    name,
                    subtype,
                    value,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(SyntheticsAPIWaitStepVisitor)
    }
}
