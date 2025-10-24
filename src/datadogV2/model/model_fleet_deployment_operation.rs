// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// A single configuration file operation to perform on the target hosts.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct FleetDeploymentOperation {
    /// Type of file operation to perform on the target configuration file.
    /// - `merge-patch`: Merges the provided patch data with the existing configuration file.
    ///   Creates the file if it doesn't exist.
    /// - `delete`: Removes the specified configuration file from the target hosts.
    #[serde(rename = "file_op")]
    pub file_op: crate::datadogV2::model::FleetDeploymentFileOp,
    /// Absolute path to the target configuration file on the host.
    #[serde(rename = "file_path")]
    pub file_path: String,
    /// Patch data in JSON format to apply to the configuration file.
    /// When using `merge-patch`, this object is merged with the existing configuration,
    /// allowing you to add, update, or override specific fields without replacing the entire file.
    /// The structure must match the target configuration file format (for example, YAML structure for Datadog Agent config).
    /// Not applicable when using the `delete` operation.
    #[serde(rename = "patch")]
    pub patch: Option<std::collections::BTreeMap<String, serde_json::Value>>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl FleetDeploymentOperation {
    pub fn new(
        file_op: crate::datadogV2::model::FleetDeploymentFileOp,
        file_path: String,
    ) -> FleetDeploymentOperation {
        FleetDeploymentOperation {
            file_op,
            file_path,
            patch: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn patch(mut self, value: std::collections::BTreeMap<String, serde_json::Value>) -> Self {
        self.patch = Some(value);
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

impl<'de> Deserialize<'de> for FleetDeploymentOperation {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct FleetDeploymentOperationVisitor;
        impl<'a> Visitor<'a> for FleetDeploymentOperationVisitor {
            type Value = FleetDeploymentOperation;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut file_op: Option<crate::datadogV2::model::FleetDeploymentFileOp> = None;
                let mut file_path: Option<String> = None;
                let mut patch: Option<std::collections::BTreeMap<String, serde_json::Value>> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "file_op" => {
                            file_op = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _file_op) = file_op {
                                match _file_op {
                                    crate::datadogV2::model::FleetDeploymentFileOp::UnparsedObject(_file_op) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        }
                        "file_path" => {
                            file_path = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "patch" => {
                            if v.is_null() {
                                continue;
                            }
                            patch = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }
                let file_op = file_op.ok_or_else(|| M::Error::missing_field("file_op"))?;
                let file_path = file_path.ok_or_else(|| M::Error::missing_field("file_path"))?;

                let content = FleetDeploymentOperation {
                    file_op,
                    file_path,
                    patch,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(FleetDeploymentOperationVisitor)
    }
}
