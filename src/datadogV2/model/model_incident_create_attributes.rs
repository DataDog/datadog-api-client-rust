// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// The incident's attributes for a create request.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct IncidentCreateAttributes {
    /// Required if `customer_impacted:"true"`. A summary of the impact customers experienced during the incident.
    #[serde(rename = "customer_impact_scope")]
    pub customer_impact_scope: Option<String>,
    /// A flag indicating whether the incident caused customer impact.
    #[serde(rename = "customer_impacted")]
    pub customer_impacted: bool,
    /// A condensed view of the user-defined fields for which to create initial selections.
    #[serde(rename = "fields")]
    pub fields: Option<
        std::collections::BTreeMap<String, crate::datadogV2::model::IncidentFieldAttributes>,
    >,
    /// A unique identifier that represents an incident type. The default incident type will be used if this property is not provided.
    #[serde(rename = "incident_type_uuid")]
    pub incident_type_uuid: Option<String>,
    /// An array of initial timeline cells to be placed at the beginning of the incident timeline.
    #[serde(rename = "initial_cells")]
    pub initial_cells: Option<Vec<crate::datadogV2::model::IncidentTimelineCellCreateAttributes>>,
    /// A flag indicating whether the incident is a test incident.
    #[serde(rename = "is_test")]
    pub is_test: Option<bool>,
    /// Notification handles that will be notified of the incident at creation.
    #[serde(rename = "notification_handles")]
    pub notification_handles: Option<Vec<crate::datadogV2::model::IncidentNotificationHandle>>,
    /// The title of the incident, which summarizes what happened.
    #[serde(rename = "title")]
    pub title: String,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl IncidentCreateAttributes {
    pub fn new(customer_impacted: bool, title: String) -> IncidentCreateAttributes {
        IncidentCreateAttributes {
            customer_impact_scope: None,
            customer_impacted,
            fields: None,
            incident_type_uuid: None,
            initial_cells: None,
            is_test: None,
            notification_handles: None,
            title,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn customer_impact_scope(mut self, value: String) -> Self {
        self.customer_impact_scope = Some(value);
        self
    }

    pub fn fields(
        mut self,
        value: std::collections::BTreeMap<String, crate::datadogV2::model::IncidentFieldAttributes>,
    ) -> Self {
        self.fields = Some(value);
        self
    }

    pub fn incident_type_uuid(mut self, value: String) -> Self {
        self.incident_type_uuid = Some(value);
        self
    }

    pub fn initial_cells(
        mut self,
        value: Vec<crate::datadogV2::model::IncidentTimelineCellCreateAttributes>,
    ) -> Self {
        self.initial_cells = Some(value);
        self
    }

    pub fn is_test(mut self, value: bool) -> Self {
        self.is_test = Some(value);
        self
    }

    pub fn notification_handles(
        mut self,
        value: Vec<crate::datadogV2::model::IncidentNotificationHandle>,
    ) -> Self {
        self.notification_handles = Some(value);
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

impl<'de> Deserialize<'de> for IncidentCreateAttributes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct IncidentCreateAttributesVisitor;
        impl<'a> Visitor<'a> for IncidentCreateAttributesVisitor {
            type Value = IncidentCreateAttributes;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut customer_impact_scope: Option<String> = None;
                let mut customer_impacted: Option<bool> = None;
                let mut fields: Option<
                    std::collections::BTreeMap<
                        String,
                        crate::datadogV2::model::IncidentFieldAttributes,
                    >,
                > = None;
                let mut incident_type_uuid: Option<String> = None;
                let mut initial_cells: Option<
                    Vec<crate::datadogV2::model::IncidentTimelineCellCreateAttributes>,
                > = None;
                let mut is_test: Option<bool> = None;
                let mut notification_handles: Option<
                    Vec<crate::datadogV2::model::IncidentNotificationHandle>,
                > = None;
                let mut title: Option<String> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "customer_impact_scope" => {
                            if v.is_null() {
                                continue;
                            }
                            customer_impact_scope =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "customer_impacted" => {
                            customer_impacted =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
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
                        "initial_cells" => {
                            if v.is_null() {
                                continue;
                            }
                            initial_cells =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "is_test" => {
                            if v.is_null() {
                                continue;
                            }
                            is_test = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "notification_handles" => {
                            if v.is_null() {
                                continue;
                            }
                            notification_handles =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
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
                let customer_impacted = customer_impacted
                    .ok_or_else(|| M::Error::missing_field("customer_impacted"))?;
                let title = title.ok_or_else(|| M::Error::missing_field("title"))?;

                let content = IncidentCreateAttributes {
                    customer_impact_scope,
                    customer_impacted,
                    fields,
                    incident_type_uuid,
                    initial_cells,
                    is_test,
                    notification_handles,
                    title,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(IncidentCreateAttributesVisitor)
    }
}
