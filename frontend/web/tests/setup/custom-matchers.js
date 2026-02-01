/**
 * Custom Jest Matchers for Sanad Interface Tests
 * Specialized matchers for UI testing scenarios
 */

// Extend Jest matchers
expect.extend({
  /**
   * Check if element is visible in viewport
   */
  toBeVisibleInViewport(received) {
    if (!received || !received.getBoundingClientRect) {
      return {
        message: () => `Expected element to be a DOM element`,
        pass: false
      };
    }
    
    const rect = received.getBoundingClientRect();
    const isVisible = rect.top >= 0 && 
                     rect.left >= 0 && 
                     rect.bottom <= window.innerHeight && 
                     rect.right <= window.innerWidth;
    
    return {
      message: () => isVisible 
        ? `Expected element not to be visible in viewport`
        : `Expected element to be visible in viewport`,
      pass: isVisible
    };
  },
  
  /**
   * Check if element has correct responsive behavior
   */
  toBeResponsive(received, breakpoint) {
    if (!received || !received.getBoundingClientRect) {
      return {
        message: () => `Expected element to be a DOM element`,
        pass: false
      };
    }
    
    const rect = received.getBoundingClientRect();
    const style = getComputedStyle(received);
    
    let isResponsive = true;
    let failureReason = '';
    
    // Check if element adapts to breakpoint
    switch (breakpoint) {
      case 'mobile':
        if (rect.width > window.innerWidth) {
          isResponsive = false;
          failureReason = 'Element width exceeds viewport width on mobile';
        }
        break;
      case 'tablet':
        if (style.display === 'none' && !received.classList.contains('mobile-only')) {
          isResponsive = false;
          failureReason = 'Element hidden on tablet without mobile-only class';
        }
        break;
      case 'desktop':
        if (style.display === 'none' && !received.classList.contains('mobile-only')) {
          isResponsive = false;
          failureReason = 'Element hidden on desktop without mobile-only class';
        }
        break;
    }
    
    return {
      message: () => isResponsive 
        ? `Expected element not to be responsive for ${breakpoint}`
        : `Expected element to be responsive for ${breakpoint}: ${failureReason}`,
      pass: isResponsive
    };
  },
  
  /**
   * Check if element has correct RTL/LTR behavior
   */
  toHaveCorrectTextDirection(received, expectedDirection) {
    if (!received) {
      return {
        message: () => `Expected element to be a DOM element`,
        pass: false
      };
    }
    
    const style = getComputedStyle(received);
    const direction = style.direction || 'ltr';
    const textAlign = style.textAlign;
    
    let isCorrect = direction === expectedDirection;
    
    // Additional checks for RTL
    if (expectedDirection === 'rtl') {
      isCorrect = isCorrect && (textAlign === 'right' || textAlign === 'start');
    } else if (expectedDirection === 'ltr') {
      isCorrect = isCorrect && (textAlign === 'left' || textAlign === 'start' || textAlign === 'initial');
    }
    
    return {
      message: () => isCorrect 
        ? `Expected element not to have ${expectedDirection} text direction`
        : `Expected element to have ${expectedDirection} text direction, but got ${direction} with text-align: ${textAlign}`,
      pass: isCorrect
    };
  },
  
  /**
   * Check if element meets accessibility standards
   */
  toBeAccessible(received) {
    if (!received) {
      return {
        message: () => `Expected element to be a DOM element`,
        pass: false
      };
    }
    
    const issues = [];
    
    // Check for minimum touch target size on interactive elements
    if (received.matches('button, a, input, select, textarea, [role="button"]')) {
      const rect = received.getBoundingClientRect();
      if (rect.width < 44 || rect.height < 44) {
        issues.push('Touch target too small (minimum 44x44px)');
      }
    }
    
    // Check for alt text on images
    if (received.tagName === 'IMG' && !received.getAttribute('alt')) {
      issues.push('Image missing alt attribute');
    }
    
    // Check for form labels
    if (received.matches('input, select, textarea') && !received.getAttribute('aria-label') && !received.getAttribute('aria-labelledby')) {
      const id = received.getAttribute('id');
      if (!id || !document.querySelector(`label[for="${id}"]`)) {
        issues.push('Form control missing label');
      }
    }
    
    // Check for heading hierarchy
    if (received.matches('h1, h2, h3, h4, h5, h6')) {
      const level = parseInt(received.tagName.charAt(1));
      const prevHeading = received.previousElementSibling?.matches?.('h1, h2, h3, h4, h5, h6');
      if (prevHeading) {
        const prevLevel = parseInt(prevHeading.tagName.charAt(1));
        if (level > prevLevel + 1) {
          issues.push('Heading hierarchy skipped levels');
        }
      }
    }
    
    // Check for focus indicators
    if (received.matches('button, a, input, select, textarea, [tabindex]')) {
      const style = getComputedStyle(received, ':focus');
      if (!style.outline || style.outline === 'none') {
        issues.push('Missing focus indicator');
      }
    }
    
    const isAccessible = issues.length === 0;
    
    return {
      message: () => isAccessible 
        ? `Expected element not to be accessible`
        : `Expected element to be accessible, but found issues: ${issues.join(', ')}`,
      pass: isAccessible
    };
  },
  
  /**
   * Check if navigation state is consistent
   */
  toHaveConsistentNavigationState(received, expectedSection) {
    const activeSection = document.querySelector('.content-section.active');
    const activeNavLink = document.querySelector('.nav-link.active');
    const appState = window.SanadApp?.state?.currentSection;
    
    const issues = [];
    
    if (!activeSection) {
      issues.push('No active section found');
    } else if (activeSection.id !== expectedSection) {
      issues.push(`Active section is ${activeSection.id}, expected ${expectedSection}`);
    }
    
    if (!activeNavLink) {
      issues.push('No active nav link found');
    } else if (activeNavLink.getAttribute('data-section') !== expectedSection) {
      issues.push(`Active nav link is for ${activeNavLink.getAttribute('data-section')}, expected ${expectedSection}`);
    }
    
    if (appState && appState !== expectedSection) {
      issues.push(`App state is ${appState}, expected ${expectedSection}`);
    }
    
    const isConsistent = issues.length === 0;
    
    return {
      message: () => isConsistent 
        ? `Expected navigation state not to be consistent`
        : `Expected consistent navigation state for ${expectedSection}, but found issues: ${issues.join(', ')}`,
      pass: isConsistent
    };
  },
  
  /**
   * Check if language state is consistent
   */
  toHaveConsistentLanguageState(received, expectedLanguage) {
    const htmlLang = document.documentElement.getAttribute('lang');
    const bodyClass = document.body.className;
    const langToggle = document.getElementById('langToggle');
    const activeOption = document.querySelector('.lang-option.active');
    const appState = window.SanadApp?.state?.currentLanguage;
    
    const issues = [];
    
    if (htmlLang !== expectedLanguage) {
      issues.push(`HTML lang is ${htmlLang}, expected ${expectedLanguage}`);
    }
    
    if (!bodyClass.includes(`lang-${expectedLanguage}`)) {
      issues.push(`Body class missing lang-${expectedLanguage}`);
    }
    
    if (langToggle && window.SanadConfig?.languages?.[expectedLanguage]) {
      const expectedName = window.SanadConfig.languages[expectedLanguage].name;
      if (langToggle.textContent !== expectedName) {
        issues.push(`Language toggle shows ${langToggle.textContent}, expected ${expectedName}`);
      }
    }
    
    if (activeOption && activeOption.getAttribute('data-lang') !== expectedLanguage) {
      issues.push(`Active language option is ${activeOption.getAttribute('data-lang')}, expected ${expectedLanguage}`);
    }
    
    if (appState && appState !== expectedLanguage) {
      issues.push(`App state is ${appState}, expected ${expectedLanguage}`);
    }
    
    const isConsistent = issues.length === 0;
    
    return {
      message: () => isConsistent 
        ? `Expected language state not to be consistent`
        : `Expected consistent language state for ${expectedLanguage}, but found issues: ${issues.join(', ')}`,
      pass: isConsistent
    };
  },
  
  /**
   * Check if element has proper loading state
   */
  toHaveLoadingState(received) {
    if (!received) {
      return {
        message: () => `Expected element to be a DOM element`,
        pass: false
      };
    }
    
    const hasLoadingClass = received.classList.contains('loading');
    const hasSpinner = received.querySelector('.spinner, .loading-spinner');
    const hasLoadingText = received.textContent.includes('جاري التحميل') || 
                          received.textContent.includes('Loading');
    const isDisabled = received.disabled || received.getAttribute('aria-disabled') === 'true';
    
    const hasLoadingState = hasLoadingClass || hasSpinner || hasLoadingText || isDisabled;
    
    return {
      message: () => hasLoadingState 
        ? `Expected element not to have loading state`
        : `Expected element to have loading state (loading class, spinner, loading text, or disabled state)`,
      pass: hasLoadingState
    };
  },
  
  /**
   * Check if element has proper error state
   */
  toHaveErrorState(received, expectedErrorMessage) {
    if (!received) {
      return {
        message: () => `Expected element to be a DOM element`,
        pass: false
      };
    }
    
    const hasErrorClass = received.classList.contains('error') || received.classList.contains('has-error');
    const hasErrorMessage = received.querySelector('.error-message, .error-text');
    const hasAriaInvalid = received.getAttribute('aria-invalid') === 'true';
    
    let hasCorrectMessage = true;
    if (expectedErrorMessage && hasErrorMessage) {
      hasCorrectMessage = hasErrorMessage.textContent.includes(expectedErrorMessage);
    }
    
    const hasErrorState = hasErrorClass || hasErrorMessage || hasAriaInvalid;
    const isValid = hasErrorState && hasCorrectMessage;
    
    return {
      message: () => isValid 
        ? `Expected element not to have error state`
        : `Expected element to have error state${expectedErrorMessage ? ` with message "${expectedErrorMessage}"` : ''}`,
      pass: isValid
    };
  }
});

// Export matchers for TypeScript support
export const customMatchers = {
  toBeVisibleInViewport: expect.toBeVisibleInViewport,
  toBeResponsive: expect.toBeResponsive,
  toHaveCorrectTextDirection: expect.toHaveCorrectTextDirection,
  toBeAccessible: expect.toBeAccessible,
  toHaveConsistentNavigationState: expect.toHaveConsistentNavigationState,
  toHaveConsistentLanguageState: expect.toHaveConsistentLanguageState,
  toHaveLoadingState: expect.toHaveLoadingState,
  toHaveErrorState: expect.toHaveErrorState
};