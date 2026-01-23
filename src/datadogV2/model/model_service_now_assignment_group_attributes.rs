// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Attributes of a ServiceNow assignment group
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct ServiceNowAssignmentGroupAttributes {
    /// The name of the assignment group
    #[serde(rename = "assignment_group_name")]
    pub assignment_group_name: String,
    /// The system ID of the assignment group in ServiceNow
    #[serde(rename = "assignment_group_sys_id")]
    pub assignment_group_sys_id: String,
    /// The ID of the ServiceNow instance
    #[serde(rename = "instance_id")]
    pub instance_id: uuid::Uuid,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl ServiceNowAssignmentGroupAttributes {
    pub fn new(
        assignment_group_name: String,
        assignment_group_sys_id: String,
        instance_id: uuid::Uuid,
    ) -> ServiceNowAssignmentGroupAttributes {
        ServiceNowAssignmentGroupAttributes {
            assignment_group_name,
            assignment_group_sys_id,
            instance_id,
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

impl<'de> Deserialize<'de> for ServiceNowAssignmentGroupAttributes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct ServiceNowAssignmentGroupAttributesVisitor;
        impl<'a> Visitor<'a> for ServiceNowAssignmentGroupAttributesVisitor {
            type Value = ServiceNowAssignmentGroupAttributes;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut assignment_group_name: Option<String> = None;
                let mut assignment_group_sys_id: Option<String> = None;
                let mut instance_id: Option<uuid::Uuid> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "assignment_group_name" => {
                            assignment_group_name =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "assignment_group_sys_id" => {
                            assignment_group_sys_id =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "instance_id" => {
                            instance_id =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }
                let assignment_group_name = assignment_group_name
                    .ok_or_else(|| M::Error::missing_field("assignment_group_name"))?;
                let assignment_group_sys_id = assignment_group_sys_id
                    .ok_or_else(|| M::Error::missing_field("assignment_group_sys_id"))?;
                let instance_id =
                    instance_id.ok_or_else(|| M::Error::missing_field("instance_id"))?;

                let content = ServiceNowAssignmentGroupAttributes {
                    assignment_group_name,
                    assignment_group_sys_id,
                    instance_id,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(ServiceNowAssignmentGroupAttributesVisitor)
    }
}
