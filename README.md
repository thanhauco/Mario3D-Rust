# Mario 3D - Rust Edition 🍄

A modern 3D Mario-style platformer game built with Rust and cutting-edge game development technologies.

## 🚀 Tech Stack (2025)

- **Bevy 0.14** - Modern ECS game engine with HDR rendering
- **Rapier3D** - High-performance 3D physics engine
- **WGPU** - Modern graphics API (via Bevy)
- **PBR Materials** - Physically-based rendering for realistic graphics

## ✨ Features

- 🎮 Smooth 3D platforming mechanics
- 🏃 Character movement with WASD/Arrow keys
- 🦘 Jump mechanics with Space bar
- 🪙 Collectible coins with score system
- 📦 Question blocks and obstacles
- 🎥 Dynamic camera follow system
- 🌟 Modern graphics with HDR and PBR materials
- ⚡ Physics-based gameplay using Rapier3D

## 🎯 Controls

- **WASD** or **Arrow Keys** - Move Mario
- **Space** - Jump
- **ESC** - Quit game

## 🛠️ Building & Running

### Prerequisites

- Rust 1.75+ (install from [rustup.rs](https://rustup.rs))
- A GPU that supports Vulkan, Metal, or DirectX 12

### Quick Start

```bash
# Clone the repository
git clone https://github.com/thanhauco/Mario3D-Rust.git
cd Mario3D-Rust

# Run in development mode
cargo run

# Build optimized release version
cargo build --release
./target/release/mario3d-rust
```

## 🎨 Game Elements

- **Player** - Red capsule character with blue cap (Mario)
- **Platforms** - Golden/brick colored platforms to jump on
- **Coins** - Rotating golden coins to collect
- **Question Blocks** - Yellow glowing blocks
- **Pipes** - Green cylindrical obstacles
- **Ground** - Grass-colored terrain

## 🔧 Development

The project is structured into modular plugins:

- `player.rs` - Player movement, jumping, and physics
- `camera.rs` - Smooth camera follow system
- `level.rs` - Level generation, platforms, and obstacles
- `collectibles.rs` - Coins and collectible items
- `ui.rs` - HUD displaying score, coins, and lives

## 📝 Future Enhancements

- [ ] 3D character models (GLTF/GLB support)
- [ ] Enemies and AI
- [ ] Multiple levels
- [ ] Power-ups (mushrooms, fire flowers)
- [ ] Sound effects and music
- [ ] Particle effects
- [ ] More complex level designs
- [ ] Save/load system

## 🤝 Contributing

Contributions are welcome! Feel free to open issues or submit pull requests.

## 📄 License

MIT License - feel free to use this project for learning or building your own games!

## 🙏 Acknowledgments

Built with ❤️ using Rust and the amazing Bevy game engine ecosystem.
