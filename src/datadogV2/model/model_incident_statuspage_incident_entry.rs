// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// A Statuspage incident entry.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct IncidentStatuspageIncidentEntry {
    /// The Datadog incident identifier.
    #[serde(rename = "incident_id")]
    pub incident_id: String,
    /// The Statuspage page identifier.
    #[serde(rename = "page_id")]
    pub page_id: String,
    /// The URL of the Statuspage incident.
    #[serde(rename = "redirect_url")]
    pub redirect_url: Option<String>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl IncidentStatuspageIncidentEntry {
    pub fn new(incident_id: String, page_id: String) -> IncidentStatuspageIncidentEntry {
        IncidentStatuspageIncidentEntry {
            incident_id,
            page_id,
            redirect_url: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn redirect_url(mut self, value: String) -> Self {
        self.redirect_url = Some(value);
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

impl<'de> Deserialize<'de> for IncidentStatuspageIncidentEntry {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct IncidentStatuspageIncidentEntryVisitor;
        impl<'a> Visitor<'a> for IncidentStatuspageIncidentEntryVisitor {
            type Value = IncidentStatuspageIncidentEntry;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut incident_id: Option<String> = None;
                let mut page_id: Option<String> = None;
                let mut redirect_url: Option<String> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "incident_id" => {
                            incident_id =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "page_id" => {
                            page_id = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "redirect_url" => {
                            if v.is_null() {
                                continue;
                            }
                            redirect_url =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }
                let incident_id =
                    incident_id.ok_or_else(|| M::Error::missing_field("incident_id"))?;
                let page_id = page_id.ok_or_else(|| M::Error::missing_field("page_id"))?;

                let content = IncidentStatuspageIncidentEntry {
                    incident_id,
                    page_id,
                    redirect_url,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(IncidentStatuspageIncidentEntryVisitor)
    }
}
