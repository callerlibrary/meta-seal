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
  if (!parsedArgs.file) {
    console.error('Error: Please provide a file path using -f or --file')
    process.exit(1)
  }
  try {
    const result = runRead(parsedArgs.file, parsedArgs.key || null, parsedArgs.config || null)
    console.log(result)
  } catch (err) {
    console.error(`Error reading VERSION file: ${err.message}`)
    process.exit(1)
  }
} else {
  console.log(`
Usage:
  meta-seal generate [options]
  meta-seal read -f <file> [options]

Options:
  -k, --key <key>       Encryption key (32 bytes)
  -c, --config <path>   Path to config file (default: .meta-sealrc)
  -f, --file <path>     Path to the VERSION file to read
  `)
  process.exit(1)
}
