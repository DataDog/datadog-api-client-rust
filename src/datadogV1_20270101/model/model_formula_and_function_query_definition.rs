// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserializer, Deserialize, Serialize};

/// Test-only generated schema for the dashboard versioning proof of concept.
#[non_exhaustive]
#[derive(Clone, Debug, PartialEq, Serialize)]
#[serde(untagged)]
pub enum FormulaAndFunctionQueryDefinition {
	FormulaMetricsQueryMetrics(Box<crate::datadogV1_20270101::model::FormulaMetricsQueryMetrics>),
	FormulaMetricsQueryCloudCost(Box<crate::datadogV1_20270101::model::FormulaMetricsQueryCloudCost>),
	FormulaMetricsQueryDatadogUsage(Box<crate::datadogV1_20270101::model::FormulaMetricsQueryDatadogUsage>),
	FormulaAndFunctionEventQueryDefinition(Box<crate::datadogV1_20270101::model::FormulaAndFunctionEventQueryDefinition>),
	FormulaAndFunctionProcessQueryDefinition(Box<crate::datadogV1_20270101::model::FormulaAndFunctionProcessQueryDefinition>),
	FormulaApmDependencyStatsQuery(Box<crate::datadogV1_20270101::model::FormulaApmDependencyStatsQuery>),
	FormulaApmResourceStatsQuery(Box<crate::datadogV1_20270101::model::FormulaApmResourceStatsQuery>),
	FormulaApmMetricsQuery(Box<crate::datadogV1_20270101::model::FormulaApmMetricsQuery>),
	FormulaIncidentsQuery(Box<crate::datadogV1_20270101::model::FormulaIncidentsQuery>),
	FormulaSloQuery(Box<crate::datadogV1_20270101::model::FormulaSloQuery>),
	FormulaCaseQuery(Box<crate::datadogV1_20270101::model::FormulaCaseQuery>),
	FormulaGraphQuery(Box<crate::datadogV1_20270101::model::FormulaGraphQuery>),
	FormulaUserJourneyQuery(Box<crate::datadogV1_20270101::model::FormulaUserJourneyQuery>),
	FormulaDoraQuery(Box<crate::datadogV1_20270101::model::FormulaDoraQuery>),
	FormulaChangeTrackingQuery(Box<crate::datadogV1_20270101::model::FormulaChangeTrackingQuery>),
	FormulaDetectionRulesQuery(Box<crate::datadogV1_20270101::model::FormulaDetectionRulesQuery>),
	AggregateFilteredQuery(Box<crate::datadogV1_20270101::model::AggregateFilteredQuery>),
	AggregateAugmentedQuery(Box<crate::datadogV1_20270101::model::AggregateAugmentedQuery>),
	AggregateSegmentQuery(Box<crate::datadogV1_20270101::model::AggregateSegmentQuery>),
	FormulaDatasetQuery(Box<crate::datadogV1_20270101::model::FormulaDatasetQuery>),
	FormulaManagedResourceQuery(Box<crate::datadogV1_20270101::model::FormulaManagedResourceQuery>),
	FormulaReferenceTableQuery(Box<crate::datadogV1_20270101::model::FormulaReferenceTableQuery>),
	FormulaRetentionQuery(Box<crate::datadogV1_20270101::model::FormulaRetentionQuery>),
	FormulaDataQualityMetricsQuery(Box<crate::datadogV1_20270101::model::FormulaDataQualityMetricsQuery>),
	FormulaProductAnalyticsExtendedQuery(Box<crate::datadogV1_20270101::model::FormulaProductAnalyticsExtendedQuery>),
	UnparsedObject(crate::datadog::UnparsedObject),
}

impl<'de> Deserialize<'de> for FormulaAndFunctionQueryDefinition {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value: serde_json::Value = Deserialize::deserialize(deserializer)?;
		if let Ok(_v) = serde_json::from_value::<Box<crate::datadogV1_20270101::model::FormulaMetricsQueryMetrics>>(value.clone()) {
			if !_v._unparsed {
                return Ok(FormulaAndFunctionQueryDefinition::FormulaMetricsQueryMetrics(_v));
            }
        }
		if let Ok(_v) = serde_json::from_value::<Box<crate::datadogV1_20270101::model::FormulaMetricsQueryCloudCost>>(value.clone()) {
			if !_v._unparsed {
                return Ok(FormulaAndFunctionQueryDefinition::FormulaMetricsQueryCloudCost(_v));
            }
        }
		if let Ok(_v) = serde_json::from_value::<Box<crate::datadogV1_20270101::model::FormulaMetricsQueryDatadogUsage>>(value.clone()) {
			if !_v._unparsed {
                return Ok(FormulaAndFunctionQueryDefinition::FormulaMetricsQueryDatadogUsage(_v));
            }
        }
		if let Ok(_v) = serde_json::from_value::<Box<crate::datadogV1_20270101::model::FormulaAndFunctionEventQueryDefinition>>(value.clone()) {
			if !_v._unparsed {
                return Ok(FormulaAndFunctionQueryDefinition::FormulaAndFunctionEventQueryDefinition(_v));
            }
        }
		if let Ok(_v) = serde_json::from_value::<Box<crate::datadogV1_20270101::model::FormulaAndFunctionProcessQueryDefinition>>(value.clone()) {
			if !_v._unparsed {
                return Ok(FormulaAndFunctionQueryDefinition::FormulaAndFunctionProcessQueryDefinition(_v));
            }
        }
		if let Ok(_v) = serde_json::from_value::<Box<crate::datadogV1_20270101::model::FormulaApmDependencyStatsQuery>>(value.clone()) {
			if !_v._unparsed {
                return Ok(FormulaAndFunctionQueryDefinition::FormulaApmDependencyStatsQuery(_v));
            }
        }
		if let Ok(_v) = serde_json::from_value::<Box<crate::datadogV1_20270101::model::FormulaApmResourceStatsQuery>>(value.clone()) {
			if !_v._unparsed {
                return Ok(FormulaAndFunctionQueryDefinition::FormulaApmResourceStatsQuery(_v));
            }
        }
		if let Ok(_v) = serde_json::from_value::<Box<crate::datadogV1_20270101::model::FormulaApmMetricsQuery>>(value.clone()) {
			if !_v._unparsed {
                return Ok(FormulaAndFunctionQueryDefinition::FormulaApmMetricsQuery(_v));
            }
        }
		if let Ok(_v) = serde_json::from_value::<Box<crate::datadogV1_20270101::model::FormulaIncidentsQuery>>(value.clone()) {
			if !_v._unparsed {
                return Ok(FormulaAndFunctionQueryDefinition::FormulaIncidentsQuery(_v));
            }
        }
		if let Ok(_v) = serde_json::from_value::<Box<crate::datadogV1_20270101::model::FormulaSloQuery>>(value.clone()) {
			if !_v._unparsed {
                return Ok(FormulaAndFunctionQueryDefinition::FormulaSloQuery(_v));
            }
        }
		if let Ok(_v) = serde_json::from_value::<Box<crate::datadogV1_20270101::model::FormulaCaseQuery>>(value.clone()) {
			if !_v._unparsed {
                return Ok(FormulaAndFunctionQueryDefinition::FormulaCaseQuery(_v));
            }
        }
		if let Ok(_v) = serde_json::from_value::<Box<crate::datadogV1_20270101::model::FormulaGraphQuery>>(value.clone()) {
			if !_v._unparsed {
                return Ok(FormulaAndFunctionQueryDefinition::FormulaGraphQuery(_v));
            }
        }
		if let Ok(_v) = serde_json::from_value::<Box<crate::datadogV1_20270101::model::FormulaUserJourneyQuery>>(value.clone()) {
			if !_v._unparsed {
                return Ok(FormulaAndFunctionQueryDefinition::FormulaUserJourneyQuery(_v));
            }
        }
		if let Ok(_v) = serde_json::from_value::<Box<crate::datadogV1_20270101::model::FormulaDoraQuery>>(value.clone()) {
			if !_v._unparsed {
                return Ok(FormulaAndFunctionQueryDefinition::FormulaDoraQuery(_v));
            }
        }
		if let Ok(_v) = serde_json::from_value::<Box<crate::datadogV1_20270101::model::FormulaChangeTrackingQuery>>(value.clone()) {
			if !_v._unparsed {
                return Ok(FormulaAndFunctionQueryDefinition::FormulaChangeTrackingQuery(_v));
            }
        }
		if let Ok(_v) = serde_json::from_value::<Box<crate::datadogV1_20270101::model::FormulaDetectionRulesQuery>>(value.clone()) {
			if !_v._unparsed {
                return Ok(FormulaAndFunctionQueryDefinition::FormulaDetectionRulesQuery(_v));
            }
        }
		if let Ok(_v) = serde_json::from_value::<Box<crate::datadogV1_20270101::model::AggregateFilteredQuery>>(value.clone()) {
			if !_v._unparsed {
                return Ok(FormulaAndFunctionQueryDefinition::AggregateFilteredQuery(_v));
            }
        }
		if let Ok(_v) = serde_json::from_value::<Box<crate::datadogV1_20270101::model::AggregateAugmentedQuery>>(value.clone()) {
			if !_v._unparsed {
                return Ok(FormulaAndFunctionQueryDefinition::AggregateAugmentedQuery(_v));
            }
        }
		if let Ok(_v) = serde_json::from_value::<Box<crate::datadogV1_20270101::model::AggregateSegmentQuery>>(value.clone()) {
			if !_v._unparsed {
                return Ok(FormulaAndFunctionQueryDefinition::AggregateSegmentQuery(_v));
            }
        }
		if let Ok(_v) = serde_json::from_value::<Box<crate::datadogV1_20270101::model::FormulaDatasetQuery>>(value.clone()) {
			if !_v._unparsed {
                return Ok(FormulaAndFunctionQueryDefinition::FormulaDatasetQuery(_v));
            }
        }
		if let Ok(_v) = serde_json::from_value::<Box<crate::datadogV1_20270101::model::FormulaManagedResourceQuery>>(value.clone()) {
			if !_v._unparsed {
                return Ok(FormulaAndFunctionQueryDefinition::FormulaManagedResourceQuery(_v));
            }
        }
		if let Ok(_v) = serde_json::from_value::<Box<crate::datadogV1_20270101::model::FormulaReferenceTableQuery>>(value.clone()) {
			if !_v._unparsed {
                return Ok(FormulaAndFunctionQueryDefinition::FormulaReferenceTableQuery(_v));
            }
        }
		if let Ok(_v) = serde_json::from_value::<Box<crate::datadogV1_20270101::model::FormulaRetentionQuery>>(value.clone()) {
			if !_v._unparsed {
                return Ok(FormulaAndFunctionQueryDefinition::FormulaRetentionQuery(_v));
            }
        }
		if let Ok(_v) = serde_json::from_value::<Box<crate::datadogV1_20270101::model::FormulaDataQualityMetricsQuery>>(value.clone()) {
			if !_v._unparsed {
                return Ok(FormulaAndFunctionQueryDefinition::FormulaDataQualityMetricsQuery(_v));
            }
        }
		if let Ok(_v) = serde_json::from_value::<Box<crate::datadogV1_20270101::model::FormulaProductAnalyticsExtendedQuery>>(value.clone()) {
			if !_v._unparsed {
                return Ok(FormulaAndFunctionQueryDefinition::FormulaProductAnalyticsExtendedQuery(_v));
            }
        }
	
        return Ok(FormulaAndFunctionQueryDefinition::UnparsedObject(crate::datadog::UnparsedObject { value }));
    }
}
