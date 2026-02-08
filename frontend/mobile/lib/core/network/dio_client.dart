import 'package:dio/dio.dart';
import 'package:flutter/foundation.dart';
import '../config/app_config.dart';
import 'interceptors/auth_interceptor.dart';
import 'interceptors/logging_interceptor.dart';
import 'interceptors/retry_interceptor.dart';
import 'interceptors/connectivity_interceptor.dart';

/// Dio HTTP client configuration for API communication
class DioClient {
  late final Dio _dio;
  
  DioClient() {
    _dio = Dio(_baseOptions);
    _setupInterceptors();
  }
  
  /// Base options for Dio client
  BaseOptions get _baseOptions => BaseOptions(
    baseUrl: AppConfig.apiBaseUrl,
    connectTimeout: const Duration(milliseconds: AppConfig.connectTimeout),
    receiveTimeout: const Duration(milliseconds: AppConfig.apiTimeout),
    sendTimeout: const Duration(milliseconds: AppConfig.apiTimeout),
    headers: {
      'Content-Type': 'application/json',
      'Accept': 'application/json',
      'X-App-Version': AppConfig.appVersion,
      'X-Platform': defaultTargetPlatform.name,
    },
    validateStatus: (status) {
      // Accept all status codes to handle them in interceptors
      return status != null && status < 500;
    },
  );
  
  /// Setup interceptors for authentication, logging, retry, etc.
  void _setupInterceptors() {
    _dio.interceptors.addAll([
      // Check connectivity before making requests
      ConnectivityInterceptor(),
      
      // Add authentication token to requests
      AuthInterceptor(),
      
      // Retry failed requests with exponential backoff
      RetryInterceptor(_dio),
      
      // Log requests and responses in debug mode
      if (kDebugMode) LoggingInterceptor(),
    ]);
  }
  
  /// Get Dio instance
  Dio get dio => _dio;
  
  /// GET request
  Future<Response<T>> get<T>(
    String path, {
    Map<String, dynamic>? queryParameters,
    Options? options,
    CancelToken? cancelToken,
    ProgressCallback? onReceiveProgress,
  }) async {
    try {
      return await _dio.get<T>(
        path,
        queryParameters: queryParameters,
        options: options,
        cancelToken: cancelToken,
        onReceiveProgress: onReceiveProgress,
      );
    } on DioException catch (e) {
      throw _handleError(e);
    }
  }
  
  /// POST request
  Future<Response<T>> post<T>(
    String path, {
    dynamic data,
    Map<String, dynamic>? queryParameters,
    Options? options,
    CancelToken? cancelToken,
    ProgressCallback? onSendProgress,
    ProgressCallback? onReceiveProgress,
  }) async {
    try {
      return await _dio.post<T>(
        path,
        data: data,
        queryParameters: queryParameters,
        options: options,
        cancelToken: cancelToken,
        onSendProgress: onSendProgress,
        onReceiveProgress: onReceiveProgress,
      );
    } on DioException catch (e) {
      throw _handleError(e);
    }
  }
  
  /// PUT request
  Future<Response<T>> put<T>(
    String path, {
    dynamic data,
    Map<String, dynamic>? queryParameters,
    Options? options,
    CancelToken? cancelToken,
    ProgressCallback? onSendProgress,
    ProgressCallback? onReceiveProgress,
  }) async {
    try {
      return await _dio.put<T>(
        path,
        data: data,
        queryParameters: queryParameters,
        options: options,
        cancelToken: cancelToken,
        onSendProgress: onSendProgress,
        onReceiveProgress: onReceiveProgress,
      );
    } on DioException catch (e) {
      throw _handleError(e);
    }
  }
  
  /// DELETE request
  Future<Response<T>> delete<T>(
    String path, {
    dynamic data,
    Map<String, dynamic>? queryParameters,
    Options? options,
    CancelToken? cancelToken,
  }) async {
    try {
      return await _dio.delete<T>(
        path,
        data: data,
        queryParameters: queryParameters,
        options: options,
        cancelToken: cancelToken,
      );
    } on DioException catch (e) {
      throw _handleError(e);
    }
  }
  
  /// PATCH request
  Future<Response<T>> patch<T>(
    String path, {
    dynamic data,
    Map<String, dynamic>? queryParameters,
    Options? options,
    CancelToken? cancelToken,
    ProgressCallback? onSendProgress,
    ProgressCallback? onReceiveProgress,
  }) async {
    try {
      return await _dio.patch<T>(
        path,
        data: data,
        queryParameters: queryParameters,
        options: options,
        cancelToken: cancelToken,
        onSendProgress: onSendProgress,
        onReceiveProgress: onReceiveProgress,
      );
    } on DioException catch (e) {
      throw _handleError(e);
    }
  }
  
  /// Handle Dio errors and convert to app-specific exceptions
  Exception _handleError(DioException error) {
    switch (error.type) {
      case DioExceptionType.connectionTimeout:
      case DioExceptionType.sendTimeout:
      case DioExceptionType.receiveTimeout:
        return NetworkException(
          message: 'Connection timeout. Please check your internet connection.',
          statusCode: 408,
        );
      
      case DioExceptionType.badResponse:
        return _handleResponseError(error);
      
      case DioExceptionType.cancel:
        return NetworkException(
          message: 'Request cancelled',
          statusCode: 499,
        );
      
      case DioExceptionType.connectionError:
        return NetworkException(
          message: 'No internet connection. Please check your network settings.',
          statusCode: 503,
        );
      
      case DioExceptionType.badCertificate:
        return NetworkException(
          message: 'SSL certificate error',
          statusCode: 495,
        );
      
      case DioExceptionType.unknown:
      default:
        return NetworkException(
          message: error.message ?? 'An unexpected error occurred',
          statusCode: 500,
        );
    }
  }
  
  /// Handle response errors based on status code
  Exception _handleResponseError(DioException error) {
    final statusCode = error.response?.statusCode ?? 500;
    final data = error.response?.data;
    
    String message = 'An error occurred';
    if (data is Map<String, dynamic> && data.containsKey('message')) {
      message = data['message'] as String;
    } else if (data is String) {
      message = data;
    }
    
    switch (statusCode) {
      case 400:
        return BadRequestException(message: message, statusCode: statusCode);
      case 401:
        return UnauthorizedException(message: message, statusCode: statusCode);
      case 403:
        return ForbiddenException(message: message, statusCode: statusCode);
      case 404:
        return NotFoundException(message: message, statusCode: statusCode);
      case 422:
        return ValidationException(
          message: message,
          statusCode: statusCode,
          errors: data is Map ? data['errors'] as Map<String, dynamic>? : null,
        );
      case 429:
        return RateLimitException(message: message, statusCode: statusCode);
      default:
        return ServerException(message: message, statusCode: statusCode);
    }
  }
}

/// Base network exception
class NetworkException implements Exception {
  final String message;
  final int statusCode;
  
  NetworkException({
    required this.message,
    required this.statusCode,
  });
  
  @override
  String toString() => 'NetworkException: $message (Status: $statusCode)';
}

/// Bad request exception (400)
class BadRequestException extends NetworkException {
  BadRequestException({required super.message, required super.statusCode});
  
  @override
  String toString() => 'BadRequestException: $message';
}

/// Unauthorized exception (401)
class UnauthorizedException extends NetworkException {
  UnauthorizedException({required super.message, required super.statusCode});
  
  @override
  String toString() => 'UnauthorizedException: $message';
}

/// Forbidden exception (403)
class ForbiddenException extends NetworkException {
  ForbiddenException({required super.message, required super.statusCode});
  
  @override
  String toString() => 'ForbiddenException: $message';
}

/// Not found exception (404)
class NotFoundException extends NetworkException {
  NotFoundException({required super.message, required super.statusCode});
  
  @override
  String toString() => 'NotFoundException: $message';
}

/// Validation exception (422)
class ValidationException extends NetworkException {
  final Map<String, dynamic>? errors;
  
  ValidationException({
    required super.message,
    required super.statusCode,
    this.errors,
  });
  
  @override
  String toString() => 'ValidationException: $message${errors != null ? ' - Errors: $errors' : ''}';
}

/// Rate limit exception (429)
class RateLimitException extends NetworkException {
  RateLimitException({required super.message, required super.statusCode});
  
  @override
  String toString() => 'RateLimitException: $message';
}

/// Server exception (5xx)
class ServerException extends NetworkException {
  ServerException({required super.message, required super.statusCode});
  
  @override
  String toString() => 'ServerException: $message';
}
