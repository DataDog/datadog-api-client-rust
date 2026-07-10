// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// The postmortem's attributes.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct IncidentPostmortemAttributes {
    /// Timestamp when the postmortem was created.
    #[serde(rename = "created")]
    pub created: chrono::DateTime<chrono::Utc>,
    /// The identifier of the postmortem document within its host platform.
    #[serde(rename = "document_id")]
    pub document_id: String,
    /// The type of document backing the postmortem (for example, `datadog_notebooks`, `confluence`, or `google_docs`). Can be empty if the document type is unknown.
    #[serde(rename = "document_type")]
    pub document_type: String,
    /// The URL of the postmortem document.
    #[serde(rename = "document_url")]
    pub document_url: String,
    /// Timestamp when the postmortem was last modified.
    #[serde(rename = "modified")]
    pub modified: chrono::DateTime<chrono::Utc>,
    /// The status of the postmortem.
    #[serde(rename = "status")]
    pub status: crate::datadogV2::model::PostmortemStatus,
    /// The title of the postmortem.
    #[serde(rename = "title")]
    pub title: String,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl IncidentPostmortemAttributes {
    pub fn new(
        created: chrono::DateTime<chrono::Utc>,
        document_id: String,
        document_type: String,
        document_url: String,
        modified: chrono::DateTime<chrono::Utc>,
        status: crate::datadogV2::model::PostmortemStatus,
        title: String,
    ) -> IncidentPostmortemAttributes {
        IncidentPostmortemAttributes {
            created,
            document_id,
            document_type,
            document_url,
            modified,
            status,
            title,
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

impl<'de> Deserialize<'de> for IncidentPostmortemAttributes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct IncidentPostmortemAttributesVisitor;
        impl<'a> Visitor<'a> for IncidentPostmortemAttributesVisitor {
            type Value = IncidentPostmortemAttributes;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut created: Option<chrono::DateTime<chrono::Utc>> = None;
                let mut document_id: Option<String> = None;
                let mut document_type: Option<String> = None;
                let mut document_url: Option<String> = None;
                let mut modified: Option<chrono::DateTime<chrono::Utc>> = None;
                let mut status: Option<crate::datadogV2::model::PostmortemStatus> = None;
                let mut title: Option<String> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "created" => {
                            created = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "document_id" => {
                            document_id =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "document_type" => {
                            document_type =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "document_url" => {
                            document_url =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "modified" => {
                            modified = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "status" => {
                            status = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _status) = status {
                                match _status {
                                    crate::datadogV2::model::PostmortemStatus::UnparsedObject(
                                        _status,
                                    ) => {
                                        _unparsed = true;
                                    }
                                    _ => {}
                                }
                            }
                        }
                        "title" => {
                            title = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }
                let created = created.ok_or_else(|| M::Error::missing_field("created"))?;
                let document_id =
                    document_id.ok_or_else(|| M::Error::missing_field("document_id"))?;
                let document_type =
                    document_type.ok_or_else(|| M::Error::missing_field("document_type"))?;
                let document_url =
                    document_url.ok_or_else(|| M::Error::missing_field("document_url"))?;
                let modified = modified.ok_or_else(|| M::Error::missing_field("modified"))?;
                let status = status.ok_or_else(|| M::Error::missing_field("status"))?;
                let title = title.ok_or_else(|| M::Error::missing_field("title"))?;

                let content = IncidentPostmortemAttributes {
                    created,
                    document_id,
                    document_type,
                    document_url,
                    modified,
                    status,
                    title,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(IncidentPostmortemAttributesVisitor)
    }
}
