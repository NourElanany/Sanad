#!/usr/bin/env node

/**
 * Bundle Size Analyzer
 * 
 * Analyzes the Next.js build output and provides detailed
 * information about bundle sizes, code splitting, and optimization opportunities.
 */

const fs = require('fs');
const path = require('path');
const { execSync } = require('child_process');

// Colors for console output
const colors = {
  reset: '\x1b[0m',
  bright: '\x1b[1m',
  red: '\x1b[31m',
  green: '\x1b[32m',
  yellow: '\x1b[33m',
  blue: '\x1b[34m',
  magenta: '\x1b[35m',
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

function formatPercentage(value) {
  return `${(value * 100).toFixed(2)}%`;
}

function analyzeBuildOutput() {
  console.log(colorize('\n📊 Bundle Size Analysis\n', 'bright'));
  
  const buildDir = path.join(process.cwd(), '.next');
  
  if (!fs.existsSync(buildDir)) {
    console.error(colorize('❌ Build directory not found. Run `npm run build` first.', 'red'));
    process.exit(1);
  }
  
  // Read build manifest
  const manifestPath = path.join(buildDir, 'build-manifest.json');
  if (!fs.existsSync(manifestPath)) {
    console.error(colorize('❌ Build manifest not found.', 'red'));
    process.exit(1);
  }
  
  const manifest = JSON.parse(fs.readFileSync(manifestPath, 'utf8'));
  
  // Analyze pages
  console.log(colorize('📄 Page Bundles:', 'cyan'));
  console.log('─'.repeat(80));
  
  const pageStats = [];
  let totalSize = 0;
  
  for (const [page, files] of Object.entries(manifest.pages)) {
    let pageSize = 0;
    
    files.forEach(file => {
      const filePath = path.join(buildDir, file);
      if (fs.existsSync(filePath)) {
        const stats = fs.statSync(filePath);
        pageSize += stats.size;
      }
    });
    
    pageStats.push({ page, size: pageSize, files: files.length });
    totalSize += pageSize;
  }
  
  // Sort by size
  pageStats.sort((a, b) => b.size - a.size);
  
  // Display page stats
  pageStats.forEach(({ page, size, files }) => {
    const sizeStr = formatBytes(size);
    const percentage = formatPercentage(size / totalSize);
    
    let color = 'green';
    if (size > 200 * 1024) color = 'red';
    else if (size > 100 * 1024) color = 'yellow';
    
    console.log(
      `${colorize(page.padEnd(40), 'blue')} ${colorize(sizeStr.padEnd(12), color)} ${colorize(percentage.padEnd(8), 'magenta')} (${files} files)`
    );
  });
  
  console.log('─'.repeat(80));
  console.log(colorize(`Total: ${formatBytes(totalSize)}`, 'bright'));
  
  // Analyze chunks
  console.log(colorize('\n📦 Chunk Analysis:', 'cyan'));
  console.log('─'.repeat(80));
  
  const chunksDir = path.join(buildDir, 'static', 'chunks');
  if (fs.existsSync(chunksDir)) {
    const chunks = [];
    
    function scanChunks(dir) {
      const files = fs.readdirSync(dir);
      
      files.forEach(file => {
        const filePath = path.join(dir, file);
        const stats = fs.statSync(filePath);
        
        if (stats.isDirectory()) {
          scanChunks(filePath);
        } else if (file.endsWith('.js')) {
          chunks.push({
            name: path.relative(chunksDir, filePath),
            size: stats.size,
          });
        }
      });
    }
    
    scanChunks(chunksDir);
    chunks.sort((a, b) => b.size - a.size);
    
    // Display top 10 largest chunks
    console.log(colorize('Top 10 Largest Chunks:', 'yellow'));
    chunks.slice(0, 10).forEach(({ name, size }, index) => {
      const sizeStr = formatBytes(size);
      let color = 'green';
      if (size > 500 * 1024) color = 'red';
      else if (size > 250 * 1024) color = 'yellow';
      
      console.log(
        `${(index + 1).toString().padStart(2)}. ${name.padEnd(50)} ${colorize(sizeStr, color)}`
      );
    });
    
    const totalChunkSize = chunks.reduce((sum, c) => sum + c.size, 0);
    console.log('─'.repeat(80));
    console.log(colorize(`Total Chunks: ${chunks.length} (${formatBytes(totalChunkSize)})`, 'bright'));
  }
  
  // Performance recommendations
  console.log(colorize('\n💡 Recommendations:', 'cyan'));
  console.log('─'.repeat(80));
  
  const recommendations = [];
  
  // Check for large pages
  const largePagesCount = pageStats.filter(p => p.size > 200 * 1024).length;
  if (largePagesCount > 0) {
    recommendations.push({
      severity: 'high',
      message: `${largePagesCount} page(s) exceed 200KB. Consider code splitting.`,
    });
  }
  
  // Check for large chunks
  const largeChunks = chunks.filter(c => c.size > 500 * 1024);
  if (largeChunks.length > 0) {
    recommendations.push({
      severity: 'high',
      message: `${largeChunks.length} chunk(s) exceed 500KB. Consider splitting further.`,
    });
  }
  
  // Check total bundle size
  if (totalSize > 2 * 1024 * 1024) {
    recommendations.push({
      severity: 'medium',
      message: 'Total bundle size exceeds 2MB. Consider lazy loading more components.',
    });
  }
  
  // Check for duplicate dependencies
  const vendorChunks = chunks.filter(c => c.name.includes('vendor'));
  if (vendorChunks.length > 1) {
    recommendations.push({
      severity: 'medium',
      message: 'Multiple vendor chunks detected. Check for duplicate dependencies.',
    });
  }
  
  if (recommendations.length === 0) {
    console.log(colorize('✅ No major issues found. Bundle is well optimized!', 'green'));
  } else {
    recommendations.forEach(({ severity, message }) => {
      const icon = severity === 'high' ? '⚠️' : 'ℹ️';
      const color = severity === 'high' ? 'red' : 'yellow';
      console.log(`${icon} ${colorize(message, color)}`);
    });
  }
  
  console.log('─'.repeat(80));
  
  // Budget check
  console.log(colorize('\n📊 Performance Budget Check:', 'cyan'));
  console.log('─'.repeat(80));
  
  const budgets = {
    'Initial Bundle': { max: 250 * 1024, current: pageStats[0]?.size || 0 },
    'Page Bundles': { max: 150 * 1024, current: Math.max(...pageStats.slice(1).map(p => p.size)) },
    'Total Size': { max: 2 * 1024 * 1024, current: totalSize },
  };
  
  for (const [name, { max, current }] of Object.entries(budgets)) {
    const percentage = (current / max) * 100;
    const status = percentage > 100 ? '❌' : percentage > 80 ? '⚠️' : '✅';
    const color = percentage > 100 ? 'red' : percentage > 80 ? 'yellow' : 'green';
    
    console.log(
      `${status} ${name.padEnd(20)} ${colorize(formatBytes(current).padEnd(12), color)} / ${formatBytes(max)} (${formatPercentage(current / max)})`
    );
  }
  
  console.log('─'.repeat(80));
  console.log(colorize('\n✨ Analysis complete!\n', 'bright'));
}

// Run analysis
try {
  analyzeBuildOutput();
} catch (error) {
  console.error(colorize(`\n❌ Error: ${error.message}\n`, 'red'));
  process.exit(1);
}
