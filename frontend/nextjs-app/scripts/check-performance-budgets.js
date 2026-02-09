#!/usr/bin/env node

/**
 * Performance Budget Checker
 * 
 * Checks if the build output meets the defined performance budgets
 * and fails the build if budgets are exceeded.
 */

const fs = require('fs');
const path = require('path');

// Colors for console output
const colors = {
  reset: '\x1b[0m',
  bright: '\x1b[1m',
  red: '\x1b[31m',
  green: '\x1b[32m',
  yellow: '\x1b[33m',
  cyan: '\x1b[36m',
};

function colorize(text, color) {
  return `${colors[color]}${text}${colors.reset}`;
}

function formatBytes(bytes) {
  if (bytes === 0) return '0 B';
  const k = 1024;
  const sizes = ['B', 'KB', 'MB', 'GB'];
  const i = Math.floor(Math.log(bytes) / Math.log(k));
  return `${(bytes / Math.pow(k, i)).toFixed(2)} ${sizes[i]}`;
}

function parseSize(sizeStr) {
  const match = sizeStr.match(/^(\d+(?:\.\d+)?)(kb|mb|gb|b)$/i);
  if (!match) throw new Error(`Invalid size format: ${sizeStr}`);
  
  const value = parseFloat(match[1]);
  const unit = match[2].toLowerCase();
  
  const multipliers = {
    b: 1,
    kb: 1024,
    mb: 1024 * 1024,
    gb: 1024 * 1024 * 1024,
  };
  
  return value * multipliers[unit];
}

function checkBudgets() {
  console.log(colorize('\n💰 Performance Budget Check\n', 'bright'));
  
  // Load budgets
  const budgetsPath = path.join(process.cwd(), 'performance-budgets.json');
  if (!fs.existsSync(budgetsPath)) {
    console.error(colorize('❌ performance-budgets.json not found', 'red'));
    process.exit(1);
  }
  
  const budgets = JSON.parse(fs.readFileSync(budgetsPath, 'utf8'));
  
  // Check build directory
  const buildDir = path.join(process.cwd(), '.next');
  if (!fs.existsSync(buildDir)) {
    console.error(colorize('❌ Build directory not found. Run `npm run build` first.', 'red'));
    process.exit(1);
  }
  
  const results = [];
  let hasErrors = false;
  let hasWarnings = false;
  
  // Check bundle budgets
  console.log(colorize('📦 Bundle Size Budgets:', 'cyan'));
  console.log('─'.repeat(100));
  
  budgets.budgets.forEach(budget => {
    const { name, description, budget: limits, path: pattern } = budget;
    
    // Find matching files
    const files = findFiles(buildDir, pattern);
    const totalSize = files.reduce((sum, file) => {
      const stats = fs.statSync(file);
      return sum + stats.size;
    }, 0);
    
    const maxSize = parseSize(limits.max);
    const warnSize = parseSize(limits.warn);
    
    const status = totalSize > maxSize ? '❌' : totalSize > warnSize ? '⚠️' : '✅';
    const color = totalSize > maxSize ? 'red' : totalSize > warnSize ? 'yellow' : 'green';
    
    if (totalSize > maxSize) hasErrors = true;
    if (totalSize > warnSize) hasWarnings = true;
    
    console.log(
      `${status} ${name.padEnd(25)} ${colorize(formatBytes(totalSize).padEnd(12), color)} / ${formatBytes(maxSize).padEnd(12)} ${description}`
    );
    
    results.push({
      name,
      status: totalSize > maxSize ? 'fail' : totalSize > warnSize ? 'warn' : 'pass',
      current: totalSize,
      max: maxSize,
      warn: warnSize,
    });
  });
  
  console.log('─'.repeat(100));
  
  // Summary
  console.log(colorize('\n📊 Summary:', 'cyan'));
  console.log('─'.repeat(100));
  
  const passed = results.filter(r => r.status === 'pass').length;
  const warned = results.filter(r => r.status === 'warn').length;
  const failed = results.filter(r => r.status === 'fail').length;
  
  console.log(`✅ Passed: ${colorize(passed.toString(), 'green')}`);
  console.log(`⚠️  Warnings: ${colorize(warned.toString(), 'yellow')}`);
  console.log(`❌ Failed: ${colorize(failed.toString(), 'red')}`);
  
  console.log('─'.repeat(100));
  
  // Recommendations
  if (hasErrors || hasWarnings) {
    console.log(colorize('\n💡 Recommendations:', 'cyan'));
    console.log('─'.repeat(100));
    
    const failedBudgets = results.filter(r => r.status === 'fail' || r.status === 'warn');
    
    failedBudgets.forEach(({ name, current, max }) => {
      const excess = current - max;
      const percentage = ((current / max - 1) * 100).toFixed(1);
      
      console.log(colorize(`\n${name}:`, 'yellow'));
      console.log(`  Current: ${formatBytes(current)}`);
      console.log(`  Budget: ${formatBytes(max)}`);
      console.log(`  Excess: ${formatBytes(excess)} (${percentage}% over budget)`);
      
      // Specific recommendations based on budget type
      if (name.includes('Bundle')) {
        console.log('  Suggestions:');
        console.log('    - Use dynamic imports for heavy components');
        console.log('    - Implement code splitting at route level');
        console.log('    - Remove unused dependencies');
        console.log('    - Use tree-shaking for libraries');
      } else if (name.includes('Image')) {
        console.log('  Suggestions:');
        console.log('    - Use Next.js Image component for optimization');
        console.log('    - Implement lazy loading for images');
        console.log('    - Use WebP/AVIF formats');
        console.log('    - Compress images before upload');
      } else if (name.includes('CSS')) {
        console.log('  Suggestions:');
        console.log('    - Remove unused CSS');
        console.log('    - Use Tailwind\'s purge feature');
        console.log('    - Minimize CSS files');
      }
    });
    
    console.log('\n' + '─'.repeat(100));
  }
  
  // Exit with error if budgets exceeded
  if (hasErrors) {
    console.log(colorize('\n❌ Performance budgets exceeded! Build failed.\n', 'red'));
    process.exit(1);
  } else if (hasWarnings) {
    console.log(colorize('\n⚠️  Performance budgets warnings detected.\n', 'yellow'));
    process.exit(0);
  } else {
    console.log(colorize('\n✅ All performance budgets met!\n', 'green'));
    process.exit(0);
  }
}

function findFiles(dir, pattern) {
  const files = [];
  
  // Convert glob pattern to regex
  const regexPattern = pattern
    .replace(/\./g, '\\.')
    .replace(/\*\*/g, '.*')
    .replace(/\*/g, '[^/]*');
  
  const regex = new RegExp(regexPattern);
  
  function scan(currentDir) {
    if (!fs.existsSync(currentDir)) return;
    
    const entries = fs.readdirSync(currentDir);
    
    entries.forEach(entry => {
      const fullPath = path.join(currentDir, entry);
      const relativePath = path.relative(dir, fullPath);
      const stats = fs.statSync(fullPath);
      
      if (stats.isDirectory()) {
        scan(fullPath);
      } else if (regex.test('/' + relativePath.replace(/\\/g, '/'))) {
        files.push(fullPath);
      }
    });
  }
  
  scan(dir);
  return files;
}

// Run check
try {
  checkBudgets();
} catch (error) {
  console.error(colorize(`\n❌ Error: ${error.message}\n`, 'red'));
  console.error(error.stack);
  process.exit(1);
}
