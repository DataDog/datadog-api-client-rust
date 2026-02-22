// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Attributes of a form.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct FormDataAttributesResponse {
    /// Creation timestamp.
    #[serde(rename = "created_at")]
    pub created_at: chrono::DateTime<chrono::Utc>,
    /// Configuration for the form's associated datastore.
    #[serde(rename = "datastore_config")]
    pub datastore_config: crate::datadogV2::model::FormDatastoreConfig,
    /// The description of the form.
    #[serde(rename = "description")]
    pub description: String,
    /// Last modification timestamp.
    #[serde(rename = "modified_at")]
    pub modified_at: chrono::DateTime<chrono::Utc>,
    /// The name of the form.
    #[serde(rename = "name")]
    pub name: String,
    /// The organization ID.
    #[serde(rename = "org_id")]
    pub org_id: i64,
    /// Publication information for the form.
    #[serde(rename = "publication")]
    pub publication: Option<crate::datadogV2::model::FormPublication>,
    /// The ID of the user who created the form.
    #[serde(rename = "user_id")]
    pub user_id: i64,
    /// The UUID of the user who created the form.
    #[serde(rename = "user_uuid")]
    pub user_uuid: uuid::Uuid,
    /// Version information for the form.
    #[serde(rename = "version")]
    pub version: Option<crate::datadogV2::model::FormVersion>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl FormDataAttributesResponse {
    pub fn new(
        created_at: chrono::DateTime<chrono::Utc>,
        datastore_config: crate::datadogV2::model::FormDatastoreConfig,
        description: String,
        modified_at: chrono::DateTime<chrono::Utc>,
        name: String,
        org_id: i64,
        user_id: i64,
        user_uuid: uuid::Uuid,
    ) -> FormDataAttributesResponse {
        FormDataAttributesResponse {
            created_at,
            datastore_config,
            description,
            modified_at,
            name,
            org_id,
            publication: None,
            user_id,
            user_uuid,
            version: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn publication(mut self, value: crate::datadogV2::model::FormPublication) -> Self {
        self.publication = Some(value);
        self
    }

    pub fn version(mut self, value: crate::datadogV2::model::FormVersion) -> Self {
        self.version = Some(value);
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

impl<'de> Deserialize<'de> for FormDataAttributesResponse {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct FormDataAttributesResponseVisitor;
        impl<'a> Visitor<'a> for FormDataAttributesResponseVisitor {
            type Value = FormDataAttributesResponse;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut created_at: Option<chrono::DateTime<chrono::Utc>> = None;
                let mut datastore_config: Option<crate::datadogV2::model::FormDatastoreConfig> =
                    None;
                let mut description: Option<String> = None;
                let mut modified_at: Option<chrono::DateTime<chrono::Utc>> = None;
                let mut name: Option<String> = None;
                let mut org_id: Option<i64> = None;
                let mut publication: Option<crate::datadogV2::model::FormPublication> = None;
                let mut user_id: Option<i64> = None;
                let mut user_uuid: Option<uuid::Uuid> = None;
                let mut version: Option<crate::datadogV2::model::FormVersion> = None;
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
                        "datastore_config" => {
                            datastore_config =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "description" => {
                            description =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "modified_at" => {
                            modified_at =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "name" => {
                            name = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "org_id" => {
                            org_id = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "publication" => {
                            if v.is_null() {
                                continue;
                            }
                            publication =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "user_id" => {
                            user_id = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "user_uuid" => {
                            user_uuid = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "version" => {
                            if v.is_null() {
                                continue;
                            }
                            version = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }
                let created_at = created_at.ok_or_else(|| M::Error::missing_field("created_at"))?;
                let datastore_config =
                    datastore_config.ok_or_else(|| M::Error::missing_field("datastore_config"))?;
                let description =
                    description.ok_or_else(|| M::Error::missing_field("description"))?;
                let modified_at =
                    modified_at.ok_or_else(|| M::Error::missing_field("modified_at"))?;
                let name = name.ok_or_else(|| M::Error::missing_field("name"))?;
                let org_id = org_id.ok_or_else(|| M::Error::missing_field("org_id"))?;
                let user_id = user_id.ok_or_else(|| M::Error::missing_field("user_id"))?;
                let user_uuid = user_uuid.ok_or_else(|| M::Error::missing_field("user_uuid"))?;

                let content = FormDataAttributesResponse {
                    created_at,
                    datastore_config,
                    description,
                    modified_at,
                    name,
                    org_id,
                    publication,
                    user_id,
                    user_uuid,
                    version,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(FormDataAttributesResponseVisitor)
    }
}
