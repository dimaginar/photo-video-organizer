# Contributing Guide

Welcome! This guide is for human engineers and AI coding agents looking to maintain or enhance the Photo & Video Organizer.

## 1. Development Stakeholders

### 1.1 For Engineers
- **Environment**: Windows 10/11 or Linux (for cross-compilation).
- **Target**: The application target is `x86_64-pc-windows-gnu`.
- **Tooling**: `rust-analyzer` is highly recommended.

### 1.2 For AI Coding Agents
- **Context**: The `docs/DESIGN.md` file is the source of truth for requirements and system design.
- **Workflow**: Always verify `cargo check` after making changes to the `AppTab` or `ProcessState` enums, as these trigger many UI branch matches.
- **Style**: Prefer vanilla CSS-like egui styling helpers rather than custom widget painting where possible.

## 2. Setting Up & Building

1. **Clone the repository**.
2. **Install dependencies**: `cargo build` will handle all Rust crates.
3. **Build for Release (Windows)**:
   ```bash
   cargo build --release --target x86_64-pc-windows-gnu
   ```
4. **Development run**: `cargo run` launches the debug build.

## 3. Technical Stack

- **Language**: [Rust](https://www.rust-lang.org/)
- **GUI Library**: [egui](https://github.com/emilk/egui)
- **Image Processing**: [image](https://github.com/image-rs/image) crate.
- **Metadata**: [kamadak-exif](https://github.com/kamadak/exif-rs) for EXIF extraction.

## 4. Project Structure Reference

- `src/core/`: Business logic. NO UI dependencies.
- `src/ui/`: egui implementation. Coordinates core logic via background threads.
- `docs/`: Design and requirement documentation.

## 4. Coding Standards

- **Error Handling**: Use `anyhow` for application-level errors and `std::io::Result` for low-level file operations.
- **Safety**: Do not use `unsafe` blocks.
- **File Moves**: Always use `fs::rename` with a `fs::copy` + `fs::remove_file` fallback to handle cross-device move operations.

## 5. Release Process

For creating releases with the application icon properly embedded, see [Icon Embedding Guide](ICON_EMBEDDING.md).
