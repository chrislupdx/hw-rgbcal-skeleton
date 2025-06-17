# rgbcal: RGB LED calibration tool
Chris 2025

- Make the measurements and give an approximate minimum frame rate and maximum percentage on time for Green and Blue to get a decent White. Add those to the README.

Prelim decent white:
- red:11
- green: 7
- blue: 10
- frame rate: 10
<currently does not have framerate fully working yet>

6/17 8:42 update: (its not bad)
red: 7
green: 7
blue: 7
frame rate: 8

todo:
Share the frame rate value between the Ui and the Rgb structs.
Adjust Rgb delays according to the frame rate.


- all of the colors need to go from off to on over 16 steps
- frame rate: need to hcange in steps of 10 frames per second 10..160

A writeup of what you did:
- used the wrong knob
- spent most of the time figuring out the code (figured out how to adjust colors, the ui's state values)

6/17: 8:41, oh yes getters are useful. what a wonderful reminder

How it went:
- challenging in the weirdest of places outside of rotary dial shennnigans. stay tuned

any other observations you find of interest: n/a

----------------------------------------------------------------------------
This tool is designed to find out a decent frame rate and
maximum RGB component values to produce a white-looking RGB
of reasonable brightness.

See below for UI.

**XXX This tool is *mostly* finished! Please wire your
hardware up (see below), finish it, comment it, and use it
to find good values. Then document those values in this
README.**

## Build and Run

Run with `cargo embed --release`. You'll need `cargo embed`, as
`cargo run` / `probe-rs run` does not reliably maintain a
connection for printing. See
https://github.com/probe-rs/probe-rs/issues/1235 for the
details.

## Wiring
 
Connect the RGB LED to the MB2 as follows:

* Red to P9 (GPIO1)
* Green to P8 (GPIO2)
* Blue to P16 (GPIO3)
* Gnd to Gnd

Connect the potentiometer (knob) to the MB2 as follows:

* Pin 1 to Gnd
* Pin 2 to P2
* Pin 3 to +3.3V

## UI

The knob controls the individual settings: frame rate and
color levels. Which parameter the knob controls should be
determined by which buttons are held. (Right now, the knob
jus always controls Blue. You should see the color change
from green to teal-blue as you turn the knob clockwise.)

* No buttons held: Change the frame rate in steps of 10
  frames per second from 10..160. <done with caveats>
* A button held: Change the blue level from off to on over
  16 steps. <done, with caveats>
* B button held: Change the green level from off to on over
  16 steps. <done, with caveats>
* A+B buttons held: Change the red level from off to on over
  16 steps. <done, with caveats>

The "frame rate" (also known as the "refresh rate") is the
time to scan out all three colors. (See the scanout code.)
At 30 frames per second, every 1/30th of a second the LED
should scan out all three colors. If the frame rate is too
low, the LED will appear to "blink". If it is too high, it
will eat CPU for no reason. <in progress>

I think the frame rate is probably set higher than it needs
to be right now: it can be tuned lower.

**LED Specifications**

[LED Wiring Diagram](https://docs.sunfounder.com/projects/sf-components/en/latest/component_rgb_led.html#:~:text=We%20use%20the%20common%20cathode%20one.&text=An%20RGB%20LED%20has%204,%2C%20GND%2C%20Green%20and%20Blue)
