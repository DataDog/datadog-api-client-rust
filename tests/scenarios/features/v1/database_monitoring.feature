@endpoint(database-monitoring)
Feature: Database Monitoring
  Query Database Monitoring (DBM) data including query explain plans. See the
  [Database Monitoring page](https://docs.datadoghq.com/database_monitoring/) for
  more information.

  Background:
    Given a valid "apiKeyAuth" key in the system
    And a valid "appKeyAuth" key in the system
    And an instance of "DatabaseMonitoring" API

  @generated @skip @team:DataDog/database-monitoring
  Scenario: List DBM explain plans returns "Bad Request" response
    Given new "ListDbmExplainPlans" request
    And request contains "type" parameter with value "databasequery"
    And body with value {"list": {"indexes": ["databasequery"], "limit": 5, "search": {"query": "dbm_type:plan"}}}
    When the request is sent
    Then the response status is 400 Bad Request

  @generated @skip @team:DataDog/database-monitoring
  Scenario: List DBM explain plans returns "OK" response
    Given new "ListDbmExplainPlans" request
    And request contains "type" parameter with value "databasequery"
    And body with value {"list": {"indexes": ["databasequery"], "limit": 5, "search": {"query": "dbm_type:plan"}}}
    When the request is sent
    Then the response status is 200 OK

  @generated @skip @team:DataDog/database-monitoring
  Scenario: List DBM query samples returns "Bad Request" response
    Given new "ListDbmQuerySamples" request
    And request contains "type" parameter with value "databasequery"
    And body with value {"list": {"indexes": ["databasequery"], "limit": 5, "search": {"query": "dbm_type:activity"}}}
    When the request is sent
    Then the response status is 400 Bad Request

  @generated @skip @team:DataDog/database-monitoring
  Scenario: List DBM query samples returns "OK" response
    Given new "ListDbmQuerySamples" request
    And request contains "type" parameter with value "databasequery"
    And body with value {"list": {"indexes": ["databasequery"], "limit": 5, "search": {"query": "dbm_type:activity"}}}
    When the request is sent
    Then the response status is 200 OK
