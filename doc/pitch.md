browsers are currently owned by the big companies, and we want that to free up, because it's old tech

unreal is branching out to record movies with AR/VR tech, pretty soon they branch out further and own that space too

oculus is facebook, and pretty closed off, kinda like Apple

now listen up to this extremely dreamy and naive story, which is actually really cool:

using WASM and Rust we can break these problems

and design a real modern AR/VR context from the ground up

redefine 'the browser' to cover three important things:

    0. everything is a WASM blob

    1. allow multiple people, either in the same space or calling in with VR, to interact in 3D by combining their hardware into a single secure distributed system, a bubble

    2. completely share all available hardware for the app, parts of an app automatically migrate around the hardware to find the optimal place

    3. from participating users, register as much real human interactions as possible; standardizing body poses, facial expressions, hand gestures, etc. on top of key presses and mouse clicks

I'm already building up the basic technology for a bunch of available hardware

huge networking questions that have already been solved, but will need to be addressed properly

we make a few exploration apps that show off the use of this sort of technology

while doing that, we start to understand better and better what the app should look like and what kinds of APIs we should provide

keeping the discussion going, a killer app will appear; this could just be shared art, or a boring conference room, or a game idea

big business ideas:

    asana+calendar+slack+figma+etc. functionality accessed by just joining the business bubble, accessing everything by just looking at it and pointing at it!!

    medical and engineering training (this very likely already exists in AR/VR land)

    design+develop booster hardware, packed with hires 360 cameras, lidar, microphones, and processing power to extract human interaction from the group

    Alexa-likes or toys that operate in a bubble of a family, or playing kids


====

Florian Radke's TEDx talk explains great futuristic visions of what AR is. There are hundreds of talks like this. Simultaneously, many startups are wrapping their heads around AR/VR. AltSpace provides a great mixed experience. VisionXR is a great platform for shared engineering and learning.

Unfortunately, it's not the wonderful future that people envision at TEDx talks...

Part of the reason for that, like always, is that we simply don't know what to make, what works well, and what people prefer to use. Another part is the insanely complex technology. All the different platforms. All the different APIs. And all corporations fencing off and protecting their IPs. It's simply not possible right now to write a large collaborative AR.VR applications without having to employ a group of highly skilled developers.

It was also like that for complicated websites in the early 2000s, let alone web apps that came later. But the world caught up. New technologies became available. Web developers are now using easier networking APIs, easier graphics APIs.

Easier interaction APIs? Ehhh, not yet...

So what tools do we need? What APIs are good to tackle the bigger questions? How can we make this available at the highest possible experience and performance levels on todays hardware?

This is what I want to research.

====

How is this different from Unity MARS or similar? It seems that doing things local might still be the big win over cloud-based. That can be verified by building a toppler game in AR and VR, which needs a good and fast physics simulation.

Carbon footprint is bigger for cloud-based solutions

====

Arguments:

1. Resource sharing across local network is potentially better for high-performance games and such.

2. Supporting human interaction of the whole group makes apps like Alexa and Siri trivial, and experiences like spatial.io much easier to develop with web dev -like people.

3. A well-placed python interpreter allows for much faster prototyping of the experiences.