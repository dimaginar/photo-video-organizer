# Changelog

All notable changes to this project will be documented in this file.

## [0.5.0] - 2025-12-29 (V5: Branding & Public Release)
### Added
- **New Identity**: Application and project renamed to **"Photo & Video Organizer"**.
- **Custom Branding**: Integrated a professional application icon (camera + film strip theme).
- **Safety Features**: Mandatory execution warning checklist modal before processing files.
- **Support Flow**: Dedicated sidebar button and modal with PayPal/iDEAL donation links.
- **License**: Added official MIT License for open-source distribution.
- **Deployment**: Detailed guide and script for embedding the application icon into the Windows executable.

### Changed
- **UI Polishing**: Refined all headers, button labels, and welcome text for consistency.
- **Native Windows Sweep**: Removed all remaining Linux-specific source code and logic for a streamlined Windows executable.
- **Docs Consolidation**: Cleaned up documentation, moving technical specs to `docs/` and establishing a user-centric main `README.md`.
- **Project Structure**: Renamed root directory to `photo-video-organizer` for branding consistency.

## [0.4.0] - 2024-12-28 (V4: Support & Windows Focus)
### Added
- "Support Development" modal with donation links (PayPal/iDEAL) to help fund a Code Signing Certificate.
- Windows-exclusive focus for builds and official support.

### Changed
- Refined build configuration and documentation to target Windows platforms.

## [0.3.0] - 2024-12-28 (V3: Pure Organizer)
### Added
- Detailed post-organization report with per-year file counts.
- Professional documentation structure in `/docs`.
- New architectural mermaid diagrams for handover.

### Changed
- Pivoted app to focus exclusively on organization workflow.
- Standardized all sidebar button sizes for a premium feel.

### Removed
- Non-functional Gallery view (to be reimagined in future versions).
- Standalone Library Report tab (replaced by post-import report).
- Dead code: Trash functionality and thumbnail processing.
- Unused CLI argument parser (app is now GUI-first).

## [0.2.0] - 2024-12-27 (V2: Modern Redesign)
### Added
- Sleek Windows 11-style UI with `egui`.
- Modern sidebar navigation.
- Desktop integration (xdg-open, explorer) via "Open Folder" button.

### Fixed
- Improved EXIF extraction reliability for HEIC files.

## [0.1.0] - 2024-12-26 (V1: MVP)
### Added
- Initial core logic for date-based file sorting.
- Simple GUI for source/target selection.
- Basic duplicate detection via file hashing.
