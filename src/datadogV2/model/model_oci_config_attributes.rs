// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Attributes for an OCI config.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct OCIConfigAttributes {
    /// The OCID of the OCI tenancy.
    #[serde(rename = "account_id")]
    pub account_id: String,
    /// The timestamp when the OCI config was created.
    #[serde(rename = "created_at")]
    pub created_at: String,
    /// The error messages for the OCI config.
    #[serde(
        rename = "error_messages",
        default,
        with = "::serde_with::rust::double_option"
    )]
    pub error_messages: Option<Option<Vec<String>>>,
    /// The status of the OCI config.
    #[serde(rename = "status")]
    pub status: String,
    /// The timestamp when the OCI config status was last updated.
    #[serde(rename = "status_updated_at")]
    pub status_updated_at: String,
    /// The timestamp when the OCI config was last updated.
    #[serde(rename = "updated_at")]
    pub updated_at: String,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl OCIConfigAttributes {
    pub fn new(
        account_id: String,
        created_at: String,
        status: String,
        status_updated_at: String,
        updated_at: String,
    ) -> OCIConfigAttributes {
        OCIConfigAttributes {
            account_id,
            created_at,
            error_messages: None,
            status,
            status_updated_at,
            updated_at,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn error_messages(mut self, value: Option<Vec<String>>) -> Self {
        self.error_messages = Some(value);
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

impl<'de> Deserialize<'de> for OCIConfigAttributes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct OCIConfigAttributesVisitor;
        impl<'a> Visitor<'a> for OCIConfigAttributesVisitor {
            type Value = OCIConfigAttributes;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut account_id: Option<String> = None;
                let mut created_at: Option<String> = None;
                let mut error_messages: Option<Option<Vec<String>>> = None;
                let mut status: Option<String> = None;
                let mut status_updated_at: Option<String> = None;
                let mut updated_at: Option<String> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "account_id" => {
                            account_id = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "created_at" => {
                            created_at = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "error_messages" => {
                            error_messages =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "status" => {
                            status = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "status_updated_at" => {
                            status_updated_at =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "updated_at" => {
                            updated_at = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }
                let account_id = account_id.ok_or_else(|| M::Error::missing_field("account_id"))?;
                let created_at = created_at.ok_or_else(|| M::Error::missing_field("created_at"))?;
                let status = status.ok_or_else(|| M::Error::missing_field("status"))?;
                let status_updated_at = status_updated_at
                    .ok_or_else(|| M::Error::missing_field("status_updated_at"))?;
                let updated_at = updated_at.ok_or_else(|| M::Error::missing_field("updated_at"))?;

                let content = OCIConfigAttributes {
                    account_id,
                    created_at,
                    error_messages,
                    status,
                    status_updated_at,
                    updated_at,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(OCIConfigAttributesVisitor)
    }
}
