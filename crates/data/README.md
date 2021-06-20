# ECHIDNA - DATA

## STEPS

1. Setup participants that own publishers and subscribers.
2. Make sure participants can see each other.
3. Make sure the publisher's internal subscriber reference list is kept up to date automatically.
4. Send short string message from publisher to subscriber on other machine.

5. Make message larger than CHUNK_SIZE.
6. Send JPEG message from publisher to subscriber on other machine.
7. Simulate dropout at receiving end and handle appropriately with chunk resends.
8. Allow multiple processes to share the same participant. First process creates the participant. Last process drops it.
9. If subscriber is local, transmit message over shared memory.
10. Maybe use multicast where possible, or make that configurable.
