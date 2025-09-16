# Expected Printed Outputs

As I think the bdd test set up is out of scope for this task I will give the manually validated outputs to each test csv. As mentioned in the future improvements I would use a bdd test suite such as cucumber-rs to enable the use of my feature files to be used to create assertable test scenarios.

## Simple Case - No failures (1)

### Input (1)

```csv

type, client, tx, amount
deposit, 1, 1, 1.0
deposit, 2, 2, 2.0
deposit, 1, 3, 2.0
withdrawal, 1, 4, 1.5
withdrawal, 2, 5, 1.0

```

### Output (1)

```csv

client,available,held,total,locked
1,1.5,0,1.5,false
2,1,0,1,false

```

### Explanation (1)

Client 1 -> + 1 + 2 - 1.5 = 1.5
Client 2 -> + 2 - 1 = 1

## Simple Case - withdraws that can cause overdrawn status (2)

### Input (2)

```csv

type, client, tx, amount
deposit, 1, 1, 1.0
deposit, 2, 2, 2.0
deposit, 1, 3, 2.0
withdrawal, 1, 4, 5.0
withdrawal, 2, 5, 3.0

```

### Output (2)

```csv

client,available,held,total,locked
1,1.5,0,1.5,false
2,1,0,1,false

```

### Explanation (2)

Client 1 -> + 1 + 2 - -5 (can't happen, nothing changes) = 3
Client 2 -> + 2 - 3 (can't happen, nothing changes) = 2

## Complex Case - Deposit Dispute for 1 client 1 transaction (3)

### Input (3)

```csv

deposit, 1, 1, 1.0
deposit, 2, 2, 2.0
deposit, 1, 3, 2.0
withdrawal, 1, 4, 1.5
withdrawal, 2, 5, 1.0
dispute 1, 1

```

### Output (3)

```csv

client,available,held,total,locked
2,1,0,1,false
1,0.5,1,1.5,false

```

### Explanation (3)

Client 1 -> + 1 + 2 - 1.5 -> dispute of 1.0 = (available 0.5, held 1.0)
Client 2 -> + 2 - 1 = 1

## Complex Case - Deposit Dispute for and resolution for client 1 transaction 1 (4)

### Input (4)

```csv

type, client, tx, amount
deposit, 1, 1, 5.0
deposit, 2, 2, 2.0
deposit, 1, 3, 2.0
withdrawal, 1, 4, 1.5
withdrawal, 2, 5, 1.0
dispute, 1, 1,
resolve, 1, 1,

```

### Output (4)

```csv

client,available,held,total,locked
1,5.5,0,5.5,false
2,1,0,1,false

```

### Explanation (4)

Client 1:

1. 2 deposits of 5 + 2 -> available 7, held 0, total 7,
2. Withdraw of 1.5 -> current available, 5.5,
3. dispute -> available 0.5, held 5.0, total 5.5,
4. resolve -> available 5.5, held 0, total 5.5,

Client 2:

Client 2 -> + 2 - 1 = 1

## Complex Case - Deposit Dispute for and chargeback for client 1 transaction 1 (5)

### Input (5)

```csv

type, client, tx, amount
deposit, 1, 1, 5.0
deposit, 2, 2, 2.0
deposit, 1, 3, 2.0
withdrawal, 1, 4, 1.5
withdrawal, 2, 5, 1.0
dispute, 1, 1,
resolve, 1, 1,

```

### Output (5)

```csv

client,available,held,total,locked
2,1,0,1,false
1,0.5,0,0.5,true

```

### Explanation (5)

Client 1:

1. 2 deposits of 5 + 2 -> available 7, held 0, total 7,
2. Withdraw of 1.5 -> current available, 5.5,
3. dispute -> available 0.5, held 5.0, total 5.5,
4. chargeback -> available 0.5, held 0, total 0.5,

Client 2:

Client 2 -> + 2 - 1 = 1
