# Adding the Application Icon (Windows)

The application icon is embedded in the window at runtime, but to make it appear in the Windows taskbar and File Explorer, you need to manually embed it into the `.exe` file after building.

## Prerequisites

You'll need `rcedit`, a tool for editing Windows executable resources.

### Option 1: Download rcedit (Recommended)
1. Download from: https://github.com/electron/rcedit/releases
2. Get `rcedit-x64.exe` (latest version)
3. Place it in your PATH or in the project root

### Option 2: Use Wine (Linux/WSL)
If you're on Linux or WSL, you can use Wine to run rcedit:
```bash
wine rcedit-x64.exe
```

## Embedding the Icon

### For WSL Users (Building on Linux, Running rcedit on Windows)

If you're building the executable on WSL/Linux but want to embed the icon from Windows:

1. **Prepare the icon**: Convert `assets/icon.png` to `.ico` format using an online converter (e.g., https://convertio.co/png-ico/)
2. **Save the `.ico` file** to `assets/icon.ico` in your WSL project directory
3. **Run rcedit from Windows PowerShell**:

```powershell
.\rcedit-x64.exe "\\wsl.localhost\Ubuntu\home\barney\photo-video-organizer\target\x86_64-pc-windows-gnu\release\photo-video-organizer.exe" --set-icon "\\wsl.localhost\Ubuntu\home\barney\photo-video-organizer\assets\icon.ico"
```

**Note**: Adjust the WSL path (`\\wsl.localhost\Ubuntu\home\barney\photo-video-organizer`) to match your actual WSL username and project location.

### For Native Windows Users

After building your release executable, run:

```bash
# Manually with rcedit on Windows
rcedit photo-video-organizer.exe --set-icon assets/icon.ico

# Or with Wine on Linux
wine rcedit-x64.exe target/x86_64-pc-windows-gnu/release/photo-video-organizer.exe --set-icon assets/icon.ico
```

## Verification

After embedding the icon:
1. The `.exe` file will show the custom icon in Windows Explorer
2. When running the app, the icon will appear in the Windows taskbar
3. Desktop shortcuts will also display the custom icon

## Note for Releases

If you're creating a GitHub release, remember to embed the icon **before** uploading the `.exe` file. Users downloading from the Releases page will get the properly branded executable.
