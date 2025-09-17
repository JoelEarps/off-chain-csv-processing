# Software Architecture

The system combines an event-based architecture with the plug-and-adapter pattern, allowing flexibility in the type of stream input. This means the same handler function can accept TCP, GRPC, WebSocket, or other stream types as long as they implement a custom trait returning a `BoxStream`.

The software comprises five main components:

1. **Application Component Manager** – loads data into memory and creates streams.  
2. **User Accounts Cache Handler** – handles all transaction logic related to accounts and provides access via a HashMap.  
3. **Transaction Cache Handler** – stores all transactions and manages logic for disputes, resolutions, and chargebacks.  
4. **User Accounts** – handles logic related to individual user account instances.  
5. **Stream Handler** – creates interfaces for streaming data through the component manager, supporting any stream type that implements the custom trait.  

## Software Flow

```mermaid
flowchart TD
    A[Start CSV version of Application Component Manager] --> B[Application Component Manager initializes Stream Handler, caches, and error structures]
    B --> C[Start streaming data]
    C --> D{Is there an item in the stream?}
    D -- Yes --> E[Pass item to User Accounts Cache Handler]
    E --> F[Cache handler matches against handler string]
    F --> G[Pass to User Accounts]
    G --> H[Handle event logic for client details and transaction details]
    H --> D
    D -- No --> I[Print results in required format]
