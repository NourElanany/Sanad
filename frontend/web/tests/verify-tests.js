/**
 * Test Verification Script for Sanad Interface Tests
 * Verifies that test files are properly structured and can be loaded
 */

const fs = require('fs');
const path = require('path');

// Colors for console output
const colors = {
  green: '\x1b[32m',
  red: '\x1b[31m',
  yellow: '\x1b[33m',
  blue: '\x1b[34m',
  reset: '\x1b[0m'
};

function log(message, color = 'reset') {
  console.log(`${colors[color]}${message}${colors.reset}`);
}

function verifyTestFiles() {
  log('🧪 Verifying Sanad Interface Test Files...', 'blue');
  log('=' .repeat(50), 'blue');
  
  const testDir = __dirname;
  const integrationDir = path.join(testDir, 'integration');
  const setupDir = path.join(testDir, 'setup');
  
  let totalTests = 0;
  let passedTests = 0;
  let failedTests = 0;
  
  // Test files to verify
  const testFiles = [
    {
      path: path.join(integrationDir, 'interface-integration.test.js'),
      name: 'Interface Integration Tests',
      required: true
    },
    {
      path: path.join(integrationDir, 'interface-property.test.js'),
      name: 'Interface Property Tests',
      required: true
    },
    {
      path: path.join(integrationDir, 'enhanced-interface-tests.js'),
      name: 'Enhanced Interface Tests',
      required: false
    }
  ];
  
  // Setup files to verify
  const setupFiles = [
    {
      path: path.join(setupDir, 'test-setup.js'),
      name: 'Test Setup',
      required: true
    },
    {
      path: path.join(setupDir, 'custom-matchers.js'),
      name: 'Custom Matchers',
      required: true
    }
  ];
  
  // Configuration files
  const configFiles = [
    {
      path: path.join(testDir, 'jest.config.js'),
      name: 'Jest Configuration',
      required: true
    },
    {
      path: path.join(testDir, '..', 'package.json'),
      name: 'Package Configuration',
      required: true
    }
  ];
  
  // Verify test files
  log('\n📋 Verifying Test Files:', 'yellow');
  testFiles.forEach(file => {
    totalTests++;
    if (fs.existsSync(file.path)) {
      const content = fs.readFileSync(file.path, 'utf8');
      const hasDescribe = content.includes('describe(');
      const hasTest = content.includes('test(') || content.includes('it(');
      
      if (hasDescribe && hasTest) {
        log(`  ✅ ${file.name}`, 'green');
        passedTests++;
      } else {
        log(`  ⚠️  ${file.name} - Missing test structure`, 'yellow');
        if (file.required) failedTests++;
      }
    } else {
      log(`  ❌ ${file.name} - File not found`, 'red');
      if (file.required) failedTests++;
    }
  });
  
  // Verify setup files
  log('\n⚙️  Verifying Setup Files:', 'yellow');
  setupFiles.forEach(file => {
    totalTests++;
    if (fs.existsSync(file.path)) {
      const content = fs.readFileSync(file.path, 'utf8');
      const hasExports = content.includes('export') || content.includes('module.exports');
      
      if (hasExports || content.length > 100) {
        log(`  ✅ ${file.name}`, 'green');
        passedTests++;
      } else {
        log(`  ⚠️  ${file.name} - Minimal content`, 'yellow');
        if (file.required) failedTests++;
      }
    } else {
      log(`  ❌ ${file.name} - File not found`, 'red');
      if (file.required) failedTests++;
    }
  });
  
  // Verify configuration files
  log('\n🔧 Verifying Configuration Files:', 'yellow');
  configFiles.forEach(file => {
    totalTests++;
    if (fs.existsSync(file.path)) {
      log(`  ✅ ${file.name}`, 'green');
      passedTests++;
    } else {
      log(`  ❌ ${file.name} - File not found`, 'red');
      if (file.required) failedTests++;
    }
  });
  
  // Verify test runner
  log('\n🏃 Verifying Test Runner:', 'yellow');
  const testRunnerPath = path.join(testDir, 'test-runner.html');
  totalTests++;
  if (fs.existsSync(testRunnerPath)) {
    const content = fs.readFileSync(testRunnerPath, 'utf8');
    if (content.includes('<!DOCTYPE html>') && content.includes('test')) {
      log(`  ✅ Interactive Test Runner`, 'green');
      passedTests++;
    } else {
      log(`  ⚠️  Interactive Test Runner - Invalid HTML`, 'yellow');
    }
  } else {
    log(`  ❌ Interactive Test Runner - File not found`, 'red');
    failedTests++;
  }
  
  // Analyze test content
  log('\n🔍 Analyzing Test Content:', 'yellow');
  
  const integrationTestPath = path.join(integrationDir, 'interface-integration.test.js');
  if (fs.existsSync(integrationTestPath)) {
    const content = fs.readFileSync(integrationTestPath, 'utf8');
    
    // Count test cases
    const testCases = (content.match(/test\(/g) || []).length;
    const describeSuites = (content.match(/describe\(/g) || []).length;
    
    log(`  📊 Integration Tests: ${testCases} test cases in ${describeSuites} suites`, 'blue');
    
    // Check for specific test categories
    const hasNavigationTests = content.includes('Navigation Tests');
    const hasLanguageTests = content.includes('Language Switching Tests') || content.includes('Language Tests');
    const hasResponsiveTests = content.includes('Responsive Design Tests') || content.includes('Responsive Tests');
    
    log(`  🧭 Navigation Tests: ${hasNavigationTests ? '✅' : '❌'}`, hasNavigationTests ? 'green' : 'red');
    log(`  🌐 Language Tests: ${hasLanguageTests ? '✅' : '❌'}`, hasLanguageTests ? 'green' : 'red');
    log(`  📱 Responsive Tests: ${hasResponsiveTests ? '✅' : '❌'}`, hasResponsiveTests ? 'green' : 'red');
  }
  
  const propertyTestPath = path.join(integrationDir, 'interface-property.test.js');
  if (fs.existsSync(propertyTestPath)) {
    const content = fs.readFileSync(propertyTestPath, 'utf8');
    
    // Check for property-based testing
    const hasFastCheck = content.includes('fast-check') || content.includes('fc.');
    const hasProperties = content.includes('fc.property') || content.includes('fc.assert');
    
    log(`  🎲 Property-Based Testing: ${hasFastCheck && hasProperties ? '✅' : '❌'}`, 
         hasFastCheck && hasProperties ? 'green' : 'red');
    
    // Count properties
    const properties = (content.match(/fc\.property/g) || []).length;
    log(`  📈 Properties Tested: ${properties}`, 'blue');
  }
  
  // Summary
  log('\n' + '='.repeat(50), 'blue');
  log('📊 Test Verification Summary:', 'blue');
  log(`  Total Checks: ${totalTests}`, 'blue');
  log(`  Passed: ${passedTests}`, 'green');
  log(`  Failed: ${failedTests}`, failedTests > 0 ? 'red' : 'green');
  log(`  Success Rate: ${Math.round((passedTests / totalTests) * 100)}%`, 
       failedTests === 0 ? 'green' : 'yellow');
  
  if (failedTests === 0) {
    log('\n🎉 All test files are properly structured and ready!', 'green');
    log('💡 To run the tests:', 'blue');
    log('   1. Install dependencies: npm install', 'blue');
    log('   2. Run tests: npm test', 'blue');
    log('   3. Open test runner: npm run test:runner', 'blue');
  } else {
    log('\n⚠️  Some issues were found. Please check the failed items above.', 'yellow');
  }
  
  return failedTests === 0;
}

// Run verification
if (require.main === module) {
  const success = verifyTestFiles();
  process.exit(success ? 0 : 1);
}

module.exports = { verifyTestFiles };