// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Attributes for creating a ServiceNow record for an incident.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct IncidentServiceNowRecordDataAttributesRequest {
    /// The ServiceNow assignment group.
    #[serde(rename = "assignment_group")]
    pub assignment_group: String,
    /// The ServiceNow configuration item mapping.
    #[serde(rename = "configuration_item_mapping")]
    pub configuration_item_mapping: String,
    /// The ServiceNow instance name.
    #[serde(rename = "instance_name")]
    pub instance_name: String,
    /// An existing ServiceNow record ID (Sys ID) to link instead of creating a new record.
    #[serde(rename = "record_id")]
    pub record_id: Option<String>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl IncidentServiceNowRecordDataAttributesRequest {
    pub fn new(
        assignment_group: String,
        configuration_item_mapping: String,
        instance_name: String,
    ) -> IncidentServiceNowRecordDataAttributesRequest {
        IncidentServiceNowRecordDataAttributesRequest {
            assignment_group,
            configuration_item_mapping,
            instance_name,
            record_id: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn record_id(mut self, value: String) -> Self {
        self.record_id = Some(value);
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

impl<'de> Deserialize<'de> for IncidentServiceNowRecordDataAttributesRequest {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct IncidentServiceNowRecordDataAttributesRequestVisitor;
        impl<'a> Visitor<'a> for IncidentServiceNowRecordDataAttributesRequestVisitor {
            type Value = IncidentServiceNowRecordDataAttributesRequest;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut assignment_group: Option<String> = None;
                let mut configuration_item_mapping: Option<String> = None;
                let mut instance_name: Option<String> = None;
                let mut record_id: Option<String> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "assignment_group" => {
                            assignment_group =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "configuration_item_mapping" => {
                            configuration_item_mapping =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "instance_name" => {
                            instance_name =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "record_id" => {
                            if v.is_null() {
                                continue;
                            }
                            record_id = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }
                let assignment_group =
                    assignment_group.ok_or_else(|| M::Error::missing_field("assignment_group"))?;
                let configuration_item_mapping = configuration_item_mapping
                    .ok_or_else(|| M::Error::missing_field("configuration_item_mapping"))?;
                let instance_name =
                    instance_name.ok_or_else(|| M::Error::missing_field("instance_name"))?;

                let content = IncidentServiceNowRecordDataAttributesRequest {
                    assignment_group,
                    configuration_item_mapping,
                    instance_name,
                    record_id,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(IncidentServiceNowRecordDataAttributesRequestVisitor)
    }
}
