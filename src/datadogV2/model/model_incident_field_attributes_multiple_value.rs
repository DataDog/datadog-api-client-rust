// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// A field with potentially multiple values selected.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct IncidentFieldAttributesMultipleValue {
    /// Type of the multiple value field definitions.
    #[serde(rename = "type")]
    pub type_: Option<crate::datadogV2::model::IncidentFieldAttributesValueType>,
    /// The multiple values selected for this field.
    #[serde(rename = "value", default, with = "::serde_with::rust::double_option")]
    pub value: Option<Option<Vec<String>>>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl IncidentFieldAttributesMultipleValue {
    pub fn new() -> IncidentFieldAttributesMultipleValue {
        IncidentFieldAttributesMultipleValue {
            type_: None,
            value: None,
            _unparsed: false,
        }
    }

    pub fn type_(
        mut self,
        value: crate::datadogV2::model::IncidentFieldAttributesValueType,
    ) -> Self {
        self.type_ = Some(value);
        self
    }

    pub fn value(mut self, value: Option<Vec<String>>) -> Self {
        self.value = Some(value);
        self
    }
}

impl Default for IncidentFieldAttributesMultipleValue {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for IncidentFieldAttributesMultipleValue {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct IncidentFieldAttributesMultipleValueVisitor;
        impl<'a> Visitor<'a> for IncidentFieldAttributesMultipleValueVisitor {
            type Value = IncidentFieldAttributesMultipleValue;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut type_: Option<crate::datadogV2::model::IncidentFieldAttributesValueType> =
                    None;
                let mut value: Option<Option<Vec<String>>> = None;
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "type" => {
                            if v.is_null() {
                                continue;
                            }
                            type_ = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _type_) = type_ {
                                match _type_ {
                                    crate::datadogV2::model::IncidentFieldAttributesValueType::UnparsedObject(_type_) => {
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

                let content = IncidentFieldAttributesMultipleValue {
                    type_,
                    value,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(IncidentFieldAttributesMultipleValueVisitor)
    }
}
