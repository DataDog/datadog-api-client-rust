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
pub struct AssignSeatsUserRequestDataAttributes {
    /// The product code for which to assign seats.
    #[serde(rename = "product_code")]
    pub product_code: String,
    /// The list of user IDs to assign seats to.
    #[serde(rename = "user_uuids")]
    pub user_uuids: Vec<String>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl AssignSeatsUserRequestDataAttributes {
    pub fn new(
        product_code: String,
        user_uuids: Vec<String>,
    ) -> AssignSeatsUserRequestDataAttributes {
        AssignSeatsUserRequestDataAttributes {
            product_code,
            user_uuids,
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

impl<'de> Deserialize<'de> for AssignSeatsUserRequestDataAttributes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct AssignSeatsUserRequestDataAttributesVisitor;
        impl<'a> Visitor<'a> for AssignSeatsUserRequestDataAttributesVisitor {
            type Value = AssignSeatsUserRequestDataAttributes;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut product_code: Option<String> = None;
                let mut user_uuids: Option<Vec<String>> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "product_code" => {
                            product_code =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "user_uuids" => {
                            user_uuids = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }
                let product_code =
                    product_code.ok_or_else(|| M::Error::missing_field("product_code"))?;
                let user_uuids = user_uuids.ok_or_else(|| M::Error::missing_field("user_uuids"))?;

                let content = AssignSeatsUserRequestDataAttributes {
                    product_code,
                    user_uuids,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(AssignSeatsUserRequestDataAttributesVisitor)
    }
}
