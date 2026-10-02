// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Analysis plan resource with its identifier and settings.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct ExperimentsAnalysisPlanV2DTOData {
    /// Statistical settings and duration targets in the saved analysis plan.
    #[serde(rename = "attributes")]
    pub attributes:
        Option<crate::datadogV2::model::ExperimentsAnalysisPlanV2MutationResponseDataAttributes>,
    /// Identifier of the experiment whose analysis plan is returned.
    #[serde(rename = "id")]
    pub id: uuid::Uuid,
    /// Analysis plans resource type.
    #[serde(rename = "type")]
    pub type_: crate::datadogV2::model::ExperimentsAnalysisPlanWriteV2RequestDataType,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl ExperimentsAnalysisPlanV2DTOData {
    pub fn new(
        id: uuid::Uuid,
        type_: crate::datadogV2::model::ExperimentsAnalysisPlanWriteV2RequestDataType,
    ) -> ExperimentsAnalysisPlanV2DTOData {
        ExperimentsAnalysisPlanV2DTOData {
            attributes: None,
            id,
            type_,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn attributes(
        mut self,
        value: crate::datadogV2::model::ExperimentsAnalysisPlanV2MutationResponseDataAttributes,
    ) -> Self {
        self.attributes = Some(value);
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

impl<'de> Deserialize<'de> for ExperimentsAnalysisPlanV2DTOData {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct ExperimentsAnalysisPlanV2DTODataVisitor;
        impl<'a> Visitor<'a> for ExperimentsAnalysisPlanV2DTODataVisitor {
            type Value = ExperimentsAnalysisPlanV2DTOData;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut attributes: Option<crate::datadogV2::model::ExperimentsAnalysisPlanV2MutationResponseDataAttributes> = None;
                let mut id: Option<uuid::Uuid> = None;
                let mut type_: Option<
                    crate::datadogV2::model::ExperimentsAnalysisPlanWriteV2RequestDataType,
                > = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "attributes" => {
                            if v.is_null() {
                                continue;
                            }
                            attributes = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "id" => {
                            id = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "type" => {
                            type_ = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _type_) = type_ {
                                match _type_ {
                                    crate::datadogV2::model::ExperimentsAnalysisPlanWriteV2RequestDataType::UnparsedObject(_type_) => {
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
                let id = id.ok_or_else(|| M::Error::missing_field("id"))?;
                let type_ = type_.ok_or_else(|| M::Error::missing_field("type_"))?;

                let content = ExperimentsAnalysisPlanV2DTOData {
                    attributes,
                    id,
                    type_,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(ExperimentsAnalysisPlanV2DTODataVisitor)
    }
}
