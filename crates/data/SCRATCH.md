# ECHIDNA - DATA

## HOW TO GET THERE

### Basic Participant/Publisher/Subscriber Support

- DONE: Each machine runs a participant. Participants automatically find each other through UDP beacons.
- DONE: Participants maintain direct TCP streams among each other. This is used to communicate state changes.
- DONE: Publishers and subscribers maintain direct TCP streams with the local participant.
- DONE: All participants learn of the new or lost publisher.
- DONE: A publisher knows about all subscribers of the same topic on the network.
- DONE: All participants learn of the new or lost subscriber.
- DONE: All publishers learn of the new of lost subscriber.

Example: participant.rs

This starts a participant on the local machine.

### Transmit Actual Messages

- DONE: Send tiny messages, fitting inside one chunk.
- DONE: Have subscriber report if chunk arrived. If not, resend.
- Send larger messages.

Examples: hello_pub.rs, hello_sub.rs

These are publisher and subscriber for the ```/hello``` topic.

### Shared Memory

- When subscriber is local, use shared memory to transport the message.

### Multicast

If possible/sensible see if managing multicast networks works.

### Target Platforms

- DOING: Linux
- Windows
- MacOS
- Android
- iOS

### More Features

- DONE: Heartbeat strategy.
- Make sure there can be only one publisher on a topic.
- Signal quality?

### Large Tests

- Artificial dropout at subscriber side.
- Various message sizes.
- Various message speeds and throughputs.
- Play with different chunk sizes.

### Diagnostics

- Publishers and subscribers should send measurement data to the participants.
- Participant can be interrogated about diagnostics data.

### Document

- Documentation.
