#!/usr/bin/env node
/**
 * check-lockfile.js
 * Warns if package.json is staged for commit but package-lock.json is not.
 * Prevents "package.json and lockfile out of sync" CI failures.
 */

const { execSync } = require('child_process');

function getStagedFiles() {
  try {
    return execSync('git diff --cached --name-only', { encoding: 'utf8' })
      .split('\n')
      .map(f => f.trim())
      .filter(Boolean);
  } catch {
    return [];
  }
}

const staged = getStagedFiles();
const hasPackageJson  = staged.includes('package.json');
const hasLockfile     = staged.includes('package-lock.json');

if (hasPackageJson && !hasLockfile) {
  console.error('❌ package.json is staged but package-lock.json is not.');
  console.error('   Run: npm install --package-lock-only');
  console.error('   Then: git add package-lock.json\n');
  process.exit(1);
}

console.log('✅ check-lockfile: OK');
process.exit(0);
