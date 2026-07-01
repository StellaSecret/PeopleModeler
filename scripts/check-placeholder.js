#!/usr/bin/env node
/**
 * check-placeholder.js
 * Ensures __GOOGLE_CLIENT_ID__ placeholder is NOT replaced by a real value
 * in committed .kt, .gradle, or .js files (the substitution must only happen in CI).
 *
 * Also checks that the old package name com.peoplemodeler is not reintroduced.
 */

const fs = require('fs');

const CHECKS = [
  {
    // Real Google Client ID committed in source — should stay as placeholder.
    // Anchored with ^ and $ (multiline) so CodeQL knows the match scope is one line.
    re: /^.*\b[0-9]{12,}-[a-z0-9]{32}\.apps\.googleusercontent\.com\b.*$/m,
    msg: 'Real Google Client ID found — use __GOOGLE_CLIENT_ID__ placeholder instead.\n'
       + '   The real value must only be injected by CI via the GOOGLE_CLIENT_ID secret.',
  },
  {
    // Old package name accidentally reintroduced — anchored to line start
    re: /^package com\.peoplemodeler(?!\.)/m,
    msg: 'Old package name "com.peoplemodeler" found — should be "com.stellasecret.peoplemodeler".',
  },
  {
    // Old import path accidentally reintroduced — anchored to line start
    re: /^import com\.peoplemodeler\./m,
    msg: 'Old import path "com.peoplemodeler" found — should be "com.stellasecret.peoplemodeler".',
  },
];

const files = process.argv.slice(2).filter(a => !a.startsWith('--'));

if (files.length === 0) {
  console.log('✅ check-placeholder: no files to check');
  process.exit(0);
}

let hasError = false;

for (const file of files) {
  if (!fs.existsSync(file)) continue;

  // Test against full file content — regexes use /m flag so ^ and $ match line boundaries
  const content = fs.readFileSync(file, 'utf8');
  for (const { re, msg } of CHECKS) {
    if (re.test(content)) {
      // Find the matching line number for a useful error message
      const match = content.match(re);
      const lineNum = match ? content.slice(0, content.indexOf(match[0])).split('\n').length : '?';
      console.error(`❌ ${file}:${lineNum} — ${msg}`);
      console.error(`   ${(match?.[0] ?? '').trim()}`);
      hasError = true;
    }
  }
}

if (hasError) {
  process.exit(1);
} else {
  console.log(`✅ check-placeholder: ${files.length} file(s) OK`);
  process.exit(0);
}
