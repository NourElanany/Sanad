import 'dart:math';
import 'package:dio/dio.dart';
import 'package:flutter/foundation.dart';

/// Interceptor to retry failed requests with exponential backoff
class RetryInterceptor extends Interceptor {
  final Dio _dio;
  final int maxRetries;
  final Duration initialDelay;
  final double backoffMultiplier;
  
  RetryInterceptor(
    this._dio, {
    this.maxRetries = 3,
    this.initialDelay = const Duration(milliseconds: 500),
    this.backoffMultiplier = 2.0,
  });
  
  @override
  void onError(
    DioException err,
    ErrorInterceptorHandler handler,
  ) async {
    // Only retry on specific error types
    if (!_shouldRetry(err)) {
      return handler.next(err);
    }
    
    final retryCount = err.requestOptions.extra['retryCount'] as int? ?? 0;
    
    if (retryCount >= maxRetries) {
      if (kDebugMode) {
        print('❌ Max retries ($maxRetries) reached for ${err.requestOptions.path}');
      }
      return handler.next(err);
    }
    
    // Calculate delay with exponential backoff
    final delay = _calculateDelay(retryCount);
    
    if (kDebugMode) {
      print('🔄 Retrying request (${retryCount + 1}/$maxRetries) after ${delay.inMilliseconds}ms: ${err.requestOptions.path}');
    }
    
    // Wait before retrying
    await Future.delayed(delay);
    
    // Update retry count
    err.requestOptions.extra['retryCount'] = retryCount + 1;
    
    try {
      // Retry the request
      final response = await _dio.fetch(err.requestOptions);
      return handler.resolve(response);
    } on DioException catch (e) {
      // If retry fails, pass the error to the next handler
      return handler.next(e);
    }
  }
  
  /// Determine if the request should be retried
  bool _shouldRetry(DioException err) {
    // Don't retry on client errors (4xx) except 408, 429
    if (err.response?.statusCode != null) {
      final statusCode = err.response!.statusCode!;
      
      // Retry on server errors (5xx)
      if (statusCode >= 500) return true;
      
      // Retry on specific client errors
      if (statusCode == 408 || statusCode == 429) return true;
      
      // Don't retry other client errors
      if (statusCode >= 400 && statusCode < 500) return false;
    }
    
    // Retry on connection errors
    switch (err.type) {
      case DioExceptionType.connectionTimeout:
      case DioExceptionType.sendTimeout:
      case DioExceptionType.receiveTimeout:
      case DioExceptionType.connectionError:
        return true;
      
      case DioExceptionType.badResponse:
        // Already handled above
        return false;
      
      case DioExceptionType.cancel:
      case DioExceptionType.badCertificate:
      case DioExceptionType.unknown:
      default:
        return false;
    }
  }
  
  /// Calculate delay with exponential backoff and jitter
  Duration _calculateDelay(int retryCount) {
    // Exponential backoff: initialDelay * (backoffMultiplier ^ retryCount)
    final exponentialDelay = initialDelay.inMilliseconds * 
                            pow(backoffMultiplier, retryCount);
    
    // Add jitter (random value between 0 and 20% of delay)
    final jitter = Random().nextDouble() * exponentialDelay * 0.2;
    
    final totalDelay = exponentialDelay + jitter;
    
    // Cap at 30 seconds
    return Duration(milliseconds: min(totalDelay.toInt(), 30000));
  }
}
