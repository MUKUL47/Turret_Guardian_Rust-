# A Simple turret game

## Overview

This project is a Rust-based game with a modular architecture, divided into several key components. Below is an overview of the file structure and the purpose of each module.


<img src="https://github.com/MUKUL47/Turret_Guardian_Rust-/blob/main/turret-demo.gif" />

## Modules Description

### core

- **game.rs**: The main game logic and structure are implemented here.
- **mod.rs**: Module declaration for the core module, facilitating the inclusion of core components.

### entities

- **enemies.rs**: Contains the definitions and behaviors of enemies in the game.
- **gun_projectile.rs**: Manages the properties and actions of gun projectiles.
- **mod.rs**: Module declaration for the entities module, including all entity-related files.
- **player.rs**: Defines the player character, including attributes and actions.
- **score_board.rs**: Handles the scorekeeping and display logic.

### state

- **enemy.rs**: Manages the state and data related to enemies.
- **gun_projectile.rs**: Manages the state and data related to gun projectiles.
- **mod.rs**: Module declaration for the state module, encompassing all state management files.
- **player.rs**: Manages the state and data related to the player.

### utils

- Placeholder for utility functions and helpers that can be used across various module

