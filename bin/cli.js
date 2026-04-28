#!/usr/bin/env node

const { runGenerate, runRead } = require('../index.js')

const args = process.argv.slice(2)
const command = args[0]

function parseArgs(args) {
  const result = {}
  for (let i = 0; i < args.length; i++) {
    const arg = args[i]
    if (arg === '-k' || arg === '--key') {
      result.key = args[++i]
    } else if (arg === '-f' || arg === '--file') {
      result.file = args[++i]
    } else if (arg === '-c' || arg === '--config') {
      result.config = args[++i]
    }
  }
  return result
}

const parsedArgs = parseArgs(args.slice(1))

if (command === 'generate') {
  try {
    runGenerate(parsedArgs.config || null, parsedArgs.key || null)
  } catch (err) {
    console.error(`Error generating VERSION file: ${err.message}`)
    process.exit(1)
  }
} else if (command === 'read') {
  try {
    const result = runRead(parsedArgs.file || null, parsedArgs.key || null, parsedArgs.config || null)
    console.log(result)
  } catch (err) {
    console.error(`Error reading VERSION file: ${err.message}`)
    process.exit(1)
  }
} else if (command === 'init') {
  const fs = require('fs')
  const path = require('path')
  const rcPath = path.join(process.cwd(), '.meta-sealrc')

  if (fs.existsSync(rcPath)) {
    console.log('.meta-sealrc already exists.')
    process.exit(0)
  }

  const defaultConfig = {
    basic_info: true,
    git_commit: true,
    git_commit_count: 3,
    git_branch: true,
    build_system: true,
    output_dir: './dist',
    encryption_key: '',
    encryption_key_env: 'META_SEAL_KEY',
  }

  try {
    fs.writeFileSync(rcPath, JSON.stringify(defaultConfig, null, 2) + '\n', 'utf-8')
    console.log('Successfully created .meta-sealrc')
  } catch (err) {
    console.error(`Error creating .meta-sealrc: ${err.message}`)
    process.exit(1)
  }
} else {
  console.log(`
Usage:
  meta-seal init
  meta-seal generate [options]
  meta-seal read [options]

Options:
  -k, --key <key>       Encryption key (32 bytes)
  -c, --config <path>   Path to config file (default: .meta-sealrc)
  -f, --file <path>     Path to the VERSION file to read (optional, default: reads from config's output_dir/VERSION)
  `)
  process.exit(1)
}
