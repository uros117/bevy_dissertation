# 3D Labyrinth Ball Game - Bevy Implementation

This repository contains the Bevy engine implementation of a 3D labyrinth ball game, developed as part of a comparative study of 3D game development tools.

## About the Project

This is one of three implementations of the same 3D game, created to compare different game development technologies. The game is inspired by classic wooden labyrinth puzzles where players tilt a platform to guide a ball through obstacles to reach the goal.

## Game Features

- **Platform Control**: Rotate the labyrinth using keyboard controls (limited to ±30 degrees)
- **Physics-Based Ball Movement**: Custom physics simulation responding to platform tilt
- **Interactive Camera**: 
  - Perspective camera with mouse controls (right-click and drag)
  - Orthographic top-down camera
  - Camera switching with 'C' key
  - Zoom with mouse wheel
- **Game Elements**:
  - Wooden textured platform with obstacles
  - Multiple holes (traps and goal)
  - Goal hole with golden rim
  - Custom collision detection
- **Visual Features**:
  - Custom skybox implementation with cube mapping
  - Textured materials using custom shaders
  - Point lighting system
  - Win condition with fade-out text

## Technology Stack

- **Engine**: Bevy 0.8
- **Language**: Rust
- **Graphics API**: wgpu (cross-platform graphics)
- **Shader Language**: WGSL (WebGPU Shading Language)
- **Platform Support**: Windows, Linux, macOS, Web (WebAssembly)

## Requirements

- Rust (latest stable version)
- Cargo (comes with Rust)

## How to Run

### Development
```bash
git clone <repository-url>
cd bevy-labyrinth-game
cargo run
```

### Release Build
```bash
cargo build --release
```

## Controls

- **Arrow Keys / WASD**: Tilt the platform
- **Right Mouse Button + Drag**: Rotate perspective camera
- **Mouse Wheel**: Zoom in/out
- **C Key**: Switch between perspective and orthographic cameras

## Architecture

This implementation uses Bevy's Entity Component System (ECS) architecture:

### Plugins
- `SkyboxPlugin`: Custom skybox rendering with cube mapping
- `ArenaPlugin`: Platform rotation and rendering
- `BallPlugin`: Ball physics and movement
- `ObstaclePlugin`: Static obstacle management
- `HolePlugin`: Hole collision detection
- `BallAnimPlugin`: Ball animation systems
- `SplashPlugin`: Win screen management
- `LevelPlugin`: Overall game coordination

### Custom Shaders
- **Skybox Material**: Custom WGSL shaders for skybox rendering that ignore camera translation
- **Standard Materials**: PBR materials for game objects

### ECS Components
- `Arena`: Platform state and rotation data
- `Rotator`: Rotation animation component
- `ReturnAnimation`: Auto-return to neutral position
- Resources for asset management and game state

## Performance Metrics

According to the comparative study:
- **Development Time**: 41 engineer-hours
- **Lines of Code**: 1,094
- **Executable Size**: 21.8 MB (smallest among all implementations)
- **Memory Usage**: 114.0 MB (constant usage)
- **Project Size**: 14.4 MB

## Key Technical Features

- **Memory Safety**: Rust's ownership system prevents common programming errors
- **Modular Design**: Plugin-based architecture for easy extension
- **Custom Graphics**: Direct control over rendering pipeline
- **Cross-Platform**: Supports desktop and web without code changes
- **Performance**: Compiled to native machine code for optimal performance

## Related Repositories

This implementation is part of a larger comparative study. See also:
- [Unity Implementation](https://github.com/uros117/unity_dissertation)
- [JavaFX Implementation](https://github.com/uros117/javafx_dissertation)
- [Main Repository with Thesis](https://github.com/uros117/3d-game-development-comparison)

## Academic Context

This implementation was developed as part of a bachelor's thesis at the University of Belgrade - Faculty of Electrical Engineering, comparing three different approaches to 3D game development.

**Thesis**: "Comparison of Tools for 3D Video Game Development in JavaFX, Bevy, and Unity 3D Technologies"  
**Author**: Uroš Filipović  
**Mentor**: dr Igor Tartalja, v.prof.  
**Year**: 2022

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Survey Results

In user testing with 21 participants, this Bevy implementation received:
- **Graphics Quality**: 8.81/10 (highest rated)
- **Responsiveness**: 8.33/10  
- **Overall Rating**: 8.52/10 (highest rated)

## Development Notes

- Bevy was version 0.8 during development (still in early development)
- Showcases Rust's capabilities for game development
- Demonstrates data-oriented design principles
- Custom shader implementation provides deep graphics control
- Modular plugin system allows for clean code organization
