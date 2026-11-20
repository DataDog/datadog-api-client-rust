// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Deserializer, Serialize};

/// Targeted recovery (the default) or query-based batch recovery.
#[non_exhaustive]
#[derive(Clone, Debug, PartialEq, Serialize)]
#[serde(untagged)]
pub enum RUMRecoveryCreateAttributes {
    RUMRecoverySessionsCreateAttributes(
        Box<crate::datadogV2::model::RUMRecoverySessionsCreateAttributes>,
    ),
    RUMRecoveryBatchCreateAttributes(
        Box<crate::datadogV2::model::RUMRecoveryBatchCreateAttributes>,
    ),
    UnparsedObject(crate::datadog::UnparsedObject),
}

impl<'de> Deserialize<'de> for RUMRecoveryCreateAttributes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value: serde_json::Value = Deserialize::deserialize(deserializer)?;
        if let Ok(_v) = serde_json::from_value::<
            Box<crate::datadogV2::model::RUMRecoverySessionsCreateAttributes>,
        >(value.clone())
        {
            if !_v._unparsed {
                return Ok(RUMRecoveryCreateAttributes::RUMRecoverySessionsCreateAttributes(_v));
            }
        }
        if let Ok(_v) = serde_json::from_value::<
            Box<crate::datadogV2::model::RUMRecoveryBatchCreateAttributes>,
        >(value.clone())
        {
            if !_v._unparsed {
                return Ok(RUMRecoveryCreateAttributes::RUMRecoveryBatchCreateAttributes(_v));
            }
        }

        return Ok(RUMRecoveryCreateAttributes::UnparsedObject(
            crate::datadog::UnparsedObject { value },
        ));
    }
}
