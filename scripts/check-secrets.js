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
  // \b anchors prevent matching inside longer words (e.g. "notapassword")
  { re: /\b(api[_-]?key|secret|password|private[_-]?key)\b\s*[:=]\s*['"][^'"]{8,}['"]/i,
    msg: 'Possible hardcoded secret (api_key/secret/password)' },

  // Google OAuth client secrets — \b prevents substring matches
  { re: /\bGOCSPX-[A-Za-z0-9_-]{28,}\b/,
    msg: 'Google OAuth client secret' },

  // AWS keys — always exactly 20 chars starting with AKIA
  { re: /\bAKIA[0-9A-Z]{16}\b/,
    msg: 'AWS Access Key ID' },

  // Generic Bearer / token patterns
  { re: /\bbearer\s+[A-Za-z0-9\-._~+/]{20,}/i,
    msg: 'Hardcoded Bearer token' },

  // Private key blocks — always at line start
  { re: /^-----BEGIN (RSA |EC |OPENSSH )?PRIVATE KEY-----/m,
    msg: 'Private key block' },
];

// ── Allowlist — lines that are OK (false positives) ───────
// Using plain string matching instead of regex avoids CodeQL's
// "missing anchor" warning and is simpler for these literal checks.
const ALLOWLIST_STRINGS = [
  '__GOOGLE_CLIENT_ID__',       // placeholder — OK to commit
  'apps.googleusercontent.com', // OAuth client ID — not a secret
  'example.com',
  'placeholder',
  'your_key_here',
  'your-key-here',
  '${{',                        // GitHub Actions secrets syntax
];

function isAllowed(line) {
  const lower = line.toLowerCase();
  return ALLOWLIST_STRINGS.some(s => lower.includes(s.toLowerCase()));
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
