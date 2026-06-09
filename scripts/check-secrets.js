#!/usr/bin/env node
/**
 * check-secrets.js
 * Scans staged (or specified) files for patterns that look like hardcoded secrets.
 * Runs as a lint-staged hook on web/**\/*.{js,css} and also standalone.
 */

const fs   = require('fs');
const path = require('path');

// ── Secret patterns ───────────────────────────────────────
const PATTERNS = [
  // Generic key/secret/password assignments
  { re: /(api[_-]?key|secret|password|private[_-]?key)\s*[:=]\s*['"][^'"]{8,}['"]/i,
    msg: 'Possible hardcoded secret (api_key/secret/password)' },

  // Google OAuth client secrets (server-side secret, NOT client IDs)
  { re: /GOCSPX-[A-Za-z0-9_-]{28,}/,
    msg: 'Google OAuth client secret' },

  // AWS keys
  { re: /AKIA[0-9A-Z]{16}/,
    msg: 'AWS Access Key ID' },

  // Generic Bearer / token patterns
  { re: /bearer\s+[A-Za-z0-9\-._~+/]{20,}/i,
    msg: 'Hardcoded Bearer token' },

  // Private key blocks
  { re: /-----BEGIN (RSA |EC |OPENSSH )?PRIVATE KEY-----/,
    msg: 'Private key block' },
];

// ── Allowlist — patterns that are OK (false positives) ────
const ALLOWLIST = [
  /__GOOGLE_CLIENT_ID__/,          // placeholder — OK to commit
  /apps\.googleusercontent\.com/,  // OAuth client ID — not a secret
  /example\.com/,
  /placeholder/i,
  /your[_-]?key[_-]?here/i,
  /\$\{\{.*\}\}/,                  // GitHub Actions secrets syntax
];

function isAllowed(line) {
  return ALLOWLIST.some(p => p.test(line));
}

// ── Main ──────────────────────────────────────────────────
const files = process.argv.slice(2).filter(a => !a.startsWith('--'));

if (files.length === 0) {
  console.log('✅ check-secrets: no files to check');
  process.exit(0);
}

let hasError = false;

for (const file of files) {
  if (!fs.existsSync(file)) continue;

  const lines = fs.readFileSync(file, 'utf8').split('\n');
  lines.forEach((line, i) => {
    if (isAllowed(line)) return;
    for (const { re, msg } of PATTERNS) {
      if (re.test(line)) {
        console.error(`❌ ${file}:${i + 1} — ${msg}`);
        console.error(`   ${line.trim()}`);
        hasError = true;
      }
    }
  });
}

if (hasError) {
  console.error('\n💡 Remove secrets before committing.');
  console.error('   Use environment variables or GitHub Actions secrets instead.\n');
  process.exit(1);
} else {
  console.log(`✅ check-secrets: ${files.length} file(s) clean`);
  process.exit(0);
}
