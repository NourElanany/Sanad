/**
 * Animation utilities for smooth 60fps animations
 * Provides standard animation configurations and helpers
 */

// Standard animation durations (in milliseconds)
export const ANIMATION_DURATION = {
  fast: 150,
  normal: 300,
  slow: 500,
} as const;

// Standard easing functions
export const EASING = {
  easeInOut: 'cubic-bezier(0.4, 0.0, 0.2, 1)',
  easeOut: 'cubic-bezier(0.0, 0.0, 0.2, 1)',
  easeIn: 'cubic-bezier(0.4, 0.0, 1, 1)',
  sharp: 'cubic-bezier(0.4, 0.0, 0.6, 1)',
  emphasized: 'cubic-bezier(0.2, 0.0, 0, 1)',
} as const;

// Framer Motion variants for common animations
export const fadeInVariants = {
  hidden: { opacity: 0 },
  visible: {
    opacity: 1,
    transition: {
      duration: ANIMATION_DURATION.normal / 1000,
      ease: [0.4, 0.0, 0.2, 1],
    },
  },
};

export const slideUpVariants = {
  hidden: { opacity: 0, y: 20 },
  visible: {
    opacity: 1,
    y: 0,
    transition: {
      duration: ANIMATION_DURATION.normal / 1000,
      ease: [0.2, 0.0, 0, 1],
    },
  },
};

export const slideDownVariants = {
  hidden: { opacity: 0, y: -20 },
  visible: {
    opacity: 1,
    y: 0,
    transition: {
      duration: ANIMATION_DURATION.normal / 1000,
      ease: [0.2, 0.0, 0, 1],
    },
  },
};

export const slideLeftVariants = {
  hidden: { opacity: 0, x: 20 },
  visible: {
    opacity: 1,
    x: 0,
    transition: {
      duration: ANIMATION_DURATION.normal / 1000,
      ease: [0.2, 0.0, 0, 1],
    },
  },
};

export const slideRightVariants = {
  hidden: { opacity: 0, x: -20 },
  visible: {
    opacity: 1,
    x: 0,
    transition: {
      duration: ANIMATION_DURATION.normal / 1000,
      ease: [0.2, 0.0, 0, 1],
    },
  },
};

export const scaleVariants = {
  hidden: { opacity: 0, scale: 0.95 },
  visible: {
    opacity: 1,
    scale: 1,
    transition: {
      duration: ANIMATION_DURATION.normal / 1000,
      ease: [0.2, 0.0, 0, 1],
    },
  },
};

export const staggerContainerVariants = {
  hidden: { opacity: 0 },
  visible: {
    opacity: 1,
    transition: {
      staggerChildren: 0.05,
      delayChildren: 0.1,
    },
  },
};

export const staggerItemVariants = {
  hidden: { opacity: 0, y: 20 },
  visible: {
    opacity: 1,
    y: 0,
    transition: {
      duration: ANIMATION_DURATION.normal / 1000,
      ease: [0.2, 0.0, 0, 1],
    },
  },
};

/**
 * Create a staggered animation for list items
 */
export function createStaggerAnimation(itemCount: number, maxStagger: number = 5) {
  return {
    container: {
      hidden: { opacity: 0 },
      visible: {
        opacity: 1,
        transition: {
          staggerChildren: 0.05,
          delayChildren: 0,
        },
      },
    },
    item: (index: number) => ({
      hidden: { opacity: 0, y: 20 },
      visible: {
        opacity: 1,
        y: 0,
        transition: {
          duration: ANIMATION_DURATION.normal / 1000,
          delay: index < maxStagger ? index * 0.05 : 0,
          ease: [0.2, 0.0, 0, 1],
        },
      },
    }),
  };
}

/**
 * CSS transition helper
 */
export function createTransition(
  properties: string[],
  duration: number = ANIMATION_DURATION.normal,
  easing: string = EASING.easeInOut
): string {
  return properties
    .map((prop) => `${prop} ${duration}ms ${easing}`)
    .join(', ');
}

/**
 * Check if user prefers reduced motion
 */
export function prefersReducedMotion(): boolean {
  if (typeof window === 'undefined') return false;
  return window.matchMedia('(prefers-reduced-motion: reduce)').matches;
}

/**
 * Get adjusted duration based on user preferences
 */
export function getAdjustedDuration(baseDuration: number): number {
  return prefersReducedMotion() ? 0 : baseDuration;
}

/**
 * Smooth scroll to element
 */
export function smoothScrollTo(
  element: HTMLElement | null,
  options?: ScrollIntoViewOptions
) {
  if (!element) return;

  const defaultOptions: ScrollIntoViewOptions = {
    behavior: prefersReducedMotion() ? 'auto' : 'smooth',
    block: 'start',
    inline: 'nearest',
  };

  element.scrollIntoView({ ...defaultOptions, ...options });
}

/**
 * Animate value with requestAnimationFrame
 */
export function animateValue(
  from: number,
  to: number,
  duration: number,
  onUpdate: (value: number) => void,
  easing: (t: number) => number = (t) => t
): () => void {
  const startTime = performance.now();
  let animationFrame: number;

  const animate = (currentTime: number) => {
    const elapsed = currentTime - startTime;
    const progress = Math.min(elapsed / duration, 1);
    const easedProgress = easing(progress);
    const currentValue = from + (to - from) * easedProgress;

    onUpdate(currentValue);

    if (progress < 1) {
      animationFrame = requestAnimationFrame(animate);
    }
  };

  animationFrame = requestAnimationFrame(animate);

  // Return cancel function
  return () => cancelAnimationFrame(animationFrame);
}

/**
 * Easing functions
 */
export const easingFunctions = {
  linear: (t: number) => t,
  easeInQuad: (t: number) => t * t,
  easeOutQuad: (t: number) => t * (2 - t),
  easeInOutQuad: (t: number) => (t < 0.5 ? 2 * t * t : -1 + (4 - 2 * t) * t),
  easeInCubic: (t: number) => t * t * t,
  easeOutCubic: (t: number) => --t * t * t + 1,
  easeInOutCubic: (t: number) =>
    t < 0.5 ? 4 * t * t * t : (t - 1) * (2 * t - 2) * (2 * t - 2) + 1,
};

/**
 * Debounce animation frame
 */
export function debounceAnimationFrame(
  callback: (...args: any[]) => void
): (...args: any[]) => void {
  let rafId: number | null = null;

  return (...args: any[]) => {
    if (rafId !== null) {
      cancelAnimationFrame(rafId);
    }

    rafId = requestAnimationFrame(() => {
      callback(...args);
      rafId = null;
    });
  };
}

/**
 * Throttle with requestAnimationFrame
 */
export function throttleAnimationFrame(
  callback: (...args: any[]) => void
): (...args: any[]) => void {
  let rafId: number | null = null;
  let lastArgs: any[] | null = null;

  const throttled = () => {
    if (lastArgs !== null) {
      callback(...lastArgs);
      lastArgs = null;
      rafId = requestAnimationFrame(throttled);
    } else {
      rafId = null;
    }
  };

  return (...args: any[]) => {
    lastArgs = args;
    if (rafId === null) {
      rafId = requestAnimationFrame(throttled);
    }
  };
}
