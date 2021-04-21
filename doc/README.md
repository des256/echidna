# Introduction

## What is it?

Orbital (https://github.com/orbitalweb) is a project to explore possibilities for a new browser. It is believed that the way forward is a microkernel that manages a bunch of tiny WASM modules.

Echidna explores a radically new interface context for apps on this new browser. Currently, web apps are evolved from a text interface with 2D widgets/icons/mouse/etc. Echidna fully incorporates 3D interfaces instead.

**The Bubble** - Echidna introduces the bubble, a virtual space for people to collaborate using the same app with any hardware. A bubble can map onto reality, for instance, onto a table in a conference room. Security and privacy inside a bubble are very important for Echidna.

**Human Interaction Events** - Next to bubbles, Echidna adds a whole bunch of new human interaction events to the already available mouse, touch and keyboard. This includes body language, facial expressions, speech, hand gestures, eye tracking, etc. Apps can and should respond to all of them, wherever sensible. In a bubble, one global understanding is maintained, of the users and their subsequently generated events.

**Complete Sharing of Resources** - Echidna also introduces complete sharing of resources inside a bubble. Users with different kinds of hardware can use all available resources from the other users in the bubble seemlessly. Code can automatically migrate between platforms to optimize the overall experience for the team. This can be for interactive reasons, but also to improve performance, battery usage, etc.

Echidna prefers not to use any cloud computation, but that might not always be achievable or practical.

## Challenges

One of the biggest challenges is the networking layer, where everyone in the bubble needs to be able to send and receive the important information smoothly. Initially, it will use a subscribe/publish system, sharing topics around, much like ROS. Later, this communication can be refined to include various kinds of streaming and shared memory solutions.

Echidna is definitively not synchronous in a way that, for instance, Croquet is. This means there will be a whole bunch of event synchronization challenges inside a bubble. Events originate depending on which hardware has the best capabilities (and which camera is best positioned).

Then there is 3D representation, which is a Vulkan-like (or even Vulkan itself) API, followed by a ThreeJS-like API for scene graph management.

Using the latest deep learning algorithms for various human interaction events, is of secondary concern. Initially, hardware and systems that already support AR/VR interfaces are just used as-is.

## Supported Systems

Echidna is being developed on Linux using Rust. This is the most reliable modern compiled language, and allows for smooth compilation to actual WASM environments, once this becomes a thing.

# Human Interaction Events

Events/features to consider:

- mouse on surface
- keyboard
- world anchor
- VR controller
- game pad
- fingers touch on surface
- face pose
- face blendshapes
- face/user ID, deduced user ID
- eyes target
- deduced high-level face events: 'smile', 'blink', 'nod', etc.
- hand pose
- finger point targets
- deduced high-level hand events: various signs, holding up fingers, etc.
- body pose
- deduced high-level body events: 'step back', 'lean in', 'sit down', 'stand up', etc.
- speech events
- 3D realization of real world
- video
- audio
- depth

# Client Modes

The various ways a user can interact with an app:

- 2D interface on monitor or laptop with a mouse and keyboard
- 3D VR interface on monitor or laptop, interacting like a 1st or 3rd person video game
- 2D interface on a phone with multitouch
- 3D VR interface on a phone with multitouch
- 3D AR interface on a phone with multitouch
- 3D AR interface on hololens or similar
- 3D VR interface on VR headset

It's probably possible to switch between different interaction modes, depending on what the app needs.

Some of the 3D information (like an office plan with monitor and camera poses) needs to be inferred from config files.

# === Scratch Area ===

The following still needs to be processed:

```
so basically:
- each piece of hardware has a description file that indicates what services it provides
- services can be derived off of other services automatically via graph completion (video stream can generate various face and hand services (at a cost))
- that means that actual high-end deep learning algorithms can be considered much later in the process
- static objects might even need some kind of 3D description (i.e. the guy in VR would like to know where the monitor is that the guy in AR looks at)

security:
- ask for allowing data to be shared
- ask to join a bubble
- ...

bubble:
- all processing is for the bubble only, should be secure/encrypted
- everyone needs to confirm their data is used for the bubble, and/or everyone in the bubble can use it
- real users (with no HW at all) join in real setting
- fixed users can join in real setting (if their laptop is there), or as VR avatar
- VR users join as projected avatars, world is streamed to their headset
- AR users join in real setting, with extra possibilities

apps:
- meeting with figma-like shared document (through makepad?)
- vape-o-tron using face expression data
- falling tower of blocks (with physics sim)
- watch movie together, throw popcorn, etc.

questions:
- Big: How much extra use can we get from this sort of collaborative environment over just a meeting?
- Big: Is it possible to add these features to already existing meeting places and protocols?
- Are there APIs for standardized avatars in the various VR product lines?
- Can real users reliably be scanned by other hardware? Is that in any way useful?
- Is there any good local speech to text software?
- Further non-cloud software?
- Are there any other projects that try to combine AR and VR in a meeting place?
- What can we distill from all of this to inform Orbital of their kernel limitations?
```