// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserializer, Deserialize, Serialize};

/// Test-only generated schema for the dashboard versioning proof of concept.
#[non_exhaustive]
#[derive(Clone, Debug, PartialEq, Serialize)]
#[serde(untagged)]
pub enum AdvancedQueryDataset {
	EventPlatformDatasetDefinition(Box<crate::datadogV1_20270101::model::EventPlatformDatasetDefinition>),
	ReferentialDatasetReferenceTable(Box<crate::datadogV1_20270101::model::ReferentialDatasetReferenceTable>),
	ReferentialDatasetManagedResource(Box<crate::datadogV1_20270101::model::ReferentialDatasetManagedResource>),
	ActionsDatastoreDataset(Box<crate::datadogV1_20270101::model::ActionsDatastoreDataset>),
	MetricDatasetCloudCostScalar(Box<crate::datadogV1_20270101::model::MetricDatasetCloudCostScalar>),
	MetricDatasetMetricScalar(Box<crate::datadogV1_20270101::model::MetricDatasetMetricScalar>),
	FormulaDataset(Box<crate::datadogV1_20270101::model::FormulaDataset>),
	AnalysisDataset(Box<crate::datadogV1_20270101::model::AnalysisDataset>),
	PublishedAnalysis(Box<crate::datadogV1_20270101::model::PublishedAnalysis>),
	SnowflakeQueryDataset(Box<crate::datadogV1_20270101::model::SnowflakeQueryDataset>),
	BigQueryDataset(Box<crate::datadogV1_20270101::model::BigQueryDataset>),
	ProductAnalyticsJourneyDataset(Box<crate::datadogV1_20270101::model::ProductAnalyticsJourneyDataset>),
	HistoricalDiscoveredProcessesDataset(Box<crate::datadogV1_20270101::model::HistoricalDiscoveredProcessesDataset>),
	UnparsedObject(crate::datadog::UnparsedObject),
}

impl<'de> Deserialize<'de> for AdvancedQueryDataset {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value: serde_json::Value = Deserialize::deserialize(deserializer)?;
		if let Ok(_v) = serde_json::from_value::<Box<crate::datadogV1_20270101::model::EventPlatformDatasetDefinition>>(value.clone()) {
			if !_v._unparsed {
                return Ok(AdvancedQueryDataset::EventPlatformDatasetDefinition(_v));
            }
        }
		if let Ok(_v) = serde_json::from_value::<Box<crate::datadogV1_20270101::model::ReferentialDatasetReferenceTable>>(value.clone()) {
			if !_v._unparsed {
                return Ok(AdvancedQueryDataset::ReferentialDatasetReferenceTable(_v));
            }
        }
		if let Ok(_v) = serde_json::from_value::<Box<crate::datadogV1_20270101::model::ReferentialDatasetManagedResource>>(value.clone()) {
			if !_v._unparsed {
                return Ok(AdvancedQueryDataset::ReferentialDatasetManagedResource(_v));
            }
        }
		if let Ok(_v) = serde_json::from_value::<Box<crate::datadogV1_20270101::model::ActionsDatastoreDataset>>(value.clone()) {
			if !_v._unparsed {
                return Ok(AdvancedQueryDataset::ActionsDatastoreDataset(_v));
            }
        }
		if let Ok(_v) = serde_json::from_value::<Box<crate::datadogV1_20270101::model::MetricDatasetCloudCostScalar>>(value.clone()) {
			if !_v._unparsed {
                return Ok(AdvancedQueryDataset::MetricDatasetCloudCostScalar(_v));
            }
        }
		if let Ok(_v) = serde_json::from_value::<Box<crate::datadogV1_20270101::model::MetricDatasetMetricScalar>>(value.clone()) {
			if !_v._unparsed {
                return Ok(AdvancedQueryDataset::MetricDatasetMetricScalar(_v));
            }
        }
		if let Ok(_v) = serde_json::from_value::<Box<crate::datadogV1_20270101::model::FormulaDataset>>(value.clone()) {
			if !_v._unparsed {
                return Ok(AdvancedQueryDataset::FormulaDataset(_v));
            }
        }
		if let Ok(_v) = serde_json::from_value::<Box<crate::datadogV1_20270101::model::AnalysisDataset>>(value.clone()) {
			if !_v._unparsed {
                return Ok(AdvancedQueryDataset::AnalysisDataset(_v));
            }
        }
		if let Ok(_v) = serde_json::from_value::<Box<crate::datadogV1_20270101::model::PublishedAnalysis>>(value.clone()) {
			if !_v._unparsed {
                return Ok(AdvancedQueryDataset::PublishedAnalysis(_v));
            }
        }
		if let Ok(_v) = serde_json::from_value::<Box<crate::datadogV1_20270101::model::SnowflakeQueryDataset>>(value.clone()) {
			if !_v._unparsed {
                return Ok(AdvancedQueryDataset::SnowflakeQueryDataset(_v));
            }
        }
		if let Ok(_v) = serde_json::from_value::<Box<crate::datadogV1_20270101::model::BigQueryDataset>>(value.clone()) {
			if !_v._unparsed {
                return Ok(AdvancedQueryDataset::BigQueryDataset(_v));
            }
        }
		if let Ok(_v) = serde_json::from_value::<Box<crate::datadogV1_20270101::model::ProductAnalyticsJourneyDataset>>(value.clone()) {
			if !_v._unparsed {
                return Ok(AdvancedQueryDataset::ProductAnalyticsJourneyDataset(_v));
            }
        }
		if let Ok(_v) = serde_json::from_value::<Box<crate::datadogV1_20270101::model::HistoricalDiscoveredProcessesDataset>>(value.clone()) {
			if !_v._unparsed {
                return Ok(AdvancedQueryDataset::HistoricalDiscoveredProcessesDataset(_v));
            }
        }
	
        return Ok(AdvancedQueryDataset::UnparsedObject(crate::datadog::UnparsedObject { value }));
    }
}
