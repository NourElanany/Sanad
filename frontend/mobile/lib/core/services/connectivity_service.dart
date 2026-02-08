import 'dart:async';
import 'package:connectivity_plus/connectivity_plus.dart';
import 'package:flutter/foundation.dart';

/// Service for monitoring network connectivity
class ConnectivityService {
  static final ConnectivityService _instance = ConnectivityService._internal();
  factory ConnectivityService() => _instance;
  ConnectivityService._internal();
  
  final Connectivity _connectivity = Connectivity();
  
  StreamController<ConnectivityStatus>? _connectivityController;
  StreamSubscription<List<ConnectivityResult>>? _connectivitySubscription;
  
  ConnectivityStatus _currentStatus = ConnectivityStatus.unknown;
  
  /// Get current connectivity status
  ConnectivityStatus get currentStatus => _currentStatus;
  
  /// Stream of connectivity status changes
  Stream<ConnectivityStatus> get onConnectivityChanged {
    _connectivityController ??= StreamController<ConnectivityStatus>.broadcast();
    return _connectivityController!.stream;
  }
  
  /// Initialize connectivity monitoring
  Future<void> init() async {
    // Get initial connectivity status
    await _updateConnectivityStatus();
    
    // Listen to connectivity changes
    _connectivitySubscription = _connectivity.onConnectivityChanged.listen(
      (List<ConnectivityResult> results) {
        _handleConnectivityChange(results);
      },
    );
    
    if (kDebugMode) {
      print('📡 Connectivity service initialized');
      print('📡 Current status: ${_currentStatus.name}');
    }
  }
  
  /// Dispose connectivity service
  void dispose() {
    _connectivitySubscription?.cancel();
    _connectivityController?.close();
  }
  
  /// Check if device is connected to internet
  Future<bool> isConnected() async {
    await _updateConnectivityStatus();
    return _currentStatus == ConnectivityStatus.connected;
  }
  
  /// Check if device is connected via WiFi
  Future<bool> isWiFi() async {
    final results = await _connectivity.checkConnectivity();
    return results.contains(ConnectivityResult.wifi);
  }
  
  /// Check if device is connected via mobile data
  Future<bool> isMobile() async {
    final results = await _connectivity.checkConnectivity();
    return results.contains(ConnectivityResult.mobile);
  }
  
  /// Update connectivity status
  Future<void> _updateConnectivityStatus() async {
    try {
      final results = await _connectivity.checkConnectivity();
      _handleConnectivityChange(results);
    } catch (e) {
      if (kDebugMode) {
        print('❌ Error checking connectivity: $e');
      }
      _updateStatus(ConnectivityStatus.unknown);
    }
  }
  
  /// Handle connectivity change
  void _handleConnectivityChange(List<ConnectivityResult> results) {
    if (results.isEmpty || results.contains(ConnectivityResult.none)) {
      _updateStatus(ConnectivityStatus.disconnected);
    } else if (results.contains(ConnectivityResult.wifi)) {
      _updateStatus(ConnectivityStatus.connected);
    } else if (results.contains(ConnectivityResult.mobile)) {
      _updateStatus(ConnectivityStatus.connected);
    } else if (results.contains(ConnectivityResult.ethernet)) {
      _updateStatus(ConnectivityStatus.connected);
    } else {
      _updateStatus(ConnectivityStatus.unknown);
    }
  }
  
  /// Update status and notify listeners
  void _updateStatus(ConnectivityStatus newStatus) {
    if (_currentStatus != newStatus) {
      final oldStatus = _currentStatus;
      _currentStatus = newStatus;
      
      if (kDebugMode) {
        print('📡 Connectivity changed: ${oldStatus.name} → ${newStatus.name}');
      }
      
      _connectivityController?.add(newStatus);
    }
  }
}

/// Connectivity status enum
enum ConnectivityStatus {
  /// Device is connected to internet
  connected,
  
  /// Device is not connected to internet
  disconnected,
  
  /// Connectivity status is unknown
  unknown,
}

/// Extension for connectivity status
extension ConnectivityStatusExtension on ConnectivityStatus {
  /// Check if connected
  bool get isConnected => this == ConnectivityStatus.connected;
  
  /// Check if disconnected
  bool get isDisconnected => this == ConnectivityStatus.disconnected;
  
  /// Get user-friendly message
  String get message {
    switch (this) {
      case ConnectivityStatus.connected:
        return 'Connected to internet';
      case ConnectivityStatus.disconnected:
        return 'No internet connection';
      case ConnectivityStatus.unknown:
        return 'Checking connection...';
    }
  }
  
  /// Get icon for status
  String get icon {
    switch (this) {
      case ConnectivityStatus.connected:
        return '✅';
      case ConnectivityStatus.disconnected:
        return '❌';
      case ConnectivityStatus.unknown:
        return '❓';
    }
  }
}
