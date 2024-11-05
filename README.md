# Gen Avatar 😎

![forthebadge](https://forthebadge.com/images/badges/open-source.svg)
![forthebadge](https://img.shields.io/github/languages/top/Engineers-Cradle/gen-avatar?logo=rust&style=for-the-badge)
![GitHub Actions Workflow Status](https://img.shields.io/github/actions/workflow/status/Engineers-Cradle/gen-avatar/build-code.yaml?logo=rust&style=for-the-badge)

Generate a User Avatar with the initials of the name.

## Features 🚀

- Generate Avatar with Initials
- Redis Cache 📦
- Built with Rust 🦀

## Installation 🛠️

```bash
git clone https://github.com/Engineers-Cradle/gen-avatar.git
cd gen-avatar
cargo build --release
```

## Configuration 🛠️

Create a `.env` file in the root directory and add the following configuration.

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
