// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Attributes of an org authorized client.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct OrgAuthorizedClientAttributes {
    /// Whether the organization has disabled this client.
    #[serde(rename = "disabled")]
    pub disabled: bool,
    /// The date and time this client was last exercised.
    #[serialize_always]
    #[serde(rename = "last_exercised")]
    pub last_exercised: Option<chrono::DateTime<chrono::Utc>>,
    /// The number of users in the organization who have authorized this client.
    #[serde(rename = "user_count")]
    pub user_count: i64,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl OrgAuthorizedClientAttributes {
    pub fn new(
        disabled: bool,
        last_exercised: Option<chrono::DateTime<chrono::Utc>>,
        user_count: i64,
    ) -> OrgAuthorizedClientAttributes {
        OrgAuthorizedClientAttributes {
            disabled,
            last_exercised,
            user_count,
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

impl<'de> Deserialize<'de> for OrgAuthorizedClientAttributes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct OrgAuthorizedClientAttributesVisitor;
        impl<'a> Visitor<'a> for OrgAuthorizedClientAttributesVisitor {
            type Value = OrgAuthorizedClientAttributes;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut disabled: Option<bool> = None;
                let mut last_exercised: Option<Option<chrono::DateTime<chrono::Utc>>> = None;
                let mut user_count: Option<i64> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "disabled" => {
                            disabled = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "last_exercised" => {
                            last_exercised =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "user_count" => {
                            user_count = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }
                let disabled = disabled.ok_or_else(|| M::Error::missing_field("disabled"))?;
                let last_exercised =
                    last_exercised.ok_or_else(|| M::Error::missing_field("last_exercised"))?;
                let user_count = user_count.ok_or_else(|| M::Error::missing_field("user_count"))?;

                let content = OrgAuthorizedClientAttributes {
                    disabled,
                    last_exercised,
                    user_count,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(OrgAuthorizedClientAttributesVisitor)
    }
}
