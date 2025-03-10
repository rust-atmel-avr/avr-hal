This project is a fork of [avr-hal by Rahix](https://github.com/Rahix/avr-hal).

# Gratitude

I have endless gratitude for all the work done by the maintainers and contributors of
`Rahix/avr-hal`, especially https://github.com/Rahix and https://github.com/stappersg. Without their
excellent work, I would have quit Rust AVR development a long time ago. This fork stands on the
shoulders of giants, and I am grateful beyond words.

# Rationale

The fork focuses on three main areas of improvements to the upstream:

1. Hardware devices supported by this project (individual MCUs and vendor boards) are represented by
   device description files, written in Rust with generous helping of macros. This should make it
   easier to add support for new devices and new device families. For an example of a device
   description file, see the [ATmega 32u4 device description](./mcu/atmega-hal/src/atmega32u4.rs).
2. Example code (both standalone and doctests) in this project is compiled by CI. This both makes it
   easier for contributors to catch regressions and makes it easier to get started on your own
   projects. For an example doctest, see ?
3. On-device tests will be added in this project, with the goal that support for new and existing
   devices can be tested on actual hardware.

Forking is not my first choice; I would prefer not to divide the attention of the contributor
community. However, I also want to make sure that my changes (which I hope substantially increase
long-term robustness of AVR support in Rust) are available to the community. I am open to merging
the fork back into the upstream in the future.

# Status

- Device description files: done for individual MCUs. Not started for vendor boards.
- Valid doctests: done for individual MCUs. Not started for vendor boards.
- On-device testing: in development for ATtiny MCUs. Not started for ATmega MCUs or vendor boards.

# Trajectory

In December 2024, I did some preliminary work in `Rahix/avr-hal` PRs and discussions to begin
implementing device description files.

In March 2025, I created `rust-atmel-avr/avr-hal` fork in order to move my work forward and make it
available to others in the community. Between March 2025 and June 2025, I plan to continue syncing
the fork with the upstream, and I will avoid making any other major disruptive changes in the fork.
I am hoping that this creates an opportunity for this work to be incorporated into the upstream.

Beyond June 2025, I will focus my effort on maintaining the fork as a separate project, and will
probably no longer have the capacity to port changes from the upstream into the fork.
