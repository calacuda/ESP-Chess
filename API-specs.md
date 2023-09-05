# API-spec

---

This document servers to document the comunication API between esp32's.

## over view

Comunication between the cluster nodes is done by way of I2C and thus there are certain limitation. For starters the periferal nodes can not talk to each other. nore can they message the controller on their own reconisanse. Because of this the API is designed in a request-based fasion.

World generation is done using wave function colapse. The world is split into levels, with each level being generated in parallele on different worker nodes. The worker nodes store the data about the levels they built. Thus the main contoller is ignorant of the world and levels. This was done intentional to save memory on the controller node. The controller only stores which worker holds the data about which level and requests zones individually when needed.

## messages

Messages are structured where the first byte is a bytecode instruction and all other bytes are considered an "argument". The messages are defined under the [request codes](#request codes), and [response codes](#response codes) sections respectively.

## transaction

all data is sent as packets (the size of these packets is determined by the `PACKET_SIZE` variable form the `src/lib.rs` file). the packets should be null byte padded and the data section of the packet should end with a new-line character. all messages are sent in pairs, the first message is one byte long and is a byte-code instruction. the subsequent message is the arguemnt for the instruction.

## request codes

| **Byte Code** | **Meaning**                             | **Arguments**                                                                                                  | **Arg Length** (in bytes) |
| ------------- | --------------------------------------- | -------------------------------------------------------------------------------------------------------------- | ------------------------- |
| 0             | request to generate a zone              | the zone description encoded as json data then converted to bytes (utf-8)                                      | variable                  |
| 1             | request a zone by it's cordinates       | three u8's, the first being the level number, the second and third being the x and y coordinates respectively. | 3                         |
| 2             | request a status update from the worker | u8, should be either 0 indicating not done yet, or anything grater then zero, meaning complete.                | 1                         |
| 3             | ping, used to check for alive workers   | N/A                                                                                                            | 0                         |

## response codes

| **Byte** | **Meaning**                                 | **Arguments**                                                  | **Arg Length** (in bytes) |
| -------- | ------------------------------------------- | -------------------------------------------------------------- | ------------------------- |
| 0        | the reply                                   | the data requested as a json string converted to bytes (utf-8) | variable                  |
| 1        | request completes but does not require data | N/A                                                            | 0                         |

## items

all items are in the players inventory by default; however the count of the item is set to 0 until acquired through chests or other means.

## enemies

enemies are spawned in the world precedurally. meaning that they can spawn in zones/levels the player has already cleared.
