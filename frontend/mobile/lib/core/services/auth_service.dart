import 'dart:convert';
import 'package:flutter/foundation.dart';
import 'package:flutter_secure_storage/flutter_secure_storage.dart';
import 'package:dio/dio.dart';
import '../config/app_config.dart';

/// Service for managing JWT authentication and tokens
class AuthService {
  static final AuthService _instance = AuthService._internal();
  factory AuthService() => _instance;
  AuthService._internal();
  
  final FlutterSecureStorage _secureStorage = const FlutterSecureStorage(
    aOptions: AndroidOptions(
      encryptedSharedPreferences: true,
    ),
    iOptions: IOSOptions(
      accessibility: KeychainAccessibility.first_unlock_this_device,
    ),
  );
  
  late final Dio _dio;
  
  /// Initialize the auth service
  void init() {
    _dio = Dio(BaseOptions(
      baseUrl: AppConfig.apiBaseUrl,
      connectTimeout: const Duration(milliseconds: AppConfig.connectTimeout),
      receiveTimeout: const Duration(milliseconds: AppConfig.apiTimeout),
      headers: {
        'Content-Type': 'application/json',
        'Accept': 'application/json',
      },
    ));
  }
  
  /// Get access token from secure storage
  Future<String?> getAccessToken() async {
    try {
      final token = await _secureStorage.read(key: AppConfig.accessTokenKey);
      
      if (token != null && !_isTokenExpired(token)) {
        return token;
      }
      
      // Token expired, try to refresh
      return await refreshAccessToken();
    } catch (e) {
      if (kDebugMode) {
        print('❌ Error getting access token: $e');
      }
      return null;
    }
  }
  
  /// Get refresh token from secure storage
  Future<String?> getRefreshToken() async {
    try {
      return await _secureStorage.read(key: AppConfig.refreshTokenKey);
    } catch (e) {
      if (kDebugMode) {
        print('❌ Error getting refresh token: $e');
      }
      return null;
    }
  }
  
  /// Save authentication tokens
  Future<void> saveTokens({
    required String accessToken,
    required String refreshToken,
  }) async {
    try {
      await Future.wait([
        _secureStorage.write(
          key: AppConfig.accessTokenKey,
          value: accessToken,
        ),
        _secureStorage.write(
          key: AppConfig.refreshTokenKey,
          value: refreshToken,
        ),
      ]);
      
      if (kDebugMode) {
        print('✅ Tokens saved successfully');
      }
    } catch (e) {
      if (kDebugMode) {
        print('❌ Error saving tokens: $e');
      }
      rethrow;
    }
  }
  
  /// Refresh access token using refresh token
  Future<String?> refreshAccessToken() async {
    try {
      final refreshToken = await getRefreshToken();
      
      if (refreshToken == null) {
        if (kDebugMode) {
          print('❌ No refresh token available');
        }
        return null;
      }
      
      if (kDebugMode) {
        print('🔄 Refreshing access token...');
      }
      
      final response = await _dio.post(
        '${AppConfig.authServicePath}/refresh',
        data: {'refresh_token': refreshToken},
      );
      
      if (response.statusCode == 200) {
        final data = response.data as Map<String, dynamic>;
        final newAccessToken = data['access_token'] as String;
        final newRefreshToken = data['refresh_token'] as String?;
        
        // Save new tokens
        await saveTokens(
          accessToken: newAccessToken,
          refreshToken: newRefreshToken ?? refreshToken,
        );
        
        if (kDebugMode) {
          print('✅ Access token refreshed successfully');
        }
        
        return newAccessToken;
      }
      
      return null;
    } catch (e) {
      if (kDebugMode) {
        print('❌ Error refreshing token: $e');
      }
      return null;
    }
  }
  
  /// Login with email and password
  Future<LoginResult> login({
    required String email,
    required String password,
  }) async {
    try {
      final response = await _dio.post(
        '${AppConfig.authServicePath}/login',
        data: {
          'email': email,
          'password': password,
        },
      );
      
      if (response.statusCode == 200) {
        final data = response.data as Map<String, dynamic>;
        final accessToken = data['access_token'] as String;
        final refreshToken = data['refresh_token'] as String;
        final userId = data['user_id'] as String;
        
        // Save tokens
        await saveTokens(
          accessToken: accessToken,
          refreshToken: refreshToken,
        );
        
        // Save user ID
        await _secureStorage.write(
          key: AppConfig.userIdKey,
          value: userId,
        );
        
        if (kDebugMode) {
          print('✅ Login successful');
        }
        
        return LoginResult(
          success: true,
          userId: userId,
        );
      }
      
      return LoginResult(
        success: false,
        error: 'Login failed',
      );
    } on DioException catch (e) {
      final message = e.response?.data?['message'] as String? ?? 
                     'Login failed. Please try again.';
      
      return LoginResult(
        success: false,
        error: message,
      );
    } catch (e) {
      if (kDebugMode) {
        print('❌ Login error: $e');
      }
      
      return LoginResult(
        success: false,
        error: 'An unexpected error occurred',
      );
    }
  }
  
  /// Register new user
  Future<LoginResult> register({
    required String email,
    required String password,
    required String name,
  }) async {
    try {
      final response = await _dio.post(
        '${AppConfig.authServicePath}/register',
        data: {
          'email': email,
          'password': password,
          'name': name,
        },
      );
      
      if (response.statusCode == 201 || response.statusCode == 200) {
        final data = response.data as Map<String, dynamic>;
        final accessToken = data['access_token'] as String;
        final refreshToken = data['refresh_token'] as String;
        final userId = data['user_id'] as String;
        
        // Save tokens
        await saveTokens(
          accessToken: accessToken,
          refreshToken: refreshToken,
        );
        
        // Save user ID
        await _secureStorage.write(
          key: AppConfig.userIdKey,
          value: userId,
        );
        
        if (kDebugMode) {
          print('✅ Registration successful');
        }
        
        return LoginResult(
          success: true,
          userId: userId,
        );
      }
      
      return LoginResult(
        success: false,
        error: 'Registration failed',
      );
    } on DioException catch (e) {
      final message = e.response?.data?['message'] as String? ?? 
                     'Registration failed. Please try again.';
      
      return LoginResult(
        success: false,
        error: message,
      );
    } catch (e) {
      if (kDebugMode) {
        print('❌ Registration error: $e');
      }
      
      return LoginResult(
        success: false,
        error: 'An unexpected error occurred',
      );
    }
  }
  
  /// Logout user and clear tokens
  Future<void> logout() async {
    try {
      await Future.wait([
        _secureStorage.delete(key: AppConfig.accessTokenKey),
        _secureStorage.delete(key: AppConfig.refreshTokenKey),
        _secureStorage.delete(key: AppConfig.userIdKey),
      ]);
      
      if (kDebugMode) {
        print('✅ Logout successful');
      }
    } catch (e) {
      if (kDebugMode) {
        print('❌ Error during logout: $e');
      }
    }
  }
  
  /// Check if user is authenticated
  Future<bool> isAuthenticated() async {
    final token = await getAccessToken();
    return token != null && token.isNotEmpty;
  }
  
  /// Get current user ID
  Future<String?> getUserId() async {
    try {
      return await _secureStorage.read(key: AppConfig.userIdKey);
    } catch (e) {
      if (kDebugMode) {
        print('❌ Error getting user ID: $e');
      }
      return null;
    }
  }
  
  /// Check if JWT token is expired
  bool _isTokenExpired(String token) {
    try {
      // JWT format: header.payload.signature
      final parts = token.split('.');
      if (parts.length != 3) return true;
      
      // Decode payload (base64)
      final payload = parts[1];
      final normalized = base64Url.normalize(payload);
      final decoded = utf8.decode(base64Url.decode(normalized));
      final payloadMap = json.decode(decoded) as Map<String, dynamic>;
      
      // Check expiration time
      if (payloadMap.containsKey('exp')) {
        final exp = payloadMap['exp'] as int;
        final expirationDate = DateTime.fromMillisecondsSinceEpoch(exp * 1000);
        
        // Add 5 minute buffer before expiration
        final bufferTime = DateTime.now().add(const Duration(minutes: 5));
        
        return bufferTime.isAfter(expirationDate);
      }
      
      return false;
    } catch (e) {
      if (kDebugMode) {
        print('❌ Error checking token expiration: $e');
      }
      return true;
    }
  }
}

/// Result of login/register operation
class LoginResult {
  final bool success;
  final String? userId;
  final String? error;
  
  LoginResult({
    required this.success,
    this.userId,
    this.error,
  });
}
