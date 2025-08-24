# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

This is a Rust implementation of "Ray Tracing in One Weekend" by Peter Shirley. The codebase implements a basic ray tracer with spheres, cameras, and progressive rendering features using modern Rust practices.

## Build and Development Commands

```bash
# Build and run the ray tracer (outputs PPM to stdout)
cargo run --release > image.ppm

# Build only
cargo build --release

# Run in debug mode
cargo run

# Check code
cargo check
```

## Architecture

The ray tracer is organized around a trait-based architecture:

### Core Components

- **Vec3/Point3** (`src/vec3.rs`): 3D vector math with operator overloading
- **Ray** (`src/ray.rs`): Ray representation with origin and direction
- **Hittable trait** (`src/hittable.rs`): Core abstraction for ray-object intersection
  - `HitRecord`: Contains intersection point, normal, t-value, and face orientation
  - `Hittable::hit()`: Returns intersection data within t_min/t_max range
- **Sphere** (`src/sphere.rs`): Implements `Hittable` for sphere primitives
- **HittableList** (`src/hittable_list.rs`): Scene container holding multiple hittable objects
- **Color** (`src/color.rs`): Color utilities and PPM output functions

### Module System

The project uses `src/lib.rs` to declare all modules and `src/prelude.rs` to re-export commonly used types. The prelude pattern allows importing everything with `use rtow_rs::prelude::*`.

### Current Implementation State

The ray tracer currently renders a single red sphere against a blue-to-white gradient background. Key features:
- Basic ray-sphere intersection
- Surface normal calculation with proper face orientation
- PPM image output format
- Progress bar using `indicatif` crate

### Rendering Pipeline

1. Camera setup with viewport calculations
2. Per-pixel ray generation
3. Ray-scene intersection testing
4. Color calculation based on hit results
5. PPM format output to stdout

## Development Notes

- Uses Rust 2024 edition
- The codebase follows the "Ray Tracing in One Weekend" tutorial structure
- Progress tracking with `indicatif` for long renders
- Current sphere intersection has a typo: `length_sqaured()` should be `length_squared()`
- The main rendering loop is in `src/main.rs` with inline `hit_sphere()` and `ray_color()` functions