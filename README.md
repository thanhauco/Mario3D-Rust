# Mario 3D - Rust Edition 🍄

A modern 3D Mario-style platformer game built with Rust and cutting-edge game development technologies.

## 🚀 Tech Stack (2025)

- **Bevy 0.14** - Modern ECS game engine with HDR rendering
- **Rapier3D** - High-performance 3D physics engine
- **WGPU** - Modern graphics API (via Bevy)
- **PBR Materials** - Physically-based rendering for realistic graphics

## ✨ Features

### Core Gameplay
- 🎮 Smooth 3D platforming mechanics with realistic physics
- 🏃 Character movement with WASD/Arrow keys
- 🦘 Jump mechanics with Space bar
- 🪙 Collectible coins with particle burst effects
- 💯 Real-time score and coin tracking

### Enemies & Combat
- 👾 Patrolling enemies with AI behavior
- 🦘 Jump stomp mechanic - defeat enemies by jumping on them
- 💥 Enemy death animations with particle effects
- ❤️ Lives system with damage from enemy collisions

### Interactive Objects
- ❓ Question blocks that spawn coins when hit from below (3 coins per block)
- 🎆 Block bounce animations
- 🍄 Power-ups: Mushrooms and Fire Flowers
- 🌟 Glowing emissive materials for collectibles

### Visual Effects
- 🎨 Modern graphics with HDR and PBR materials
- ✨ Particle effects for coin collection and enemy defeats
- 🎥 Smooth camera follow system with lerp
- 💡 Dynamic lighting with directional and ambient lights
- 🌈 Emissive materials for glowing objects

### Level Design
- 🏗️ Multiple platforms at varying heights
- 🟢 Green pipes as obstacles
- 🌱 Grass-textured ground plane
- 📦 Strategic placement of collectibles and enemies

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
- **Enemies** - Red patrolling enemies with glowing eyes
- **Platforms** - Golden/brick colored platforms to jump on
- **Coins** - Rotating golden coins with emissive glow (30+ scattered in level)
- **Question Blocks** - Yellow glowing blocks that spawn coins (10 blocks, 3 coins each)
- **Power-ups** - Mushrooms (red with white spots) and Fire Flowers (orange)
- **Pipes** - Green cylindrical obstacles
- **Ground** - Grass-colored terrain

## 🔧 Development

The project is structured into modular plugins:

- `main.rs` - Game initialization and state management
- `player.rs` - Player movement, jumping, and physics
- `camera.rs` - Smooth camera follow system with lerp
- `level.rs` - Level generation, platforms, obstacles, and question block interactions
- `collectibles.rs` - Coins with particle effects on collection
- `enemies.rs` - Enemy AI, patrol behavior, and jump stomp mechanics
- `powerups.rs` - Power-up spawning and collection system
- `ui.rs` - HUD displaying score, coins, and lives

## 📝 Future Enhancements

- [ ] 3D character models (GLTF/GLB support)
- [ ] Sound effects and music
- [ ] Multiple levels with progression
- [ ] Fire flower shooting mechanic
- [ ] Star power-up invincibility mode
- [ ] More enemy types (flying, jumping)
- [ ] Boss battles
- [ ] Checkpoints and save system
- [ ] Multiplayer support
- [ ] Level editor

## 🤝 Contributing

Contributions are welcome! Feel free to open issues or submit pull requests.

## 📄 License

MIT License - feel free to use this project for learning or building your own games!

## 🙏 Acknowledgments

Built with ❤️ using Rust and the amazing Bevy game engine ecosystem.
