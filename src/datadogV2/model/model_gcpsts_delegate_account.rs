// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Datadog principal service account info.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct GCPSTSDelegateAccount {
    /// Your delegate account attributes.
    #[serde(rename = "attributes")]
    pub attributes: Option<crate::datadogV2::model::GCPSTSDelegateAccountAttributes>,
    /// The ID of the delegate service account.
    #[serde(rename = "id")]
    pub id: Option<String>,
    /// The type of account.
    #[serde(rename = "type")]
    pub type_: Option<crate::datadogV2::model::GCPSTSDelegateAccountType>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl GCPSTSDelegateAccount {
    pub fn new() -> GCPSTSDelegateAccount {
        GCPSTSDelegateAccount {
            attributes: None,
            id: None,
            type_: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn attributes(
        mut self,
        value: crate::datadogV2::model::GCPSTSDelegateAccountAttributes,
    ) -> Self {
        self.attributes = Some(value);
        self
    }

    pub fn id(mut self, value: String) -> Self {
        self.id = Some(value);
        self
    }

    pub fn type_(mut self, value: crate::datadogV2::model::GCPSTSDelegateAccountType) -> Self {
        self.type_ = Some(value);
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

impl Default for GCPSTSDelegateAccount {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for GCPSTSDelegateAccount {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct GCPSTSDelegateAccountVisitor;
        impl<'a> Visitor<'a> for GCPSTSDelegateAccountVisitor {
            type Value = GCPSTSDelegateAccount;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut attributes: Option<
                    crate::datadogV2::model::GCPSTSDelegateAccountAttributes,
                > = None;
                let mut id: Option<String> = None;
                let mut type_: Option<crate::datadogV2::model::GCPSTSDelegateAccountType> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "attributes" => {
                            if v.is_null() {
                                continue;
                            }
                            attributes = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "id" => {
                            if v.is_null() {
                                continue;
                            }
                            id = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "type" => {
                            if v.is_null() {
                                continue;
                            }
                            type_ = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _type_) = type_ {
                                match _type_ {
                                    crate::datadogV2::model::GCPSTSDelegateAccountType::UnparsedObject(_type_) => {
                                        _unparsed = true;
                                    },
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

                let content = GCPSTSDelegateAccount {
                    attributes,
                    id,
                    type_,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(GCPSTSDelegateAccountVisitor)
    }
}
