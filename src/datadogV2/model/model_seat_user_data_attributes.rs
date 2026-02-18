// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct SeatUserDataAttributes {
    /// The date and time the seat was assigned.
    #[serde(
        rename = "assigned_at",
        default,
        with = "::serde_with::rust::double_option"
    )]
    pub assigned_at: Option<Option<chrono::DateTime<chrono::Utc>>>,
    /// The email of the user.
    #[serde(rename = "email", default, with = "::serde_with::rust::double_option")]
    pub email: Option<Option<String>>,
    /// The name of the user.
    #[serde(rename = "name", default, with = "::serde_with::rust::double_option")]
    pub name: Option<Option<String>>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl SeatUserDataAttributes {
    pub fn new() -> SeatUserDataAttributes {
        SeatUserDataAttributes {
            assigned_at: None,
            email: None,
            name: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn assigned_at(mut self, value: Option<chrono::DateTime<chrono::Utc>>) -> Self {
        self.assigned_at = Some(value);
        self
    }

    pub fn email(mut self, value: Option<String>) -> Self {
        self.email = Some(value);
        self
    }

    pub fn name(mut self, value: Option<String>) -> Self {
        self.name = Some(value);
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

impl Default for SeatUserDataAttributes {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for SeatUserDataAttributes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct SeatUserDataAttributesVisitor;
        impl<'a> Visitor<'a> for SeatUserDataAttributesVisitor {
            type Value = SeatUserDataAttributes;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut assigned_at: Option<Option<chrono::DateTime<chrono::Utc>>> = None;
                let mut email: Option<Option<String>> = None;
                let mut name: Option<Option<String>> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "assigned_at" => {
                            assigned_at =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "email" => {
                            email = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "name" => {
                            name = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }

                let content = SeatUserDataAttributes {
                    assigned_at,
                    email,
                    name,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(SeatUserDataAttributesVisitor)
    }
}
