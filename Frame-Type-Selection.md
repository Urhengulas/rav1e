## Current Features/Process
* The first frame of the video is always a key frame.
* rav1e looks ahead up to the maximum number of frames in a sub-GOP
to detect "flashes" of content, which are then marked ineligible for
a scenecut.
* If the user has forced the current frame to be a key frame, it is marked as a key frame.
This overrides all other criteria for frame type selection. (TODO: How does a user do this?)
* If there have been fewer than the number of frames specified by `--min-keyint`
since the last key frame, the current frame is marked as an inter frame.
* If there have been equal to the number of frames specified by `--keyint`
since the last key frame, the current frame is marked as a key frame.
* If no other criteria have been met, the current frame is compared with
the previous frame using a simple difference detection to see if it is a scenecut.
If it is a scenecut, it is marked as a key frame, otherwise it is marked as an inter frame.

## Desired Features
* If the max keyint length is in the middle of a flash of content, the key frame should be placed at either the start or end of the flash, instead of in the middle (exactly on the max keyint).
* Replace the current scenecut detection algorithm with a smarter, cost-based one. [#1528](https://github.com/xiph/rav1e/issues/1528)