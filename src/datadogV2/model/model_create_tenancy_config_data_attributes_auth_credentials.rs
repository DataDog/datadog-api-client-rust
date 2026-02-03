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
pub struct CreateTenancyConfigDataAttributesAuthCredentials {
    #[serde(rename = "fingerprint")]
    pub fingerprint: Option<String>,
    #[serde(rename = "private_key")]
    pub private_key: String,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl CreateTenancyConfigDataAttributesAuthCredentials {
    pub fn new(private_key: String) -> CreateTenancyConfigDataAttributesAuthCredentials {
        CreateTenancyConfigDataAttributesAuthCredentials {
            fingerprint: None,
            private_key,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn fingerprint(mut self, value: String) -> Self {
        self.fingerprint = Some(value);
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

impl<'de> Deserialize<'de> for CreateTenancyConfigDataAttributesAuthCredentials {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct CreateTenancyConfigDataAttributesAuthCredentialsVisitor;
        impl<'a> Visitor<'a> for CreateTenancyConfigDataAttributesAuthCredentialsVisitor {
            type Value = CreateTenancyConfigDataAttributesAuthCredentials;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut fingerprint: Option<String> = None;
                let mut private_key: Option<String> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "fingerprint" => {
                            if v.is_null() {
                                continue;
                            }
                            fingerprint =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "private_key" => {
                            private_key =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }
                let private_key =
                    private_key.ok_or_else(|| M::Error::missing_field("private_key"))?;

                let content = CreateTenancyConfigDataAttributesAuthCredentials {
                    fingerprint,
                    private_key,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(CreateTenancyConfigDataAttributesAuthCredentialsVisitor)
    }
}
