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
- DONE: Send larger messages.
- Improve retransmit system.

Examples: hello_pub.rs, hello_sub.rs

These are publisher and subscriber for the ```/hello``` topic.

### Retransmits

Learn about other reliable UDP systems to find a better pattern.

- IDEA: after having received N chunks, the subscriber sends back acknowledgements of those chunks. publisher records this. after having sent everything, publisher periodically resends anything that wasn't acknowledged.

- IDEA: publisher sends N chunks, followed by a heartbeat, followed by N chunks, etc. subscriber responds to the heartbeats with acknowledgements. separate process of publisher listens to incoming acknowledgements and records them. After first process is done sending the chunks, it starts resending missing N chunks, followed by a heartbeat, etc. ==> let's try this.

There are many parameters to play with and many delay strategies. This might take a few more days.

I did not follow a quick hunch that TCP would be easier.

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
