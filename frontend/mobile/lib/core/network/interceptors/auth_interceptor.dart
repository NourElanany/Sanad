import 'package:dio/dio.dart';
import 'package:flutter/foundation.dart';
import '../../services/auth_service.dart';

/// Interceptor to add JWT authentication token to requests
class AuthInterceptor extends Interceptor {
  final AuthService _authService = AuthService();
  
  @override
  void onRequest(
    RequestOptions options,
    RequestInterceptorHandler handler,
  ) async {
    // Skip authentication for auth endpoints
    if (_isAuthEndpoint(options.path)) {
      return handler.next(options);
    }
    
    try {
      // Get access token
      final token = await _authService.getAccessToken();
      
      if (token != null && token.isNotEmpty) {
        // Add Bearer token to headers
        options.headers['Authorization'] = 'Bearer $token';
        
        if (kDebugMode) {
          print('🔐 Added auth token to request: ${options.path}');
        }
      }
      
      handler.next(options);
    } catch (e) {
      if (kDebugMode) {
        print('❌ Error adding auth token: $e');
      }
      handler.next(options);
    }
  }
  
  @override
  void onError(
    DioException err,
    ErrorInterceptorHandler handler,
  ) async {
    // Handle 401 Unauthorized - token expired or invalid
    if (err.response?.statusCode == 401) {
      if (kDebugMode) {
        print('🔄 Token expired, attempting refresh...');
      }
      
      try {
        // Try to refresh the token
        final newToken = await _authService.refreshAccessToken();
        
        if (newToken != null) {
          // Retry the original request with new token
          final options = err.requestOptions;
          options.headers['Authorization'] = 'Bearer $newToken';
          
          if (kDebugMode) {
            print('✅ Token refreshed, retrying request');
          }
          
          try {
            final response = await Dio().fetch(options);
            return handler.resolve(response);
          } catch (e) {
            return handler.reject(err);
          }
        } else {
          // Refresh failed, user needs to login again
          if (kDebugMode) {
            print('❌ Token refresh failed, logging out user');
          }
          await _authService.logout();
          return handler.reject(err);
        }
      } catch (e) {
        if (kDebugMode) {
          print('❌ Error refreshing token: $e');
        }
        await _authService.logout();
        return handler.reject(err);
      }
    }
    
    handler.next(err);
  }
  
  /// Check if the endpoint is an authentication endpoint
  bool _isAuthEndpoint(String path) {
    return path.contains('/auth/login') ||
           path.contains('/auth/register') ||
           path.contains('/auth/refresh') ||
           path.contains('/auth/forgot-password');
  }
}
