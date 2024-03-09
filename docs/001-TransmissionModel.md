# Transmission Model and Packet

## Model of Transmission

We defined 4 type model.

1. `Non`: Non-Confirmation Send
2. `Con`: Confirmation Send
3. `Re2`: Request-Response
4. `Stm`: Stream

### Non-Confirmation Send

Simply send packet to remote, without confirmation.

Typical protocol:

- IP
- UDP

### Confirmation Send

Send packet to remote with confirmation.

### Request-Response

Send request to remote, then wait response from remote. Finally, remote will receive confirmation.

Typical protocol:

- HTTP

### Stream

Open a Simplex / Duplex Stream, it can send data in order.

## Model Convert

Two directions to convert transmission model. Combination and Compatibility.

Low level protocol can combine to high level. And high level protocol can compact low level protocol.

High level

## Packet

This protocol built on any `Non` low-level protocol.

Each packet have 1450 bytes.
