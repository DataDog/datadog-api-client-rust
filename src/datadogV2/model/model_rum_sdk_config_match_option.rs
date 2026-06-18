// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// A match option used for URL or origin pattern matching.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct RumSdkConfigMatchOption {
    /// The type of match pattern, either a literal string or a regex.
    #[serde(rename = "rc_serialized_type")]
    pub rc_serialized_type: crate::datadogV2::model::RumSdkConfigMatchOptionSerializedType,
    /// The value to match against.
    #[serde(rename = "value")]
    pub value: String,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl RumSdkConfigMatchOption {
    pub fn new(
        rc_serialized_type: crate::datadogV2::model::RumSdkConfigMatchOptionSerializedType,
        value: String,
    ) -> RumSdkConfigMatchOption {
        RumSdkConfigMatchOption {
            rc_serialized_type,
            value,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn additional_properties(
        mut self,
        value: std::collections::BTreeMap<String, serde_json::Value>,
    ) -> Self {
        self.additional_properties = value;
        self
    }
}

impl<'de> Deserialize<'de> for RumSdkConfigMatchOption {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct RumSdkConfigMatchOptionVisitor;
        impl<'a> Visitor<'a> for RumSdkConfigMatchOptionVisitor {
            type Value = RumSdkConfigMatchOption;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut rc_serialized_type: Option<
                    crate::datadogV2::model::RumSdkConfigMatchOptionSerializedType,
                > = None;
                let mut value: Option<String> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "rc_serialized_type" => {
                            rc_serialized_type =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _rc_serialized_type) = rc_serialized_type {
                                match _rc_serialized_type {
                                    crate::datadogV2::model::RumSdkConfigMatchOptionSerializedType::UnparsedObject(_rc_serialized_type) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        }
                        "value" => {
                            value = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }
                let rc_serialized_type = rc_serialized_type
                    .ok_or_else(|| M::Error::missing_field("rc_serialized_type"))?;
                let value = value.ok_or_else(|| M::Error::missing_field("value"))?;

                let content = RumSdkConfigMatchOption {
                    rc_serialized_type,
                    value,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(RumSdkConfigMatchOptionVisitor)
    }
}
