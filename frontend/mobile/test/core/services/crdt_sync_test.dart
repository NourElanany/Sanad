import 'package:flutter_test/flutter_test.dart';
import '../../../lib/core/services/conflict_resolution_service.dart';

void main() {
  group('ConflictResolutionService', () {
    late ConflictResolutionService service;

    setUp(() {
      service = ConflictResolutionService();
    });

    group('Last-Write-Wins Resolution', () {
      test('should keep data with latest timestamp', () {
        final localData = {
          'value': 'local_value',
          'updated_at': '2024-01-01T10:00:00Z',
          'device_id': 'device_1',
        };

        final remoteData = {
          'value': 'remote_value',
          'updated_at': '2024-01-01T11:00:00Z',
          'device_id': 'device_2',
        };

        final result = service.resolveConflict(
          dataType: 'user_preferences',
          localData: localData,
          remoteData: remoteData,
        );

        expect(result.resolvedData['value'], equals('remote_value'));
        expect(result.hadConflict, isTrue);
        expect(result.resolutionStrategy, equals('last_write_wins'));
      });

      test('should use device_id as tiebreaker for same timestamp', () {
        final localData = {
          'value': 'local_value',
          'updated_at': '2024-01-01T10:00:00Z',
          'device_id': 'device_2',
        };

        final remoteData = {
          'value': 'remote_value',
          'updated_at': '2024-01-01T10:00:00Z',
          'device_id': 'device_1',
        };

        final result = service.resolveConflict(
          dataType: 'user_preferences',
          localData: localData,
          remoteData: remoteData,
        );

        expect(result.resolvedData['value'], equals('local_value'));
        expect(result.resolutionStrategy,
            equals('last_write_wins_device_tiebreak'));
      });
    });

    group('Set Union Resolution', () {
      test('should merge bookmark lists', () {
        final localData = {
          'bookmarks': [
            {'id': '1', 'surah': 2},
            {'id': '2', 'surah': 3},
          ],
          'updated_at': '2024-01-01T10:00:00Z',
        };

        final remoteData = {
          'bookmarks': [
            {'id': '2', 'surah': 3},
            {'id': '3', 'surah': 4},
          ],
          'updated_at': '2024-01-01T11:00:00Z',
        };

        final result = service.resolveConflict(
          dataType: 'bookmarks',
          localData: localData,
          remoteData: remoteData,
        );

        final mergedBookmarks = result.resolvedData['bookmarks'] as List;
        expect(mergedBookmarks.length, equals(3));
        expect(result.hadConflict, isTrue);
        expect(result.resolutionStrategy, equals('set_union'));
      });

      test('should handle empty local set', () {
        final localData = {
          'bookmarks': <dynamic>[],
          'updated_at': '2024-01-01T10:00:00Z',
        };

        final remoteData = {
          'bookmarks': [
            {'id': '1', 'surah': 2},
          ],
          'updated_at': '2024-01-01T11:00:00Z',
        };

        final result = service.resolveConflict(
          dataType: 'bookmarks',
          localData: localData,
          remoteData: remoteData,
        );

        final mergedBookmarks = result.resolvedData['bookmarks'] as List;
        expect(mergedBookmarks.length, equals(1));
        expect(result.hadConflict, isTrue);
      });
    });

    group('Max Value Resolution', () {
      test('should keep maximum reading progress', () {
        final localData = {
          'last_ayah_read': 50,
          'completion_percentage': 45.5,
          'updated_at': '2024-01-01T10:00:00Z',
        };

        final remoteData = {
          'last_ayah_read': 75,
          'completion_percentage': 68.2,
          'updated_at': '2024-01-01T11:00:00Z',
        };

        final result = service.resolveConflict(
          dataType: 'reading_progress',
          localData: localData,
          remoteData: remoteData,
        );

        expect(result.resolvedData['last_ayah_read'], equals(75));
        expect(result.resolvedData['completion_percentage'], equals(68.2));
        expect(result.hadConflict, isTrue);
        expect(result.resolutionStrategy, equals('max_value'));
      });

      test('should handle mixed max values', () {
        final localData = {
          'surah_1_progress': 100,
          'surah_2_progress': 30,
          'updated_at': '2024-01-01T10:00:00Z',
        };

        final remoteData = {
          'surah_1_progress': 80,
          'surah_2_progress': 50,
          'updated_at': '2024-01-01T11:00:00Z',
        };

        final result = service.resolveConflict(
          dataType: 'reading_progress',
          localData: localData,
          remoteData: remoteData,
        );

        expect(result.resolvedData['surah_1_progress'], equals(100));
        expect(result.resolvedData['surah_2_progress'], equals(50));
        expect(result.hadConflict, isTrue);
      });
    });

    group('Custom Reading Progress Resolution', () {
      test('should keep furthest progress for each surah', () {
        final localData = {
          'quran_progress': {
            '2': {
              'last_ayah_read': 100,
              'last_read_at': '2024-01-01T10:00:00Z',
            },
            '3': {
              'last_ayah_read': 50,
              'last_read_at': '2024-01-01T09:00:00Z',
            },
          },
        };

        final remoteData = {
          'quran_progress': {
            '2': {
              'last_ayah_read': 80,
              'last_read_at': '2024-01-01T11:00:00Z',
            },
            '3': {
              'last_ayah_read': 75,
              'last_read_at': '2024-01-01T11:00:00Z',
            },
            '4': {
              'last_ayah_read': 20,
              'last_read_at': '2024-01-01T11:00:00Z',
            },
          },
        };

        final result = service.resolveConflict(
          dataType: 'reading_progress_detailed',
          localData: localData,
          remoteData: remoteData,
        );

        final progress =
            result.resolvedData['quran_progress'] as Map<String, dynamic>;

        // Surah 2: local has higher ayah (100 > 80)
        expect(progress['2']['last_ayah_read'], equals(100));

        // Surah 3: remote has higher ayah (75 > 50)
        expect(progress['3']['last_ayah_read'], equals(75));

        // Surah 4: only in remote
        expect(progress['4']['last_ayah_read'], equals(20));

        expect(result.hadConflict, isTrue);
      });
    });

    group('Version Vector Operations', () {
      test('should merge version vectors correctly', () {
        final local = {
          'device_1': 5,
          'device_2': 3,
        };

        final remote = {
          'device_1': 4,
          'device_2': 6,
          'device_3': 2,
        };

        final merged = service.mergeVersionVectors(local, remote);

        expect(merged['device_1'], equals(5)); // max(5, 4)
        expect(merged['device_2'], equals(6)); // max(3, 6)
        expect(merged['device_3'], equals(2)); // only in remote
      });

      test('should detect concurrent versions', () {
        final local = {
          'device_1': 5,
          'device_2': 3,
        };

        final remote = {
          'device_1': 4,
          'device_2': 6,
        };

        final isConcurrent = service.areVersionsConcurrent(local, remote);

        expect(isConcurrent, isTrue);
      });

      test('should detect non-concurrent versions', () {
        final local = {
          'device_1': 5,
          'device_2': 6,
        };

        final remote = {
          'device_1': 4,
          'device_2': 3,
        };

        final isConcurrent = service.areVersionsConcurrent(local, remote);

        expect(isConcurrent, isFalse);
      });
    });

    group('Khatma Progress Resolution', () {
      test('should keep khatma with more completed portions', () {
        final localData = {
          'khatma_id': 'khatma_1',
          'completed_portions': 15,
          'total_portions': 30,
          'updated_at': '2024-01-01T10:00:00Z',
        };

        final remoteData = {
          'khatma_id': 'khatma_1',
          'completed_portions': 20,
          'total_portions': 30,
          'updated_at': '2024-01-01T11:00:00Z',
        };

        final result = service.resolveConflict(
          dataType: 'khatma_plan',
          localData: localData,
          remoteData: remoteData,
        );

        expect(result.resolvedData['completed_portions'], equals(20));
        expect(result.hadConflict, isTrue);
        expect(result.resolutionStrategy, equals('khatma_max_completion'));
      });
    });

    group('Personal Notes Resolution', () {
      test('should keep latest version of each note', () {
        final localData = {
          'notes': {
            'note_1': {
              'text': 'Local note 1',
              'updated_at': '2024-01-01T10:00:00Z',
            },
            'note_2': {
              'text': 'Local note 2',
              'updated_at': '2024-01-01T12:00:00Z',
            },
          },
        };

        final remoteData = {
          'notes': {
            'note_1': {
              'text': 'Remote note 1',
              'updated_at': '2024-01-01T11:00:00Z',
            },
            'note_3': {
              'text': 'Remote note 3',
              'updated_at': '2024-01-01T11:00:00Z',
            },
          },
        };

        final result = service.resolveConflict(
          dataType: 'personal_notes',
          localData: localData,
          remoteData: remoteData,
        );

        final notes = result.resolvedData['notes'] as Map<String, dynamic>;

        // Note 1: remote is newer
        expect(notes['note_1']['text'], equals('Remote note 1'));

        // Note 2: only in local
        expect(notes['note_2']['text'], equals('Local note 2'));

        // Note 3: only in remote
        expect(notes['note_3']['text'], equals('Remote note 3'));

        expect(result.hadConflict, isTrue);
      });
    });
  });
}
