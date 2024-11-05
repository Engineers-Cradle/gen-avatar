# Gen Avatar 😎

Generate a User Avatar with the initials of the name.

## Features 🚀

- Generate Avatar with Initials
- Redis Cache 📦
- Built with Rust 🦀

## Installation 🛠️

```bash
git clone https://github.com/EngineersCraddle/gen-avatar.git
cd gen-avatar
cargo build --release
```

## Configuration 🛠️

```bash
WEB_SERVER_PORT=
REDIS_URL=
NUM_WORKERS=
LOG_LEVEL=
```

## Usage 🚀

```bash
cargo run --release
```

## API Endpoints 🚀

- `GET /avatar/:name?theme=light` - Generate Avatar with Initials

## License 📄

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details
