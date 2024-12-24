# Remote Field Game

**Keywords**: `Bevy Engine`, `Rust`, `2D Graphics`, `Network Control`, `HTTP API`, `Socket Communication`, `Color Field`, `Game Rendering`, `POC`

A proof-of-concept implementation of a color field game built with the Bevy game engine, featuring network control capabilities.

## Overview

This project demonstrates a color field game rendering system with the unique ability to be controlled remotely through network APIs. It combines:
- Bevy engine for game rendering
- Parallel HTTP server for network control
- Socket file interface for local control

## Status

⚠️ **Note**: This is an archived proof-of-concept project that remains unfinished and is kept for future reference. The project is no longer in active development.

## Prerequisites

- Rust and Cargo installed on your system
- Bevy engine dependencies (will be handled by Cargo)

## Building and Running

To run the project:

```bash
cargo run -- --appname mesh_2d
```

## Features

- Real-time color field rendering using Bevy
- HTTP API for remote control of game state
- Socket file interface for local control
- Dynamic rendering updates based on network requests

## Architecture

The project consists of two main components:
1. A Bevy-based game engine handling the rendering
2. A parallel HTTP server processing network requests that can modify the game state

## Network API

The game can be controlled remotely through HTTP endpoints (documentation pending).

## Socket Control

Local control is available through a socket file interface (documentation pending).

## Project Status

This project is archived and no longer maintained. It serves as a reference implementation for future projects involving Bevy engine and network-controlled game states.

## License

[License information pending]
