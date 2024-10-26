# Pink Parquet

**Pink Parquet** is a free and open-source, user-friendly viewer
for Parquet files. It allows you to easily open, view, and analyze
Parquet data without any hassle.

### Features

- **User-Friendly Interface**: Navigate your Parquet files with ease.
- **Open Source**: Completely free and open-sourced under the MIT License.
- **Windows Support**: Pre-built binaries available for Windows.
- **Build from Source**: Supports macOS and Linux through source compilation.
- **Lightweight and Fast**: Built with Tauri, Polars (Rust), and Nuxt.js for efficient performance.
- **Drag and Drop**: Simply drag your Parquet files into the app to view them.

### Screenshot

![Screenshot](img/app_screenshot.png)

## Technologies Used

Tauri: For building the desktop application with minimal footprint.
Polars (Rust): A fast DataFrame library for Rust, used for data manipulation.
Nuxt.js: A Vue.js framework for building the frontend interface.

## Installation

### Windows

1. Download the latest Windows installer from the [project's homepage](https://pinkparquet.com).
2. Run the installer and follow the on-screen instructions.

## macOS and Linux

Pre-built binaries are not available for macOS and Linux at this time. However,
you can build the application from source by following the instructions below.

## Building from Source

### Prerequisites

- [Rust](https://www.rust-lang.org/): Install via `rustup.rs`.
- [Node.js](https://nodejs.org/en): Install from nodejs.org.
- [Tauri](https://tauri.app/) Requirements: Follow the Tauri prerequisites for your operating system.

### Steps

Clone the repository:

```bash
git clone https://github.com/marepilc/pink-parquet.git
```

Navigate to the project directory:

```bash
cd pink-parquet
```

Install the dependencies:

```bash
npm install
```

Build the application:

- For development:

```bash
cargo tauri dev
```

- For production:

```bash
cargo tauri build
```

he built application will be located in the src-tauri/target directory.

## Contributing

Contributions are welcome! If you have any ideas, suggestions, or bug reports,
please open an issue or submit a pull request.

