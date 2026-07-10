# Manual audio UAT guide

Use this guide to validate audio-device behavior that CI cannot reproduce.
Run every applicable scenario on real hardware before a release candidate is
approved.

## Prepare the test run

1. Record the operating system version, application commit, interface model,
   driver version, connection type, sample rate, and requested buffer size.
2. Build and launch the standalone application by following the
   [cross-platform build guide](BUILD.md).
3. Open **Audio Settings**, select the required host and devices, and start a
   continuous input signal or known looped source.
4. Record the visible `underruns` and `overflows` before each scenario.
5. Run normal scenarios for at least 10 minutes. Run hot-unplug scenarios for
   at least 30 seconds before disconnecting the device.

Use headphones or low monitor volume during routing and hot-unplug tests.

## Record results

Create one row for every scenario. Do not mark a scenario as passed without
recording all three observability fields: underruns, overflows, and error
banner behavior.

| Field | Value |
| --- | --- |
| Tester and date | |
| Commit | |
| OS and version | |
| Host and mode | |
| Input device and format | |
| Output device and format | |
| Driver and firmware | |
| Sample rate and buffer | |

| Scenario | Duration | Underruns before → after | Overflows before → after | Error banner appeared? | Banner text or notes | Pass? |
| --- | --- | --- | --- | --- | --- | --- |
| macOS: single device | | | | | | |
| macOS: distinct physical devices | | | | | | |
| macOS: aggregate device | | | | | | |
| macOS: hot-unplug | | | | | | |
| WASAPI: shared mode | | | | | | |
| WASAPI: exclusive mode | | | | | | |
| WASAPI: hot-unplug | | | | | | |
| ASIO: real I32 interface | | | | | | |
| ASIO: duplicate device names (T30) | | | | | | |
| ASIO: 24-bit-only driver (T19) | | | | | | |
| ASIO: single-device duplex (T18) | | | | | | |

For hot-unplug tests, an error banner is expected. For normal playback tests,
an error banner is a failure. Explain any counter increase; a sustained or
silent increase is a failure even when audio appears to continue.

## macOS and CoreAudio

### Single device

1. Select the same physical CoreAudio interface for input and output.
2. Select matching physical channels and start playback.
3. Run for at least 10 minutes while changing the requested buffer size once.
4. Confirm that audio remains continuous and correctly routed.
5. Record underruns, overflows, and whether an error banner appeared.

### Distinct physical input and output devices

This scenario must use two independent physical devices. Do not substitute an
aggregate device because the purpose is to exercise real clock drift.

1. Select one physical device for input and a different physical device for
   output.
2. If possible, configure different nominal rates first, such as 44.1 kHz
   input and 48 kHz output, then repeat with matching nominal rates.
3. Run for at least 10 minutes so independent clocks have time to drift.
4. Confirm that playback does not gradually crackle, stall, speed up, or slow
   down.
5. Record underruns, overflows, and whether an error banner appeared.

### Aggregate device

1. In **Audio MIDI Setup**, create an aggregate device from at least two
   physical interfaces.
2. Choose an appropriate clock source and enable drift correction for the
   secondary device.
3. Select the aggregate device in the standalone application for input and
   output.
4. Exercise channels from both physical interfaces for at least 10 minutes.
5. Confirm that channel mapping remains correct and audio stays continuous.
6. Record underruns, overflows, and whether an error banner appeared.

### Hot-unplug during playback

1. Start playback through an external CoreAudio interface.
2. Disconnect the interface while audio is active.
3. Confirm that the application stays open and shows an audio error banner.
4. Reconnect the interface, refresh the device list, and restore routing.
5. Confirm that the stale device selection is not silently routed to a
   different device.
6. Record underruns, overflows, whether the banner appeared, and its text.

## Windows and WASAPI

Use a build without the `asio` feature and select the WASAPI host. Configure
shared or exclusive access in Windows or the device control panel, and record
how the active mode was verified.

### Shared mode

1. Configure the device for WASAPI shared access.
2. Start another application that uses the same output device.
3. Start standalone playback and run both applications for at least 10
   minutes.
4. Confirm that both applications remain audible and the standalone stream is
   stable.
5. Record underruns, overflows, and whether an error banner appeared.

### Exclusive mode

1. Enable exclusive access for the endpoint and select exclusive mode through
   the available Windows or driver control.
2. Close other applications that may hold the endpoint.
3. Start standalone playback and verify that the endpoint is operating in
   exclusive mode using the same control or driver diagnostics.
4. Run for at least 10 minutes and confirm stable audio.
5. Record underruns, overflows, and whether an error banner appeared.

If the installed driver exposes no way to select or verify exclusive mode,
mark this scenario as blocked and record the limitation. Do not infer
exclusive operation from the Windows permission checkbox alone.

### Hot-unplug during playback

1. Start WASAPI playback through an external interface.
2. Disconnect the interface while audio is active.
3. Confirm that the application stays open and shows an audio error banner.
4. Reconnect the interface, refresh devices, and restore routing.
5. Record underruns, overflows, whether the banner appeared, and its text.

## Windows and ASIO

Build with `--features asio` as described in the
[Windows ASIO build instructions](BUILD.md#optional-asio-support), then select
the ASIO host. Use the vendor's native driver, not ASIO4ALL or another WASAPI
wrapper, unless a scenario explicitly records that substitution.

### Real interface with I32 input

1. Connect a physical ASIO interface whose input negotiates `I32`.
2. Confirm the negotiated format through driver diagnostics or application
   logging and record the evidence.
3. Route a known signal through each selected input channel.
4. Confirm correct polarity, level, channel order, and playback speed.
5. Run for at least 10 minutes.
6. Record underruns, overflows, and whether an error banner appeared.

### Two interfaces with identical names (T30)

1. Connect two interfaces that expose exactly the same ASIO device name.
2. Refresh the device list and select the second enumerated interface.
3. Route a signal that is physically connected only to that second interface.
4. Refresh the list again and confirm that routing still targets the second
   interface rather than the first name match.
5. Repeat after restarting the standalone application if the device order is
   stable on the test system.
6. Record underruns, overflows, and whether an error banner appeared.

### Driver locked to 24-bit (T19)

1. Configure a real ASIO interface or driver for its 24-bit-only mode.
2. Refresh the device list.
3. Confirm that the interface remains visible and is marked as unsupported,
   rather than disappearing from the picker.
4. Confirm that the application does not attempt to start an invalid stream or
   crash when the entry is inspected.
5. Record underruns, overflows, and whether an error banner appeared.

### Duplex from one device (T18)

1. Select one physical ASIO interface for both input and output.
2. Route a live input through the standalone processing path to the same
   interface's output.
3. Confirm that starting the output stream does not stop or replace the input
   stream.
4. Exercise at least two channels in each direction for 10 minutes.
5. Confirm that both directions remain active and correctly mapped.
6. Record underruns, overflows, and whether an error banner appeared.

## Completion criteria

A platform passes UAT only when all applicable scenarios have completed
records and no unexplained sustained counter growth, silent stream failure,
crash, or missing hot-unplug error banner remains. File defects with the full
result row, exact banner text, logs, and reproduction steps.
