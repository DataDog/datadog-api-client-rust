// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Deserializer, Serialize};

/// AWS Metrics namespace filters
#[non_exhaustive]
#[derive(Clone, Debug, PartialEq, Serialize)]
#[serde(untagged)]
pub enum AWSNamespaceFilters {
    AWSNamespaceFiltersExcludeAll(Box<crate::datadogV2::model::AWSNamespaceFiltersExcludeAll>),
    AWSNamespaceFiltersExcludeOnly(Box<crate::datadogV2::model::AWSNamespaceFiltersExcludeOnly>),
    AWSNamespaceFiltersIncludeAll(Box<crate::datadogV2::model::AWSNamespaceFiltersIncludeAll>),
    AWSNamespaceFiltersIncludeOnly(Box<crate::datadogV2::model::AWSNamespaceFiltersIncludeOnly>),
    UnparsedObject(crate::datadog::UnparsedObject),
}

impl<'de> Deserialize<'de> for AWSNamespaceFilters {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value: serde_json::Value = Deserialize::deserialize(deserializer)?;
        if let Ok(_v) = serde_json::from_value::<
            Box<crate::datadogV2::model::AWSNamespaceFiltersExcludeAll>,
        >(value.clone())
        {
            if !_v._unparsed {
                return Ok(AWSNamespaceFilters::AWSNamespaceFiltersExcludeAll(_v));
            }
        }
        if let Ok(_v) = serde_json::from_value::<
            Box<crate::datadogV2::model::AWSNamespaceFiltersExcludeOnly>,
        >(value.clone())
        {
            if !_v._unparsed {
                return Ok(AWSNamespaceFilters::AWSNamespaceFiltersExcludeOnly(_v));
            }
        }
        if let Ok(_v) = serde_json::from_value::<
            Box<crate::datadogV2::model::AWSNamespaceFiltersIncludeAll>,
        >(value.clone())
        {
            if !_v._unparsed {
                return Ok(AWSNamespaceFilters::AWSNamespaceFiltersIncludeAll(_v));
            }
        }
        if let Ok(_v) = serde_json::from_value::<
            Box<crate::datadogV2::model::AWSNamespaceFiltersIncludeOnly>,
        >(value.clone())
        {
            if !_v._unparsed {
                return Ok(AWSNamespaceFilters::AWSNamespaceFiltersIncludeOnly(_v));
            }
        }

        return Ok(AWSNamespaceFilters::UnparsedObject(
            crate::datadog::UnparsedObject { value },
        ));
    }
}
