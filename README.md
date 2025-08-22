# 🌌 RayTracingInOneWeekend-rs

> Rust implementation of *"Ray Tracing in One Weekend"*  
> Inspired by Peter Shirley's classic ray tracing series, rebuilt with modern Rust practices.

---

## ✨ Features

- 📐 **Vec3 / Ray / Camera** implemented from scratch
- 🌑 **Sphere & Hittable trait system**
- 🎨 **Diffuse, metal, dielectric materials**
- 🔦 **Antialiasing, gamma correction**
- 🖼️ **PPM & PNG image output**
- ⚡ (Planned) Parallel rendering with [`rayon`](https://crates.io/crates/rayon)

---

## 📦 Getting Started

### 1. Clone the repo
```bash
git clone https://github.com/Penguin-SAMA/rtiow-rs.git
cd rtiow-rs
```

### 2. Build & Run

```bash
cargo run --release > image.ppm
```

This will generate an image file.

------

## 🖼️ Example Output

![image.ppm](https://raytracing.github.io/images/img-1.01-first-ppm-image.png)

------

## 📂 Project Structure

```
src/
  ├── vec3.rs         # Vector math
  ├── ray.rs          # Ray struct & helpers
  ├── camera.rs       # Camera implementation
  ├── hittable.rs     # Trait + hit record
  ├── sphere.rs       # Sphere primitive
  ├── color.rs        # Color utils & write function
  ├── main.rs         # Entry point
```

------

## 🚀 Roadmap

-  Basic Vec3 and Ray implementation
-  Sphere intersection
-  Antialiasing and gamma correction
-  Parallel rendering (rayon)
-  Materials (Lambertian, Metal, Dielectric)
-  Camera depth of field
-  BVH acceleration

------

## 🔧 Requirements

- Rust **1.85+** (2024 edition recommended)
- Arch Linux / macOS / Windows (tested on Arch Linux with Hyprland)

------

## 🤝 Contributing

Contributions are welcome! Feel free to open an **issue** or submit a **pull request**.

------

## 📚 References

- 📘 [*Ray Tracing in One Weekend*](https://raytracing.github.io/books/RayTracingInOneWeekend.html) by Peter Shirley
- 🦀 [The Rust Programming Language](https://doc.rust-lang.org/book/)
- 🛠️ [Rustlings](https://github.com/rust-lang/rustlings)

------

## 📜 License

MIT License © 2025 Penguin-SAMA
