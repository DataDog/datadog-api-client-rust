// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use std::fmt::{self, Formatter};
use serde::{Deserializer, Deserialize, Serialize};
use serde::de::{Error, MapAccess, Visitor};
use serde_with::skip_serializing_none;

/// Test-only generated schema for the dashboard versioning proof of concept.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct CohortScope20270101TargetOneOf4454865216 {
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "type")]
    pub type_: crate::datadogV1_20270101::model::CohortScopeTargetAggregationType,
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "value")]
    pub value: crate::datadogV1_20270101::model::CohortScopeTargetAnyOfAnyOfItemValue,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool
}

impl CohortScope20270101TargetOneOf4454865216 {
    pub fn new(type_: crate::datadogV1_20270101::model::CohortScopeTargetAggregationType, value: crate::datadogV1_20270101::model::CohortScopeTargetAnyOfAnyOfItemValue) -> CohortScope20270101TargetOneOf4454865216 {
        CohortScope20270101TargetOneOf4454865216 {
            type_,
            value,
            _unparsed: false,
        }
    }
    
}



impl<'de> Deserialize<'de> for CohortScope20270101TargetOneOf4454865216 {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct CohortScope20270101TargetOneOf4454865216Visitor;
        impl<'a> Visitor<'a> for CohortScope20270101TargetOneOf4454865216Visitor {
            type Value = CohortScope20270101TargetOneOf4454865216;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut type_: Option<crate::datadogV1_20270101::model::CohortScopeTargetAggregationType> = None;
                let mut value: Option<crate::datadogV1_20270101::model::CohortScopeTargetAnyOfAnyOfItemValue> = None;
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "type" => {
                            type_ = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _type_) = type_ {
                                match _type_ {
                                    crate::datadogV1_20270101::model::CohortScopeTargetAggregationType::UnparsedObject(_type_) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        },
                        "value" => {
                            value = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _value) = value {
                                match _value {
                                    crate::datadogV1_20270101::model::CohortScopeTargetAnyOfAnyOfItemValue::UnparsedObject(_value) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        },
                        &_ => {
                            return Err(serde::de::Error::custom("Additional properties not allowed"));
                        },
                    }
                }
                let type_ = type_.ok_or_else(|| M::Error::missing_field("type_"))?;
                let value = value.ok_or_else(|| M::Error::missing_field("value"))?;

                let content = CohortScope20270101TargetOneOf4454865216 {
                    type_,
                    value,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(CohortScope20270101TargetOneOf4454865216Visitor)
    }
}
