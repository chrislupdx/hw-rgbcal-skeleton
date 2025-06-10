19:35:44.314: frame rate: 100ERROR nusb::platform::macos_iokit::transfer: Failed to submit transfer on endpoint 5: e000404f    
WARN probe_rs::session: Could not clear all hardware breakpoints: An ARM specific error occurred.
Caused by:
0: The debug probe encountered an error.
1: An error which is specific to the debug probe in use occurred.
2: Error handling CMSIS-DAP command Transfer.
3: Error in the USB access.
ERROR nusb::platform::macos_iokit::transfer: Failed to submit transfer on endpoint 5: e000404f    
WARN probe_rs::session: Failed to deconfigure device during shutdown: Arm(Probe(ProbeSpecific(BoxedProbeError(Send { command_id: Transfer, source: UsbError(Custom { kind: Other, error: Stall }) }))))
ERROR nusb::platform::macos_iokit::transfer: Failed to submit transfer on endpoint 5: e00002d8    
ERROR nusb::platform::macos_iokit::transfer: Failed to submit transfer on endpoint 5: e0004061    
Error An ARM specific error occurred.
Caused by:
0: The debug probe encountered an error.
1: An error which is specific to the debug probe in use occurred.
2: Error handling CMSIS-DAP command Transfer.
3: Error in the USB access.

https://github.com/probe-rs/probe-rs/issues/2338

