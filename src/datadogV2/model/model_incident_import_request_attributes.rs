// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// The incident's attributes for an import request.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct IncidentImportRequestAttributes {
    /// Timestamp when the incident was declared.
    #[serde(rename = "declared")]
    pub declared: Option<chrono::DateTime<chrono::Utc>>,
    /// Timestamp when the incident was detected.
    #[serde(rename = "detected")]
    pub detected: Option<chrono::DateTime<chrono::Utc>>,
    /// A condensed view of the user-defined fields for which to create initial selections.
    #[serde(rename = "fields")]
    pub fields: Option<
        std::collections::BTreeMap<String, crate::datadogV2::model::IncidentImportFieldAttributes>,
    >,
    /// A unique identifier that represents the incident type. If not provided, the default incident type is used.
    #[serde(rename = "incident_type_uuid")]
    pub incident_type_uuid: Option<String>,
    /// Timestamp when the incident was resolved. Can only be set when the state field is set to 'resolved'.
    #[serde(rename = "resolved")]
    pub resolved: Option<chrono::DateTime<chrono::Utc>>,
    /// The title of the incident that summarizes what happened.
    #[serde(rename = "title")]
    pub title: String,
    /// The visibility of the incident.
    #[serde(rename = "visibility")]
    pub visibility: Option<crate::datadogV2::model::IncidentImportVisibility>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl IncidentImportRequestAttributes {
    pub fn new(title: String) -> IncidentImportRequestAttributes {
        IncidentImportRequestAttributes {
            declared: None,
            detected: None,
            fields: None,
            incident_type_uuid: None,
            resolved: None,
            title,
            visibility: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn declared(mut self, value: chrono::DateTime<chrono::Utc>) -> Self {
        self.declared = Some(value);
        self
    }

    pub fn detected(mut self, value: chrono::DateTime<chrono::Utc>) -> Self {
        self.detected = Some(value);
        self
    }

    pub fn fields(
        mut self,
        value: std::collections::BTreeMap<
            String,
            crate::datadogV2::model::IncidentImportFieldAttributes,
        >,
    ) -> Self {
        self.fields = Some(value);
        self
    }

    pub fn incident_type_uuid(mut self, value: String) -> Self {
        self.incident_type_uuid = Some(value);
        self
    }

    pub fn resolved(mut self, value: chrono::DateTime<chrono::Utc>) -> Self {
        self.resolved = Some(value);
        self
    }

    pub fn visibility(mut self, value: crate::datadogV2::model::IncidentImportVisibility) -> Self {
        self.visibility = Some(value);
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

impl<'de> Deserialize<'de> for IncidentImportRequestAttributes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct IncidentImportRequestAttributesVisitor;
        impl<'a> Visitor<'a> for IncidentImportRequestAttributesVisitor {
            type Value = IncidentImportRequestAttributes;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut declared: Option<chrono::DateTime<chrono::Utc>> = None;
                let mut detected: Option<chrono::DateTime<chrono::Utc>> = None;
                let mut fields: Option<
                    std::collections::BTreeMap<
                        String,
                        crate::datadogV2::model::IncidentImportFieldAttributes,
                    >,
                > = None;
                let mut incident_type_uuid: Option<String> = None;
                let mut resolved: Option<chrono::DateTime<chrono::Utc>> = None;
                let mut title: Option<String> = None;
                let mut visibility: Option<crate::datadogV2::model::IncidentImportVisibility> =
                    None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "declared" => {
                            if v.is_null() {
                                continue;
                            }
                            declared = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "detected" => {
                            if v.is_null() {
                                continue;
                            }
                            detected = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "fields" => {
                            if v.is_null() {
                                continue;
                            }
                            fields = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "incident_type_uuid" => {
                            if v.is_null() {
                                continue;
                            }
                            incident_type_uuid =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "resolved" => {
                            if v.is_null() {
                                continue;
                            }
                            resolved = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "title" => {
                            title = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "visibility" => {
                            if v.is_null() {
                                continue;
                            }
                            visibility = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _visibility) = visibility {
                                match _visibility {
                                    crate::datadogV2::model::IncidentImportVisibility::UnparsedObject(_visibility) => {
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
                let title = title.ok_or_else(|| M::Error::missing_field("title"))?;

                let content = IncidentImportRequestAttributes {
                    declared,
                    detected,
                    fields,
                    incident_type_uuid,
                    resolved,
                    title,
                    visibility,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(IncidentImportRequestAttributesVisitor)
    }
}
