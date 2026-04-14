# meta-seal

[![npm version](https://img.shields.io/npm/v/meta-seal.svg)](https://npmjs.org/package/meta-seal)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

[English](./README-en.md) | 简体中文

`meta-seal` 是一个为前端工程设计的版本描述文件生成工具。它可以在项目构建时，自动收集项目基础信息、Git 提交记录、分支信息以及构建系统信息，并将这些信息使用 AES-256-GCM 算法加密，最终生成一个 `VERSION` 文件输出到指定目录。

该工具基于 **Rust** 编写，并通过 **NAPI-RS** 提供 Node.js 绑定，支持全平台跨系统使用，无需本地安装 Rust 环境。

---

## ✨ 核心功能

- **信息收集**：自动收集 `package.json` 中的名称和版本、Git 当前分支、最近 N 条提交记录（Hash和信息）、以及构建环境的操作系统、架构和时间。
- **高安全性**：使用 `AES-256-GCM` 工业级加密算法生成安全的 `VERSION` 文件。
- **高度可配**：通过项目根目录的 `.meta-sealrc` 进行灵活配置，支持开启/关闭特定的信息收集。
- **跨平台支持**：预编译分发 Windows、macOS（Intel & Apple Silicon）、Linux 的 `.node` 二进制文件。
- **无缝集成**：可像 ESLint 一样直接在 npm scripts 中调用。

## 📦 安装

在你的前端工程中使用你喜欢的包管理器安装：

```bash
npm install -D @callerlibrary/meta-seal
# 或者
yarn add -D @callerlibrary/meta-seal
# 或者
pnpm add -D @callerlibrary/meta-seal
```

## ⚙️ 配置

在项目根目录下创建一个 `.meta-sealrc` JSON 文件（可选，如果没有配置则使用默认值）：

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

### 配置项说明：

- `basic_info`: 是否收集 package.json 中的 name 和 version（默认: `true`）
- `git_commit`: 是否收集 git 提交记录（默认: `true`）
- `git_commit_count`: 收集最近的几条 git 提交记录（默认: `3`）
- `git_branch`: 是否收集 git 当前分支（默认: `true`）
- `build_system`: 是否收集构建系统的 OS、架构和时间（默认: `true`）
- `output_dir`: VERSION 文件的输出目录（默认: `"./dist"`）
- `encryption_key`: 直接在配置中写死的 32 字节加密密钥
- `encryption_key_env`: 指定从哪个环境变量读取密钥（默认会兜底尝试读取 `META_SEAL_KEY`）

## 🚀 使用方法

### 1. 结合 NPM Scripts（推荐）

在你的 `package.json` 中配置脚本：

```json
{
  "scripts": {
    "build": "vite build && npm run meta:generate",
    "meta:generate": "meta-seal generate",
    "meta:read": "meta-seal read -f ./dist/VERSION"
  }
}
```

在执行 `npm run build` 或者 `npm run meta:generate` 前，请确保已通过配置文件、命令行参数或环境变量提供了 32 字节长度的密钥。

### 2. 命令行 (CLI) 调用

你可以使用 `npx` 或者 `pnpm dlx` 临时调用工具：

#### 生成并加密 VERSION 文件

```bash
npx meta-seal generate -k 12345678901234567890123456789012
```

#### 读取并解密 VERSION 文件

解密后，工具会在控制台格式化输出明文的 JSON 信息：

```bash
npx meta-seal read -f ./dist/VERSION -k 12345678901234567890123456789012
```

**解密输出示例：**

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

## 🔐 密钥优先级逻辑

工具在寻找 32 字节加密/解密密钥时，按照以下优先级进行读取（优先级从高到低）：

1. 命令行参数 `-k` 或 `--key`
2. `.meta-sealrc` 文件中的 `encryption_key` 字段
3. `.meta-sealrc` 文件中的 `encryption_key_env` 字段指定的系统环境变量
4. 默认的系统环境变量 `META_SEAL_KEY`

## 📄 开源协议

MIT License
