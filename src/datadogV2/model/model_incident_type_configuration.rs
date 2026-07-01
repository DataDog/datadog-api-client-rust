// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// The incident-type-scoped behavior settings. All fields are optional on update. Any field omitted from a PATCH request keeps its current value. This object is read-only on the incident type resource itself and is only mutated through the update (PATCH) endpoint.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct IncidentTypeConfiguration {
    /// Whether incidents of this type can be deleted.
    #[serde(rename = "allow_incident_deletion")]
    pub allow_incident_deletion: Option<bool>,
    /// Whether automation workflows can be triggered for incidents of this type.
    #[serde(rename = "allow_workflows")]
    pub allow_workflows: Option<bool>,
    /// An optional message shown to users when they declare an incident of this type.
    #[serde(rename = "create_message")]
    pub create_message: Option<String>,
    /// Whether the out-of-the-box postmortem template is disabled for incidents of this type.
    #[serde(rename = "disable_out_of_the_box_postmortem_template")]
    pub disable_out_of_the_box_postmortem_template: Option<bool>,
    /// Whether responders can edit incident timestamps for incidents of this type.
    #[serde(rename = "editable_timestamps")]
    pub editable_timestamps: Option<bool>,
    /// Whether responders can create private incidents of this type. This is an opt-in setting, distinct from `private_incidents_by_default`, which controls whether incidents are created private automatically.
    #[serde(rename = "private_incidents")]
    pub private_incidents: Option<bool>,
    /// Whether incidents of this type are created as private by default.
    #[serde(rename = "private_incidents_by_default")]
    pub private_incidents_by_default: Option<bool>,
    /// When set to `servicenow`, incidents will display the ServiceNow record ID instead of the public ID. If no ServiceNow integration exists, the public ID will be displayed.
    #[serde(rename = "slug_source")]
    pub slug_source: Option<crate::datadogV2::model::IncidentTypeSlugSource>,
    /// Whether incidents of this type are treated as test incidents.
    #[serde(rename = "test_incidents")]
    pub test_incidents: Option<bool>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl IncidentTypeConfiguration {
    pub fn new() -> IncidentTypeConfiguration {
        IncidentTypeConfiguration {
            allow_incident_deletion: None,
            allow_workflows: None,
            create_message: None,
            disable_out_of_the_box_postmortem_template: None,
            editable_timestamps: None,
            private_incidents: None,
            private_incidents_by_default: None,
            slug_source: None,
            test_incidents: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn allow_incident_deletion(mut self, value: bool) -> Self {
        self.allow_incident_deletion = Some(value);
        self
    }

    pub fn allow_workflows(mut self, value: bool) -> Self {
        self.allow_workflows = Some(value);
        self
    }

    pub fn create_message(mut self, value: String) -> Self {
        self.create_message = Some(value);
        self
    }

    pub fn disable_out_of_the_box_postmortem_template(mut self, value: bool) -> Self {
        self.disable_out_of_the_box_postmortem_template = Some(value);
        self
    }

    pub fn editable_timestamps(mut self, value: bool) -> Self {
        self.editable_timestamps = Some(value);
        self
    }

    pub fn private_incidents(mut self, value: bool) -> Self {
        self.private_incidents = Some(value);
        self
    }

    pub fn private_incidents_by_default(mut self, value: bool) -> Self {
        self.private_incidents_by_default = Some(value);
        self
    }

    pub fn slug_source(mut self, value: crate::datadogV2::model::IncidentTypeSlugSource) -> Self {
        self.slug_source = Some(value);
        self
    }

    pub fn test_incidents(mut self, value: bool) -> Self {
        self.test_incidents = Some(value);
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

impl Default for IncidentTypeConfiguration {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for IncidentTypeConfiguration {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct IncidentTypeConfigurationVisitor;
        impl<'a> Visitor<'a> for IncidentTypeConfigurationVisitor {
            type Value = IncidentTypeConfiguration;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut allow_incident_deletion: Option<bool> = None;
                let mut allow_workflows: Option<bool> = None;
                let mut create_message: Option<String> = None;
                let mut disable_out_of_the_box_postmortem_template: Option<bool> = None;
                let mut editable_timestamps: Option<bool> = None;
                let mut private_incidents: Option<bool> = None;
                let mut private_incidents_by_default: Option<bool> = None;
                let mut slug_source: Option<crate::datadogV2::model::IncidentTypeSlugSource> = None;
                let mut test_incidents: Option<bool> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "allow_incident_deletion" => {
                            if v.is_null() {
                                continue;
                            }
                            allow_incident_deletion =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "allow_workflows" => {
                            if v.is_null() {
                                continue;
                            }
                            allow_workflows =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "create_message" => {
                            if v.is_null() {
                                continue;
                            }
                            create_message =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "disable_out_of_the_box_postmortem_template" => {
                            if v.is_null() {
                                continue;
                            }
                            disable_out_of_the_box_postmortem_template =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "editable_timestamps" => {
                            if v.is_null() {
                                continue;
                            }
                            editable_timestamps =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "private_incidents" => {
                            if v.is_null() {
                                continue;
                            }
                            private_incidents =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "private_incidents_by_default" => {
                            if v.is_null() {
                                continue;
                            }
                            private_incidents_by_default =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "slug_source" => {
                            if v.is_null() {
                                continue;
                            }
                            slug_source =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _slug_source) = slug_source {
                                match _slug_source {
                                    crate::datadogV2::model::IncidentTypeSlugSource::UnparsedObject(_slug_source) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        }
                        "test_incidents" => {
                            if v.is_null() {
                                continue;
                            }
                            test_incidents =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }

                let content = IncidentTypeConfiguration {
                    allow_incident_deletion,
                    allow_workflows,
                    create_message,
                    disable_out_of_the_box_postmortem_template,
                    editable_timestamps,
                    private_incidents,
                    private_incidents_by_default,
                    slug_source,
                    test_incidents,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(IncidentTypeConfigurationVisitor)
    }
}
