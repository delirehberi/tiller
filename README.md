# Tiller

Tiller is a command-line application to help manage your Today I Learned (TIL) notes. 
It automates the process of creating, and committing new TIL entries using your preferred text editor and Git.

## Features

- Opens your preferred text editor to create a new TIL entry.
- Automatically generates a new filename based on existing TIL entries.
- Prepends a customizable template to each new TIL entry.
- Commits and pushes the new TIL entry to your Git repository.

## Configuration

Tiller requires a configuration file to be placed in `~/.config/tiller/config.json`. 
The configuration file should be in JSON format and include the following fields:

- `editor`: The command to open your preferred text editor.
- `til_folder`: The path to the folder where TIL entries are stored.
- `repo_path`: The path to your Git repository.

Example `config.json`:

```json
{
    "editor": "nano",
    "til_folder": "/path/to/til/folder",
    "repo_path": "/path/to/git/repo"
}
```

Additionally, you can create a `prepend.md` file in the same configuration directory to customize the content that is prepended to each new TIL entry. The file can include placeholders for the filename and date:

- `$TITLE`: Will be replaced with the filename (without extension).
- `$DATE`: Will be replaced with the current date and time.

Example `prepend.md`:

```
+++
title = '#$TITLE'
date = $DATE
draft = false
+++
```

## Installation

1. Clone the repository:

```sh
git clone https://github.com/delirehberi/tiller.git
cd tiller
```

2. Build the project using Cargo:

```sh
cargo build --release
```

3. Move the binary to a directory in your `PATH` (optional):

```sh
cp target/release/tiller /usr/local/bin/
```

## Usage

Simply run the `tiller` command to create a new TIL entry:

```sh
tiller
```

This will:

1. Open your preferred text editor to create a new TIL entry.
2. Automatically generate a new filename based on existing TIL entries.
3. Prepend the content of `prepend.md` to the new TIL entry.
4. Commit and push the new TIL entry to your Git repository.

## Error Handling

If any errors occur during the execution, they will be printed to the console. Common errors include:

- Missing configuration file.
- Failed to open the text editor.
- Issues with reading or writing files.
- Git commit or push failures.

## Dependencies

- [dirs](https://crates.io/crates/dirs)
- [tempfile](https://crates.io/crates/tempfile)
- [chrono](https://crates.io/crates/chrono)
- [serde](https://crates.io/crates/serde)
- [serde_json](https://crates.io/crates/serde_json)

Make sure to add these dependencies in your `Cargo.toml` file:

```toml
[dependencies]
dirs = "3.0"
tempfile = "3.3"
chrono = "0.4"
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
```

## Contributing

Contributions are welcome! Feel free to open an issue or submit a pull request on GitHub.

## License

This project is licensed under the MIT License.

