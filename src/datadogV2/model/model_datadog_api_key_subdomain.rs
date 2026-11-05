// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Deserializer, Serialize};

/// Custom subdomain used for Datadog URLs generated with this Connection. For example, if this org uses `<https://acme.datadoghq.com`> to access Datadog, set this field to `acme`. If this field is omitted, generated URLs will use the default site URL for its datacenter (see [<https://docs.datadoghq.com/getting_started/site]>(<https://docs.datadoghq.com/getting_started/site>)).
#[non_exhaustive]
#[derive(Clone, Debug, PartialEq, Serialize)]
#[serde(untagged)]
pub enum DatadogAPIKeySubdomain {
    ActionConnectionLiteralValue(String),
    ActionConnectionAWSSecretsManagerReference(
        Box<crate::datadogV2::model::ActionConnectionAWSSecretsManagerReference>,
    ),
    UnparsedObject(crate::datadog::UnparsedObject),
}

impl<'de> Deserialize<'de> for DatadogAPIKeySubdomain {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value: serde_json::Value = Deserialize::deserialize(deserializer)?;
        if let Ok(_v) = serde_json::from_value::<String>(value.clone()) {
            return Ok(DatadogAPIKeySubdomain::ActionConnectionLiteralValue(_v));
        }
        if let Ok(_v) = serde_json::from_value::<
            Box<crate::datadogV2::model::ActionConnectionAWSSecretsManagerReference>,
        >(value.clone())
        {
            if !_v._unparsed {
                return Ok(DatadogAPIKeySubdomain::ActionConnectionAWSSecretsManagerReference(_v));
            }
        }

        return Ok(DatadogAPIKeySubdomain::UnparsedObject(
            crate::datadog::UnparsedObject { value },
        ));
    }
}
