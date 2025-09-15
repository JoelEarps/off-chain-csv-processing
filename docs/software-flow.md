# Softare Flow Diagram

Initial thoughts

Make this event based

1. Get TX
2. Store in collection - most likely a hashmap
3. If record doesn't exist insert with current state
4. 


Combination of Event Based Arch with the Plug and adapter pattern to handle (what if became a TCP server, now if you pass stream of any type e.g. could be GRPC, Websocket or tcp stream, all passed in the handler function).