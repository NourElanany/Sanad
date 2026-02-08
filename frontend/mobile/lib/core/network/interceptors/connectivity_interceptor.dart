import 'package:dio/dio.dart';
import 'package:flutter/foundation.dart';
import '../../services/connectivity_service.dart';

/// Interceptor to check network connectivity before making requests
class ConnectivityInterceptor extends Interceptor {
  final ConnectivityService _connectivityService = ConnectivityService();
  
  @override
  void onRequest(
    RequestOptions options,
    RequestInterceptorHandler handler,
  ) async {
    // Check if device has internet connection
    final isConnected = await _connectivityService.isConnected();
    
    if (!isConnected) {
      if (kDebugMode) {
        print('❌ No internet connection for request: ${options.path}');
      }
      
      return handler.reject(
        DioException(
          requestOptions: options,
          type: DioExceptionType.connectionError,
          error: 'No internet connection',
          message: 'Please check your internet connection and try again.',
        ),
      );
    }
    
    handler.next(options);
  }
}
