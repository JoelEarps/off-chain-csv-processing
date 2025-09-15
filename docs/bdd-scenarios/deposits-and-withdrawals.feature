Feature: Deposit and Withdraw

Scenario: Deposit attempted with no user in Client Account Store
    Given A deposit is found within the transaction events
    When the client id doesn't exist
    Then An account is created and the funds deposit 
    And made available

Scenario: Withdraw attempted with no user in Client Account Store
    Given A withdraw is found within the transaction events
    When the client id doesn't exist
    Then Nothing occurs and an error is created

Scenario: User Exists deposit attempted
    Given A deposit is found within the transaction events
    When A client id exists
    Then the amount deposit is made available to the user

Scenario: User Exists withdraw attempted
    Given A withdraw is found within the transaction events
    When v
    Then Nothing occurs and an error is created