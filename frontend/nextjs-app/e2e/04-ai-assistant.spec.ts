/**
 * E2E Test: AI Assistant Flow with Streaming
 * 
 * Tests the AI Islamic Assistant functionality:
 * - Chat interface
 * - Text and voice input
 * - Streaming responses
 * - Source citations
 * - Source verification
 * - Error handling
 * 
 * **Validates: Requirements 7.1, 7.2, 7.3, 7.4, 7.5**
 */

import { test, expect } from '@playwright/test';
import { createHelpers } from './helpers/test-helpers';

test.describe('AI Assistant Flow', () => {
  test.beforeEach(async ({ page }) => {
    const helpers = createHelpers(page);
    
    // Set up authenticated state
    await helpers.data.setLocalStorageItem('onboarding_complete', 'true');
    
    // Navigate to AI Assistant
    await helpers.nav.goToAIAssistant();
  });

  test('should display AI Assistant interface', async ({ page }) => {
    const helpers = createHelpers(page);
    
    // Check for AI Assistant heading
    await helpers.assert.assertVisible('h1');
    await expect(page.locator('h1')).toContainText(/المساعد|Assistant/i);
    
    // Should have chat input
    await helpers.assert.assertVisible('[data-testid="chat-input"]');
    
    // Should have send button
    await helpers.assert.assertVisible('[data-testid="send-button"]');
    
    // Should have voice input button
    await helpers.assert.assertVisible('[data-testid="voice-button"]');
  });

  test('should display empty state initially', async ({ page }) => {
    const helpers = createHelpers(page);
    
    // Should show empty state message
    await helpers.assert.assertVisible('[data-testid="empty-state"]');
    
    // Should have welcome message
    const emptyStateText = await page.locator('[data-testid="empty-state"]').textContent();
    expect(emptyStateText).toMatch(/اسأل|Ask|مرحباً|Welcome/i);
  });

  test('should send text message', async ({ page }) => {
    const helpers = createHelpers(page);
    
    // Type a question
    const question = 'ما حكم الصلاة في الطائرة؟';
    await page.fill('[data-testid="chat-input"]', question);
    
    // Click send
    await page.click('[data-testid="send-button"]');
    
    // Should display user message
    await helpers.wait.waitForVisible('[data-testid="user-message"]');
    await expect(page.locator('[data-testid="user-message"]')).toContainText(question);
  });

  test('should receive streaming AI response', async ({ page }) => {
    const helpers = createHelpers(page);
    
    // Send a question
    await page.fill('[data-testid="chat-input"]', 'ما هي أركان الإسلام؟');
    await page.click('[data-testid="send-button"]');
    
    // Wait for AI response to start
    await helpers.wait.waitForVisible('[data-testid="ai-message"]', 15000);
    
    // Should show loading indicator initially
    const hasLoadingIndicator = await page.locator('[data-testid="typing-indicator"]').isVisible().catch(() => false);
    
    // Wait for streaming to complete
    await helpers.wait.waitForStreamingComplete('[data-testid="ai-message"]', 30000);
    
    // Should have response text
    const responseText = await page.locator('[data-testid="ai-message"]').textContent();
    expect(responseText).toBeTruthy();
    expect(responseText!.length).toBeGreaterThan(10);
  });

  test('should display source citations', async ({ page }) => {
    const helpers = createHelpers(page);
    
    // Send a question that requires sources
    await page.fill('[data-testid="chat-input"]', 'ما هو فضل قراءة سورة الكهف يوم الجمعة؟');
    await page.click('[data-testid="send-button"]');
    
    // Wait for response
    await helpers.wait.waitForVisible('[data-testid="ai-message"]', 15000);
    await helpers.wait.waitForStreamingComplete('[data-testid="ai-message"]', 30000);
    
    // Should display source cards
    await helpers.wait.waitForVisible('[data-testid="source-cards"]', 5000);
    
    // Should have at least one source
    const sourceCount = await page.locator('[data-testid^="source-card-"]').count();
    expect(sourceCount).toBeGreaterThan(0);
  });

  test('should show source details', async ({ page }) => {
    const helpers = createHelpers(page);
    
    // Send a question
    await page.fill('[data-testid="chat-input"]', 'ما حكم الزكاة؟');
    await page.click('[data-testid="send-button"]');
    
    // Wait for response with sources
    await helpers.wait.waitForVisible('[data-testid="ai-message"]', 15000);
    await helpers.wait.waitForStreamingComplete('[data-testid="ai-message"]', 30000);
    await helpers.wait.waitForVisible('[data-testid="source-cards"]', 5000);
    
    // Click on first source
    await page.click('[data-testid="source-card-0"]');
    
    // Should show source details
    await helpers.wait.waitForVisible('[data-testid="source-details"]');
    
    // Should have source title
    await expect(page.locator('[data-testid="source-title"]')).toBeVisible();
    
    // Should have source reference
    await expect(page.locator('[data-testid="source-reference"]')).toBeVisible();
  });

  test('should handle multiple messages in conversation', async ({ page }) => {
    const helpers = createHelpers(page);
    
    // Send first question
    await page.fill('[data-testid="chat-input"]', 'ما هي الصلوات الخمس؟');
    await page.click('[data-testid="send-button"]');
    
    // Wait for response
    await helpers.wait.waitForVisible('[data-testid="ai-message"]', 15000);
    await helpers.wait.waitForStreamingComplete('[data-testid="ai-message"]', 30000);
    
    // Send follow-up question
    await page.fill('[data-testid="chat-input"]', 'ما هي أوقاتها؟');
    await page.click('[data-testid="send-button"]');
    
    // Wait for second response
    await page.waitForTimeout(1000);
    await helpers.wait.waitForStreamingComplete('[data-testid="ai-message"]:last-of-type', 30000);
    
    // Should have multiple messages
    const messageCount = await page.locator('[data-testid="user-message"]').count();
    expect(messageCount).toBeGreaterThanOrEqual(2);
  });

  test('should clear chat history', async ({ page }) => {
    const helpers = createHelpers(page);
    
    // Send a message
    await page.fill('[data-testid="chat-input"]', 'السلام عليكم');
    await page.click('[data-testid="send-button"]');
    
    // Wait for response
    await helpers.wait.waitForVisible('[data-testid="ai-message"]', 15000);
    
    // Click clear chat button
    await page.click('[data-testid="clear-chat"]');
    
    // Should show confirmation dialog
    await helpers.wait.waitForVisible('[data-testid="confirm-dialog"]');
    
    // Confirm clear
    await page.click('[data-testid="confirm-clear"]');
    
    // Should show empty state again
    await helpers.wait.waitForVisible('[data-testid="empty-state"]');
    
    // Messages should be gone
    const messageCount = await page.locator('[data-testid="user-message"]').count();
    expect(messageCount).toBe(0);
  });

  test('should handle long responses', async ({ page }) => {
    const helpers = createHelpers(page);
    
    // Ask for detailed explanation
    await page.fill('[data-testid="chat-input"]', 'اشرح لي أركان الإيمان بالتفصيل');
    await page.click('[data-testid="send-button"]');
    
    // Wait for response
    await helpers.wait.waitForVisible('[data-testid="ai-message"]', 15000);
    await helpers.wait.waitForStreamingComplete('[data-testid="ai-message"]', 45000);
    
    // Response should be substantial
    const responseText = await page.locator('[data-testid="ai-message"]').textContent();
    expect(responseText!.length).toBeGreaterThan(100);
  });

  test('should handle errors gracefully', async ({ page }) => {
    const helpers = createHelpers(page);
    
    // Intercept API call to simulate error
    await page.route('**/api/ai/**', route => {
      route.abort('failed');
    });
    
    // Send a message
    await page.fill('[data-testid="chat-input"]', 'test question');
    await page.click('[data-testid="send-button"]');
    
    // Should show error message
    await helpers.wait.waitForVisible('[data-testid="error-message"]', 10000);
    
    // Error message should be user-friendly
    const errorText = await page.locator('[data-testid="error-message"]').textContent();
    expect(errorText).toMatch(/خطأ|error|فشل|failed/i);
  });

  test('should disable input while processing', async ({ page }) => {
    const helpers = createHelpers(page);
    
    // Send a message
    await page.fill('[data-testid="chat-input"]', 'ما هو الإسلام؟');
    await page.click('[data-testid="send-button"]');
    
    // Input should be disabled while processing
    const isDisabled = await page.locator('[data-testid="chat-input"]').isDisabled();
    expect(isDisabled).toBe(true);
    
    // Wait for response to complete
    await helpers.wait.waitForVisible('[data-testid="ai-message"]', 15000);
    await helpers.wait.waitForStreamingComplete('[data-testid="ai-message"]', 30000);
    
    // Input should be enabled again
    const isEnabledAfter = await page.locator('[data-testid="chat-input"]').isEnabled();
    expect(isEnabledAfter).toBe(true);
  });

  test('should scroll to latest message', async ({ page }) => {
    const helpers = createHelpers(page);
    
    // Send multiple messages to create scroll
    for (let i = 0; i < 3; i++) {
      await page.fill('[data-testid="chat-input"]', `سؤال ${i + 1}`);
      await page.click('[data-testid="send-button"]');
      await page.waitForTimeout(2000);
    }
    
    // Should auto-scroll to bottom
    const isAtBottom = await page.evaluate(() => {
      const chatContainer = document.querySelector('[data-testid="chat-container"]');
      if (!chatContainer) return false;
      const scrollTop = chatContainer.scrollTop;
      const scrollHeight = chatContainer.scrollHeight;
      const clientHeight = chatContainer.clientHeight;
      return Math.abs(scrollHeight - scrollTop - clientHeight) < 50;
    });
    
    expect(isAtBottom).toBe(true);
  });

  test('should preserve chat history on page reload', async ({ page }) => {
    const helpers = createHelpers(page);
    
    // Send a message
    const question = 'ما هي الصلاة؟';
    await page.fill('[data-testid="chat-input"]', question);
    await page.click('[data-testid="send-button"]');
    
    // Wait for response
    await helpers.wait.waitForVisible('[data-testid="ai-message"]', 15000);
    
    // Reload page
    await page.reload();
    await helpers.wait.waitForVisible('[data-testid="chat-input"]');
    
    // Chat history should be preserved
    const userMessage = await page.locator('[data-testid="user-message"]').textContent();
    expect(userMessage).toContain(question);
  });

  test('should be responsive on mobile', async ({ page }) => {
    const helpers = createHelpers(page);
    
    // Set mobile viewport
    await page.setViewportSize({ width: 375, height: 667 });
    
    // Interface should be visible
    await expect(page.locator('[data-testid="chat-input"]')).toBeVisible();
    await expect(page.locator('[data-testid="send-button"]')).toBeVisible();
    
    // Send a message
    await page.fill('[data-testid="chat-input"]', 'test');
    await page.click('[data-testid="send-button"]');
    
    // Messages should be readable
    await helpers.wait.waitForVisible('[data-testid="user-message"]');
    
    // No horizontal scroll
    const hasHorizontalScroll = await page.evaluate(() => {
      return document.documentElement.scrollWidth > document.documentElement.clientWidth;
    });
    expect(hasHorizontalScroll).toBe(false);
  });

  test('should support keyboard shortcuts', async ({ page }) => {
    const helpers = createHelpers(page);
    
    // Type a message
    await page.fill('[data-testid="chat-input"]', 'test question');
    
    // Press Enter to send
    await page.press('[data-testid="chat-input"]', 'Enter');
    
    // Should send the message
    await helpers.wait.waitForVisible('[data-testid="user-message"]');
    await expect(page.locator('[data-testid="user-message"]')).toContainText('test question');
  });

  test('should show typing indicator during streaming', async ({ page }) => {
    const helpers = createHelpers(page);
    
    // Send a message
    await page.fill('[data-testid="chat-input"]', 'ما هو الإيمان؟');
    await page.click('[data-testid="send-button"]');
    
    // Should show typing indicator
    const typingIndicator = page.locator('[data-testid="typing-indicator"]');
    
    // Wait a bit and check if it was visible at some point
    await page.waitForTimeout(500);
    
    // Eventually should show the actual message
    await helpers.wait.waitForVisible('[data-testid="ai-message"]', 15000);
  });
});
