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
- DONE: Improve retransmit system.

Examples: hello_pub.rs, hello_sub.rs

These are publisher and subscriber for the ```/hello``` topic.

### Retransmits

Learn about other reliable UDP systems to find a better pattern.

- IDEA: after having received N chunks, the subscriber sends back acknowledgements of those chunks. publisher records this. after having sent everything, publisher periodically resends anything that wasn't acknowledged.

Fails miserably on local loopback. Most chunks are lost and retransmitted at insane rates.

- IDEA: publisher sends N chunks, followed by a heartbeat, followed by N chunks, etc. subscriber responds to the heartbeats with acknowledgements. separate process of publisher listens to incoming acknowledgements and records them. After first process is done sending the chunks, it starts resending missing N chunks, followed by a heartbeat, etc. ==> let's try this.

Fails similarly miserable on local loopback. Most chunks are lost and retransmitted at insane rates.

There are many parameters to play with and many delay strategies. This might take a few more days. Also, let's read up on strategies in other protocols (what about TCP?) to see if we can Frankencopy something.

- IDEA: implement something similar to TCP, but stuff more chunks inside RTO. publisher sends N chunks, followed by heartbeat, subscriber acknowledges heartbeat only, if acknowledgement arrives before RTO, publisher calculates new RTT, derives new RTO, sends next N chunks, of which some might be retransmits, etc. until entire message is sent. If acknowledgement does not arrive, publisher retransmits same N chunks, followed by heartbeat. RTT is calculated from heartbeat sending to acknowledgement arrival, RTO is RTT + some constant.

Works very well on local loopback, with small N (< 7). Larger means the heartbeat gets lost (it appears subscriber-side IP rcvbuf is too small). Assume that very small N (2?) will work for now.

#### Some Measurements

comparison: iperf3 on TCP does 4596MB/s for local loopback and 112MB/s for network

- IDEA: publisher sends chunks at specific rate, listens to acknowledgments in parallel; Subscriber sends back either Ack (I got everything up to, but not including X), or NAck (I still need X up to, but not including Y). Publisher 

Works kind of. Algorithm starts to complain (lost packets) when rate gets above 5kHz, and this is not enough.

IMPORTANT: Local loopback TCP is probably implemented as memory pipe, so the speeds are not realistic!

So, move test to two machine setup.

This works very well. Maximum TCP throughput is 112 MBytes/sec, and this strategy gets up to 118 MBytes/sec after a few tweaks. There are more things to tune and tweak:

- piggyback heartbeats onto chunks to reduce traffic
- make interval dynamically adjust to reduce missing chunks
- reduce the need for mutexes where possible

And finally we need to test with a lot more traffic, especially send messages faster than they can be processed. One strategy, instead of canceling the tasks when a new message appears is to cut the incoming message instead. That way, when the network is really congested, messages are still coming through. --> config setting

Also, when a subscriber disappears, transmitting to that subscriber should stop. --> automatically handled by next point

And, if a subscriber doesn't respond after a certain countdown, transmitting to that subscriber should stop. - so in general make sure the transmission always ends, either by success, or by canceling subscribers. --> timeout on read, close associated task

Assuming incoming data is queued, replacing the two task system with just one task per subscriber works.

Further measurements...

- IDEA: measure throughput by taking only entries with 0% waste; also measure average waste, and spread in waste

The most stable configuration for now is large chunks (51200 bytes), 300usec transmit interval, no waiting for heartbeat.

Still found a lot of waste, so there are many packets sent, but not needed. Maybe do some research there still.

### Shared Memory

When subscriber is local, use shared memory to transport the message.

- IDEA: participant manages shm_open object, publisher requests from participant at start, subscriber receives shm_open fd from participant at start, so it can immediately connect

### Multicast

If possible/sensible see if managing multicast networks works.

### Target Platforms

- DOING: Linux
- Windows
- MacOS
- DOING: Android
- DOING: iOS

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
