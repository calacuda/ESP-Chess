# ESP-rogue-xork

A procedurally-generated text-based adventure game run for esp32 cluster utilizing distributed computing.

## TLDR

- The goal: Have multiple esp32s pool processor reasorces to serve a procedurally generated text based adventure game.
- The How: The computer and the esp32 control-node will comunicate over either wireless or serial. The control-node will orchestrate multiple esp32's which comunicate with each other (likely over I2C) to generate the world.
- The Why: It's a fun way to learn about distributed computing in an embedded context. (and also, why not.)

## Note

This project is in it's infancy, EVERYTHING IS SUBJECT TO CHANGE. I'm still waiting to order more eps32's to properly test project properly.

## How it Works

every time a new game is started the the esp32 configured as the ambassador will generate a new world of the configured dimensions. The game world is multi-level and generated using wave-function collapse in parallel on cluster. The entry to a tower is hidden on a randomly chosen level.
