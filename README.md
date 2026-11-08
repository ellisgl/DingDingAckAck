# Ding Ding Ack Ack

`ding_ding_ack_ack` is a crate for parsing and formatting Ding Ding Ack Ack protocol messages.

Here's the Ding Ding Ack Ack protocol specification:

```plaintext
Request Frame (8  -  39 bytes):
 Byte 1: Start byte 1 (0x07)
 Byte 2: Start byte 2 (0x01)
 Byte 3: Command (0x05 for read, 0x1A for write, 0x07 for ping)
 Byte 4: Variable (user-defined)
 Byte 5: Data Length in N Bytes (1-32)   (ignored in read operations)
 Byte 6 - Byte (5 + N): Data (User defined) (ignored in read operations)
 Byte 6 + N: End byte 1 (0x17)
 Byte 7 + N: End byte 2 (0x04)

Response Frame for Successful Calls (8  -  39 bytes):
 Byte 1: Start byte 1 (0x06)
 Byte 2: Start byte 2 (0x01)
 Byte 3: Command that was sent
 Byte 4: Variable that was sent
 Byte 5: Data Length in N Bytes (1 - 32)
 Byte 6 - Byte (5 + N): Message (User defined)
 Byte 6 + N: End byte 1 (0x17)
 Byte 7 + N: End byte 2 (0x04)

Response Frame for Failed Calls (8 -  39 bytes):
 Byte 1: Start byte 1 (0x15)
 Byte 2: Start byte 2 (0x01)
 Byte 3: Command that was sent
 Byte 4: Variable that was sent
 Byte 5: Data Length in N Bytes (1-32)
 Byte 6 - Byte (5 + N): Error Message (User defined)
 Byte 6 + N: End byte 1 (0x17)
 Byte 7 + N: End byte 2 (0x04)

Ping:
 Request Frame (8 - 39 bytes):
  Byte 1: 0x07
  Byte 2: 0x01
  Byte 3: 0x07
  Byte 4: 0x00
  Byte 5: Data Length in N Bytes (1 - 32) --- can be used for discovery / identification
  Byte 6 - Byte (5 + N): Message (User defined)
  Byte 6 + N: End byte 1 (0x17)
  Byte 7 + N: End byte 2 (0x04)

 Response Frame(8 - 39 bytes):
  Byte 1: 0x06
  Byte 2: 0x01
  Byte 3: 0x07
  Byte 4: 0x00
  Byte 5: Data Length in N Bytes (1 - 32)
  Byte 6 - Byte (5 + N): Message (User defined)
  Byte 6 + N: End byte 1 (0x17)
  Byte 7 + N: End byte 2 (0x04)
```
