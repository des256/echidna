# ECHIDNA

## Motivation

The idea is to reach into the future as far as possible, and try to realize this today.

### Low-level Internet Technology of the Future

(compiled from various futurists, podcasts and own experiences)

#### True Multi-User Environment

Contrary to todays single-user world, where we connect to a network through browsers and apps, the interface of the future is going to be bits of software that run on all sorts of devices, working together to provide the most natural and optimal interactive experiences to the users.

#### AR/VR/XR becomes simply "Reality"

Currently, AR/VR (or XR) is accessed through the use of specific hardware and/or a browser. Often
also by becoming a member of something, or logging in to something. In the future, all devices and screens will disappear, and we'll be wearing strange sunglasses at most. Apps will always be 3D (with the occasional smart use of 2D surfaces).

#### Fully Integrated Perception

Since communication and collaboration is about humans, the technology will naturally form itself more and more around understanding humans and in particular, human social signals. The local network of devices around a group of people will maintain a high fidelity model of what is going on with and around the people.

### Echidna

Echidna is a testing garage for this kind of technology. Roughly based on similar efforts in (Social) Robotics. It will be developed entirely in Rust, because of its development speed and built-in safety around memory and multi-threading. Echnidna runs on all platforms.

### Potential Directions

#### The Browser of the Future

As mentioned, browsing the Internet should be a multi-user thing, not designed around a tiny window into Facebook, Twitter or Google.

#### Voice Assistants, Social Robots and Internet of Things

The technology can be used as a rich modeling service for a wide variety of social robots and related toys. In order to enhance perception capabilities, the robots can simply connect and take part in the system. This should boost interactive possibilities beyond what is possible now.

It would also potentially lower the manufacturing cost of these robots, since most of the complex perception code is not running on the robot any more.

#### Video Games

Using this technology allows for development of new kinds of video games, were (groups of) people can play naturally together in any setting.

One of the first steps in this direction is the integration of a Python interpreter over the basic APIs that access Echidna.

## Roadmap

Parallel to the items in this roadmap starts an ongoing trajectory of experiments that can be showcased in blogposts and videos.

### 1. Basic Hyper-fast Ubiquitous Data Network

The first step is to reimagine or adopt DDS/RTPS or similar technology to allow for the fastest and most economic possible network communication between all participating devices. Initially, a simplified reliable multicast UDP transport will suffice.

### 2. Data Inspection and Visualization

Next to a working data network, start building a tool that allows full realtime inspection of the data, timing, content, etc. Some data could be visualized. Ideally/later this tool can be accessed in a 3D context as wel as a 2D context.

### 3. Data Recording and Playback

The other pillar of the data network is a storage and playback system, that allows examining and re-rendering of activities from earlier.

### 4. Elementary Computer Vision and Analysis

The next step is to implement as many vision/analysis algorithms as possible on each device separately. To save time and effort, leverage existing XR possibilities directly. This should at some point freeze into a rich set of accessible metadata about what each device senses.

### 5. Combining into the World Model

THIS IS POTENTIALLY THE MOST DIFFICULT!

From what each device gathers individually, a dynamic best World Model is created. Dynamic in that devices can join or leave whenever they want, and the World Model will adapt/switch to lower fidelity information. Best in that the more information is available, the more accurate the World Model will be.

### 6. Integrating AR and VR Platforms

For all supported platforms, integrate the World Model with existing AR and VR facilities.

### 7. One Specific Target

Hopefully, a more focused (and funded) use for Echidna will present itself along the way.
