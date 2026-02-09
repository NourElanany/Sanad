import 'dart:async';
import 'package:flutter/foundation.dart';
import 'local_storage_service.dart';

/// Download status
enum DownloadStatus {
  queued,
  downloading,
  paused,
  completed,
  failed,
  cancelled,
}

/// Download item
class DownloadItem {
  final String id;
  final String key;
  final String title;
  final String? description;
  final StoragePriority priority;
  final int estimatedSize;
  final Future<List<int>> Function() downloader;
  
  DownloadStatus status;
  int downloadedBytes;
  String? error;
  DateTime? startedAt;
  DateTime? completedAt;
  List<DownloadChunk>? chunks;
  double? downloadSpeed; // bytes per second
  double? remainingTime; // seconds

  DownloadItem({
    required this.id,
    required this.key,
    required this.title,
    this.description,
    required this.priority,
    required this.estimatedSize,
    required this.downloader,
    this.status = DownloadStatus.queued,
    this.downloadedBytes = 0,
    this.error,
    this.startedAt,
    this.completedAt,
    this.chunks,
    this.downloadSpeed,
    this.remainingTime,
  });

  double get progress {
    if (estimatedSize == 0) return 0;
    return (downloadedBytes / estimatedSize).clamp(0.0, 1.0);
  }

  bool get isActive => status == DownloadStatus.downloading || status == DownloadStatus.queued;
  bool get isCompleted => status == DownloadStatus.completed;
  bool get isFailed => status == DownloadStatus.failed;
  bool get isPaused => status == DownloadStatus.paused;

  DownloadItem copyWith({
    DownloadStatus? status,
    int? downloadedBytes,
    String? error,
    DateTime? startedAt,
    DateTime? completedAt,
    List<DownloadChunk>? chunks,
    double? downloadSpeed,
    double? remainingTime,
  }) {
    return DownloadItem(
      id: id,
      key: key,
      title: title,
      description: description,
      priority: priority,
      estimatedSize: estimatedSize,
      downloader: downloader,
      status: status ?? this.status,
      downloadedBytes: downloadedBytes ?? this.downloadedBytes,
      error: error ?? this.error,
      startedAt: startedAt ?? this.startedAt,
      completedAt: completedAt ?? this.completedAt,
      chunks: chunks ?? this.chunks,
      downloadSpeed: downloadSpeed ?? this.downloadSpeed,
      remainingTime: remainingTime ?? this.remainingTime,
    );
  }

  Map<String, dynamic> toJson() => {
        'id': id,
        'key': key,
        'title': title,
        'description': description,
        'priority': priority.index,
        'estimatedSize': estimatedSize,
        'status': status.index,
        'downloadedBytes': downloadedBytes,
        'error': error,
        'startedAt': startedAt?.toIso8601String(),
        'completedAt': completedAt?.toIso8601String(),
        'downloadSpeed': downloadSpeed,
        'remainingTime': remainingTime,
      };
}

/// Download chunk for progressive loading
class DownloadChunk {
  final int index;
  final int start;
  final int end;
  bool downloaded;

  DownloadChunk({
    required this.index,
    required this.start,
    required this.end,
    this.downloaded = false,
  });
}

/// Download manager configuration
class DownloadManagerConfig {
  final int maxConcurrentDownloads;
  final bool autoRetry;
  final int maxRetries;
  final Duration retryDelay;
  final bool wifiOnly;
  final int chunkSize; // Size of each chunk for progressive download
  final bool enableProgressiveDownload;

  const DownloadManagerConfig({
    this.maxConcurrentDownloads = 3,
    this.autoRetry = true,
    this.maxRetries = 3,
    this.retryDelay = const Duration(seconds: 5),
    this.wifiOnly = false,
    this.chunkSize = 1024 * 1024, // 1MB chunks
    this.enableProgressiveDownload = true,
  });
}

/// Download manager service
class DownloadManagerService {
  final LocalStorageService _storageService;
  final DownloadManagerConfig config;
  
  final Map<String, DownloadItem> _downloads = {};
  final List<String> _queue = [];
  final Set<String> _activeDownloads = {};
  final Map<String, int> _retryCount = {};
  
  final StreamController<List<DownloadItem>> _downloadsController = 
      StreamController<List<DownloadItem>>.broadcast();

  DownloadManagerService(
    this._storageService, {
    this.config = const DownloadManagerConfig(),
  });

  /// Stream of all downloads
  Stream<List<DownloadItem>> get downloadsStream => _downloadsController.stream;

  /// Get all downloads
  List<DownloadItem> get downloads => _downloads.values.toList();

  /// Get active downloads
  List<DownloadItem> get activeDownloads => 
      _downloads.values.where((d) => d.isActive).toList();

  /// Get completed downloads
  List<DownloadItem> get completedDownloads => 
      _downloads.values.where((d) => d.isCompleted).toList();

  /// Get failed downloads
  List<DownloadItem> get failedDownloads => 
      _downloads.values.where((d) => d.isFailed).toList();

  /// Queue a download
  Future<String> queueDownload({
    required String key,
    required String title,
    String? description,
    required StoragePriority priority,
    required int estimatedSize,
    required Future<List<int>> Function() downloader,
  }) async {
    final id = DateTime.now().millisecondsSinceEpoch.toString();
    
    final item = DownloadItem(
      id: id,
      key: key,
      title: title,
      description: description,
      priority: priority,
      estimatedSize: estimatedSize,
      downloader: downloader,
    );

    _downloads[id] = item;
    _queue.add(id);
    
    _notifyListeners();
    _processQueue();

    return id;
  }

  /// Start/resume a download
  Future<void> startDownload(String id) async {
    final item = _downloads[id];
    if (item == null) return;

    if (item.status == DownloadStatus.paused) {
      item.status = DownloadStatus.queued;
      if (!_queue.contains(id)) {
        _queue.add(id);
      }
      _notifyListeners();
      _processQueue();
    }
  }

  /// Pause a download
  Future<void> pauseDownload(String id) async {
    final item = _downloads[id];
    if (item == null) return;

    if (item.status == DownloadStatus.downloading || 
        item.status == DownloadStatus.queued) {
      item.status = DownloadStatus.paused;
      _queue.remove(id);
      _activeDownloads.remove(id);
      _notifyListeners();
    }
  }

  /// Cancel a download
  Future<void> cancelDownload(String id) async {
    final item = _downloads[id];
    if (item == null) return;

    item.status = DownloadStatus.cancelled;
    _queue.remove(id);
    _activeDownloads.remove(id);
    _downloads.remove(id);
    _retryCount.remove(id);
    
    _notifyListeners();
  }

  /// Retry a failed download
  Future<void> retryDownload(String id) async {
    final item = _downloads[id];
    if (item == null || !item.isFailed) return;

    item.status = DownloadStatus.queued;
    item.error = null;
    item.downloadedBytes = 0;
    _retryCount[id] = 0;
    
    if (!_queue.contains(id)) {
      _queue.add(id);
    }
    
    _notifyListeners();
    _processQueue();
  }

  /// Clear completed downloads
  Future<void> clearCompleted() async {
    final completedIds = _downloads.entries
        .where((e) => e.value.isCompleted)
        .map((e) => e.key)
        .toList();

    for (var id in completedIds) {
      _downloads.remove(id);
    }

    _notifyListeners();
  }

  /// Get download by ID
  DownloadItem? getDownload(String id) => _downloads[id];

  /// Get total download size
  int getTotalSize() {
    return _downloads.values.fold(0, (sum, item) => sum + item.estimatedSize);
  }

  /// Get downloaded size
  int getDownloadedSize() {
    return _downloads.values.fold(0, (sum, item) => sum + item.downloadedBytes);
  }

  /// Get overall progress
  double getOverallProgress() {
    final total = getTotalSize();
    if (total == 0) return 0;
    return (getDownloadedSize() / total).clamp(0.0, 1.0);
  }

  /// Estimate required space for pending downloads
  int getRequiredSpace() {
    return _downloads.values
        .where((d) =>
            d.status == DownloadStatus.queued ||
            d.status == DownloadStatus.downloading ||
            d.status == DownloadStatus.paused)
        .fold(0, (sum, item) => sum + (item.estimatedSize - item.downloadedBytes));
  }

  /// Check if there's enough space for downloads
  Future<bool> hasEnoughSpace() async {
    final required = getRequiredSpace();
    final stats = await _storageService.getStats();
    final available = stats.totalSize - stats.usedSpace;
    return available >= required;
  }

  /// Get space availability info
  Future<SpaceInfo> getSpaceInfo() async {
    final required = getRequiredSpace();
    final stats = await _storageService.getStats();
    final available = stats.totalSize - stats.usedSpace;
    final hasEnough = available >= required;
    final deficit = hasEnough ? 0 : required - available;

    return SpaceInfo(
      required: required,
      available: available,
      hasEnough: hasEnough,
      deficit: deficit,
    );
  }

  /// Get estimated completion time for all downloads
  double getEstimatedCompletionTime() {
    final active = activeDownloads;
    if (active.isEmpty) return 0;

    final totalRemaining = active.fold<double>(
      0,
      (sum, item) => sum + (item.remainingTime ?? 0),
    );

    return totalRemaining / active.length;
  }

  /// Process download queue
  Future<void> _processQueue() async {
    // Check if we can start more downloads
    while (_activeDownloads.length < config.maxConcurrentDownloads && 
           _queue.isNotEmpty) {
      // Sort queue by priority
      _queue.sort((a, b) {
        final itemA = _downloads[a];
        final itemB = _downloads[b];
        if (itemA == null || itemB == null) return 0;
        return itemA.priority.index.compareTo(itemB.priority.index);
      });

      final id = _queue.removeAt(0);
      final item = _downloads[id];
      
      if (item == null || item.status != DownloadStatus.queued) {
        continue;
      }

      _activeDownloads.add(id);
      _downloadItem(id);
    }
  }

  /// Download an item
  Future<void> _downloadItem(String id) async {
    final item = _downloads[id];
    if (item == null) return;

    try {
      item.status = DownloadStatus.downloading;
      item.startedAt = DateTime.now();
      _notifyListeners();

      final startTime = DateTime.now();

      // Download with progress tracking
      await _downloadWithProgress(item, (progress) {
        final elapsed = DateTime.now().difference(startTime).inSeconds;
        if (elapsed > 0) {
          final speed = progress / elapsed.toDouble();
          final remaining = (item.estimatedSize - progress) / speed;

          item.downloadedBytes = progress;
          item.downloadSpeed = speed;
          item.remainingTime = remaining;
          _notifyListeners();
        }
      });

      item.status = DownloadStatus.completed;
      item.completedAt = DateTime.now();
      item.downloadedBytes = item.estimatedSize;
      item.downloadSpeed = null;
      item.remainingTime = null;
      _retryCount.remove(id);
      
      debugPrint('Download completed: ${item.title}');
    } catch (e) {
      debugPrint('Download failed: ${item.title} - $e');
      
      final retries = _retryCount[id] ?? 0;
      
      if (config.autoRetry && retries < config.maxRetries) {
        _retryCount[id] = retries + 1;
        item.status = DownloadStatus.queued;
        
        // Add back to queue after delay
        await Future.delayed(config.retryDelay);
        if (!_queue.contains(id)) {
          _queue.add(id);
        }
        
        debugPrint('Retrying download: ${item.title} (${retries + 1}/${config.maxRetries})');
      } else {
        item.status = DownloadStatus.failed;
        item.error = e.toString();
      }
    } finally {
      _activeDownloads.remove(id);
      _notifyListeners();
      _processQueue();
    }
  }

  /// Download with progress tracking
  Future<void> _downloadWithProgress(
    DownloadItem item,
    void Function(int bytes) onProgress,
  ) async {
    if (config.enableProgressiveDownload && item.estimatedSize > config.chunkSize) {
      await _downloadProgressive(item, onProgress);
    } else {
      // Simple download for small files
      await _storageService.downloadContent(
        item.key,
        item.downloader,
        priority: item.priority,
        onProgress: (received, total) {
          onProgress(received);
        },
      );
    }
  }

  /// Progressive download in chunks
  Future<void> _downloadProgressive(
    DownloadItem item,
    void Function(int bytes) onProgress,
  ) async {
    // Initialize chunks if not already done
    if (item.chunks == null) {
      final numChunks = (item.estimatedSize / config.chunkSize).ceil();
      item.chunks = List.generate(
        numChunks,
        (i) => DownloadChunk(
          index: i,
          start: i * config.chunkSize,
          end: ((i + 1) * config.chunkSize).clamp(0, item.estimatedSize),
        ),
      );
    }

    // Download content with chunk tracking
    await _storageService.downloadContent(
      item.key,
      item.downloader,
      priority: item.priority,
      onProgress: (received, total) {
        // Update chunk status
        if (item.chunks != null) {
          for (final chunk in item.chunks!) {
            if (received >= chunk.end) {
              chunk.downloaded = true;
            }
          }
        }
        onProgress(received);
      },
    );
  }

  /// Notify listeners
  void _notifyListeners() {
    _downloadsController.add(downloads);
  }

  /// Dispose
  void dispose() {
    _downloadsController.close();
  }
}

/// Space availability information
class SpaceInfo {
  final int required;
  final int available;
  final bool hasEnough;
  final int deficit;

  const SpaceInfo({
    required this.required,
    required this.available,
    required this.hasEnough,
    required this.deficit,
  });

  Map<String, dynamic> toJson() => {
        'required': required,
        'available': available,
        'hasEnough': hasEnough,
        'deficit': deficit,
      };
}
