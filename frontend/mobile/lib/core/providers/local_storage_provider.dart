import 'package:flutter_riverpod/flutter_riverpod.dart';
import '../services/local_storage_service.dart';
import '../services/download_manager_service.dart';

/// Local storage service provider
final localStorageServiceProvider = Provider<LocalStorageService>((ref) {
  throw UnimplementedError('localStorageServiceProvider must be overridden');
});

/// Download manager service provider
final downloadManagerServiceProvider = Provider<DownloadManagerService>((ref) {
  final storageService = ref.watch(localStorageServiceProvider);
  return DownloadManagerService(storageService);
});

/// Storage statistics provider
final storageStatsProvider = FutureProvider<StorageStats>((ref) async {
  final storageService = ref.watch(localStorageServiceProvider);
  return await storageService.getStats();
});

/// Downloads stream provider
final downloadsStreamProvider = StreamProvider<List<DownloadItem>>((ref) {
  final downloadManager = ref.watch(downloadManagerServiceProvider);
  return downloadManager.downloadsStream;
});

/// Active downloads provider
final activeDownloadsProvider = Provider<List<DownloadItem>>((ref) {
  final downloads = ref.watch(downloadsStreamProvider).value ?? [];
  return downloads.where((d) => d.isActive).toList();
});

/// Completed downloads provider
final completedDownloadsProvider = Provider<List<DownloadItem>>((ref) {
  final downloads = ref.watch(downloadsStreamProvider).value ?? [];
  return downloads.where((d) => d.isCompleted).toList();
});

/// Failed downloads provider
final failedDownloadsProvider = Provider<List<DownloadItem>>((ref) {
  final downloads = ref.watch(downloadsStreamProvider).value ?? [];
  return downloads.where((d) => d.isFailed).toList();
});

/// Overall download progress provider
final overallDownloadProgressProvider = Provider<double>((ref) {
  final downloadManager = ref.watch(downloadManagerServiceProvider);
  return downloadManager.getOverallProgress();
});

/// Storage cleanup notifier
class StorageCleanupNotifier extends StateNotifier<AsyncValue<void>> {
  final LocalStorageService _storageService;

  StorageCleanupNotifier(this._storageService) : super(const AsyncValue.data(null));

  Future<void> performCleanup({bool force = false}) async {
    state = const AsyncValue.loading();
    try {
      await _storageService.performCleanup(force: force);
      state = const AsyncValue.data(null);
    } catch (e, stack) {
      state = AsyncValue.error(e, stack);
    }
  }
}

/// Storage cleanup provider
final storageCleanupProvider = StateNotifierProvider<StorageCleanupNotifier, AsyncValue<void>>((ref) {
  final storageService = ref.watch(localStorageServiceProvider);
  return StorageCleanupNotifier(storageService);
});

/// Content download actions
class ContentDownloadActions {
  final DownloadManagerService _downloadManager;

  ContentDownloadActions(this._downloadManager);

  /// Download Quran surah
  Future<String> downloadSurah(
    int surahNumber,
    String surahName,
    Future<List<int>> Function() downloader,
  ) async {
    return await _downloadManager.queueDownload(
      key: 'quran_surah_$surahNumber',
      title: 'سورة $surahName',
      description: 'تحميل سورة $surahName للقراءة دون اتصال',
      priority: StoragePriority.high,
      estimatedSize: 50 * 1024, // 50KB estimate
      downloader: downloader,
    );
  }

  /// Download Quran juz
  Future<String> downloadJuz(
    int juzNumber,
    Future<List<int>> Function() downloader,
  ) async {
    return await _downloadManager.queueDownload(
      key: 'quran_juz_$juzNumber',
      title: 'الجزء $juzNumber',
      description: 'تحميل الجزء $juzNumber للقراءة دون اتصال',
      priority: StoragePriority.high,
      estimatedSize: 500 * 1024, // 500KB estimate
      downloader: downloader,
    );
  }

  /// Download tafsir for surah
  Future<String> downloadTafsir(
    int surahNumber,
    String surahName,
    String tafsirSource,
    Future<List<int>> Function() downloader,
  ) async {
    return await _downloadManager.queueDownload(
      key: 'tafsir_${tafsirSource}_$surahNumber',
      title: 'تفسير $surahName - $tafsirSource',
      description: 'تحميل تفسير سورة $surahName',
      priority: StoragePriority.medium,
      estimatedSize: 200 * 1024, // 200KB estimate
      downloader: downloader,
    );
  }

  /// Download hadith collection
  Future<String> downloadHadithCollection(
    String collectionName,
    Future<List<int>> Function() downloader,
  ) async {
    return await _downloadManager.queueDownload(
      key: 'hadith_$collectionName',
      title: collectionName,
      description: 'تحميل مجموعة $collectionName',
      priority: StoragePriority.medium,
      estimatedSize: 1024 * 1024, // 1MB estimate
      downloader: downloader,
    );
  }

  /// Download audio recitation
  Future<String> downloadAudioRecitation(
    int surahNumber,
    String surahName,
    String reciter,
    Future<List<int>> Function() downloader,
    int estimatedSize,
  ) async {
    return await _downloadManager.queueDownload(
      key: 'audio_${reciter}_$surahNumber',
      title: 'تلاوة $surahName - $reciter',
      description: 'تحميل تلاوة سورة $surahName',
      priority: StoragePriority.low,
      estimatedSize: estimatedSize,
      downloader: downloader,
    );
  }

  /// Download prayer times for month
  Future<String> downloadPrayerTimes(
    String location,
    int year,
    int month,
    Future<List<int>> Function() downloader,
  ) async {
    return await _downloadManager.queueDownload(
      key: 'prayer_times_${location}_${year}_$month',
      title: 'مواقيت الصلاة - $location',
      description: 'تحميل مواقيت الصلاة لشهر $month/$year',
      priority: StoragePriority.critical,
      estimatedSize: 10 * 1024, // 10KB estimate
      downloader: downloader,
    );
  }
}

/// Content download actions provider
final contentDownloadActionsProvider = Provider<ContentDownloadActions>((ref) {
  final downloadManager = ref.watch(downloadManagerServiceProvider);
  return ContentDownloadActions(downloadManager);
});
