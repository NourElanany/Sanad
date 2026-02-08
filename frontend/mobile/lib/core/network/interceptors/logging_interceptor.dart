import 'package:dio/dio.dart';
import 'package:flutter/foundation.dart';

/// Interceptor to log HTTP requests and responses in debug mode
class LoggingInterceptor extends Interceptor {
  @override
  void onRequest(RequestOptions options, RequestInterceptorHandler handler) {
    if (kDebugMode) {
      print('┌─────────────────────────────────────────────────────────────');
      print('│ 📤 REQUEST');
      print('├─────────────────────────────────────────────────────────────');
      print('│ Method: ${options.method}');
      print('│ URL: ${options.uri}');
      
      if (options.headers.isNotEmpty) {
        print('│ Headers:');
        options.headers.forEach((key, value) {
          // Don't log sensitive headers
          if (key.toLowerCase() == 'authorization') {
            print('│   $key: Bearer ***');
          } else {
            print('│   $key: $value');
          }
        });
      }
      
      if (options.queryParameters.isNotEmpty) {
        print('│ Query Parameters:');
        options.queryParameters.forEach((key, value) {
          print('│   $key: $value');
        });
      }
      
      if (options.data != null) {
        print('│ Body:');
        print('│   ${_formatData(options.data)}');
      }
      
      print('└─────────────────────────────────────────────────────────────');
    }
    
    handler.next(options);
  }
  
  @override
  void onResponse(Response response, ResponseInterceptorHandler handler) {
    if (kDebugMode) {
      print('┌─────────────────────────────────────────────────────────────');
      print('│ 📥 RESPONSE');
      print('├─────────────────────────────────────────────────────────────');
      print('│ Status Code: ${response.statusCode}');
      print('│ URL: ${response.requestOptions.uri}');
      
      if (response.headers.map.isNotEmpty) {
        print('│ Headers:');
        response.headers.map.forEach((key, value) {
          print('│   $key: ${value.join(', ')}');
        });
      }
      
      if (response.data != null) {
        print('│ Body:');
        print('│   ${_formatData(response.data)}');
      }
      
      print('└─────────────────────────────────────────────────────────────');
    }
    
    handler.next(response);
  }
  
  @override
  void onError(DioException err, ErrorInterceptorHandler handler) {
    if (kDebugMode) {
      print('┌─────────────────────────────────────────────────────────────');
      print('│ ❌ ERROR');
      print('├─────────────────────────────────────────────────────────────');
      print('│ Type: ${err.type}');
      print('│ URL: ${err.requestOptions.uri}');
      print('│ Message: ${err.message}');
      
      if (err.response != null) {
        print('│ Status Code: ${err.response?.statusCode}');
        print('│ Response:');
        print('│   ${_formatData(err.response?.data)}');
      }
      
      if (err.stackTrace != null) {
        print('│ Stack Trace:');
        print('│   ${err.stackTrace}');
      }
      
      print('└─────────────────────────────────────────────────────────────');
    }
    
    handler.next(err);
  }
  
  /// Format data for logging (truncate if too long)
  String _formatData(dynamic data) {
    if (data == null) return 'null';
    
    final dataString = data.toString();
    const maxLength = 500;
    
    if (dataString.length > maxLength) {
      return '${dataString.substring(0, maxLength)}... (truncated)';
    }
    
    return dataString;
  }
}
