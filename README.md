## Protocol
This protocol is lopsided as the server and client have different responsibilities

### Client
opcode (1 byte) | data (rest)

opcode
- [0] Error (message)
- [1] Connect (size, mine_count)
- [2] Reveal (index)

### Server
opcode (1 byte) | data (rest)

opcode
- [0] Error (message)
- [1] Connection Accepted
- [2] Reveal Cells ([index, value])
- [3] GameEnd (win/loss, time, [index])
