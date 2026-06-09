#!/usr/bin/env node
/**
 * check-placeholder.js
 * Ensures __GOOGLE_CLIENT_ID__ placeholder is NOT replaced by a real value
 * in committed .kt or .gradle files (the substitution must only happen in CI).
 *
 * Also checks that the old package name com.peoplemodeler is not reintroduced.
 */

const fs = require('fs');

const CHECKS = [
  {
    // Real Google Client ID committed in source — should stay as placeholder
    re: /[0-9]{12,}-[a-z0-9]{32}\.apps\.googleusercontent\.com/,
    msg: 'Real Google Client ID found — use __GOOGLE_CLIENT_ID__ placeholder instead.\n'
       + '   The real value must only be injected by CI via the GOOGLE_CLIENT_ID secret.',
  },
  {
    // Old package name accidentally reintroduced
    re: /package com\.peoplemodeler(?!\.)/,
    msg: 'Old package name "com.peoplemodeler" found — should be "com.stellasecret.peoplemodeler".',
  },
  {
    // Old import path accidentally reintroduced
    re: /import com\.peoplemodeler\./,
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

  const lines = fs.readFileSync(file, 'utf8').split('\n');
  lines.forEach((line, i) => {
    for (const { re, msg } of CHECKS) {
      if (re.test(line)) {
        console.error(`❌ ${file}:${i + 1} — ${msg}`);
        console.error(`   ${line.trim()}`);
        hasError = true;
      }
    }
  });
}

if (hasError) {
  process.exit(1);
} else {
  console.log(`✅ check-placeholder: ${files.length} file(s) OK`);
  process.exit(0);
}
