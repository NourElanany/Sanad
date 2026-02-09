import 'package:flutter/material.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'package:dio/dio.dart';

/// Error types
enum ErrorType {
  network,
  authentication,
  validation,
  notFound,
  serverError,
  unknown,
}

/// App error model
class AppError {
  final ErrorType type;
  final String message;
  final String? details;
  final int? statusCode;
  final DateTime timestamp;

  AppError({
    required this.type,
    required this.message,
    this.details,
    this.statusCode,
    DateTime? timestamp,
  }) : timestamp = timestamp ?? DateTime.now();

  factory AppError.fromException(dynamic error) {
    if (error is DioException) {
      return AppError._fromDioError(error);
    } else if (error is AppError) {
      return error;
    } else {
      return AppError(
        type: ErrorType.unknown,
        message: 'حدث خطأ غير متوقع',
        details: error.toString(),
      );
    }
  }

  factory AppError._fromDioError(DioException error) {
    switch (error.type) {
      case DioExceptionType.connectionTimeout:
      case DioExceptionType.sendTimeout:
      case DioExceptionType.receiveTimeout:
        return AppError(
          type: ErrorType.network,
          message: 'انتهت مهلة الاتصال. يرجى المحاولة مرة أخرى',
          details: error.message,
        );

      case DioExceptionType.badResponse:
        final statusCode = error.response?.statusCode;
        if (statusCode == 401 || statusCode == 403) {
          return AppError(
            type: ErrorType.authentication,
            message: 'انتهت جلستك. يرجى تسجيل الدخول مرة أخرى',
            statusCode: statusCode,
          );
        } else if (statusCode == 404) {
          return AppError(
            type: ErrorType.notFound,
            message: 'المحتوى المطلوب غير موجود',
            statusCode: statusCode,
          );
        } else if (statusCode != null && statusCode >= 500) {
          return AppError(
            type: ErrorType.serverError,
            message: 'خطأ في الخادم. يرجى المحاولة لاحقاً',
            statusCode: statusCode,
          );
        } else {
          return AppError(
            type: ErrorType.validation,
            message: error.response?.data['message'] ?? 'بيانات غير صحيحة',
            statusCode: statusCode,
          );
        }

      case DioExceptionType.cancel:
        return AppError(
          type: ErrorType.unknown,
          message: 'تم إلغاء العملية',
        );

      case DioExceptionType.connectionError:
        return AppError(
          type: ErrorType.network,
          message: 'لا يوجد اتصال بالإنترنت',
          details: error.message,
        );

      default:
        return AppError(
          type: ErrorType.unknown,
          message: 'حدث خطأ غير متوقع',
          details: error.message,
        );
    }
  }

  String get userFriendlyMessage {
    switch (type) {
      case ErrorType.network:
        return 'يرجى التحقق من اتصالك بالإنترنت والمحاولة مرة أخرى';
      case ErrorType.authentication:
        return 'يرجى تسجيل الدخول للمتابعة';
      case ErrorType.validation:
        return message;
      case ErrorType.notFound:
        return 'المحتوى المطلوب غير متوفر';
      case ErrorType.serverError:
        return 'نواجه مشكلة مؤقتة. يرجى المحاولة لاحقاً';
      case ErrorType.unknown:
        return 'حدث خطأ. يرجى المحاولة مرة أخرى';
    }
  }

  IconData get icon {
    switch (type) {
      case ErrorType.network:
        return Icons.wifi_off;
      case ErrorType.authentication:
        return Icons.lock_outline;
      case ErrorType.validation:
        return Icons.error_outline;
      case ErrorType.notFound:
        return Icons.search_off;
      case ErrorType.serverError:
        return Icons.cloud_off;
      case ErrorType.unknown:
        return Icons.warning_amber;
    }
  }

  Color get color {
    switch (type) {
      case ErrorType.network:
        return Colors.orange;
      case ErrorType.authentication:
        return Colors.red;
      case ErrorType.validation:
        return Colors.amber;
      case ErrorType.notFound:
        return Colors.grey;
      case ErrorType.serverError:
        return Colors.red;
      case ErrorType.unknown:
        return Colors.grey;
    }
  }
}

/// Error handler state
class ErrorHandlerState {
  final AppError? currentError;
  final List<AppError> errorHistory;

  const ErrorHandlerState({
    this.currentError,
    this.errorHistory = const [],
  });

  ErrorHandlerState copyWith({
    AppError? currentError,
    List<AppError>? errorHistory,
  }) {
    return ErrorHandlerState(
      currentError: currentError,
      errorHistory: errorHistory ?? this.errorHistory,
    );
  }
}

/// Error handler notifier
class ErrorHandlerNotifier extends StateNotifier<ErrorHandlerState> {
  ErrorHandlerNotifier() : super(const ErrorHandlerState());

  void handleError(dynamic error) {
    final appError = AppError.fromException(error);
    state = state.copyWith(
      currentError: appError,
      errorHistory: [...state.errorHistory, appError],
    );
  }

  void clearError() {
    state = state.copyWith(currentError: null);
  }

  void clearHistory() {
    state = state.copyWith(errorHistory: []);
  }
}

/// Error handler provider
final errorHandlerProvider = StateNotifierProvider<ErrorHandlerNotifier, ErrorHandlerState>((ref) {
  return ErrorHandlerNotifier();
});

/// Show error snackbar helper
void showErrorSnackbar(BuildContext context, AppError error) {
  ScaffoldMessenger.of(context).showSnackBar(
    SnackBar(
      content: Row(
        children: [
          Icon(error.icon, color: Colors.white),
          const SizedBox(width: 12),
          Expanded(
            child: Column(
              crossAxisAlignment: CrossAxisAlignment.start,
              mainAxisSize: MainAxisSize.min,
              children: [
                Text(
                  error.message,
                  style: const TextStyle(
                    fontWeight: FontWeight.bold,
                    color: Colors.white,
                  ),
                ),
                if (error.details != null) ...[
                  const SizedBox(height: 4),
                  Text(
                    error.details!,
                    style: const TextStyle(
                      fontSize: 12,
                      color: Colors.white70,
                    ),
                    maxLines: 2,
                    overflow: TextOverflow.ellipsis,
                  ),
                ],
              ],
            ),
          ),
        ],
      ),
      backgroundColor: error.color,
      duration: const Duration(seconds: 4),
      behavior: SnackBarBehavior.floating,
      action: SnackBarAction(
        label: 'حسناً',
        textColor: Colors.white,
        onPressed: () {},
      ),
    ),
  );
}
