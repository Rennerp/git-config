# git-config

Basically I always forget the commands to do this so now I'm learning rust so here it is.

A small Rust CLI that sets your global Git identity:

- `user.name`
- `user.email`

It first checks that Git is installed and available on `PATH`, then runs:

- `git config --global user.name`
- `git config --global user.email`

## Download A Release

1. Open the repository's `Releases` page on GitHub.
2. Download the binary for your platform.
3. Extract it if the asset is packaged as `.zip` or `.tar.gz`.

Current release assets are intended to look like:

- `git-config-windows-x86_64.zip`
- `git-config-linux-x86_64.tar.gz`
- `git-config-macos-arm64.tar.gz`

## Usage

Pass your Git name and email as arguments.

### Windows

Open PowerShell in the folder where you extracted the binary and run:

```powershell
.\git-config.exe "Your Name" "you@example.com"
```

### macOS

Open Terminal in the folder where you extracted the binary and run:

```bash
./git-config "Your Name" "you@example.com"
```

### Linux

Open a terminal in the folder where you extracted the binary and run:

```bash
./git-config "Your Name" "you@example.com"
```

If the binary is not executable on macOS or Linux, run:

```bash
chmod +x ./git-config
```

## Verify

After running the tool, confirm the values were saved:

```bash
git config --global --get user.name
git config --global --get user.email
```

## Build From Source

If you want to build it yourself instead of downloading a release:

```bash
cargo build --release
```

## License

Licensed under either of these, at your option:

- MIT
- Apache-2.0

See [LICENSE-MIT](LICENSE-MIT) and [LICENSE-APACHE](LICENSE-APACHE).
