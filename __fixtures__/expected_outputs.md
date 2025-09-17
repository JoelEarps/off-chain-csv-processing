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
