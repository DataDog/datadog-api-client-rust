// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Run a historical job request.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct RunHistoricalJobRequestAttributes {
    /// Definition of a historical job based on a security monitoring rule.
    #[serde(rename = "fromRule")]
    pub from_rule: Option<crate::datadogV2::model::JobDefinitionFromRule>,
    /// Request ID.
    #[serde(rename = "id")]
    pub id: Option<String>,
    /// Definition of a historical job.
    #[serde(rename = "jobDefinition")]
    pub job_definition: Option<crate::datadogV2::model::JobDefinition>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl RunHistoricalJobRequestAttributes {
    pub fn new() -> RunHistoricalJobRequestAttributes {
        RunHistoricalJobRequestAttributes {
            from_rule: None,
            id: None,
            job_definition: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn from_rule(mut self, value: crate::datadogV2::model::JobDefinitionFromRule) -> Self {
        self.from_rule = Some(value);
        self
    }

    pub fn id(mut self, value: String) -> Self {
        self.id = Some(value);
        self
    }

    pub fn job_definition(mut self, value: crate::datadogV2::model::JobDefinition) -> Self {
        self.job_definition = Some(value);
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

impl Default for RunHistoricalJobRequestAttributes {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for RunHistoricalJobRequestAttributes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct RunHistoricalJobRequestAttributesVisitor;
        impl<'a> Visitor<'a> for RunHistoricalJobRequestAttributesVisitor {
            type Value = RunHistoricalJobRequestAttributes;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut from_rule: Option<crate::datadogV2::model::JobDefinitionFromRule> = None;
                let mut id: Option<String> = None;
                let mut job_definition: Option<crate::datadogV2::model::JobDefinition> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "fromRule" => {
                            if v.is_null() {
                                continue;
                            }
                            from_rule = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "id" => {
                            if v.is_null() {
                                continue;
                            }
                            id = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "jobDefinition" => {
                            if v.is_null() {
                                continue;
                            }
                            job_definition =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }

                let content = RunHistoricalJobRequestAttributes {
                    from_rule,
                    id,
                    job_definition,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(RunHistoricalJobRequestAttributesVisitor)
    }
}
