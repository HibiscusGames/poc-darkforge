# DarkForge POC

A command-line proof of concept for testing [Blades in the Dark](https://bladesinthedark.com/) mechanics in a video game context.

## Overview

This is a lightweight CLI tool that simulates Blades in the Dark's core gameplay loops, focusing on:

- Character creation and management
- Action resolution with position/effect
- Consequences and resistance rolls
- Basic obstacle generation

The goal is to validate design assumptions and explore how the tabletop mechanics translate to a digital format.

## Development

### Prerequisites

- [Rust](https://www.rust-lang.org/) (latest stable version)
- [mise](https://mise.jdx.dev/) (for development environment setup)

### Setup

```bash
# Initialize development environment
mise bootstrap
```

### Development Workflow

Use bacon to rerun the tests and lints as you write code.

```bash
bacon
```

```bash
# Run tests
cargo test

# Format code
cargo fmt
```

## License

Copyright (C) 2025 Pierre Fouilloux

This program is free software: you can redistribute it and/or modify
it under the terms of the GNU Affero General Public License as published by
the Free Software Foundation, either version 3 of the License, or
(at your option) any later version.

This program is distributed in the hope that it will be useful,
but WITHOUT ANY WARRANTY; without even the implied warranty of
MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
GNU Affero General Public License for more details.

You should have received a copy of the GNU Affero General Public License
along with this program.  If not, see <https://www.gnu.org/licenses/>.

## Disclaimer

This is a proof of concept and not affiliated with or endorsed by One Seven Design, the creators of Blades in the Dark. The Blades in the Dark SRD is used under the [Creative Commons Attribution 3.0 Unported license](http://creativecommons.org/licenses/by/3.0/).
