<p align="center">
  <img src="./src-tauri/icons/Square150x150Logo.png" />
  <h1 style="text-align: center;">Galaxy Window Manager</h1>
  <p style="text-align: center;">*Application name and icon are temporary.*</p>
</p>


A powerful, lightweight window management utility for Windows that runs quietly in your system tray while providing instant window positioning and resizing through keyboard shortcuts. Inspired by [Rectangle](https://rectangleapp.com/) for macOS.

## Known Issues
- Some window's bounding boxes are not correctly detected, leading to gaps around certain edges of an application.

### System Requirements
Though is is a Tauri application, it is only available for Windows. There are other options for macOS and linux. This app was created to solve my issue of not having something to move windows around with my keyboard that worked for me.

## Default Keyboard Shortcuts

### Basic Window Positioning
| Action | Shortcut |
|--------|----------|
| Left Half | `Ctrl+Alt+←` |
| Right Half | `Ctrl+Alt+→` |
| Top Half | `Ctrl+Alt+↑` |
| Bottom Half | `Ctrl+Alt+↓` |

### Corner Positioning
| Action | Shortcut |
|--------|----------|
| Top Left | `Ctrl+Alt+U` |
| Top Right | `Ctrl+Alt+I` |
| Bottom Left | `Ctrl+Alt+J` |
| Bottom Right | `Ctrl+Alt+K` |

### Third-Based Positioning
| Action | Shortcut |
|--------|----------|
| First Third | `Ctrl+Alt+O` |
| Center Third | `Ctrl+Alt+P` |
| Last Third | `Ctrl+Alt+[` |
| First Two Thirds | `Ctrl+Alt+L` |
| Last Two Thirds | `Ctrl+Alt+;` |

### Window Actions
| Action | Shortcut |
|--------|----------|
| Maximize | `Ctrl+Alt+Enter` |
| Almost Maximize | `Shift+Ctrl+Alt+Enter` |
| Maximize Height | `Shift+Ctrl+Alt+↑` |
| Center Window | `Ctrl+Alt+C` |
| Make Larger | `Ctrl+Alt+=` |
| Make Smaller | `Ctrl+Alt+-` |
| Restore | `Ctrl+Alt+Backspace` |

### Multi-Monitor
| Action | Shortcut |
|--------|----------|
| Move to Left Monitor | `Shift+Ctrl+Alt+←` |
| Move to Right Monitor | `Shift+Ctrl+Alt+→` |

## Installation

### Download & Run
1. Download the latest release from the [Releases](https://github.com/your-username/galaxy-window-manager/releases) page
2. Run the installer (`Galaxy-Window-Manager-Setup.exe`)
3. The application will start automatically and appear in your system tray

### From Source
```bash
# Clone the repository
git clone https://github.com/your-username/galaxy-window-manager.git
cd galaxy-window-manager

# Install dependencies
pnpm install

# Run in development mode
pnpm tauri dev

# Build for production
pnpm tauri build
```

## Usage

### Getting Started
1. **Launch**: The app starts minimized in your system tray
2. **Access Settings**: Right-click the tray icon and select "Open Settings"
3. **Customize**: Click the keyboard icon in any shortcut field and press your desired key combination
4. **Use**: Press any configured shortcut to instantly position your active window

### Customizing Shortcuts
1. Open the settings window from the system tray
2. Click the keyboard icon in any shortcut field
3. Press your desired key combination
4. The shortcut is saved automatically and takes effect immediately

### System Tray Menu
- **Open Settings**: Show the configuration window
- **Quit**: Exit the application completely

## Technical Details

### Built With
- **Frontend**: Vue.js 3 + Vuetify 3 (Material Design)
- **Backend**: Rust + Tauri 2.0
- **Platform**: Windows (with Windows API integration)

### Architecture
- **Modular Design**: Each window action is implemented as a separate, testable module
- **Global Shortcuts**: Uses Windows API for system-wide keyboard hook registration
- **Multi-Monitor**: Leverages Windows display enumeration for accurate positioning
- **DPI Scaling**: Automatic handling of different display scaling factors

### System Requirements
- **Operating System**: Windows 10 or later
- **Memory**: ~10MB RAM usage
- **CPU**: Minimal impact, event-driven architecture
- **Permissions**: No administrator privileges required

## Configuration

### Settings File
Shortcuts are stored in `src/shortcuts.json` and can be manually edited:

```json
{
  "shortcuts": {
    "leftHalf": {
      "name": "Snap to Left Half",
      "defaultShortcut": "Control+Alt+ArrowLeft"
    }
  }
}
```

### Key Format
- **Modifiers**: `Control`, `Alt`, `Shift`
- **Keys**: `ArrowLeft`, `ArrowRight`, `Enter`, `A-Z`, `0-9`, etc.
- **Format**: `Modifier+Modifier+Key` (e.g., `Control+Alt+ArrowLeft`)

## Troubleshooting

### Common Issues

**Shortcuts not working**
- Check if another application is using the same shortcut
- Restart the application from the system tray
- Verify shortcuts in the settings window

**Window not positioning correctly**
- Ensure the target monitor is properly detected
- Check Windows display scaling settings
- Try different window positioning modes

**Application not starting**
- Check Windows Event Viewer for error messages
- Verify Windows version compatibility

## Contributing

I welcome contributions! Please open a discussion to dicuss changes you would like to make.

### Development Setup
1. Install [Rust](https://rustup.rs/) and [Node.js](https://nodejs.org/)
2. Clone the repository
3. Run `npm install` to install dependencies
4. Use `npm run tauri dev` for development
5. Use `npm run tauri build` for production builds

## License

This project is licensed under the GPL v2 License - see the [LICENSE](LICENSE) file for details.

## Acknowledgments

- Built with [Tauri](https://tauri.app/) for native performance
- Icons and UI design with [Vuetify](https://vuetifyjs.com/)
- Inspired by window managers like Rectangle (macOS)

---

**Galaxy Window Manager** - Organize your windows with stellar precision ⭐
