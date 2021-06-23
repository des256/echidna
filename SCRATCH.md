# Challenges

## I. Basic Software

This boils down to `codec`, `data`, `ui`, `record` and `visualize` crates:

### `codec`

`serde`, but smaller and faster.

### `data`

Publish/subscribe service over reliable UDP, mimicking ROS/ROS2/DDS.

### `ui`

Basic UI API, could be an existing one, with support for 3D. Currently not very important, but it shouldn't suck.

### `record`

Facility to record and playback messages in a `data` session.

### `visualize`

Facility to visualize signals passing around in a `data` session.

## II. Accessing Existing Hardware and APIs

Research what is available, and implement adapters for iOS ARKit, Oculus Quest, mouse/keyboard, Intel RealSense, OpenCV camera, etc.

## III. World Model

In order to get a working understanding of the surroundings, all hardware should be working together. Since vision is most likely going to be the biggest contributor, start by exploring cameras, camera tracking, multiple camera setups, etc.

As soon as a 3D space is established, 

### Mapping

SLAM-like algorithm to figure out the geometry of a 3D space by combining multiple static cameras. Get there in steps to really understand the difficulties.

Also, first look more in-depth at existing SLAM problems, and how they are solved.

Some ideas:

#### 1. Two RGB-D Cameras

Two RGB-D cameras look at a same scene from arbitrary positions and angles, with overlap. By converting the RGB-D images into point clouds, it might be possible to estimate the camera poses by reducing some metric in NLS.

#### 2. Two RGB cameras

For two RGB cameras, this becomes a lot harder. One idea is to train a neural network from synthetic data. Unsure how accurate this is.

#### 3. RGB-D and RGB camera

Extract the point cloud from the RGB-D camera, and estimate the camera pose of the other camera by reducing some metric in NLS.
