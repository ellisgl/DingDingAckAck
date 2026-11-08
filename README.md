# Ding Ding Ack Ack (DDAA) Protocol

DDAA is a small, framed, request/response protocol for talking to a microcontroller over a serial link. Either side can send a request. The receiver answers with a success (ACK) or failure (NAK) response that echoes the command and variable it is answering.

The protocol is used by:

* [MewtureButton-Firmware](https://github.com/ellisgl/MewtureButton-Firmware): an Arduino/ESP8266 microphone mute button with a status LED.
* [MewtureButton-Host](https://github.com/ellisgl/MewtureButton-Host): a Linux daemon that links the button to a PulseAudio source's mute state.
* [`ddaa_protocol`](https://crates.io/crates/ddaa_protocol): the Rust crate in this repository. It is the reference encoder and parser, and the host uses it.

The name comes from the ASCII control characters that start each frame. A request starts with `BEL` (0x07, the "ding"). A successful response starts with `ACK` (0x06, the "ack").

---

## Rust crate

```toml
[dependencies]
ddaa_protocol = "0.2"
```

```rust
use ddaa_protocol::{create_protocol_buffer, parse_protocol_message, Command, MessageType};

// Build a frame: write 0x01 to variable 0x00.
let frame = create_protocol_buffer(MessageType::Request, Command::Write, 0x00, &[0x01]);
assert_eq!(frame, [0x07, 0x01, 0x1A, 0x00, 0x01, 0x01, 0x17, 0x04]);

// Parse it back.
let message = parse_protocol_message(&mut frame.clone()).unwrap();
assert_eq!(message.command, Command::Write);
assert_eq!(message.data, vec![0x01]);
```

`parse_protocol_message` takes one buffer and returns the first frame in it.

* It drops leading bytes until it finds a start pair, so it changes the `Vec` you pass in.
* It does not report how many bytes the frame used.
* It returns `None` when a frame is incomplete and when a frame is invalid, so you can't tell the two apart.
* It needs at least 8 bytes, so it never returns a frame with no data (N = 0).

When you read from a serial port, collect bytes until you have a whole frame first, and then parse it. [MewtureButton-Host's `FrameAccumulator`](https://github.com/ellisgl/MewtureButton-Host/blob/fix/ddaa-frame-reassembly/mewture_shared/src/frame.rs) does this.

---

## 1. Transport

| Setting      | Value                                       |
|--------------|---------------------------------------------|
| Link         | Serial (in practice USB CDC / USB-UART)     |
| Baud rate    | 115200                                      |
| Framing      | 8N1 (the default on both sides)             |
| Flow control | None                                        |

The protocol does not define a master. Either side can send a request at any time.

---

## 2. Frame format

Every frame has the same layout. Offsets start at 0.

```
 Offset  Size  Field
 ------  ----  -----------------------------------------------
 0       1     Start byte 1 / message type  (0x07, 0x06, 0x15)
 1       1     Start byte 2                 (always 0x01, SOH)
 2       1     Command                      (0x05, 0x1A, 0x07)
 3       1     Variable                     (application-defined)
 4       1     Data length N                (0–32, see note)
 5       N     Data                         (application-defined)
 5+N     1     End byte 1                   (always 0x17, ETB)
 6+N     1     End byte 2                   (always 0x04, EOT)
```

Frame length is `7 + N` bytes. With the usual N = 1 to 32, that is **8 to 39 bytes**.

> **Data length.** The crate's encoder will produce N = 0 (a 7-byte frame), and its parser accepts one. The MewtureButton host only tries to parse reads of more than 7 bytes, so a 7-byte frame would be ignored. **Always send at least one data byte.** Use `0x00` as a placeholder when the command has nothing to carry (for example, a read request).

### 2.1 Message type (byte 0)

| Value  | ASCII | Meaning                      |
|--------|-------|------------------------------|
| `0x07` | BEL   | Request                      |
| `0x06` | ACK   | Response: success            |
| `0x15` | NAK   | Response: failure            |

### 2.2 Commands (byte 2)

| Value  | ASCII | Command | Purpose                                              |
|--------|-------|---------|------------------------------------------------------|
| `0x05` | ENQ   | Read    | Ask the peer for the value of a variable            |
| `0x1A` | SUB   | Write   | Set a variable on the peer                          |
| `0x07` | BEL   | Ping    | Presence, discovery and keep-alive; carries an identity payload |

A response always echoes the **command** and **variable** of the request it answers.

### 2.3 Variable (byte 3)

The variable is an application-defined register number. The protocol gives it no meaning. Section 5 lists the variables MewtureButton uses.

### 2.4 Escaping

**There is none.** No byte stuffing or escaping is done, and there is no checksum. This has two consequences:

* A receiver that finds the end of a frame by scanning for `0x17 0x04` (the firmware does this) will cut the frame short if the data contains that pair. **Data must not contain `0x17` followed by `0x04`.**
* A receiver that resynchronises by scanning for a start pair (both implementations do) can lock onto a `0x07 0x01` or `0x06 0x01` that appears inside a frame's data.

---

## 3. Message semantics

### 3.1 Read

```
Request:  07 01 05 <var> 01 00 17 04          (data is a 1-byte placeholder)
Response: 06 01 05 <var> <N> <value...> 17 04
```

The request's length and data fields are ignored, but they must still be present. The response carries the variable's current value.

### 3.2 Write

```
Request:  07 01 1A <var> <N> <value...> 17 04
Response: 06 01 1A <var> <N> <value...> 17 04   (success: echoes the written data)
          15 01 1A <var> <N> <value...> 17 04   (failure: echoes the rejected data)
```

### 3.3 Ping

```
Request:  07 01 07 00 <N> <identity...> 17 04
Response: 06 01 07 00 <N> <identity...> 17 04   (data echoed back unchanged)
```

The variable is `0x00`. The data is an identity payload the sender chooses. The responder echoes it back, so the sender can check that the reply is for its ping. Ping also serves for discovery (section 6.1).

### 3.4 Failure responses

A NAK echoes the request's command, variable and data. The spec allows the data to be replaced with an application-defined error payload, but the MewtureButton host just echoes the request data.

---

## 4. Byte-level examples (MewtureButton)

| Direction     | Meaning                         | Bytes |
|---------------|---------------------------------|-------|
| Device → Host | Ping with identity              | `07 01 07 00 0C 00 00 00 01 00 00 00 01 00 00 00 01 17 04` |
| Host → Device | Ping reply (echo)               | `06 01 07 00 0C 00 00 00 01 00 00 00 01 00 00 00 01 17 04` |
| Device → Host | Read mute state                 | `07 01 05 00 01 00 17 04` |
| Host → Device | Mute state is "muted"           | `06 01 05 00 01 01 17 04` |
| Device → Host | Button press: toggle mute       | `07 01 1A 00 01 02 17 04` |
| Host → Device | Toggle accepted                 | `06 01 1A 00 01 02 17 04` |
| Host → Device | Push new state: unmuted         | `07 01 1A 00 01 00 17 04` |
| Host → Device | Push new state: muted           | `07 01 1A 00 01 01 17 04` |
| Host → Device | Rejected (bad value `0x09`)     | `15 01 1A 00 01 09 17 04` |

---

## 5. MewtureButton application profile

### 5.1 Ping identity payload (12 bytes)

| Bytes | Field         | Firmware default |
|-------|---------------|------------------|
| 0–3   | Vendor ID     | `00 00 00 01`    |
| 4–7   | Product ID    | `00 00 00 01`    |
| 8–11  | Serial number | `00 00 00 01`    |

The firmware accepts a ping reply only if its length byte is `0x0C` (12). It does not compare the echoed bytes.

### 5.2 Variables

| Var    | Name       | Owner | Values |
|--------|------------|-------|--------|
| `0x00` | Mute state | Host  | `0x00` = unmuted, `0x01` = muted, `0x02` = toggle (write only) |

The host owns the mute state; it is the PulseAudio source's mute flag. The device keeps only a copy, used to drive the LED.

### 5.3 Who sends what

| Sender | Command        | Var    | Handled by receiver as |
|--------|----------------|--------|------------------------|
| Device | Ping           | `0x00` | Host echoes it with an ACK |
| Device | Read           | `0x00` | Host replies with an ACK carrying the current mute state |
| Device | Write `0x02`   | `0x00` | Host toggles mute and replies with an ACK (or a NAK for an unknown value) |
| Host   | Write `0x00/01`| `0x00` | Device updates its LED. **No response is sent.** |

The host never sends pings or read requests. The device never sends a response to a host write.

---

## 6. Sequences

### 6.1 Discovery (`mewture_setup`)

```
setup tool                                   device (any USB serial port)
    |  open port @115200, 6 s read timeout          |
    |                    <---- 07 01 07 ... (ping)  |   device pings every 2 s at startup
    |  parse; command == Ping  => port is a button  |
```

The setup tool tries every USB serial port. It makes up to 5 read attempts per port. A port counts as a button if the tool parses any frame from it whose command is Ping. The setup tool does not reply.

### 6.2 Connect and sync

```
device                                         host daemon
  |  ---- PING (identity) ----------------------->  |
  |  <--- ACK PING (identity echoed) -------------  |
  |  ---- READ var 0x00 -------------------------->  |
  |  <--- ACK READ var 0x00 [mute] ---------------  |
  |  LED: green (0x00) / red (0x01)                  |
```

### 6.3 Button press

```
device                                         host daemon
  |  ---- WRITE var 0x00 [0x02 toggle] ---------->  |  set PulseAudio mute = !mute
  |  <--- ACK WRITE var 0x00 [0x02] --------------  |
  |                                                 |  next poll sees the state change
  |  <--- WRITE var 0x00 [new state] -------------  |
  |  LED updated (no reply sent)                    |
```

The device changes its LED only when the host's follow-up write arrives. The ACK alone does not change it. The device therefore always shows the host's real state.

### 6.4 State changed on the host (e.g. muted in the desktop UI)

```
device                                         host daemon
  |  <--- WRITE var 0x00 [new state] -------------  |  poll detected the change
  |  LED updated (no reply sent)                    |
```

---

## 7. Timing and liveness (firmware)

The firmware records the time of the last frame it handled without error. This is its "last good communication" time. Frames received from the host count; ACKs to the button's toggle writes usually don't (see section 8). Its ping schedule depends on how long ago that was:

| Time since last good comms | Ping interval | LED |
|----------------------------|---------------|-----|
| Startup (never connected)  | 2 s           | Blue, breathing |
| < 30 s                     | No pings      | Green / red |
| 30 s – 60 s                | 10 s          | Green / red |
| ≥ 60 s                     | 2 s           | Blue, breathing (shows the link is lost) |

When connected, the device pings about every 30 seconds. Each successful ping/read exchange also refreshes the LED state.

Other constants:

| Constant | Value | Where |
|----------|-------|-------|
| Button debounce | 50 ms re-read | Firmware |
| Receive buffer | 39 bytes (7 + 32) | Firmware |
| Serial read timeout | 300 ms | Host daemon |
| Port open retry | every 10 s | Host daemon |
| Host read buffer | 64 bytes | Host daemon |

The protocol has no retransmission. A lost frame is recovered by the next keep-alive cycle (the ping and then the read), not by resending the frame.

---

## 8. Implementation notes and known limitations

These notes cover behaviour that matters if you want to write a compatible implementation, and bugs in older versions.

**There is no escaping (section 2.4).** All current implementations use the length byte to find the end of a frame, so data containing `17 04` is safe. A receiver that is resynchronising can still lock onto a start pair inside a frame's data. It then recovers when the stop bytes aren't where that false frame's length byte says they should be.

**The host handles requests only.** The daemon processes frames whose type is Request and ignores ACKs and NAKs.

**The firmware ignores NAKs.** A failure response is accepted and nothing more happens.

### Fixed in newer versions

| Issue | Affected | Fixed in |
|-------|----------|----------|
| `parse_protocol_message` didn't accept `15 01` as a start pair, so it could never return a NAK frame. | `ddaa_protocol` 0.2.0 | `ddaa_protocol` 0.2.1 |
| The crate README gave the end bytes at "6 + N + 1" and "6 + N + 2". In 1-based numbering they are at 6+N and 7+N. | `ddaa_protocol` 0.2.0 README | `ddaa_protocol` 0.2.1 |
| The firmware found the end of a frame by scanning for `17 04`, so data containing `17 04` cut the frame short. If a frame overran the 39-byte buffer, the firmware kept overwriting its last two bytes. | Firmware `main` | Firmware branch `fix/ddaa-framing-and-dispatch` |
| The firmware treated the next frame after a ping or read as the response to it. A host mute write that arrived in that window was discarded. The ACK to a button toggle was treated as an error and didn't refresh the liveness timer. | Firmware `main` | Firmware branch `fix/ddaa-framing-and-dispatch` |
| The daemon parsed at most one frame per `read()` and then cleared its buffer. A frame split across two reads was lost, and if two frames arrived in one read, the second was dropped. | Host `main` | Host branch `fix/ddaa-frame-reassembly` |
