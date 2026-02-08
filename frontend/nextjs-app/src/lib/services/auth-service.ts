import axios from 'axios';

/**
 * Authentication service for managing JWT tokens and user authentication
 */
class AuthService {
  private readonly ACCESS_TOKEN_KEY = 'access_token';
  private readonly REFRESH_TOKEN_KEY = 'refresh_token';
  private readonly USER_ID_KEY = 'user_id';
  private readonly API_BASE_URL = process.env.NEXT_PUBLIC_API_BASE_URL || 'https://api.sanad.app';

  /**
   * Get access token from localStorage
   */
  getAccessToken(): string | null {
    if (typeof window === 'undefined') return null;
    return localStorage.getItem(this.ACCESS_TOKEN_KEY);
  }

  /**
   * Get refresh token from localStorage
   */
  getRefreshToken(): string | null {
    if (typeof window === 'undefined') return null;
    return localStorage.getItem(this.REFRESH_TOKEN_KEY);
  }

  /**
   * Save authentication tokens
   */
  saveTokens(accessToken: string, refreshToken: string): void {
    if (typeof window === 'undefined') return;
    
    localStorage.setItem(this.ACCESS_TOKEN_KEY, accessToken);
    localStorage.setItem(this.REFRESH_TOKEN_KEY, refreshToken);
    
    console.log('✅ Tokens saved successfully');
  }

  /**
   * Save user ID
   */
  saveUserId(userId: string): void {
    if (typeof window === 'undefined') return;
    localStorage.setItem(this.USER_ID_KEY, userId);
  }

  /**
   * Get user ID
   */
  getUserId(): string | null {
    if (typeof window === 'undefined') return null;
    return localStorage.getItem(this.USER_ID_KEY);
  }

  /**
   * Check if user is authenticated
   */
  isAuthenticated(): boolean {
    const token = this.getAccessToken();
    if (!token) return false;
    
    return !this.isTokenExpired(token);
  }

  /**
   * Check if JWT token is expired
   */
  private isTokenExpired(token: string): boolean {
    try {
      const parts = token.split('.');
      if (parts.length !== 3) return true;

      const payload = JSON.parse(atob(parts[1]));
      
      if (payload.exp) {
        const expirationDate = new Date(payload.exp * 1000);
        const bufferTime = new Date(Date.now() + 5 * 60 * 1000); // 5 minute buffer
        
        return bufferTime > expirationDate;
      }

      return false;
    } catch (error) {
      console.error('❌ Error checking token expiration:', error);
      return true;
    }
  }

  /**
   * Refresh access token using refresh token
   */
  async refreshAccessToken(): Promise<string | null> {
    try {
      const refreshToken = this.getRefreshToken();
      
      if (!refreshToken) {
        console.error('❌ No refresh token available');
        return null;
      }

      console.log('🔄 Refreshing access token...');

      const response = await axios.post(
        `${this.API_BASE_URL}/api/auth/refresh`,
        { refresh_token: refreshToken },
        {
          headers: {
            'Content-Type': 'application/json',
          },
        }
      );

      if (response.status === 200) {
        const { access_token, refresh_token } = response.data;
        
        this.saveTokens(access_token, refresh_token || refreshToken);
        
        console.log('✅ Access token refreshed successfully');
        return access_token;
      }

      return null;
    } catch (error) {
      console.error('❌ Error refreshing token:', error);
      return null;
    }
  }

  /**
   * Login with email and password
   */
  async login(email: string, password: string): Promise<LoginResult> {
    try {
      const response = await axios.post(
        `${this.API_BASE_URL}/api/auth/login`,
        { email, password },
        {
          headers: {
            'Content-Type': 'application/json',
          },
        }
      );

      if (response.status === 200) {
        const { access_token, refresh_token, user_id } = response.data;
        
        this.saveTokens(access_token, refresh_token);
        this.saveUserId(user_id);
        
        console.log('✅ Login successful');
        
        return {
          success: true,
          userId: user_id,
        };
      }

      return {
        success: false,
        error: 'Login failed',
      };
    } catch (error: any) {
      const message = error.response?.data?.message || 'Login failed. Please try again.';
      
      return {
        success: false,
        error: message,
      };
    }
  }

  /**
   * Register new user
   */
  async register(email: string, password: string, name: string): Promise<LoginResult> {
    try {
      const response = await axios.post(
        `${this.API_BASE_URL}/api/auth/register`,
        { email, password, name },
        {
          headers: {
            'Content-Type': 'application/json',
          },
        }
      );

      if (response.status === 200 || response.status === 201) {
        const { access_token, refresh_token, user_id } = response.data;
        
        this.saveTokens(access_token, refresh_token);
        this.saveUserId(user_id);
        
        console.log('✅ Registration successful');
        
        return {
          success: true,
          userId: user_id,
        };
      }

      return {
        success: false,
        error: 'Registration failed',
      };
    } catch (error: any) {
      const message = error.response?.data?.message || 'Registration failed. Please try again.';
      
      return {
        success: false,
        error: message,
      };
    }
  }

  /**
   * Logout user and clear tokens
   */
  logout(): void {
    if (typeof window === 'undefined') return;
    
    localStorage.removeItem(this.ACCESS_TOKEN_KEY);
    localStorage.removeItem(this.REFRESH_TOKEN_KEY);
    localStorage.removeItem(this.USER_ID_KEY);
    
    console.log('✅ Logout successful');
  }

  /**
   * Clear all authentication data
   */
  clearAuth(): void {
    this.logout();
  }
}

/**
 * Login/Register result interface
 */
export interface LoginResult {
  success: boolean;
  userId?: string;
  error?: string;
}

// Export singleton instance
export const authService = new AuthService();
