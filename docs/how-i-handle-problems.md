# How I solve problems such as these

## Part 1: As simple as possible - represented by branch name `initial build`

Simplest case
Simplest code scenario - in our case no disputes
Always a TDD approach
Scenarios are written as BDD to make them easy and testable - allowing all areas of business to understand what the current capabilities of the system are

For this particular case it was about loading data into a stream and then being able to deserialise tx's into an event driven architecture, matching events as they happening, in this case adding and subtracting from held funds for a particular client.

Implementing display

## Part 2: Tidy up code into maintainable format - represented by branch name `code maintainability and reorg`

Code that follows solid principles
Each folder represents a domain within the current system
Types and any utils inherit the parent dir name

## Part 3: Increase Complexity - represented by branch `state-machine`

This included handling

## Part 4: Submission and notes on further improvements

Demo of TCP stream example owr websocket into stream
