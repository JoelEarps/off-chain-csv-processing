# How I solve problems such as these

## Stage 1: As simple as possible - represented by branch name `initial build`

Simplest case
Simplest code scenario - in our case no disputes
Always a TDD approach
Scenarios are written as BDD to make them easy and testable - allowing all areas of business to understand what the current capabilities of the system are

For this particular case it was about loading data into a stream and then being able to deserialise tx's into an event driven architecture, matching events as they happening, in this case adding and subtracting from held funds for a particular client.

Implementing display

## Stage 2: Tidy up code into maintainable format - represented by branch name `code maintainability and reorg`

Code that follows solid principles
Each folder represents a domain within the current system
Types and any utils inherit the parent dir name
Seperation of concern - each class does its own job and easy to expand logic for a particular function
Also makes the code more easibly testable and allows for dependecy injection e.g. the generic BoxStream I made.

## Part 3: Increase Complexity - represented by branch `state-machine`

This included handling o

## Stage 4: Submission and notes on further improvements, and finalisation of docs for reviwers

## Testing approach

I write scenarios typically using Gherking format, to make them human readbable for all stakeholders in the product, this way I can liaise with product, sales etc and we can all have a common agreement on the way things currently/ need to work

### Unit Tests

The core logic is tested with unit tests, I used a combination of rstest and rusts in built tes functionality to write tests in the respective file. I always follow a TDD approach, using the Gherkin logic to create very low level unit tests of each bit of functionality.

### Integration tests

How I normally approach
What would I do
