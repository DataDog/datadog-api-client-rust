// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Attributes of a ServiceNow template
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct ServiceNowTemplateAttributes {
    /// The ID of the assignment group
    #[serde(rename = "assignment_group_id")]
    pub assignment_group_id: Option<uuid::Uuid>,
    /// The ID of the business service
    #[serde(rename = "business_service_id")]
    pub business_service_id: Option<uuid::Uuid>,
    /// Custom field mappings for the template
    #[serde(rename = "fields_mapping")]
    pub fields_mapping: Option<std::collections::BTreeMap<String, String>>,
    /// The handle name of the template
    #[serde(rename = "handle_name")]
    pub handle_name: String,
    /// The ID of the ServiceNow instance
    #[serde(rename = "instance_id")]
    pub instance_id: uuid::Uuid,
    /// The name of the destination ServiceNow table
    #[serde(rename = "servicenow_tablename")]
    pub servicenow_tablename: String,
    /// The ID of the user
    #[serde(rename = "user_id")]
    pub user_id: Option<uuid::Uuid>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl ServiceNowTemplateAttributes {
    pub fn new(
        handle_name: String,
        instance_id: uuid::Uuid,
        servicenow_tablename: String,
    ) -> ServiceNowTemplateAttributes {
        ServiceNowTemplateAttributes {
            assignment_group_id: None,
            business_service_id: None,
            fields_mapping: None,
            handle_name,
            instance_id,
            servicenow_tablename,
            user_id: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn assignment_group_id(mut self, value: uuid::Uuid) -> Self {
        self.assignment_group_id = Some(value);
        self
    }

    pub fn business_service_id(mut self, value: uuid::Uuid) -> Self {
        self.business_service_id = Some(value);
        self
    }

    pub fn fields_mapping(mut self, value: std::collections::BTreeMap<String, String>) -> Self {
        self.fields_mapping = Some(value);
        self
    }

    pub fn user_id(mut self, value: uuid::Uuid) -> Self {
        self.user_id = Some(value);
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

impl<'de> Deserialize<'de> for ServiceNowTemplateAttributes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct ServiceNowTemplateAttributesVisitor;
        impl<'a> Visitor<'a> for ServiceNowTemplateAttributesVisitor {
            type Value = ServiceNowTemplateAttributes;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut assignment_group_id: Option<uuid::Uuid> = None;
                let mut business_service_id: Option<uuid::Uuid> = None;
                let mut fields_mapping: Option<std::collections::BTreeMap<String, String>> = None;
                let mut handle_name: Option<String> = None;
                let mut instance_id: Option<uuid::Uuid> = None;
                let mut servicenow_tablename: Option<String> = None;
                let mut user_id: Option<uuid::Uuid> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "assignment_group_id" => {
                            if v.is_null() {
                                continue;
                            }
                            assignment_group_id =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "business_service_id" => {
                            if v.is_null() {
                                continue;
                            }
                            business_service_id =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "fields_mapping" => {
                            if v.is_null() {
                                continue;
                            }
                            fields_mapping =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "handle_name" => {
                            handle_name =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "instance_id" => {
                            instance_id =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "servicenow_tablename" => {
                            servicenow_tablename =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "user_id" => {
                            if v.is_null() {
                                continue;
                            }
                            user_id = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }
                let handle_name =
                    handle_name.ok_or_else(|| M::Error::missing_field("handle_name"))?;
                let instance_id =
                    instance_id.ok_or_else(|| M::Error::missing_field("instance_id"))?;
                let servicenow_tablename = servicenow_tablename
                    .ok_or_else(|| M::Error::missing_field("servicenow_tablename"))?;

                let content = ServiceNowTemplateAttributes {
                    assignment_group_id,
                    business_service_id,
                    fields_mapping,
                    handle_name,
                    instance_id,
                    servicenow_tablename,
                    user_id,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(ServiceNowTemplateAttributesVisitor)
    }
}
