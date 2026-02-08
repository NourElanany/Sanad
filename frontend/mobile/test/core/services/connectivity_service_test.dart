import 'package:flutter_test/flutter_test.dart';
import 'package:sanad_mobile/core/services/connectivity_service.dart';

void main() {
  group('ConnectivityService', () {
    late ConnectivityService connectivityService;

    setUp(() {
      connectivityService = ConnectivityService();
    });

    test('should initialize successfully', () async {
      await connectivityService.init();
      
      expect(connectivityService.currentStatus, isA<ConnectivityStatus>());
    });

    test('should provide connectivity status stream', () {
      final stream = connectivityService.onConnectivityChanged;
      
      expect(stream, isA<Stream<ConnectivityStatus>>());
    });

    test('should check if connected', () async {
      final isConnected = await connectivityService.isConnected();
      
      expect(isConnected, isA<bool>());
    });

    test('should check if WiFi', () async {
      final isWiFi = await connectivityService.isWiFi();
      
      expect(isWiFi, isA<bool>());
    });

    test('should check if mobile data', () async {
      final isMobile = await connectivityService.isMobile();
      
      expect(isMobile, isA<bool>());
    });

    tearDown(() {
      connectivityService.dispose();
    });
  });

  group('ConnectivityStatus', () {
    test('should have correct enum values', () {
      expect(ConnectivityStatus.connected, isA<ConnectivityStatus>());
      expect(ConnectivityStatus.disconnected, isA<ConnectivityStatus>());
      expect(ConnectivityStatus.unknown, isA<ConnectivityStatus>());
    });
  });

  group('ConnectivityStatusExtension', () {
    test('should check if connected', () {
      expect(ConnectivityStatus.connected.isConnected, isTrue);
      expect(ConnectivityStatus.disconnected.isConnected, isFalse);
      expect(ConnectivityStatus.unknown.isConnected, isFalse);
    });

    test('should check if disconnected', () {
      expect(ConnectivityStatus.disconnected.isDisconnected, isTrue);
      expect(ConnectivityStatus.connected.isDisconnected, isFalse);
      expect(ConnectivityStatus.unknown.isDisconnected, isFalse);
    });

    test('should provide user-friendly messages', () {
      expect(
        ConnectivityStatus.connected.message,
        equals('Connected to internet'),
      );
      expect(
        ConnectivityStatus.disconnected.message,
        equals('No internet connection'),
      );
      expect(
        ConnectivityStatus.unknown.message,
        equals('Checking connection...'),
      );
    });

    test('should provide status icons', () {
      expect(ConnectivityStatus.connected.icon, equals('✅'));
      expect(ConnectivityStatus.disconnected.icon, equals('❌'));
      expect(ConnectivityStatus.unknown.icon, equals('❓'));
    });
  });
}
