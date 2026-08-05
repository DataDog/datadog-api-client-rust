// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// The supported relationships for creating a backfilled maintenance.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct CreateBackfilledMaintenanceRequestDataRelationships {
    /// The template used to create the backfilled maintenance.
    #[serde(rename = "template")]
    pub template: Option<
        crate::datadogV2::model::CreateBackfilledMaintenanceRequestDataRelationshipsTemplate,
    >,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl CreateBackfilledMaintenanceRequestDataRelationships {
    pub fn new() -> CreateBackfilledMaintenanceRequestDataRelationships {
        CreateBackfilledMaintenanceRequestDataRelationships {
            template: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn template(
        mut self,
        value: crate::datadogV2::model::CreateBackfilledMaintenanceRequestDataRelationshipsTemplate,
    ) -> Self {
        self.template = Some(value);
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

impl Default for CreateBackfilledMaintenanceRequestDataRelationships {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for CreateBackfilledMaintenanceRequestDataRelationships {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct CreateBackfilledMaintenanceRequestDataRelationshipsVisitor;
        impl<'a> Visitor<'a> for CreateBackfilledMaintenanceRequestDataRelationshipsVisitor {
            type Value = CreateBackfilledMaintenanceRequestDataRelationships;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut template: Option<crate::datadogV2::model::CreateBackfilledMaintenanceRequestDataRelationshipsTemplate> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "template" => {
                            if v.is_null() {
                                continue;
                            }
                            template = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }

                let content = CreateBackfilledMaintenanceRequestDataRelationships {
                    template,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(CreateBackfilledMaintenanceRequestDataRelationshipsVisitor)
    }
}
