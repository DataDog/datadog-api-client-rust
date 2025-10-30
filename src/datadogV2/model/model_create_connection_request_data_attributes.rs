// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct CreateConnectionRequestDataAttributes {
    #[serde(rename = "fields")]
    pub fields:
        Option<Vec<crate::datadogV2::model::CreateConnectionRequestDataAttributesFieldsItems>>,
    #[serde(rename = "join_attribute")]
    pub join_attribute: String,
    #[serde(rename = "join_type")]
    pub join_type: String,
    #[serde(rename = "metadata")]
    pub metadata: Option<std::collections::BTreeMap<String, String>>,
    #[serde(rename = "type")]
    pub type_: String,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl CreateConnectionRequestDataAttributes {
    pub fn new(
        join_attribute: String,
        join_type: String,
        type_: String,
    ) -> CreateConnectionRequestDataAttributes {
        CreateConnectionRequestDataAttributes {
            fields: None,
            join_attribute,
            join_type,
            metadata: None,
            type_,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn fields(
        mut self,
        value: Vec<crate::datadogV2::model::CreateConnectionRequestDataAttributesFieldsItems>,
    ) -> Self {
        self.fields = Some(value);
        self
    }

    pub fn metadata(mut self, value: std::collections::BTreeMap<String, String>) -> Self {
        self.metadata = Some(value);
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

impl<'de> Deserialize<'de> for CreateConnectionRequestDataAttributes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct CreateConnectionRequestDataAttributesVisitor;
        impl<'a> Visitor<'a> for CreateConnectionRequestDataAttributesVisitor {
            type Value = CreateConnectionRequestDataAttributes;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut fields: Option<
                    Vec<crate::datadogV2::model::CreateConnectionRequestDataAttributesFieldsItems>,
                > = None;
                let mut join_attribute: Option<String> = None;
                let mut join_type: Option<String> = None;
                let mut metadata: Option<std::collections::BTreeMap<String, String>> = None;
                let mut type_: Option<String> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "fields" => {
                            if v.is_null() {
                                continue;
                            }
                            fields = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "join_attribute" => {
                            join_attribute =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "join_type" => {
                            join_type = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "metadata" => {
                            if v.is_null() {
                                continue;
                            }
                            metadata = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "type" => {
                            type_ = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }
                let join_attribute =
                    join_attribute.ok_or_else(|| M::Error::missing_field("join_attribute"))?;
                let join_type = join_type.ok_or_else(|| M::Error::missing_field("join_type"))?;
                let type_ = type_.ok_or_else(|| M::Error::missing_field("type_"))?;

                let content = CreateConnectionRequestDataAttributes {
                    fields,
                    join_attribute,
                    join_type,
                    metadata,
                    type_,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(CreateConnectionRequestDataAttributesVisitor)
    }
}
