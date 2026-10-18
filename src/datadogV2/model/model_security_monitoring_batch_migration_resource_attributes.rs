// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Attributes of a batch migration resource.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct SecurityMonitoringBatchMigrationResourceAttributes {
    /// The UUID of the batch migration that contains the resource.
    #[serde(rename = "batchId")]
    pub batch_id: uuid::Uuid,
    /// The creation date of the resource, in RFC 3339 format.
    #[serde(rename = "createdAt")]
    pub created_at: chrono::DateTime<chrono::Utc>,
    /// The UUID of the user or service account who created the resource.
    #[serde(rename = "createdBy")]
    pub created_by: Option<String>,
    /// The description of the resource.
    #[serde(rename = "description")]
    pub description: Option<String>,
    /// The source query or natural language description to translate.
    #[serde(rename = "input")]
    pub input: String,
    /// The type of the source rule: `spl` for a Splunk SPL search or saved search definition,
    /// `natural_language` for a plain-English description of the behavior to detect, or `kql`
    /// for a Microsoft KQL query. SPL inputs must be self-contained, with search macros expanded.
    #[serde(rename = "inputType")]
    pub input_type: crate::datadogV2::model::SecurityMonitoringBatchMigrationResourceInputType,
    /// The last update date of the resource, in RFC 3339 format.
    #[serde(rename = "lastUpdatedAt")]
    pub last_updated_at: chrono::DateTime<chrono::Utc>,
    /// The UUID of the user or service account who last updated the resource,
    /// or the name of a system actor.
    #[serde(rename = "lastUpdatedBy")]
    pub last_updated_by: Option<String>,
    /// The name of the resource.
    #[serde(rename = "name")]
    pub name: String,
    /// The translation output. When the resource is `completed`, a Cloud SIEM detection rule
    /// definition in the same format as the create detection rule API. When the resource is
    /// `failed`, an object with an `error` describing the failure, with `kind` set to
    /// `translation_error` or `internal_error`, and optional `type` and `message`. Otherwise, null.
    #[serialize_always]
    #[serde(rename = "output")]
    pub output: Option<std::collections::BTreeMap<String, serde_json::Value>>,
    /// The type of the translation output. Currently always `siem_rule`.
    #[serde(rename = "outputType")]
    pub output_type: String,
    /// The ID of the Cloud SIEM detection rule created when the resource was published.
    #[serde(rename = "ruleId")]
    pub rule_id: Option<String>,
    /// The status of a batch migration resource. `completed` resources have a translation in
    /// `output` that is ready to be published. `duplicate` is reserved for future duplicate rule
    /// detection.
    #[serde(rename = "status")]
    pub status: crate::datadogV2::model::SecurityMonitoringBatchMigrationResourceStatus,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl SecurityMonitoringBatchMigrationResourceAttributes {
    pub fn new(
        batch_id: uuid::Uuid,
        created_at: chrono::DateTime<chrono::Utc>,
        input: String,
        input_type: crate::datadogV2::model::SecurityMonitoringBatchMigrationResourceInputType,
        last_updated_at: chrono::DateTime<chrono::Utc>,
        name: String,
        output: Option<std::collections::BTreeMap<String, serde_json::Value>>,
        output_type: String,
        status: crate::datadogV2::model::SecurityMonitoringBatchMigrationResourceStatus,
    ) -> SecurityMonitoringBatchMigrationResourceAttributes {
        SecurityMonitoringBatchMigrationResourceAttributes {
            batch_id,
            created_at,
            created_by: None,
            description: None,
            input,
            input_type,
            last_updated_at,
            last_updated_by: None,
            name,
            output,
            output_type,
            rule_id: None,
            status,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn created_by(mut self, value: String) -> Self {
        self.created_by = Some(value);
        self
    }

    pub fn description(mut self, value: String) -> Self {
        self.description = Some(value);
        self
    }

    pub fn last_updated_by(mut self, value: String) -> Self {
        self.last_updated_by = Some(value);
        self
    }

    pub fn rule_id(mut self, value: String) -> Self {
        self.rule_id = Some(value);
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

impl<'de> Deserialize<'de> for SecurityMonitoringBatchMigrationResourceAttributes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct SecurityMonitoringBatchMigrationResourceAttributesVisitor;
        impl<'a> Visitor<'a> for SecurityMonitoringBatchMigrationResourceAttributesVisitor {
            type Value = SecurityMonitoringBatchMigrationResourceAttributes;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut batch_id: Option<uuid::Uuid> = None;
                let mut created_at: Option<chrono::DateTime<chrono::Utc>> = None;
                let mut created_by: Option<String> = None;
                let mut description: Option<String> = None;
                let mut input: Option<String> = None;
                let mut input_type: Option<
                    crate::datadogV2::model::SecurityMonitoringBatchMigrationResourceInputType,
                > = None;
                let mut last_updated_at: Option<chrono::DateTime<chrono::Utc>> = None;
                let mut last_updated_by: Option<String> = None;
                let mut name: Option<String> = None;
                let mut output: Option<
                    Option<std::collections::BTreeMap<String, serde_json::Value>>,
                > = None;
                let mut output_type: Option<String> = None;
                let mut rule_id: Option<String> = None;
                let mut status: Option<
                    crate::datadogV2::model::SecurityMonitoringBatchMigrationResourceStatus,
                > = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "batchId" => {
                            batch_id = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "createdAt" => {
                            created_at = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "createdBy" => {
                            if v.is_null() {
                                continue;
                            }
                            created_by = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "description" => {
                            if v.is_null() {
                                continue;
                            }
                            description =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "input" => {
                            input = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "inputType" => {
                            input_type = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _input_type) = input_type {
                                match _input_type {
                                    crate::datadogV2::model::SecurityMonitoringBatchMigrationResourceInputType::UnparsedObject(_input_type) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        }
                        "lastUpdatedAt" => {
                            last_updated_at =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "lastUpdatedBy" => {
                            if v.is_null() {
                                continue;
                            }
                            last_updated_by =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "name" => {
                            name = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "output" => {
                            output = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "outputType" => {
                            output_type =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "ruleId" => {
                            if v.is_null() {
                                continue;
                            }
                            rule_id = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "status" => {
                            status = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _status) = status {
                                match _status {
                                    crate::datadogV2::model::SecurityMonitoringBatchMigrationResourceStatus::UnparsedObject(_status) => {
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
                let batch_id = batch_id.ok_or_else(|| M::Error::missing_field("batch_id"))?;
                let created_at = created_at.ok_or_else(|| M::Error::missing_field("created_at"))?;
                let input = input.ok_or_else(|| M::Error::missing_field("input"))?;
                let input_type = input_type.ok_or_else(|| M::Error::missing_field("input_type"))?;
                let last_updated_at =
                    last_updated_at.ok_or_else(|| M::Error::missing_field("last_updated_at"))?;
                let name = name.ok_or_else(|| M::Error::missing_field("name"))?;
                let output = output.ok_or_else(|| M::Error::missing_field("output"))?;
                let output_type =
                    output_type.ok_or_else(|| M::Error::missing_field("output_type"))?;
                let status = status.ok_or_else(|| M::Error::missing_field("status"))?;

                let content = SecurityMonitoringBatchMigrationResourceAttributes {
                    batch_id,
                    created_at,
                    created_by,
                    description,
                    input,
                    input_type,
                    last_updated_at,
                    last_updated_by,
                    name,
                    output,
                    output_type,
                    rule_id,
                    status,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(SecurityMonitoringBatchMigrationResourceAttributesVisitor)
    }
}
