<!--
Darkforge Maskfile
This file contains tasks for the Darkforge project.
Cross-platform compatible for both Linux and Windows.
-->

# Darkforge Tasks

## dev

> Run all development tasks

```bash
mask fmt
mask lint
mask check
mask run
```

## fmt

> Run all formatting tasks

* rustfmt - Format code with rustfmt

```bash
mask fmt rustfmt
```

### fmt rustfmt

> Format code with rustfmt

```bash
cargo +nightly fmt --all -- --config-path .config
```

## lint

> Run all linting tasks

* clippy - Lint code with clippy
* rustfmt - Lint code with rustfmt
* spellcheck - Run spell check

```bash
mask lint clippy
mask lint rustfmt
mask lint spellcheck
```

### lint clippy

> Lint code with clippy

```bash
cargo clippy --no-deps --all
```

### lint rustfmt

> Lint code with rustfmt

```bash
cargo +nightly fmt --all --check -- --config-path .config
```

### lint spellcheck

> Run spell check

```bash
bun spellcheck
```

## fix

> Fix all linting problems

* clippy - Fix code with clippy

```bash
mask fix clippy
```

### fix clippy

> Fix code with clippy

```bash
cargo clippy --allow-dirty --no-deps --all --fix
```

## audit

> Check for vulnerabilities and security advisories

```bash
cargo audit
```

## check

> Run cargo check

**OPTIONS**
* release
  * flags: --release -r
  * type: flag
  * desc: Test in release mode (default: debug)
* linux
  * flags: --linux
  * type: flag
  * desc: Test for Linux platform
* windows
  * flags: --windows
  * type: flag
  * desc: Test for Windows platform
* 32bit
  * flags: --32bit
  * type: flag
  * desc: Test for 32-bit architecture as well

```bash
args=()
if [ "$mode" == "release" ]; then
  args+=("--release")
fi

targets=()
if [ "$linux" == "true" ]; then
  if [ "$32bit" == "true" ]; then
    targets+=("--target i686-unknown-linux-gnu")
  fi

  targets+=("--target x86_64-unknown-linux-gnu")
fi

if [ "$windows" == "true" ]; then
    targets+=("--target x86_64-pc-windows-gnu")
fi

if [ "${#targets[@]}" -eq 0 ]; then
    echo "No targets specified, running current platform"
    cross check ${args[@]}
else
    for target in "${targets[@]}"; do
        cross check ${args[@]} ${target}
    done
fi
````

## test

> Run tests with optional flags for mode, architecture, and platform

**OPTIONS**
* release
  * flags: --release -r
  * type: flag
  * desc: Test in release mode (default: debug)
* linux
  * flags: --linux
  * type: flag
  * desc: Test for Linux platform
* windows
  * flags: --windows
  * type: flag
  * desc: Test for Windows platform
* 32bit
  * flags: --32bit
  * type: flag
  * desc: Test for 32-bit architecture as well

```bash
args=()
if [ "$mode" == "release" ]; then
  args+=("--release")
fi

targets=()
if [ "$linux" == "true" ]; then
  if [ "$32bit" == "true" ]; then
    targets+=("--target i686-unknown-linux-gnu")
  fi

  targets+=("--target x86_64-unknown-linux-gnu")
fi

if [ "$windows" == "true" ]; then
    targets+=("--target x86_64-pc-windows-gnu")
fi

if [ "${#targets[@]}" -eq 0 ]; then
    echo "No targets specified, running current platform"
    cross nextest run ${args[@]}
else
    for target in "${targets[@]}"; do
        cross nextest run ${args[@]} ${target}
    done
fi
```

## build

> Build the project with optional flags for mode, architecture, and platform

**OPTIONS**
* release
  * flags: --release -r
  * type: flag
  * desc: Test in release mode (default: debug)
* linux
  * flags: --linux
  * type: flag
  * desc: Test for Linux platform
* windows
  * flags: --windows
  * type: flag
  * desc: Test for Windows platform
* 32bit
  * flags: --32bit
  * type: flag
  * desc: Test for 32-bit architecture as well

```bash
args=()
if [ "$mode" == "release" ]; then
  args+=("--release")
fi

targets=()
if [ "$linux" == "true" ]; then
  if [ "$32bit" == "true" ]; then
    targets+=("--target i686-unknown-linux-gnu")
  fi

  targets+=("--target x86_64-unknown-linux-gnu")
fi

if [ "$windows" == "true" ]; then
    targets+=("--target x86_64-pc-windows-gnu")
fi

if [ "${#targets[@]}" -eq 0 ]; then
    echo "No targets specified, running current platform"
    cross build ${args[@]}
else
    for target in "${targets[@]}"; do
        cross build ${args[@]} ${target}
    done
fi
```

## run

> Run the project with the root command

```bash
cargo run --bin dfplay
```
