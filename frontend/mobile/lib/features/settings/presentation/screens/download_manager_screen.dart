import 'package:flutter/material.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';
import '../../../../core/providers/local_storage_provider.dart';
import '../../../../core/services/local_storage_service.dart';
import '../../../../core/services/download_manager_service.dart';
import '../../../../core/theme/app_colors.dart';

class DownloadManagerScreen extends ConsumerWidget {
  const DownloadManagerScreen({Key? key}) : super(key: key);

  @override
  Widget build(BuildContext context, WidgetRef ref) {
    final downloadsAsync = ref.watch(downloadsStreamProvider);
    final storageStatsAsync = ref.watch(storageStatsProvider);

    return Scaffold(
      appBar: AppBar(
        title: const Text('إدارة التحميلات'),
        actions: [
          IconButton(
            icon: const Icon(Icons.cleaning_services),
            onPressed: () => _showCleanupDialog(context, ref),
            tooltip: 'تنظيف التخزين',
          ),
        ],
      ),
      body: Column(
        children: [
          // Storage stats card
          storageStatsAsync.when(
            data: (stats) => _buildStorageStatsCard(context, stats),
            loading: () => const LinearProgressIndicator(),
            error: (error, _) => Padding(
              padding: const EdgeInsets.all(16),
              child: Text('خطأ في تحميل الإحصائيات: $error'),
            ),
          ),

          // Downloads list
          Expanded(
            child: downloadsAsync.when(
              data: (downloads) {
                if (downloads.isEmpty) {
                  return const Center(
                    child: Column(
                      mainAxisAlignment: MainAxisAlignment.center,
                      children: [
                        Icon(Icons.download, size: 64, color: Colors.grey),
                        SizedBox(height: 16),
                        Text(
                          'لا توجد تحميلات',
                          style: TextStyle(fontSize: 18, color: Colors.grey),
                        ),
                      ],
                    ),
                  );
                }

                return DefaultTabController(
                  length: 3,
                  child: Column(
                    children: [
                      TabBar(
                        labelColor: AppColors.primary,
                        tabs: [
                          Tab(text: 'نشط (${downloads.where((d) => d.isActive).length})'),
                          Tab(text: 'مكتمل (${downloads.where((d) => d.isCompleted).length})'),
                          Tab(text: 'فشل (${downloads.where((d) => d.isFailed).length})'),
                        ],
                      ),
                      Expanded(
                        child: TabBarView(
                          children: [
                            _buildDownloadsList(
                              context,
                              ref,
                              downloads.where((d) => d.isActive).toList(),
                            ),
                            _buildDownloadsList(
                              context,
                              ref,
                              downloads.where((d) => d.isCompleted).toList(),
                            ),
                            _buildDownloadsList(
                              context,
                              ref,
                              downloads.where((d) => d.isFailed).toList(),
                            ),
                          ],
                        ),
                      ),
                    ],
                  ),
                );
              },
              loading: () => const Center(child: CircularProgressIndicator()),
              error: (error, _) => Center(
                child: Text('خطأ في تحميل القائمة: $error'),
              ),
            ),
          ),
        ],
      ),
    );
  }

  Widget _buildStorageStatsCard(BuildContext context, StorageStats stats) {
    final usedMB = stats.usedSpace / (1024 * 1024);
    final totalMB = stats.totalSize / (1024 * 1024);
    final percentage = stats.usagePercentage;

    Color progressColor = AppColors.success;
    if (stats.isCritical) {
      progressColor = AppColors.error;
    } else if (stats.isNearCapacity) {
      progressColor = AppColors.warning;
    }

    return Card(
      margin: const EdgeInsets.all(16),
      child: Padding(
        padding: const EdgeInsets.all(16),
        child: Column(
          crossAxisAlignment: CrossAxisAlignment.start,
          children: [
            Row(
              mainAxisAlignment: MainAxisAlignment.spaceBetween,
              children: [
                const Text(
                  'مساحة التخزين',
                  style: TextStyle(fontSize: 18, fontWeight: FontWeight.bold),
                ),
                Text(
                  '${usedMB.toStringAsFixed(1)} / ${totalMB.toStringAsFixed(1)} MB',
                  style: const TextStyle(fontSize: 14, color: Colors.grey),
                ),
              ],
            ),
            const SizedBox(height: 12),
            LinearProgressIndicator(
              value: percentage / 100,
              backgroundColor: Colors.grey[200],
              valueColor: AlwaysStoppedAnimation<Color>(progressColor),
              minHeight: 8,
            ),
            const SizedBox(height: 8),
            Text(
              '${percentage.toStringAsFixed(1)}% مستخدم',
              style: TextStyle(fontSize: 12, color: progressColor),
            ),
            const SizedBox(height: 16),
            Row(
              mainAxisAlignment: MainAxisAlignment.spaceAround,
              children: [
                _buildStatItem(
                  'العناصر',
                  stats.itemCount.toString(),
                  Icons.inventory_2,
                ),
                _buildStatItem(
                  'آخر تنظيف',
                  _formatDate(stats.lastCleanup),
                  Icons.cleaning_services,
                ),
              ],
            ),
          ],
        ),
      ),
    );
  }

  Widget _buildStatItem(String label, String value, IconData icon) {
    return Column(
      children: [
        Icon(icon, size: 24, color: AppColors.primary),
        const SizedBox(height: 4),
        Text(
          value,
          style: const TextStyle(fontSize: 16, fontWeight: FontWeight.bold),
        ),
        Text(
          label,
          style: const TextStyle(fontSize: 12, color: Colors.grey),
        ),
      ],
    );
  }

  Widget _buildDownloadsList(
    BuildContext context,
    WidgetRef ref,
    List<DownloadItem> downloads,
  ) {
    if (downloads.isEmpty) {
      return const Center(
        child: Text('لا توجد تحميلات في هذه الفئة'),
      );
    }

    return ListView.builder(
      itemCount: downloads.length,
      itemBuilder: (context, index) {
        final download = downloads[index];
        return _buildDownloadItem(context, ref, download);
      },
    );
  }

  Widget _buildDownloadItem(
    BuildContext context,
    WidgetRef ref,
    DownloadItem download,
  ) {
    final downloadManager = ref.read(downloadManagerServiceProvider);

    return Card(
      margin: const EdgeInsets.symmetric(horizontal: 16, vertical: 8),
      child: Padding(
        padding: const EdgeInsets.all(12),
        child: Column(
          crossAxisAlignment: CrossAxisAlignment.start,
          children: [
            Row(
              children: [
                _buildStatusIcon(download.status),
                const SizedBox(width: 12),
                Expanded(
                  child: Column(
                    crossAxisAlignment: CrossAxisAlignment.start,
                    children: [
                      Text(
                        download.title,
                        style: const TextStyle(
                          fontSize: 16,
                          fontWeight: FontWeight.bold,
                        ),
                      ),
                      if (download.description != null) ...[
                        const SizedBox(height: 4),
                        Text(
                          download.description!,
                          style: const TextStyle(
                            fontSize: 12,
                            color: Colors.grey,
                          ),
                        ),
                      ],
                    ],
                  ),
                ),
                _buildActionButton(context, ref, download, downloadManager),
              ],
            ),
            if (download.isActive) ...[
              const SizedBox(height: 12),
              LinearProgressIndicator(
                value: download.progress,
                backgroundColor: Colors.grey[200],
                valueColor: AlwaysStoppedAnimation<Color>(AppColors.primary),
              ),
              const SizedBox(height: 4),
              Row(
                mainAxisAlignment: MainAxisAlignment.spaceBetween,
                children: [
                  Text(
                    '${(download.progress * 100).toStringAsFixed(0)}%',
                    style: const TextStyle(fontSize: 12, color: Colors.grey),
                  ),
                  Text(
                    '${_formatBytes(download.downloadedBytes)} / ${_formatBytes(download.estimatedSize)}',
                    style: const TextStyle(fontSize: 12, color: Colors.grey),
                  ),
                ],
              ),
            ],
            if (download.isFailed && download.error != null) ...[
              const SizedBox(height: 8),
              Text(
                'خطأ: ${download.error}',
                style: const TextStyle(fontSize: 12, color: Colors.red),
              ),
            ],
          ],
        ),
      ),
    );
  }

  Widget _buildStatusIcon(DownloadStatus status) {
    IconData icon;
    Color color;

    switch (status) {
      case DownloadStatus.queued:
        icon = Icons.schedule;
        color = Colors.orange;
        break;
      case DownloadStatus.downloading:
        icon = Icons.downloading;
        color = AppColors.primary;
        break;
      case DownloadStatus.paused:
        icon = Icons.pause_circle;
        color = Colors.grey;
        break;
      case DownloadStatus.completed:
        icon = Icons.check_circle;
        color = AppColors.success;
        break;
      case DownloadStatus.failed:
        icon = Icons.error;
        color = AppColors.error;
        break;
      case DownloadStatus.cancelled:
        icon = Icons.cancel;
        color = Colors.grey;
        break;
    }

    return Icon(icon, color: color, size: 32);
  }

  Widget _buildActionButton(
    BuildContext context,
    WidgetRef ref,
    DownloadItem download,
    DownloadManagerService downloadManager,
  ) {
    if (download.isActive) {
      return IconButton(
        icon: const Icon(Icons.pause),
        onPressed: () => downloadManager.pauseDownload(download.id),
        tooltip: 'إيقاف مؤقت',
      );
    } else if (download.isPaused) {
      return IconButton(
        icon: const Icon(Icons.play_arrow),
        onPressed: () => downloadManager.startDownload(download.id),
        tooltip: 'استئناف',
      );
    } else if (download.isFailed) {
      return IconButton(
        icon: const Icon(Icons.refresh),
        onPressed: () => downloadManager.retryDownload(download.id),
        tooltip: 'إعادة المحاولة',
      );
    } else if (download.isCompleted) {
      return IconButton(
        icon: const Icon(Icons.delete),
        onPressed: () => downloadManager.cancelDownload(download.id),
        tooltip: 'حذف',
      );
    }

    return const SizedBox.shrink();
  }

  void _showCleanupDialog(BuildContext context, WidgetRef ref) {
    showDialog(
      context: context,
      builder: (context) => AlertDialog(
        title: const Text('تنظيف التخزين'),
        content: const Text(
          'سيتم حذف المحتوى القديم وغير المستخدم لتوفير المساحة. هل تريد المتابعة؟',
        ),
        actions: [
          TextButton(
            onPressed: () => Navigator.pop(context),
            child: const Text('إلغاء'),
          ),
          ElevatedButton(
            onPressed: () {
              Navigator.pop(context);
              ref.read(storageCleanupProvider.notifier).performCleanup(force: true);
              ScaffoldMessenger.of(context).showSnackBar(
                const SnackBar(content: Text('جاري تنظيف التخزين...')),
              );
            },
            child: const Text('تنظيف'),
          ),
        ],
      ),
    );
  }

  String _formatBytes(int bytes) {
    if (bytes < 1024) return '$bytes B';
    if (bytes < 1024 * 1024) return '${(bytes / 1024).toStringAsFixed(1)} KB';
    return '${(bytes / (1024 * 1024)).toStringAsFixed(1)} MB';
  }

  String _formatDate(DateTime date) {
    final now = DateTime.now();
    final diff = now.difference(date);

    if (diff.inDays == 0) return 'اليوم';
    if (diff.inDays == 1) return 'أمس';
    if (diff.inDays < 7) return 'منذ ${diff.inDays} أيام';
    return '${date.day}/${date.month}/${date.year}';
  }
}
