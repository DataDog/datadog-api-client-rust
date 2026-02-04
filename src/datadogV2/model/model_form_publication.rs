// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Publication information for the form.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct FormPublication {
    /// Creation timestamp.
    #[serde(rename = "created_at")]
    pub created_at: chrono::DateTime<chrono::Utc>,
    /// The form identifier.
    #[serde(rename = "form_id")]
    pub form_id: uuid::Uuid,
    /// The version of the form that was published.
    #[serde(rename = "form_version")]
    pub form_version: Option<i64>,
    /// The unique identifier of the publication.
    #[serde(rename = "id")]
    pub id: String,
    /// Last modification timestamp.
    #[serde(rename = "modified_at")]
    pub modified_at: chrono::DateTime<chrono::Utc>,
    /// The organization ID.
    #[serde(rename = "org_id")]
    pub org_id: i64,
    /// The publication sequence number.
    #[serde(rename = "publish_seq")]
    pub publish_seq: Option<i64>,
    /// The ID of the user who published.
    #[serde(rename = "user_id")]
    pub user_id: i64,
    /// The UUID of the user who published.
    #[serde(rename = "user_uuid")]
    pub user_uuid: uuid::Uuid,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl FormPublication {
    pub fn new(
        created_at: chrono::DateTime<chrono::Utc>,
        form_id: uuid::Uuid,
        id: String,
        modified_at: chrono::DateTime<chrono::Utc>,
        org_id: i64,
        user_id: i64,
        user_uuid: uuid::Uuid,
    ) -> FormPublication {
        FormPublication {
            created_at,
            form_id,
            form_version: None,
            id,
            modified_at,
            org_id,
            publish_seq: None,
            user_id,
            user_uuid,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn form_version(mut self, value: i64) -> Self {
        self.form_version = Some(value);
        self
    }

    pub fn publish_seq(mut self, value: i64) -> Self {
        self.publish_seq = Some(value);
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

impl<'de> Deserialize<'de> for FormPublication {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct FormPublicationVisitor;
        impl<'a> Visitor<'a> for FormPublicationVisitor {
            type Value = FormPublication;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut created_at: Option<chrono::DateTime<chrono::Utc>> = None;
                let mut form_id: Option<uuid::Uuid> = None;
                let mut form_version: Option<i64> = None;
                let mut id: Option<String> = None;
                let mut modified_at: Option<chrono::DateTime<chrono::Utc>> = None;
                let mut org_id: Option<i64> = None;
                let mut publish_seq: Option<i64> = None;
                let mut user_id: Option<i64> = None;
                let mut user_uuid: Option<uuid::Uuid> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "created_at" => {
                            created_at = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "form_id" => {
                            form_id = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "form_version" => {
                            if v.is_null() {
                                continue;
                            }
                            form_version =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "id" => {
                            id = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "modified_at" => {
                            modified_at =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "org_id" => {
                            org_id = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "publish_seq" => {
                            if v.is_null() {
                                continue;
                            }
                            publish_seq =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "user_id" => {
                            user_id = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "user_uuid" => {
                            user_uuid = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }
                let created_at = created_at.ok_or_else(|| M::Error::missing_field("created_at"))?;
                let form_id = form_id.ok_or_else(|| M::Error::missing_field("form_id"))?;
                let id = id.ok_or_else(|| M::Error::missing_field("id"))?;
                let modified_at =
                    modified_at.ok_or_else(|| M::Error::missing_field("modified_at"))?;
                let org_id = org_id.ok_or_else(|| M::Error::missing_field("org_id"))?;
                let user_id = user_id.ok_or_else(|| M::Error::missing_field("user_id"))?;
                let user_uuid = user_uuid.ok_or_else(|| M::Error::missing_field("user_uuid"))?;

                let content = FormPublication {
                    created_at,
                    form_id,
                    form_version,
                    id,
                    modified_at,
                    org_id,
                    publish_seq,
                    user_id,
                    user_uuid,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(FormPublicationVisitor)
    }
}
