VoidCLI/
├── Cargo.toml              # Root workspace configuration
├── src/
│   └── main.rs             # Application entry point
│
├── crates/
│   ├── core/               # Core application functionality
│   │   ├── Cargo.toml
│   │   └── src/
│   │       ├── lib.rs
│   │       ├── app.rs      # Main application state
│   │       ├── state.rs    # State management
│   │       ├── events.rs   # Event loop and handling
│   │       └── error.rs    # Error handling
│   │
│   ├── term/               # Terminal emulation
│   │   ├── Cargo.toml
│   │   └── src/
│   │       ├── lib.rs
│   │       ├── pty.rs      # Pseudoterminal implementation
│   │       ├── parser.rs   # ANSI/VT sequence parser
│   │       ├── process.rs  # Shell process management
│   │       └── vt.rs       # VT emulation
│   │
│   ├── ui/                 # UI rendering system
│   │   ├── Cargo.toml
│   │   └── src/
│   │       ├── lib.rs
│   │       ├── renderer.rs # GPU-accelerated renderer
│   │       ├── window.rs   # Window management
│   │       ├── font.rs     # Font handling
│   │       ├── shaders.rs  # WGPU shaders
│   │       └── widgets.rs  # UI components
│   │
│   ├── blocks/             # Block-based UI management
│   │   ├── Cargo.toml
│   │   └── src/
│   │       ├── lib.rs
│   │       ├── block.rs    # Block implementation
│   │       ├── command.rs  # Command handling
│   │       ├── output.rs   # Command output handling
│   │       └── navigation.rs # Block navigation
│   │
│   ├── config/             # Configuration management
│   │   ├── Cargo.toml
│   │   └── src/
│   │       └── lib.rs      # Config structures and loading
│   │
│   ├── themes/             # Theme management
│   │   ├── Cargo.toml
│   │   └── src/
│   │       └── lib.rs      # Theme implementation
│   │
│   └── commands/           # Command tools and suggestions
│       ├── Cargo.toml
│       └── src/
│           ├── lib.rs
│           ├── history.rs  # Command history
│           ├── completion.rs # Tab completion
│           ├── suggestions.rs # Command suggestions
│           └── palette.rs  # Command palette
│
└── .gitignore