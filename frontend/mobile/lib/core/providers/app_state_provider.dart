import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'package:connectivity_plus/connectivity_plus.dart';

/// Global app state provider for managing application-wide state
class AppState {
  final bool isOnline;
  final bool isLoading;
  final String? error;
  final String? currentUserId;

  const AppState({
    this.isOnline = true,
    this.isLoading = false,
    this.error,
    this.currentUserId,
  });

  AppState copyWith({
    bool? isOnline,
    bool? isLoading,
    String? error,
    String? currentUserId,
  }) {
    return AppState(
      isOnline: isOnline ?? this.isOnline,
      isLoading: isLoading ?? this.isLoading,
      error: error ?? this.error,
      currentUserId: currentUserId ?? this.currentUserId,
    );
  }
}

class AppStateNotifier extends StateNotifier<AppState> {
  AppStateNotifier() : super(const AppState()) {
    _initConnectivity();
  }

  void _initConnectivity() {
    Connectivity().onConnectivityChanged.listen((result) {
      state = state.copyWith(
        isOnline: result != ConnectivityResult.none,
      );
    });
  }

  void setLoading(bool loading) {
    state = state.copyWith(isLoading: loading);
  }

  void setError(String? error) {
    state = state.copyWith(error: error);
  }

  void setUserId(String? userId) {
    state = state.copyWith(currentUserId: userId);
  }

  void clearError() {
    state = state.copyWith(error: null);
  }
}

/// Global app state provider
final appStateProvider = StateNotifierProvider<AppStateNotifier, AppState>((ref) {
  return AppStateNotifier();
});

/// Connectivity provider
final connectivityProvider = StreamProvider<ConnectivityResult>((ref) {
  return Connectivity().onConnectivityChanged;
});

/// Online status provider
final isOnlineProvider = Provider<bool>((ref) {
  final connectivity = ref.watch(connectivityProvider);
  return connectivity.when(
    data: (result) => result != ConnectivityResult.none,
    loading: () => true,
    error: (_, __) => false,
  );
});
