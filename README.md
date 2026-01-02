# Pink Parquet

**Pink Parquet** is a lightweight, high-performance Parquet file viewer and explorer. It provides a clean, modern
interface for analyzing Parquet data using powerful SQL queries and seamless file management.

### Key Features

- **Blazing Fast**: Powered by [Polars](https://pola.rs/), the fastest DataFrame library, for near-instant data loading
  and processing.
- **SQL Explorer**: Full SQL support for querying your data. Filter, aggregate, and join multiple Parquet files
  effortlessly.
- **Multi-File Sessions**: Open multiple Parquet files in tabs. Each file is automatically registered as a table in the
  SQL engine.
- **Smart Autocomplete**: SQL editor with context-aware completion for SQL keywords, table names, and column names.
- **In-Depth Metadata**: View detailed file statistics including row counts, schema details, compression types, and row
  group information.
- **Modern Desktop Experience**: Built with Tauri for a native feel, small footprint, and high performance on Windows
  and macOS.
- **Drag & Drop**: Quickly open files by dropping them anywhere in the application.

### Tech Stack

- **Frontend**: [Svelte 5](https://svelte.dev/) for a reactive and
  beautiful UI.
- **Backend**: [Rust](https://www.rust-lang.org/) with [Tauri](https://tauri.app/) for secure and efficient desktop
  integration.
- **Data Engine**: [Polars](https://github.com/pola-rs/polars) for high-performance data manipulation and SQL execution.

### Screenshot

![Screenshot](img/app_screenshot.png)

## Installation

The easiest way to install **Pink Parquet** is to download the pre-built binaries for your operating system
from [www.pinkparquet.com](https://www.pinkparquet.com) or
the [Releases](https://github.com/marepilc/pink-parquet/releases) page.

### Windows

1. Download the `.msi` installer.
2. Run the installer and follow the instructions.

### macOS

1. Download the `.dmg` file.
2. Open the `.dmg` file and drag **Pink Parquet** to your **Applications** folder.

### Linux

Currently, Linux users are encouraged to [build from source](#building-from-source). Pre-built packages for Linux are
coming soon.

## Building from Source

### Prerequisites

- **Rust**: [Install via rustup](https://rustup.rs/)
- **Node.js**: [LTS version recommended](https://nodejs.org/)
- **Tauri Dependencies**: Follow the [Tauri setup guide](https://tauri.app/v1/guides/getting-started/prerequisites) for
  your OS.

### Build Steps

1. Clone the repository:
   ```bash
   git clone https://github.com/marepilc/pink-parquet.git
   cd pink-parquet
   ```

2. Install dependencies:
   ```bash
   npm install
   ```

3. Run in development mode:
   ```bash
   npm run tauri dev
   ```

4. Build for production:
   ```bash
   npm run tauri build
   ```

## Contributing

Contributions are what make the open-source community such an amazing place to learn, inspire, and create. Any
contributions you make are **greatly appreciated**.

1. Fork the Project
2. Create your Feature Branch (`git checkout -b feature/AmazingFeature`)
3. Commit your Changes (`git commit -m 'Add some AmazingFeature'`)
4. Push to the Branch (`git push origin feature/AmazingFeature`)
5. Open a Pull Request

## License

Distributed under the MIT License. See `LICENSE` for more information.

