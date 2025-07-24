# Spriso

Converts from screen coords to a 3d grid world and vice versa for an isometric projection system.

## Functionality

```rust
use glam::{ IVec3, Vec3 };
use spriso::IsometricProjection;

let proj: IsometricProjection = IsometricProjection::new::<14, 14>();

let pos: IVec3 = IVec3::new(10, 20, 30);
let screen_pos: Vec3 = proj.world_to_screen(pos);

assert!(screen_pos.x != 0.0);
assert!(screen_pos.y != 0.0);
assert!(screen_pos.z != 0.0);

let world_pos: IVec3 = proj.screen_to_world(screen_pos);

assert_eq!(pos, world_pos);
```

## Getting Started

```toml
[dependencies]
spriso = { git = "https://github.com/penguinawesome1/spriso.git", tag = "v0.1.0" }
glam = "0.30.4"
```

## License

This project is licensed under the [MIT License](LICENSE) - see the `LICENSE` file for details.
