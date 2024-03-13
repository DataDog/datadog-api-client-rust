// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// The form data submitted to upload IdP metadata
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct IdPMetadataFormData {
    /// The IdP metadata XML file
    #[serde(rename = "idp_file")]
    pub idp_file: Option<Vec<u8>>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl IdPMetadataFormData {
    pub fn new() -> IdPMetadataFormData {
        IdPMetadataFormData {
            idp_file: None,
            _unparsed: false,
        }
    }

    pub fn idp_file(mut self, value: Vec<u8>) -> Self {
        self.idp_file = Some(value);
        self
    }
}

impl Default for IdPMetadataFormData {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for IdPMetadataFormData {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct IdPMetadataFormDataVisitor;
        impl<'a> Visitor<'a> for IdPMetadataFormDataVisitor {
            type Value = IdPMetadataFormData;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut idp_file: Option<Vec<u8>> = None;
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "idp_file" => {
                            if v.is_null() {
                                continue;
                            }
                            idp_file = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {}
                    }
                }

                let content = IdPMetadataFormData {
                    idp_file,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(IdPMetadataFormDataVisitor)
    }
}
