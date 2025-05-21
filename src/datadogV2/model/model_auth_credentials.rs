// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// The auth credentials of the user. Consists of a public key fingerprint and private key.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct AuthCredentials {
    /// The public key fingerprint.
    #[serde(rename = "fingerprint")]
    pub fingerprint: String,
    /// The `RSA` private key in `PEM` format.
    #[serde(rename = "private_key")]
    pub private_key: String,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl AuthCredentials {
    pub fn new(fingerprint: String, private_key: String) -> AuthCredentials {
        AuthCredentials {
            fingerprint,
            private_key,
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

impl<'de> Deserialize<'de> for AuthCredentials {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct AuthCredentialsVisitor;
        impl<'a> Visitor<'a> for AuthCredentialsVisitor {
            type Value = AuthCredentials;

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
                let fingerprint =
                    fingerprint.ok_or_else(|| M::Error::missing_field("fingerprint"))?;
                let private_key =
                    private_key.ok_or_else(|| M::Error::missing_field("private_key"))?;

                let content = AuthCredentials {
                    fingerprint,
                    private_key,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(AuthCredentialsVisitor)
    }
}
