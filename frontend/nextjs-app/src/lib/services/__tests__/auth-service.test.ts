import { authService, LoginResult } from '../auth-service';

describe('AuthService', () => {
  beforeEach(() => {
    // Clear localStorage before each test
    if (typeof window !== 'undefined') {
      localStorage.clear();
    }
  });

  describe('Token Management', () => {
    it('should save tokens to localStorage', () => {
      const accessToken = 'test_access_token';
      const refreshToken = 'test_refresh_token';

      authService.saveTokens(accessToken, refreshToken);

      expect(localStorage.getItem('access_token')).toBe(accessToken);
      expect(localStorage.getItem('refresh_token')).toBe(refreshToken);
    });

    it('should retrieve access token from localStorage', () => {
      const token = 'test_token';
      localStorage.setItem('access_token', token);

      const retrievedToken = authService.getAccessToken();

      expect(retrievedToken).toBe(token);
    });

    it('should retrieve refresh token from localStorage', () => {
      const token = 'test_refresh_token';
      localStorage.setItem('refresh_token', token);

      const retrievedToken = authService.getRefreshToken();

      expect(retrievedToken).toBe(token);
    });

    it('should return null when no token exists', () => {
      const token = authService.getAccessToken();

      expect(token).toBeNull();
    });
  });

  describe('User ID Management', () => {
    it('should save user ID to localStorage', () => {
      const userId = 'user123';

      authService.saveUserId(userId);

      expect(localStorage.getItem('user_id')).toBe(userId);
    });

    it('should retrieve user ID from localStorage', () => {
      const userId = 'user123';
      localStorage.setItem('user_id', userId);

      const retrievedUserId = authService.getUserId();

      expect(retrievedUserId).toBe(userId);
    });
  });

  describe('Authentication Status', () => {
    it('should return false when no token exists', () => {
      const isAuth = authService.isAuthenticated();

      expect(isAuth).toBe(false);
    });

    it('should return false when token is expired', () => {
      // Create an expired JWT token (exp: 2018)
      const expiredToken = 'eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.' +
        'eyJzdWIiOiIxMjM0NTY3ODkwIiwibmFtZSI6IkpvaG4gRG9lIiwiZXhwIjoxNTE2MjM5MDIyfQ.' +
        'SflKxwRJSMeKKF2QT4fwpMeJf36POk6yJV_adQssw5c';
      
      localStorage.setItem('access_token', expiredToken);

      const isAuth = authService.isAuthenticated();

      expect(isAuth).toBe(false);
    });
  });

  describe('Logout', () => {
    it('should clear all tokens and user data', () => {
      localStorage.setItem('access_token', 'token1');
      localStorage.setItem('refresh_token', 'token2');
      localStorage.setItem('user_id', 'user123');

      authService.logout();

      expect(localStorage.getItem('access_token')).toBeNull();
      expect(localStorage.getItem('refresh_token')).toBeNull();
      expect(localStorage.getItem('user_id')).toBeNull();
    });
  });

  describe('Clear Auth', () => {
    it('should clear all authentication data', () => {
      localStorage.setItem('access_token', 'token1');
      localStorage.setItem('refresh_token', 'token2');
      localStorage.setItem('user_id', 'user123');

      authService.clearAuth();

      expect(localStorage.getItem('access_token')).toBeNull();
      expect(localStorage.getItem('refresh_token')).toBeNull();
      expect(localStorage.getItem('user_id')).toBeNull();
    });
  });
});

describe('LoginResult', () => {
  it('should create successful login result', () => {
    const result: LoginResult = {
      success: true,
      userId: 'user123',
    };

    expect(result.success).toBe(true);
    expect(result.userId).toBe('user123');
    expect(result.error).toBeUndefined();
  });

  it('should create failed login result', () => {
    const result: LoginResult = {
      success: false,
      error: 'Invalid credentials',
    };

    expect(result.success).toBe(false);
    expect(result.error).toBe('Invalid credentials');
    expect(result.userId).toBeUndefined();
  });
});
