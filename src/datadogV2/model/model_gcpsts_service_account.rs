// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Info on your service account.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct GCPSTSServiceAccount {
    /// Attributes associated with your service account.
    #[serde(rename = "attributes")]
    pub attributes: Option<crate::datadogV2::model::GCPSTSServiceAccountAttributes>,
    /// Your service account's unique ID.
    #[serde(rename = "id")]
    pub id: Option<String>,
    /// Additional information related to your service account.
    #[serde(rename = "meta")]
    pub meta: Option<crate::datadogV2::model::GCPServiceAccountMeta>,
    /// The type of account.
    #[serde(rename = "type")]
    pub type_: Option<crate::datadogV2::model::GCPServiceAccountType>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl GCPSTSServiceAccount {
    pub fn new() -> GCPSTSServiceAccount {
        GCPSTSServiceAccount {
            attributes: None,
            id: None,
            meta: None,
            type_: None,
            _unparsed: false,
        }
    }

    pub fn attributes(
        &mut self,
        value: crate::datadogV2::model::GCPSTSServiceAccountAttributes,
    ) -> &mut Self {
        self.attributes = Some(value);
        self
    }

    pub fn id(&mut self, value: String) -> &mut Self {
        self.id = Some(value);
        self
    }

    pub fn meta(&mut self, value: crate::datadogV2::model::GCPServiceAccountMeta) -> &mut Self {
        self.meta = Some(value);
        self
    }

    pub fn type_(&mut self, value: crate::datadogV2::model::GCPServiceAccountType) -> &mut Self {
        self.type_ = Some(value);
        self
    }
}

impl Default for GCPSTSServiceAccount {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for GCPSTSServiceAccount {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct GCPSTSServiceAccountVisitor;
        impl<'a> Visitor<'a> for GCPSTSServiceAccountVisitor {
            type Value = GCPSTSServiceAccount;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut attributes: Option<
                    crate::datadogV2::model::GCPSTSServiceAccountAttributes,
                > = None;
                let mut id: Option<String> = None;
                let mut meta: Option<crate::datadogV2::model::GCPServiceAccountMeta> = None;
                let mut type_: Option<crate::datadogV2::model::GCPServiceAccountType> = None;
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
                        "meta" => {
                            if v.is_null() {
                                continue;
                            }
                            meta = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "type" => {
                            if v.is_null() {
                                continue;
                            }
                            type_ = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _type_) = type_ {
                                match _type_ {
                                    crate::datadogV2::model::GCPServiceAccountType::UnparsedObject(_type_) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        }
                        &_ => {}
                    }
                }

                let content = GCPSTSServiceAccount {
                    attributes,
                    id,
                    meta,
                    type_,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(GCPSTSServiceAccountVisitor)
    }
}
