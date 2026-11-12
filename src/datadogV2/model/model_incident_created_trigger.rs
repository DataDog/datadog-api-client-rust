// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Trigger a workflow when an incident is declared.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct IncidentCreatedTrigger {
    /// Deprecated. Use `incidentTypes` instead. The type of incident that triggers the workflow.
    #[deprecated]
    #[serde(rename = "incidentType")]
    pub incident_type: Option<String>,
    /// The types of incident that trigger the workflow.
    #[serde(rename = "incidentTypes")]
    pub incident_types: Option<Vec<crate::datadogV2::model::IncidentTriggerIncidentType>>,
    /// Conditions that determine which incidents trigger the workflow.
    #[serde(rename = "tagCondition")]
    pub tag_condition: Option<crate::datadogV2::model::IncidentCondition>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl IncidentCreatedTrigger {
    pub fn new() -> IncidentCreatedTrigger {
        #[allow(deprecated)]
        IncidentCreatedTrigger {
            incident_type: None,
            incident_types: None,
            tag_condition: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    #[allow(deprecated)]
    pub fn incident_type(mut self, value: String) -> Self {
        self.incident_type = Some(value);
        self
    }

    #[allow(deprecated)]
    pub fn incident_types(
        mut self,
        value: Vec<crate::datadogV2::model::IncidentTriggerIncidentType>,
    ) -> Self {
        self.incident_types = Some(value);
        self
    }

    #[allow(deprecated)]
    pub fn tag_condition(mut self, value: crate::datadogV2::model::IncidentCondition) -> Self {
        self.tag_condition = Some(value);
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

impl Default for IncidentCreatedTrigger {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for IncidentCreatedTrigger {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct IncidentCreatedTriggerVisitor;
        impl<'a> Visitor<'a> for IncidentCreatedTriggerVisitor {
            type Value = IncidentCreatedTrigger;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut incident_type: Option<String> = None;
                let mut incident_types: Option<
                    Vec<crate::datadogV2::model::IncidentTriggerIncidentType>,
                > = None;
                let mut tag_condition: Option<crate::datadogV2::model::IncidentCondition> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "incidentType" => {
                            if v.is_null() {
                                continue;
                            }
                            incident_type =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "incidentTypes" => {
                            if v.is_null() {
                                continue;
                            }
                            incident_types =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "tagCondition" => {
                            if v.is_null() {
                                continue;
                            }
                            tag_condition =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }

                #[allow(deprecated)]
                let content = IncidentCreatedTrigger {
                    incident_type,
                    incident_types,
                    tag_condition,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(IncidentCreatedTriggerVisitor)
    }
}
