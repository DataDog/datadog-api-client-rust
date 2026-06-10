// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// The attributes of a form.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct FormDataAttributes {
    /// Whether the form is currently active.
    #[serde(rename = "active")]
    pub active: bool,
    /// Whether the form accepts anonymous submissions.
    #[serde(rename = "anonymous")]
    pub anonymous: bool,
    /// The time at which the form was created.
    #[serde(rename = "created_at")]
    pub created_at: chrono::DateTime<chrono::Utc>,
    /// The datastore configuration for a form.
    #[serde(rename = "datastore_config")]
    pub datastore_config: crate::datadogV2::model::FormDatastoreConfigAttributes,
    /// The description of the form.
    #[serde(rename = "description")]
    pub description: String,
    /// The date and time at which the form stops accepting responses.
    #[serde(
        rename = "end_date",
        default,
        with = "::serde_with::rust::double_option"
    )]
    pub end_date: Option<Option<chrono::DateTime<chrono::Utc>>>,
    /// Whether the current user has already submitted this form. Only present for forms with `single_response` set to `true`.
    #[serde(
        rename = "has_submitted",
        default,
        with = "::serde_with::rust::double_option"
    )]
    pub has_submitted: Option<Option<bool>>,
    /// Whether the form is an IDP survey.
    #[serde(rename = "idp_survey")]
    pub idp_survey: bool,
    /// The time at which the form was last modified.
    #[serde(rename = "modified_at")]
    pub modified_at: chrono::DateTime<chrono::Utc>,
    /// The name of the form.
    #[serde(rename = "name")]
    pub name: String,
    /// The ID of the organization that owns this form.
    #[serde(rename = "org_id")]
    pub org_id: i64,
    /// The attributes of a form publication.
    #[serde(rename = "publication")]
    pub publication: Option<crate::datadogV2::model::FormPublicationAttributes>,
    /// Whether the form is available in the self-service catalog.
    #[serde(rename = "self_service")]
    pub self_service: bool,
    /// Whether each user can only submit one response.
    #[serde(rename = "single_response")]
    pub single_response: bool,
    /// The ID of the user who created this form.
    #[serde(rename = "user_id")]
    pub user_id: i64,
    /// The UUID of the user who created this form.
    #[serde(rename = "user_uuid")]
    pub user_uuid: uuid::Uuid,
    /// The attributes of a form version.
    #[serde(rename = "version")]
    pub version: Option<crate::datadogV2::model::FormVersionAttributes>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl FormDataAttributes {
    pub fn new(
        active: bool,
        anonymous: bool,
        created_at: chrono::DateTime<chrono::Utc>,
        datastore_config: crate::datadogV2::model::FormDatastoreConfigAttributes,
        description: String,
        idp_survey: bool,
        modified_at: chrono::DateTime<chrono::Utc>,
        name: String,
        org_id: i64,
        self_service: bool,
        single_response: bool,
        user_id: i64,
        user_uuid: uuid::Uuid,
    ) -> FormDataAttributes {
        FormDataAttributes {
            active,
            anonymous,
            created_at,
            datastore_config,
            description,
            end_date: None,
            has_submitted: None,
            idp_survey,
            modified_at,
            name,
            org_id,
            publication: None,
            self_service,
            single_response,
            user_id,
            user_uuid,
            version: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn end_date(mut self, value: Option<chrono::DateTime<chrono::Utc>>) -> Self {
        self.end_date = Some(value);
        self
    }

    pub fn has_submitted(mut self, value: Option<bool>) -> Self {
        self.has_submitted = Some(value);
        self
    }

    pub fn publication(
        mut self,
        value: crate::datadogV2::model::FormPublicationAttributes,
    ) -> Self {
        self.publication = Some(value);
        self
    }

    pub fn version(mut self, value: crate::datadogV2::model::FormVersionAttributes) -> Self {
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

impl<'de> Deserialize<'de> for FormDataAttributes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct FormDataAttributesVisitor;
        impl<'a> Visitor<'a> for FormDataAttributesVisitor {
            type Value = FormDataAttributes;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut active: Option<bool> = None;
                let mut anonymous: Option<bool> = None;
                let mut created_at: Option<chrono::DateTime<chrono::Utc>> = None;
                let mut datastore_config: Option<
                    crate::datadogV2::model::FormDatastoreConfigAttributes,
                > = None;
                let mut description: Option<String> = None;
                let mut end_date: Option<Option<chrono::DateTime<chrono::Utc>>> = None;
                let mut has_submitted: Option<Option<bool>> = None;
                let mut idp_survey: Option<bool> = None;
                let mut modified_at: Option<chrono::DateTime<chrono::Utc>> = None;
                let mut name: Option<String> = None;
                let mut org_id: Option<i64> = None;
                let mut publication: Option<crate::datadogV2::model::FormPublicationAttributes> =
                    None;
                let mut self_service: Option<bool> = None;
                let mut single_response: Option<bool> = None;
                let mut user_id: Option<i64> = None;
                let mut user_uuid: Option<uuid::Uuid> = None;
                let mut version: Option<crate::datadogV2::model::FormVersionAttributes> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "active" => {
                            active = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "anonymous" => {
                            anonymous = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
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
                        "end_date" => {
                            end_date = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "has_submitted" => {
                            has_submitted =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "idp_survey" => {
                            idp_survey = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
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
                        "self_service" => {
                            self_service =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "single_response" => {
                            single_response =
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
                let active = active.ok_or_else(|| M::Error::missing_field("active"))?;
                let anonymous = anonymous.ok_or_else(|| M::Error::missing_field("anonymous"))?;
                let created_at = created_at.ok_or_else(|| M::Error::missing_field("created_at"))?;
                let datastore_config =
                    datastore_config.ok_or_else(|| M::Error::missing_field("datastore_config"))?;
                let description =
                    description.ok_or_else(|| M::Error::missing_field("description"))?;
                let idp_survey = idp_survey.ok_or_else(|| M::Error::missing_field("idp_survey"))?;
                let modified_at =
                    modified_at.ok_or_else(|| M::Error::missing_field("modified_at"))?;
                let name = name.ok_or_else(|| M::Error::missing_field("name"))?;
                let org_id = org_id.ok_or_else(|| M::Error::missing_field("org_id"))?;
                let self_service =
                    self_service.ok_or_else(|| M::Error::missing_field("self_service"))?;
                let single_response =
                    single_response.ok_or_else(|| M::Error::missing_field("single_response"))?;
                let user_id = user_id.ok_or_else(|| M::Error::missing_field("user_id"))?;
                let user_uuid = user_uuid.ok_or_else(|| M::Error::missing_field("user_uuid"))?;

                let content = FormDataAttributes {
                    active,
                    anonymous,
                    created_at,
                    datastore_config,
                    description,
                    end_date,
                    has_submitted,
                    idp_survey,
                    modified_at,
                    name,
                    org_id,
                    publication,
                    self_service,
                    single_response,
                    user_id,
                    user_uuid,
                    version,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(FormDataAttributesVisitor)
    }
}
