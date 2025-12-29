# 📸 Photo & Video Organizer (Rust)

A lightweight, high-performance tool built in Rust to solve "import chaos." This app automatically organizes your photos and videos into a clear folder structure by year and safely isolates duplicates.

## ✨ Key Features

- **Automatic Sorting**: Moves media files into year-based folders (e.g., `/2025/image.jpg`).
- **Smart Duplicate Handling**: Suspected duplicates are moved to a dedicated `/Duplicates` folder rather than being deleted.
- **Powered by Rust**: Fast, memory-safe execution.
- **Open Source**: Full transparency. Review the code to see exactly how your data is handled.

## 🛡️ Transparency & Safety

This project was developed with the assistance of AI coding tools. To build trust, I have made the entire source code public for community audit.

**Why the "Unknown Publisher" warning?** To remove the Windows SmartScreen warning, an app must be signed with a Code Signing Certificate. These certificates are expensive annual subscriptions. As an independent developer, I currently do not have one.

- **Safety First**: The app is designed to move rather than delete. You always remain in control of your original files.
- **Verified Code**: If you are tech-savvy, please feel free to audit the Rust source files.

> [!WARNING]
> Always create a backup of your photos before running the organizer. While the tool is tested, data management always carries a risk. The author is not responsible for any data loss.

## ⚠️ Technical Notes

- **Supported Formats**: Designed for standard media (JPG, PNG, MP4, MOV, etc.).
- **Sidecar Files (.AAE)**: iPhone edit metadata files (.AAE) are currently not moved. These stay in your source folder to prevent data loss but will not be sorted into year-folders.
- **Non-Media Files**: Any other files (PDFs, documents) are ignored and left in the source folder.

## ☕ Support Development

If this tool saved you hours of work, consider supporting its development. Donations help fund a Code Signing Certificate to remove Windows warnings and make this tool more trusted for everyone.

- [Donate with PayPal](https://www.paypal.com/donate/?business=Q4JJUB58QT7SN&no_recurring=1&item_name=Donations+help+me+purchase+a+Code+Signing+Certificate+to+remove+the+Unknown+Publisher+warning+and+build+trust+for+all+users.&currency_code=EUR)
- [Donate with iDEAL](https://betaalverzoek.rabobank.nl/betaalverzoek/?id=MiDjVyNBSN-Qy288Zb0sJg)

## 🚀 Quick Start

1. Download `photo-video-organizer.exe` from the [Releases](../../releases) section.
   - **Recommended Browsers**: Using **Firefox** or **Vivaldi** is preferred, as they typically allow the download without additional warnings.
   - **Note for Microsoft Edge**: You may see a warning that the file is "not commonly downloaded." To proceed, click the **three dots (...)** next to the download, select **Keep**, and then **Show more -> Keep anyway**.
   - **Note for Work Laptops**: On some managed work environments, downloading `.exe` files from GitHub may be disabled by system policy.
2. Select your **Source Folder** (the chaos) and your **Destination Folder**.
3. Review the warning and hit **Proceed**.

## 📚 Documentation

For developers and contributors:
- [Design Document](docs/DESIGN.md) - Architecture and specifications
- [Contributing Guide](docs/CONTRIBUTING.md) - Build instructions and development guidelines
- [Changelog](docs/CHANGELOG.md) - Version history

## 🛡️ Security Note

To ensure your safety, only download the official `.exe` from this GitHub repository. This is the only version maintained and verified by Dimaginar.

## ⚖️ License

This project is available under the [MIT License](LICENSE). This means the software is provided "as is," without warranty of any kind. See the LICENSE file for details.
