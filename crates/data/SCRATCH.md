# ECHIDNA - DATA

## STEPS

1. Setup participants that own publishers and subscribers.
2. Make sure participants can see each other.
3. Make sure the publisher's internal subscriber reference list is kept up to date automatically.
4. Send short string message from publisher to subscriber on other machine.

-- interlude: rework to full system, much like what we had in 2020

5. Make message larger than CHUNK_SIZE.
6. Send JPEG message from publisher to subscriber on other machine.
7. Simulate dropout at receiving end and handle appropriately with chunk resends.
8. Allow multiple processes to share the same participant. First process creates the participant. Last process drops it.
9. If subscriber is local, transmit message over shared memory.
10. Maybe use multicast where possible, or make that configurable.

## STEP 8

For now, create separate process for participant. Participant listens on fixed port for local TCP connections. Participant also communicates with all other participants over TCP. Connections have a small protocol:

Beacon:
    Hello(port)
        I'm a participant, you can connect to me at port

PartToPart:
    Init([{ id, topic, port, }],[{ id, topic, port, }])
        I'm new here, these are my publishers and subscribers
    InsertPublisher({ id, topic, port, })
        I have a new publisher
    RemovePublisher({ id, topic, port, })
        I have one less publisher
    InsertSubscriber({ id, topic, port, })
        I have a new subscriber
    RemoveSubscriber({ id, })
        I have one less subscriber

PartToPub:
    Init([{ id, ip, port, }])
        These are the current subscribers
    InitFailed(reason)
        Cannot initialize
    InsertSubscriber({ id, ip, port, })
        There is a new subscriber for you
    RemoveSubscriber({ id, })
        There is one subscriber less for you

PartToSub:
    Init
        Ok
    InitFailed(reason)
        Cannot initialize

ToPart:
    InitPub(topic)
        I publish this topic
    InitSub({ id, port, topic, })
        I subscribe to this topic

### Participant

- UDP beacon receiver 7331 Beacon
- UDP beacon broadcaster Beacon
- TCP listener: participants
- TCP connection: participants PartToPart
- TCP listener: local 7332
- TCP connection: locals ToPart; PartToSub + PartToPub

Create participant listener
    connect: communicate with remote participant PartToPart
    lost: remove proxy and inform all local publishers that remote subscribers have disappeared

Create local listener at 7332
    connect: receive ToPart, inform all local publishers that local subscriber has joined, inform all remote participants that local publisher/subscriber has joined, keep communicating with PartToSub or PartToPub
    lost: inform all local publishers that local subscriber has left, inform all remote publishers that local subscriber has left

Create beacon receiver at 7331
    message: if participant proxy unknown, connect to this port; keep alive

Create beacon broadcaster
    every n seconds: broadcast Beacon with participant listener port; kill off participant proxies that did not get refreshed
