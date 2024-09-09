## Protocol V1
This protocol is lopsided as the server and client have different responsibilities

### Client
opcode (1 byte) | data (rest)

opcode
- [0] Error (message)
- [1] Connect (usize, mine_count)
- [2] Reveal (index)

### Server
opcode (1 byte) | data (rest)

opcode
- [0] Error (message)
- [1] Connection Accepted
- [2] Reveal Cells ([index, value])
- [3] GameWin (time, [index, value])
- [4] GameLoss (time, [index])

## Protocol V2
This is the updated protocol with more checks and more efficient data flow

### Client
opcode
- [0] Error
    size: (u16)
    name: (error_code)

- [1] Connect
    size: (u8, u8, u16)
    name: (width, height, mine_count)
    If width or height exceed 100 then throws an error.
    If mine_count exceeds 100*100 - 1 then throws an error.

- [2] NewGame
    size: (u8, u8, u16)
    name: (width, height, mine_count)
    If width or height exceed 100 then throws an error.
    If mine_count exceeds 100*100 - 1 then throws an error.

- [3] Reveal
    size: (u16)
    name: (index)

- [4] GetTime
    size: ()
    name: ()

### Server
opcode
- [0] Error
    size: (u16)
    name: (error_code)

- [1] Connection Accepted
    size: ()
    name: ()

- [2] Reveal Cells 
    size: ([u8; u16])
    name: ([val; width*height])

- [3] GameWin 
    size: ([u8; u16])
    name: ([val; width*height])

- [4] GameLoss
    size: (Vec<u16>)
    name: (Vec<index>)

- [5] Time
    size: (String)
    name: (time)
