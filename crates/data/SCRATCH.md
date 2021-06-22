# ECHIDNA - DATA

## STEPS

### Basic Participant/Publisher/Subscriber Support

One participant for each machine. Many publishers and subscribers for each machine. Publishers and subscribers transmit on a topic. Each publisher needs to be kept aware of where the relevant subscribers are in the network. Participants need to automatically see each other and reconnect. Publishers and participants need to automatically see each other and reconnect. Subscribers and participants need to automatically see each other and reconnect.

### Transmit Messages

First tiny, fitting in one chunk. Have subscriber report if chunk arrived. If not, resend. Then multiple chunks.

### Damage Simulation

Simulate dropout at subscriber side.

### Shared Memory

If subscriber is local, transmit message over shared memory.

### Multicast

If possible/sensible see if managing multicast networks works.

### Large Tests

### Improve and Document
