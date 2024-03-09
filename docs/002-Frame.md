# Frame

Each packet contains many Frame. Frame is only have a header. Each packet only have one payload and multi frame header.

## Frame Header

Frame Header have common structure. These structure only have one byte.

- version: 3bit
- type: 5bit

Currently, version is 0. Type used to specify the frame type.

## Frame Type

### Connection Frame

### Transmission Frame

### Security Frame

### Application Frame

### Proxy Frame
