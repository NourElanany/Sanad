/**
 * Jest Configuration for Sanad Interface Tests
 * Comprehensive testing setup for integration and property-based tests
 */

module.exports = {
  // Test environment
  testEnvironment: 'jsdom',
  
  // Test file patterns
  testMatch: [
    '<rootDir>/tests/**/*.test.js',
    '<rootDir>/tests/**/*.spec.js'
  ],
  
  // Setup files
  setupFilesAfterEnv: [
    '<rootDir>/tests/setup/test-setup.js'
  ],
  
  // Module paths
  moduleNameMapping: {
    '^@/(.*)$': '<rootDir>/js/$1',
    '^@tests/(.*)$': '<rootDir>/tests/$1'
  },
  
  // Coverage configuration
  collectCoverage: true,
  collectCoverageFrom: [
    'js/**/*.js',
    '!js/config.js',
    '!**/node_modules/**',
    '!**/vendor/**'
  ],
  coverageDirectory: '<rootDir>/tests/coverage',
  coverageReporters: [
    'text',
    'lcov',
    'html',
    'json-summary'
  ],
  coverageThreshold: {
    global: {
      branches: 80,
      functions: 80,
      lines: 80,
      statements: 80
    }
  },
  
  // Test timeout
  testTimeout: 30000,
  
  // Verbose output
  verbose: true,
  
  // Transform files
  transform: {
    '^.+\\.js$': 'babel-jest'
  },
  
  // Module file extensions
  moduleFileExtensions: [
    'js',
    'json',
    'html'
  ],
  
  // Global variables
  globals: {
    'window': {},
    'document': {},
    'navigator': {},
    'localStorage': {},
    'sessionStorage': {}
  },
  
  // Test reporters
  reporters: [
    'default',
    [
      'jest-html-reporters',
      {
        publicPath: '<rootDir>/tests/reports',
        filename: 'test-report.html',
        expand: true,
        hideIcon: false,
        pageTitle: 'Sanad Interface Tests Report',
        logoImgPath: undefined,
        inlineSource: false
      }
    ]
  ],
  
  // Clear mocks between tests
  clearMocks: true,
  
  // Restore mocks after each test
  restoreMocks: true,
  
  // Reset modules between tests
  resetModules: true,
  
  // Error handling
  errorOnDeprecated: true,
  
  // Notify mode
  notify: false,
  
  // Watch mode configuration
  watchman: true,
  
  // Test result processor
  testResultsProcessor: undefined,
  
  // Custom matchers
  setupFilesAfterEnv: [
    '<rootDir>/tests/setup/test-setup.js',
    '<rootDir>/tests/setup/custom-matchers.js'
  ],
  
  // Test suites
  projects: [
    {
      displayName: 'Integration Tests',
      testMatch: ['<rootDir>/tests/integration/**/*.test.js'],
      setupFilesAfterEnv: ['<rootDir>/tests/setup/integration-setup.js']
    },
    {
      displayName: 'Property-Based Tests',
      testMatch: ['<rootDir>/tests/integration/**/*property*.test.js'],
      setupFilesAfterEnv: ['<rootDir>/tests/setup/property-setup.js']
    },
    {
      displayName: 'Enhanced Tests',
      testMatch: ['<rootDir>/tests/integration/enhanced-*.test.js'],
      setupFilesAfterEnv: ['<rootDir>/tests/setup/enhanced-setup.js']
    }
  ]
};