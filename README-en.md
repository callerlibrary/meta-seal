# meta-seal

[![npm version](https://img.shields.io/npm/v/meta-seal.svg)](https://npmjs.org/package/meta-seal)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

English | [简体中文](./README.md)

`meta-seal` is a version description file generation tool designed for front-end engineering. During the project build process, it automatically collects basic project information, Git commit history, branch details, and build system environment variables. It then encrypts this data using the AES-256-GCM algorithm and generates a `VERSION` file in the specified output directory.

This tool is written in **Rust** and provides Node.js bindings via **NAPI-RS**, supporting cross-platform usage (Windows, macOS, Linux) without requiring a local Rust environment.

---

## ✨ Features

- **Information Collection**: Automatically retrieves the `name` and `version` from `package.json`, the current Git branch, the last N commit records (Hash and message), and the build environment's OS, architecture, and time.
- **High Security**: Uses the industrial-grade `AES-256-GCM` encryption algorithm to generate a secure `VERSION` file.
- **Highly Configurable**: Flexible configuration via a `.meta-sealrc` file in the project root, allowing specific data collection to be enabled/disabled.
- **Cross-Platform Support**: Distributes pre-compiled `.node` binaries for Windows, macOS (Intel & Apple Silicon), and Linux.
- **Seamless Integration**: Can be invoked directly within npm scripts, just like ESLint.

## 📦 Installation

Install using your preferred package manager in your front-end project:

```bash
npm install -D @callerlibrary/meta-seal
# or
yarn add -D @callerlibrary/meta-seal
# or
pnpm add -D @callerlibrary/meta-seal
```

## ⚙️ Configuration

Create a `.meta-sealrc` JSON file in your project root directory (optional, default values will be used if omitted):

```json
{
  "basic_info": true,
  "git_commit": true,
  "git_commit_count": 3,
  "git_branch": true,
  "build_system": true,
  "output_dir": "./dist",
  "encryption_key": "12345678901234567890123456789012",
  "encryption_key_env": "META_SEAL_KEY"
}
```

### Configuration Options:

- `basic_info`: Whether to collect `name` and `version` from `package.json` (Default: `true`)
- `git_commit`: Whether to collect Git commit records (Default: `true`)
- `git_commit_count`: Number of recent Git commit records to collect (Default: `3`)
- `git_branch`: Whether to collect the current Git branch (Default: `true`)
- `build_system`: Whether to collect the build system OS, architecture, and timestamp (Default: `true`)
- `output_dir`: Output directory for the `VERSION` file (Default: `"./dist"`)
- `encryption_key`: A hardcoded 32-byte encryption key directly in the config
- `encryption_key_env`: Specify an environment variable to read the key from (Fallbacks to `META_SEAL_KEY` by default)

## 🚀 Usage

### 1. Using NPM Scripts (Recommended)

Configure scripts in your `package.json`:

```json
{
  "scripts": {
    "build": "vite build && npm run meta:generate",
    "meta:generate": "meta-seal generate",
    "meta:read": "meta-seal read -f ./dist/VERSION"
  }
}
```

Before running `npm run build` or `npm run meta:generate`, ensure a 32-byte key is provided via the config file, CLI argument, or environment variable.

### 2. CLI Invocation

You can temporarily invoke the tool using `npx` or `pnpm dlx`:

#### Initialize Config File

Automatically generate a default `.meta-sealrc` configuration file in the current directory:

```bash
npx meta-seal init
```

#### Generate and Encrypt VERSION file

```bash
npx meta-seal generate -k 12345678901234567890123456789012
```

#### Read and Decrypt VERSION file

After decryption, the tool will format and output the plaintext JSON to the console:

```bash
npx meta-seal read -f ./dist/VERSION -k 12345678901234567890123456789012
```

**Example Decrypted Output:**

```json
{
  "build": {
    "architecture": "x86_64",
    "build_time": "2026-04-13T10:00:00+00:00",
    "os": "windows"
  },
  "git": {
    "branch": "feature/init_app",
    "commits": [
      {
        "hash": "bd6ecaedf0ee45cafa14974c0acdc3e9bb0b7c84",
        "message": "Initial commit"
      }
    ]
  },
  "project": {
    "name": "my-awesome-app",
    "version": "1.0.0"
  }
}
```

## 🔐 Key Priority Logic

When searching for the 32-byte encryption/decryption key, the tool evaluates the following sources in descending order of priority:

1. CLI arguments `-k` or `--key`
2. `encryption_key` field in the `.meta-sealrc` file
3. The system environment variable specified by `encryption_key_env` in `.meta-sealrc`
4. The default system environment variable `META_SEAL_KEY`

## 📄 License

MIT License
