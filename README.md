# Covert 
<img align="right" width="25%" alt="image" src="https://github.com/user-attachments/assets/8a0c1a6b-963d-4f91-af37-3f4687ed044c" />

Covert is a adapter for mobile device providing a small converter from USB to Jack (for audio) with other awsome features.
It is currently in brainstorming stage.

----

## Proposed Peripherals:

These are the proposed Peripherals for the project.
1.  Audio input: 3.5 mm jack → ADC or I²S peripheral
2.  USB output: USB Audio Class device (or passthrough endpoint)
3.  Bluetooth transmitter: uses a UART/SPI interface to a BT module
4.  Button: GPIO input with interrupt

## Logic flow and Modes:

### Logic Flow:
1. By default, jack audio → USB output.
2. On button press → switch route: jack/USB → Bluetooth TX.
3. Power always from USB VBUS (so we can use USB 5 V as main supply).

### Suggested Modes:

a) Mode 1 — Passive (No Power)
1. Jack → USB (headphone mic uplink)
2. USB → Jack (headphone audio downlink)

This is analog passive routing using physical switches or FETs. MCU is OFF, so no code is run,
but we keep this mode in the enum for completeness.

b) Mode 2 — Powered USB, Single Bluetooth Link
1. Audio OUT (from phone/jack) → Bluetooth
2. Mic IN (from BT headset) → phone via USB or jack
3. MCU handles Bluetooth streaming.

c) Mode 3 — Powered USB, Multi-Device Bluetooth
- Audio & mic routed over multiple Bluetooth connections (e.g., group listening or collab)
- Requires handling multiple BT sinks/sources.

## Proposed mIcrocontroller

<img width="158" height="219" alt="image" src="https://github.com/user-attachments/assets/cf906f8d-5c0c-41f1-93a8-b1b715913352" />

**Pro Micro nRF52840**:
This microcontroller is used for the specific purpose of better rust compatability. It is also compact in nature and can be easily attached like a accessory 
to any modern mobile system.

### Specifications

1. **Core**	ARM Cortex-M4F (floating-point unit)
2. **Flash memory**	1 MB
3. **RAM**	256 KB
4. **Wireless protocols**	Bluetooth 5 (or later), Thread, Zigbee, ANT, 2.4 GHz proprietary radio
5. **USB interface**	Native USB 2.0 full-speed (12 Mbps) peripheral in the SoC
6. **Typical supply voltage / board support**	The board supports USB/Type-C, Li-ion battery interface (3.7 V), and regulated input. Example spec: Operating voltage ~3.6V – 6V for board.
7. **Standby / low power**	Some board listings claim standby current ~1 mA or ~20 µA depending on configuration.
8. **Board dimensions**	~ 33 mm × 17.8 mm (board size as per one listing)
9. **I/O & peripherals**	ADC, PWM, SPI, I2C, UART, GPIO and I2S/PDM audio interface.
