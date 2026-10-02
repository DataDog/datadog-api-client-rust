// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Details of the variant result.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct ExperimentsVariantResultsV2DTODataAttributes {
    /// Number of subjects assigned to this variant.
    #[serde(rename = "assignment_count")]
    pub assignment_count: Option<i64>,
    /// ID of the experiment associated with this result.
    #[serde(rename = "experiment_id")]
    pub experiment_id: Option<String>,
    /// Whether this variant is the experiment's control.
    #[serde(rename = "is_control")]
    pub is_control: Option<bool>,
    /// Metrics reported for this variant.
    #[serde(rename = "metrics")]
    pub metrics: Option<
        Vec<crate::datadogV2::model::ExperimentsVariantResultsV2DTODataAttributesMetricsItems>,
    >,
    /// Key that identifies the experiment variant.
    #[serde(rename = "variant_key")]
    pub variant_key: Option<String>,
    /// Display name of the experiment variant.
    #[serde(rename = "variant_name")]
    pub variant_name: Option<String>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl ExperimentsVariantResultsV2DTODataAttributes {
    pub fn new() -> ExperimentsVariantResultsV2DTODataAttributes {
        ExperimentsVariantResultsV2DTODataAttributes {
            assignment_count: None,
            experiment_id: None,
            is_control: None,
            metrics: None,
            variant_key: None,
            variant_name: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn assignment_count(mut self, value: i64) -> Self {
        self.assignment_count = Some(value);
        self
    }

    pub fn experiment_id(mut self, value: String) -> Self {
        self.experiment_id = Some(value);
        self
    }

    pub fn is_control(mut self, value: bool) -> Self {
        self.is_control = Some(value);
        self
    }

    pub fn metrics(
        mut self,
        value: Vec<
            crate::datadogV2::model::ExperimentsVariantResultsV2DTODataAttributesMetricsItems,
        >,
    ) -> Self {
        self.metrics = Some(value);
        self
    }

    pub fn variant_key(mut self, value: String) -> Self {
        self.variant_key = Some(value);
        self
    }

    pub fn variant_name(mut self, value: String) -> Self {
        self.variant_name = Some(value);
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

impl Default for ExperimentsVariantResultsV2DTODataAttributes {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for ExperimentsVariantResultsV2DTODataAttributes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct ExperimentsVariantResultsV2DTODataAttributesVisitor;
        impl<'a> Visitor<'a> for ExperimentsVariantResultsV2DTODataAttributesVisitor {
            type Value = ExperimentsVariantResultsV2DTODataAttributes;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut assignment_count: Option<i64> = None;
                let mut experiment_id: Option<String> = None;
                let mut is_control: Option<bool> = None;
                let mut metrics: Option<Vec<crate::datadogV2::model::ExperimentsVariantResultsV2DTODataAttributesMetricsItems>> = None;
                let mut variant_key: Option<String> = None;
                let mut variant_name: Option<String> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "assignment_count" => {
                            if v.is_null() {
                                continue;
                            }
                            assignment_count =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "experiment_id" => {
                            if v.is_null() {
                                continue;
                            }
                            experiment_id =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "is_control" => {
                            if v.is_null() {
                                continue;
                            }
                            is_control = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "metrics" => {
                            if v.is_null() {
                                continue;
                            }
                            metrics = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "variant_key" => {
                            if v.is_null() {
                                continue;
                            }
                            variant_key =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "variant_name" => {
                            if v.is_null() {
                                continue;
                            }
                            variant_name =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }

                let content = ExperimentsVariantResultsV2DTODataAttributes {
                    assignment_count,
                    experiment_id,
                    is_control,
                    metrics,
                    variant_key,
                    variant_name,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(ExperimentsVariantResultsV2DTODataAttributesVisitor)
    }
}
