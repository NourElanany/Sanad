// Sentry error tracking and performance monitoring

import * as Sentry from '@sentry/nextjs';

export const initSentry = () => {
  if (process.env.NEXT_PUBLIC_SENTRY_DSN) {
    Sentry.init({
      dsn: process.env.NEXT_PUBLIC_SENTRY_DSN,
      environment: process.env.NEXT_PUBLIC_APP_ENV || 'development',
      
      // Performance Monitoring
      tracesSampleRate: process.env.NODE_ENV === 'production' ? 0.1 : 1.0,
      
      // Session Replay
      replaysSessionSampleRate: 0.1,
      replaysOnErrorSampleRate: 1.0,
      
      // Integrations
      integrations: [
        new Sentry.BrowserTracing({
          tracePropagationTargets: [
            'localhost',
            /^https:\/\/.*\.sanad\.app/,
          ],
        }),
        new Sentry.Replay({
          maskAllText: true,
          blockAllMedia: true,
        }),
      ],
      
      // Filter out certain errors
      beforeSend(event, hint) {
        // Don't send errors in development
        if (process.env.NODE_ENV === 'development') {
          return null;
        }
        
        // Filter out network errors
        if (event.exception?.values?.[0]?.type === 'NetworkError') {
          return null;
        }
        
        return event;
      },
    });
  }
};

// Custom error tracking functions
export const captureException = (error: Error, context?: Record<string, any>) => {
  Sentry.captureException(error, {
    extra: context,
  });
};

export const captureMessage = (message: string, level: Sentry.SeverityLevel = 'info') => {
  Sentry.captureMessage(message, level);
};

export const setUser = (userId: string, email?: string) => {
  Sentry.setUser({
    id: userId,
    email: email,
  });
};

export const clearUser = () => {
  Sentry.setUser(null);
};

// Performance monitoring
export const startTransaction = (name: string, op: string) => {
  return Sentry.startTransaction({
    name,
    op,
  });
};

export const addBreadcrumb = (message: string, category: string, data?: Record<string, any>) => {
  Sentry.addBreadcrumb({
    message,
    category,
    data,
    level: 'info',
  });
};
