Feature: Disputes, Resolution and Charge Backs

Scenario: A Disupte on a transaction is raised
    Given A transaction has already occured
    When A disupte event is received
    Then the transaction state is changed to dispute 
    And the amount from that transaction is placed in held

Scenario: A Dispute on a transaction is raised, but the transaction doesn't exist
    Given A transaction has not already occured
    When A disupte event is received
    Then the transaction state is changed remains unchanged
    And the held amount is not changed

Scenario: A dispute is resolved
    Given A resolved transaction is received
    And the transaction is already in dispute
    When the transaction is processed
    Then the funds are moved back to available

Scenario: A Dispute is resolved, but the transaction doesn't exist
    Given A transaction has not already occured
    When A resolved transaction is received
    Then the transaction state is changed remains unchanged
    And the held amount is not changed

Scenario: A Dispute is resolved, but the transaction is not in the dispute state
    Given A transaction has ocurred
    And is not in the dispute state
    When A resolved resolve transaction is received
    Then the transaction state is changed remains unchanged
    And the held amount is not changed


Scenario: A chargeback occurs
    Given A transaction has ocurred
    And the transaction is in the dispute state
    When a chargeback tx is received
    Then then clients held amount 
    And the clients total amount should be reduced by the tx amount
    And the account should be locked and banned from further tx's

Scenario: A chargeback occurs, but the transaction is not in the dispute state
    Given A transaction has ocurred
    And is not in the dispute state
    When A resolved resolve transaction is received
    Then the transaction state is changed remains unchanged
    And the held amount is not changed

Scenario: A chargeback occurs, but the transaction doesn't exist
    Given A transaction has not already occured
    When A resolved transaction is received
    Then the transaction state is changed remains unchanged
    And the held amount is not changed



