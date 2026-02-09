import 'package:flutter_test/flutter_test.dart';
import 'package:integration_test/integration_test.dart';
import 'package:dio/dio.dart';
import '../../lib/core/network/dio_client.dart';
import '../../lib/core/services/auth_service.dart';
import '../../lib/core/services/local_storage_service.dart';

/// Integration tests for authentication flows
/// **Validates: Requirements 20.1, 20.3**
void main() {
  IntegrationTestWidgetsFlutterBinding.ensureInitialized();

  group('Authentication Integration Tests', () {
    late DioClient dioClient;
    late AuthService authService;
    late LocalStorageService localStorageService;

    setUpAll(() async {
      dioClient = DioClient(baseUrl: 'https://api.sanad.app');
      authService = AuthService(dioClient);
      localStorageService = await LocalStorageService.init();
    });

    tearDown(() async {
      // Clean up after each test
      await authService.logout();
      await localStorageService.clear();
    });

    group('User Registration Flow', () {
      test('should register new user with valid data', () async {
        // Arrange
        final testEmail = 'test_${DateTime.now().millisecondsSinceEpoch}@example.com';
        final testPassword = 'SecurePassword123!';
        final testName = 'Test User';

        // Act
        final result = await authService.register(
          email: testEmail,
          password: testPassword,
          name: testName,
        );

        // Assert
        expect(result.success, isTrue);
        expect(result.user, isNotNull);
        expect(result.user!.email, equals(testEmail));
        expect(result.user!.name, equals(testName));
        expect(result.accessToken, isNotNull);
        expect(result.refreshToken, isNotNull);
        expect(result.accessToken!.length, greaterThan(20));
      });

      test('should fail registration with invalid email', () async {
        // Act & Assert
        expect(
          () => authService.register(
            email: 'invalid-email',
            password: 'ValidPassword123!',
            name: 'Test User',
          ),
          throwsA(
            predicate((e) =>
                e is DioException &&
                e.response?.statusCode == 422),
          ),
        );
      });

      test('should fail registration with weak password', () async {
        // Act & Assert
        expect(
          () => authService.register(
            email: 'test@example.com',
            password: '123',
            name: 'Test User',
          ),
          throwsA(
            predicate((e) =>
                e is DioException &&
                e.response?.statusCode == 422),
          ),
        );
      });

      test('should fail registration with duplicate email', () async {
        // Arrange
        final testEmail = 'duplicate_${DateTime.now().millisecondsSinceEpoch}@example.com';
        
        await authService.register(
          email: testEmail,
          password: 'Password123!',
          name: 'First User',
        );

        // Act & Assert
        expect(
          () => authService.register(
            email: testEmail,
            password: 'Password123!',
            name: 'Second User',
          ),
          throwsA(
            predicate((e) =>
                e is DioException &&
                e.response?.statusCode == 409),
          ),
        );
      });

      test('should store tokens after successful registration', () async {
        // Arrange
        final testEmail = 'storage_test_${DateTime.now().millisecondsSinceEpoch}@example.com';

        // Act
        final result = await authService.register(
          email: testEmail,
          password: 'Password123!',
          name: 'Storage Test',
        );

        // Assert
        final storedAccessToken = await localStorageService.getAccessToken();
        final storedRefreshToken = await localStorageService.getRefreshToken();
        
        expect(storedAccessToken, equals(result.accessToken));
        expect(storedRefreshToken, equals(result.refreshToken));
      });
    });

    group('User Login Flow', () {
      late String testEmail;
      late String testPassword;

      setUp(() async {
        // Create a test user for login tests
        testEmail = 'login_test_${DateTime.now().millisecondsSinceEpoch}@example.com';
        testPassword = 'LoginPassword123!';
        
        await authService.register(
          email: testEmail,
          password: testPassword,
          name: 'Login Test User',
        );
        
        // Logout to test login
        await authService.logout();
      });

      test('should login with valid credentials', () async {
        // Act
        final result = await authService.login(
          email: testEmail,
          password: testPassword,
        );

        // Assert
        expect(result.success, isTrue);
        expect(result.accessToken, isNotNull);
        expect(result.refreshToken, isNotNull);
        expect(result.user, isNotNull);
        expect(result.user!.email, equals(testEmail));
      });

      test('should fail login with invalid email', () async {
        // Act & Assert
        expect(
          () => authService.login(
            email: 'nonexistent@example.com',
            password: testPassword,
          ),
          throwsA(
            predicate((e) =>
                e is DioException &&
                e.response?.statusCode == 401),
          ),
        );
      });

      test('should fail login with wrong password', () async {
        // Act & Assert
        expect(
          () => authService.login(
            email: testEmail,
            password: 'WrongPassword123!',
          ),
          throwsA(
            predicate((e) =>
                e is DioException &&
                e.response?.statusCode == 401),
          ),
        );
      });

      test('should update last login timestamp', () async {
        // Arrange
        final beforeLogin = DateTime.now();

        // Act
        await authService.login(
          email: testEmail,
          password: testPassword,
        );

        // Assert
        final user = await authService.getCurrentUser();
        expect(user, isNotNull);
        expect(user!.lastLoginAt, isNotNull);
        expect(
          user.lastLoginAt!.isAfter(beforeLogin.subtract(const Duration(seconds: 5))),
          isTrue,
        );
      });

      test('should maintain session across app restarts', () async {
        // Arrange - Login
        await authService.login(
          email: testEmail,
          password: testPassword,
        );

        // Act - Simulate app restart by creating new service instance
        final newAuthService = AuthService(dioClient);
        await newAuthService.loadStoredTokens();

        // Assert
        expect(await newAuthService.isAuthenticated(), isTrue);
        final user = await newAuthService.getCurrentUser();
        expect(user, isNotNull);
        expect(user!.email, equals(testEmail));
      });
    });

    group('Token Refresh Flow', () {
      late String testEmail;
      late String testPassword;

      setUp(() async {
        testEmail = 'refresh_test_${DateTime.now().millisecondsSinceEpoch}@example.com';
        testPassword = 'RefreshPassword123!';
        
        await authService.register(
          email: testEmail,
          password: testPassword,
          name: 'Refresh Test User',
        );
      });

      test('should refresh access token with valid refresh token', () async {
        // Arrange
        final originalAccessToken = await localStorageService.getAccessToken();

        // Wait a moment to ensure new token is different
        await Future.delayed(const Duration(milliseconds: 100));

        // Act
        final newAccessToken = await authService.refreshAccessToken();

        // Assert
        expect(newAccessToken, isNotNull);
        expect(newAccessToken, isNot(equals(originalAccessToken)));
        
        final storedToken = await localStorageService.getAccessToken();
        expect(storedToken, equals(newAccessToken));
      });

      test('should automatically refresh expired token', () async {
        // Arrange - Manually expire the token
        await localStorageService.storeAccessToken('expired_token');

        // Act - Make an authenticated request
        final user = await authService.getCurrentUser();

        // Assert - Should have refreshed token and succeeded
        expect(user, isNotNull);
        
        final newToken = await localStorageService.getAccessToken();
        expect(newToken, isNot(equals('expired_token')));
      });

      test('should fail refresh with invalid refresh token', () async {
        // Arrange
        await localStorageService.storeRefreshToken('invalid_refresh_token');

        // Act & Assert
        expect(
          () => authService.refreshAccessToken(),
          throwsA(
            predicate((e) =>
                e is DioException &&
                e.response?.statusCode == 401),
          ),
        );
      });

      test('should logout when refresh token is expired', () async {
        // Arrange
        await localStorageService.storeRefreshToken('expired_refresh_token');

        // Act
        try {
          await authService.refreshAccessToken();
        } catch (e) {
          // Expected to fail
        }

        // Assert - Should have logged out
        expect(await authService.isAuthenticated(), isFalse);
        expect(await localStorageService.getAccessToken(), isNull);
      });
    });

    group('Logout Flow', () {
      late String testEmail;
      late String testPassword;

      setUp(() async {
        testEmail = 'logout_test_${DateTime.now().millisecondsSinceEpoch}@example.com';
        testPassword = 'LogoutPassword123!';
        
        await authService.register(
          email: testEmail,
          password: testPassword,
          name: 'Logout Test User',
        );
      });

      test('should clear all tokens on logout', () async {
        // Act
        await authService.logout();

        // Assert
        expect(await localStorageService.getAccessToken(), isNull);
        expect(await localStorageService.getRefreshToken(), isNull);
        expect(await authService.isAuthenticated(), isFalse);
      });

      test('should invalidate token on server', () async {
        // Arrange
        final accessToken = await localStorageService.getAccessToken();

        // Act
        await authService.logout();

        // Assert - Try to use old token
        await localStorageService.storeAccessToken(accessToken!);
        
        expect(
          () => authService.getCurrentUser(),
          throwsA(
            predicate((e) =>
                e is DioException &&
                e.response?.statusCode == 401),
          ),
        );
      });

      test('should clear user data on logout', () async {
        // Arrange
        await localStorageService.storeUserPreferences({
          'theme': 'dark',
          'language': 'ar',
        });

        // Act
        await authService.logout();

        // Assert
        final preferences = await localStorageService.getUserPreferences();
        expect(preferences, isEmpty);
      });
    });

    group('Password Reset Flow', () {
      late String testEmail;

      setUp(() async {
        testEmail = 'reset_test_${DateTime.now().millisecondsSinceEpoch}@example.com';
        
        await authService.register(
          email: testEmail,
          password: 'OriginalPassword123!',
          name: 'Reset Test User',
        );
        
        await authService.logout();
      });

      test('should send password reset email', () async {
        // Act
        final result = await authService.requestPasswordReset(testEmail);

        // Assert
        expect(result.success, isTrue);
        expect(result.message, contains('email'));
      });

      test('should fail reset for non-existent email', () async {
        // Act & Assert
        expect(
          () => authService.requestPasswordReset('nonexistent@example.com'),
          throwsA(
            predicate((e) =>
                e is DioException &&
                e.response?.statusCode == 404),
          ),
        );
      });

      test('should reset password with valid token', () async {
        // Arrange
        await authService.requestPasswordReset(testEmail);
        final resetToken = 'valid_reset_token'; // In real test, get from email

        // Act
        final result = await authService.resetPassword(
          token: resetToken,
          newPassword: 'NewPassword123!',
        );

        // Assert
        expect(result.success, isTrue);
        
        // Verify can login with new password
        final loginResult = await authService.login(
          email: testEmail,
          password: 'NewPassword123!',
        );
        expect(loginResult.success, isTrue);
      });
    });

    group('Multi-Device Authentication', () {
      late String testEmail;
      late String testPassword;

      setUp(() async {
        testEmail = 'multidevice_${DateTime.now().millisecondsSinceEpoch}@example.com';
        testPassword = 'MultiDevice123!';
        
        await authService.register(
          email: testEmail,
          password: testPassword,
          name: 'Multi Device User',
        );
      });

      test('should allow login from multiple devices', () async {
        // Arrange - Simulate second device
        final device2AuthService = AuthService(dioClient);

        // Act - Login from second device
        final result = await device2AuthService.login(
          email: testEmail,
          password: testPassword,
        );

        // Assert - Both devices should be authenticated
        expect(result.success, isTrue);
        expect(await authService.isAuthenticated(), isTrue);
        expect(await device2AuthService.isAuthenticated(), isTrue);
      });

      test('should list active sessions', () async {
        // Arrange - Login from multiple devices
        final device2AuthService = AuthService(dioClient);
        await device2AuthService.login(
          email: testEmail,
          password: testPassword,
        );

        // Act
        final sessions = await authService.getActiveSessions();

        // Assert
        expect(sessions.length, greaterThanOrEqualTo(2));
        expect(
          sessions.every((s) => s.userId == authService.currentUserId),
          isTrue,
        );
      });

      test('should revoke specific session', () async {
        // Arrange
        final device2AuthService = AuthService(dioClient);
        await device2AuthService.login(
          email: testEmail,
          password: testPassword,
        );

        final sessions = await authService.getActiveSessions();
        final sessionToRevoke = sessions.last;

        // Act
        await authService.revokeSession(sessionToRevoke.id);

        // Assert
        final updatedSessions = await authService.getActiveSessions();
        expect(
          updatedSessions.any((s) => s.id == sessionToRevoke.id),
          isFalse,
        );
      });

      test('should revoke all other sessions', () async {
        // Arrange - Login from multiple devices
        final device2AuthService = AuthService(dioClient);
        await device2AuthService.login(
          email: testEmail,
          password: testPassword,
        );

        // Act
        await authService.revokeAllOtherSessions();

        // Assert
        final sessions = await authService.getActiveSessions();
        expect(sessions.length, equals(1));
        expect(await authService.isAuthenticated(), isTrue);
        expect(await device2AuthService.isAuthenticated(), isFalse);
      });
    });

    group('Security Features', () {
      test('should enforce rate limiting on login attempts', () async {
        // Arrange
        final testEmail = 'ratelimit_${DateTime.now().millisecondsSinceEpoch}@example.com';
        
        await authService.register(
          email: testEmail,
          password: 'Password123!',
          name: 'Rate Limit Test',
        );
        
        await authService.logout();

        // Act - Make multiple failed login attempts
        var rateLimitHit = false;
        for (var i = 0; i < 10; i++) {
          try {
            await authService.login(
              email: testEmail,
              password: 'WrongPassword',
            );
          } catch (e) {
            if (e is DioException && e.response?.statusCode == 429) {
              rateLimitHit = true;
              break;
            }
          }
        }

        // Assert
        expect(rateLimitHit, isTrue);
      });

      test('should detect suspicious login activity', () async {
        // Arrange
        final testEmail = 'suspicious_${DateTime.now().millisecondsSinceEpoch}@example.com';
        
        await authService.register(
          email: testEmail,
          password: 'Password123!',
          name: 'Suspicious Test',
        );

        // Act - Login from unusual location
        final result = await authService.login(
          email: testEmail,
          password: 'Password123!',
          metadata: {
            'location': 'Unusual Country',
            'device': 'Unknown Device',
          },
        );

        // Assert - Should require additional verification
        expect(result.requiresVerification, isTrue);
        expect(result.verificationMethod, isNotNull);
      });

      test('should encrypt sensitive data in storage', () async {
        // Arrange
        final testEmail = 'encryption_${DateTime.now().millisecondsSinceEpoch}@example.com';
        
        await authService.register(
          email: testEmail,
          password: 'Password123!',
          name: 'Encryption Test',
        );

        // Act
        final rawStoredToken = await localStorageService.getRawValue('access_token');

        // Assert - Token should be encrypted
        expect(rawStoredToken, isNot(equals(await localStorageService.getAccessToken())));
        expect(rawStoredToken, isNot(contains('eyJ'))); // Not a plain JWT
      });
    });
  });
}
