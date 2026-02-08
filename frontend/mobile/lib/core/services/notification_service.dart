import 'package:flutter_local_notifications/flutter_local_notifications.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'package:shared_preferences/shared_preferences.dart';
import 'prayer_times_service.dart';

/// Provider for notification service
final notificationServiceProvider = Provider<NotificationService>((ref) {
  return NotificationService();
});

/// Service for managing prayer time notifications
class NotificationService {
  static final FlutterLocalNotificationsPlugin _notifications =
      FlutterLocalNotificationsPlugin();
  
  static const String _notificationsEnabledKey = 'prayer_notifications_enabled';
  static bool _initialized = false;

  /// Initialize the notification service
  Future<void> initialize() async {
    if (_initialized) return;

    const androidSettings = AndroidInitializationSettings('@mipmap/ic_launcher');
    const iosSettings = DarwinInitializationSettings(
      requestAlertPermission: true,
      requestBadgePermission: true,
      requestSoundPermission: true,
    );

    const initSettings = InitializationSettings(
      android: androidSettings,
      iOS: iosSettings,
    );

    await _notifications.initialize(
      initSettings,
      onDidReceiveNotificationResponse: _onNotificationTapped,
    );

    _initialized = true;
  }

  static void _onNotificationTapped(NotificationResponse response) {
    // Handle notification tap
    // Navigate to prayer times screen or perform action
  }

  /// Check if notifications are enabled
  Future<bool> areNotificationsEnabled() async {
    final prefs = await SharedPreferences.getInstance();
    return prefs.getBool(_notificationsEnabledKey) ?? false;
  }

  /// Enable prayer time notifications
  Future<void> enablePrayerNotifications(PrayerTimes prayerTimes) async {
    await initialize();
    
    // Request permissions
    final granted = await _requestPermissions();
    if (!granted) return;

    // Schedule notifications for all prayer times
    await _schedulePrayerNotifications(prayerTimes);

    // Save preference
    final prefs = await SharedPreferences.getInstance();
    await prefs.setBool(_notificationsEnabledKey, true);
  }

  /// Disable prayer time notifications
  Future<void> disablePrayerNotifications() async {
    await _notifications.cancelAll();
    
    final prefs = await SharedPreferences.getInstance();
    await prefs.setBool(_notificationsEnabledKey, false);
  }

  /// Request notification permissions
  Future<bool> _requestPermissions() async {
    final androidPlugin = _notifications.resolvePlatformSpecificImplementation<
        AndroidFlutterLocalNotificationsPlugin>();
    
    if (androidPlugin != null) {
      final granted = await androidPlugin.requestPermission();
      if (granted != true) return false;
    }

    final iosPlugin = _notifications.resolvePlatformSpecificImplementation<
        IOSFlutterLocalNotificationsPlugin>();
    
    if (iosPlugin != null) {
      final granted = await iosPlugin.requestPermissions(
        alert: true,
        badge: true,
        sound: true,
      );
      if (granted != true) return false;
    }

    return true;
  }

  /// Schedule notifications for all prayer times
  Future<void> _schedulePrayerNotifications(PrayerTimes prayerTimes) async {
    final prayers = [
      {'id': 1, 'name': 'الفجر', 'time': prayerTimes.fajr},
      {'id': 2, 'name': 'الظهر', 'time': prayerTimes.dhuhr},
      {'id': 3, 'name': 'العصر', 'time': prayerTimes.asr},
      {'id': 4, 'name': 'المغرب', 'time': prayerTimes.maghrib},
      {'id': 5, 'name': 'العشاء', 'time': prayerTimes.isha},
    ];

    for (final prayer in prayers) {
      await _schedulePrayerNotification(
        id: prayer['id'] as int,
        prayerName: prayer['name'] as String,
        prayerTime: prayer['time'] as String,
      );
    }
  }

  /// Schedule a single prayer notification
  Future<void> _schedulePrayerNotification({
    required int id,
    required String prayerName,
    required String prayerTime,
  }) async {
    final scheduledTime = _parseTimeToDateTime(prayerTime);
    
    // If the time has passed today, schedule for tomorrow
    final now = DateTime.now();
    final notificationTime = scheduledTime.isBefore(now)
        ? scheduledTime.add(const Duration(days: 1))
        : scheduledTime;

    const androidDetails = AndroidNotificationDetails(
      'prayer_times',
      'أوقات الصلاة',
      channelDescription: 'تنبيهات أوقات الصلاة',
      importance: Importance.high,
      priority: Priority.high,
      sound: RawResourceAndroidNotificationSound('adhan'),
      playSound: true,
      enableVibration: true,
    );

    const iosDetails = DarwinNotificationDetails(
      sound: 'adhan.aiff',
      presentAlert: true,
      presentBadge: true,
      presentSound: true,
    );

    const details = NotificationDetails(
      android: androidDetails,
      iOS: iosDetails,
    );

    await _notifications.zonedSchedule(
      id,
      'حان وقت صلاة $prayerName',
      'الصلاة خير من النوم',
      notificationTime,
      details,
      androidScheduleMode: AndroidScheduleMode.exactAllowWhileIdle,
      uiLocalNotificationDateInterpretation:
          UILocalNotificationDateInterpretation.absoluteTime,
      matchDateTimeComponents: DateTimeComponents.time,
    );
  }

  /// Parse time string to DateTime
  DateTime _parseTimeToDateTime(String time) {
    final parts = time.split(':');
    final now = DateTime.now();
    return DateTime(
      now.year,
      now.month,
      now.day,
      int.parse(parts[0]),
      int.parse(parts[1]),
    );
  }

  /// Schedule a reminder notification (e.g., 10 minutes before prayer)
  Future<void> scheduleReminderNotification({
    required int id,
    required String prayerName,
    required String prayerTime,
    required int minutesBefore,
  }) async {
    final prayerDateTime = _parseTimeToDateTime(prayerTime);
    final reminderTime = prayerDateTime.subtract(Duration(minutes: minutesBefore));
    
    final now = DateTime.now();
    final notificationTime = reminderTime.isBefore(now)
        ? reminderTime.add(const Duration(days: 1))
        : reminderTime;

    const androidDetails = AndroidNotificationDetails(
      'prayer_reminders',
      'تذكير الصلاة',
      channelDescription: 'تذكير قبل أوقات الصلاة',
      importance: Importance.defaultImportance,
      priority: Priority.defaultPriority,
    );

    const iosDetails = DarwinNotificationDetails(
      presentAlert: true,
      presentBadge: true,
      presentSound: true,
    );

    const details = NotificationDetails(
      android: androidDetails,
      iOS: iosDetails,
    );

    await _notifications.zonedSchedule(
      id + 100, // Offset ID to avoid conflicts
      'تذكير: صلاة $prayerName',
      'باقي $minutesBefore دقيقة على صلاة $prayerName',
      notificationTime,
      details,
      androidScheduleMode: AndroidScheduleMode.exactAllowWhileIdle,
      uiLocalNotificationDateInterpretation:
          UILocalNotificationDateInterpretation.absoluteTime,
      matchDateTimeComponents: DateTimeComponents.time,
    );
  }

  /// Show immediate notification
  Future<void> showNotification({
    required int id,
    required String title,
    required String body,
  }) async {
    await initialize();

    const androidDetails = AndroidNotificationDetails(
      'general',
      'عام',
      channelDescription: 'إشعارات عامة',
      importance: Importance.defaultImportance,
      priority: Priority.defaultPriority,
    );

    const iosDetails = DarwinNotificationDetails();

    const details = NotificationDetails(
      android: androidDetails,
      iOS: iosDetails,
    );

    await _notifications.show(id, title, body, details);
  }
}
