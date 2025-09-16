Feature: User account cannot perform action

Scenario: User account is locked due to chargeback
    Given a user performed a chargeback
    When another transaction is received
    Then nothing will be recieved
    And an error added to the error dumps