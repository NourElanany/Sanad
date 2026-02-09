'use client';

import { useEffect, useState } from 'react';
import {
  getDownloadManager,
  DownloadItem,
  DownloadStatus,
} from '@/lib/services/download-manager-service';
import { LocalStorageService, StorageStats } from '@/lib/services/local-storage-service';

export default function DownloadsPage() {
  const [downloads, setDownloads] = useState<DownloadItem[]>([]);
  const [stats, setStats] = useState<StorageStats | null>(null);
  const [spaceInfo, setSpaceInfo] = useState<{
    required: number;
    available: number;
    hasEnough: boolean;
    deficit: number;
  } | null>(null);
  const [activeTab, setActiveTab] = useState<'active' | 'completed' | 'failed'>('active');

  useEffect(() => {
    const downloadManager = getDownloadManager();

    // Subscribe to download updates
    const unsubscribe = downloadManager.subscribe(() => {
      setDownloads(downloadManager.getDownloads());
      loadSpaceInfo();
    });

    // Load initial data
    setDownloads(downloadManager.getDownloads());
    loadStats();
    loadSpaceInfo();

    return unsubscribe;
  }, []);

  const loadStats = async () => {
    const storageStats = await LocalStorageService.getStats();
    setStats(storageStats);
  };

  const loadSpaceInfo = async () => {
    const downloadManager = getDownloadManager();
    const info = await downloadManager.getSpaceInfo();
    setSpaceInfo(info);
  };

  const handleCleanup = async () => {
    if (confirm('هل تريد تنظيف التخزين وحذف المحتوى القديم؟')) {
      await LocalStorageService.performCleanup(true);
      await loadStats();
      await loadSpaceInfo();
    }
  };

  const activeDownloads = downloads.filter(
    (d) => d.status === DownloadStatus.DOWNLOADING || d.status === DownloadStatus.QUEUED
  );
  const completedDownloads = downloads.filter((d) => d.status === DownloadStatus.COMPLETED);
  const failedDownloads = downloads.filter((d) => d.status === DownloadStatus.FAILED);

  return (
    <div className="min-h-screen bg-gray-50 p-6" dir="rtl">
      <div className="max-w-6xl mx-auto">
        <div className="flex justify-between items-center mb-6">
          <h1 className="text-3xl font-bold text-gray-900">إدارة التحميلات</h1>
          <button
            onClick={handleCleanup}
            className="px-4 py-2 bg-blue-600 text-white rounded-lg hover:bg-blue-700"
          >
            تنظيف التخزين
          </button>
        </div>

        {/* Storage Stats Card */}
        {stats && <StorageStatsCard stats={stats} />}

        {/* Space Info Card */}
        {spaceInfo && !spaceInfo.hasEnough && (
          <div className="bg-red-50 border border-red-200 rounded-lg p-4 mb-6">
            <div className="flex items-center gap-2 text-red-800">
              <span className="text-2xl">⚠️</span>
              <div>
                <h3 className="font-bold">مساحة غير كافية</h3>
                <p className="text-sm">
                  تحتاج إلى {formatBytes(spaceInfo.deficit)} إضافية لإكمال التحميلات
                </p>
              </div>
            </div>
          </div>
        )}

        {/* Tabs */}
        <div className="bg-white rounded-lg shadow-md mb-6">
          <div className="flex border-b">
            <TabButton
              active={activeTab === 'active'}
              onClick={() => setActiveTab('active')}
              count={activeDownloads.length}
            >
              نشط
            </TabButton>
            <TabButton
              active={activeTab === 'completed'}
              onClick={() => setActiveTab('completed')}
              count={completedDownloads.length}
            >
              مكتمل
            </TabButton>
            <TabButton
              active={activeTab === 'failed'}
              onClick={() => setActiveTab('failed')}
              count={failedDownloads.length}
            >
              فشل
            </TabButton>
          </div>

          {/* Downloads List */}
          <div className="p-4">
            {activeTab === 'active' && <DownloadsList downloads={activeDownloads} />}
            {activeTab === 'completed' && <DownloadsList downloads={completedDownloads} />}
            {activeTab === 'failed' && <DownloadsList downloads={failedDownloads} />}
          </div>
        </div>
      </div>
    </div>
  );
}

function StorageStatsCard({ stats }: { stats: StorageStats }) {
  const usedMB = stats.usedSpace / (1024 * 1024);
  const totalMB = stats.totalSize / (1024 * 1024);
  const percentage = (stats.usedSpace / stats.totalSize) * 100;

  let progressColor = 'bg-green-500';
  if (percentage > 95) progressColor = 'bg-red-500';
  else if (percentage > 80) progressColor = 'bg-yellow-500';

  return (
    <div className="bg-white rounded-lg shadow-md p-6 mb-6">
      <div className="flex justify-between items-center mb-4">
        <h2 className="text-xl font-bold">مساحة التخزين</h2>
        <span className="text-gray-600">
          {usedMB.toFixed(1)} / {totalMB.toFixed(1)} MB
        </span>
      </div>

      <div className="w-full bg-gray-200 rounded-full h-3 mb-2">
        <div
          className={`${progressColor} h-3 rounded-full transition-all`}
          style={{ width: `${percentage}%` }}
        />
      </div>

      <div className="flex justify-between text-sm text-gray-600">
        <span>{percentage.toFixed(1)}% مستخدم</span>
        <span>{stats.itemCount} عنصر</span>
      </div>
    </div>
  );
}

function TabButton({
  active,
  onClick,
  count,
  children,
}: {
  active: boolean;
  onClick: () => void;
  count: number;
  children: React.ReactNode;
}) {
  return (
    <button
      onClick={onClick}
      className={`flex-1 px-4 py-3 font-medium ${
        active
          ? 'text-blue-600 border-b-2 border-blue-600'
          : 'text-gray-600 hover:text-gray-900'
      }`}
    >
      {children} ({count})
    </button>
  );
}

function DownloadsList({ downloads }: { downloads: DownloadItem[] }) {
  const downloadManager = getDownloadManager();

  if (downloads.length === 0) {
    return (
      <div className="text-center py-12 text-gray-500">
        لا توجد تحميلات في هذه الفئة
      </div>
    );
  }

  return (
    <div className="space-y-4">
      {downloads.map((download) => (
        <DownloadCard
          key={download.id}
          download={download}
          onPause={() => downloadManager.pauseDownload(download.id)}
          onResume={() => downloadManager.startDownload(download.id)}
          onRetry={() => downloadManager.retryDownload(download.id)}
          onCancel={() => downloadManager.cancelDownload(download.id)}
        />
      ))}
    </div>
  );
}

function DownloadCard({
  download,
  onPause,
  onResume,
  onRetry,
  onCancel,
}: {
  download: DownloadItem;
  onPause: () => void;
  onResume: () => void;
  onRetry: () => void;
  onCancel: () => void;
}) {
  const progress = (download.downloadedBytes / download.estimatedSize) * 100;

  return (
    <div className="border rounded-lg p-4 hover:shadow-md transition-shadow">
      <div className="flex items-start justify-between mb-2">
        <div className="flex-1">
          <h3 className="font-semibold text-lg">{download.title}</h3>
          {download.description && (
            <p className="text-sm text-gray-600 mt-1">{download.description}</p>
          )}
          {download.status === DownloadStatus.DOWNLOADING && download.downloadSpeed && (
            <div className="flex gap-4 text-xs text-gray-500 mt-1">
              <span>السرعة: {formatBytes(download.downloadSpeed)}/ث</span>
              {download.remainingTime && (
                <span>الوقت المتبقي: {formatTime(download.remainingTime)}</span>
              )}
            </div>
          )}
        </div>
        <StatusIcon status={download.status} />
      </div>

      {(download.status === DownloadStatus.DOWNLOADING ||
        download.status === DownloadStatus.QUEUED) && (
        <div className="mb-2">
          <div className="w-full bg-gray-200 rounded-full h-2 mb-1">
            <div
              className="bg-blue-600 h-2 rounded-full transition-all"
              style={{ width: `${progress}%` }}
            />
          </div>
          <div className="flex justify-between text-xs text-gray-600">
            <span>{progress.toFixed(0)}%</span>
            <span>
              {formatBytes(download.downloadedBytes)} / {formatBytes(download.estimatedSize)}
            </span>
          </div>
          {download.chunks && (
            <div className="flex gap-1 mt-2">
              {download.chunks.map((chunk) => (
                <div
                  key={chunk.index}
                  className={`h-1 flex-1 rounded ${
                    chunk.downloaded ? 'bg-green-500' : 'bg-gray-300'
                  }`}
                  title={`Chunk ${chunk.index + 1}`}
                />
              ))}
            </div>
          )}
        </div>
      )}

      {download.status === DownloadStatus.FAILED && download.error && (
        <p className="text-sm text-red-600 mb-2">خطأ: {download.error}</p>
      )}

      <div className="flex gap-2">
        {(download.status === DownloadStatus.DOWNLOADING ||
          download.status === DownloadStatus.QUEUED) && (
          <button
            onClick={onPause}
            className="px-3 py-1 text-sm bg-gray-200 hover:bg-gray-300 rounded"
          >
            إيقاف مؤقت
          </button>
        )}
        {download.status === DownloadStatus.PAUSED && (
          <button
            onClick={onResume}
            className="px-3 py-1 text-sm bg-blue-600 text-white hover:bg-blue-700 rounded"
          >
            استئناف
          </button>
        )}
        {download.status === DownloadStatus.FAILED && (
          <button
            onClick={onRetry}
            className="px-3 py-1 text-sm bg-green-600 text-white hover:bg-green-700 rounded"
          >
            إعادة المحاولة
          </button>
        )}
        {download.status !== DownloadStatus.CANCELLED && (
          <button
            onClick={onCancel}
            className="px-3 py-1 text-sm bg-red-600 text-white hover:bg-red-700 rounded"
          >
            {download.status === DownloadStatus.COMPLETED ? 'حذف' : 'إلغاء'}
          </button>
        )}
      </div>
    </div>
  );
}

function StatusIcon({ status }: { status: DownloadStatus }) {
  const icons = {
    [DownloadStatus.QUEUED]: '⏳',
    [DownloadStatus.DOWNLOADING]: '⬇️',
    [DownloadStatus.PAUSED]: '⏸️',
    [DownloadStatus.COMPLETED]: '✅',
    [DownloadStatus.FAILED]: '❌',
    [DownloadStatus.CANCELLED]: '🚫',
  };

  return <span className="text-2xl">{icons[status]}</span>;
}

function formatBytes(bytes: number): string {
  if (bytes < 1024) return `${bytes} B`;
  if (bytes < 1024 * 1024) return `${(bytes / 1024).toFixed(1)} KB`;
  return `${(bytes / (1024 * 1024)).toFixed(1)} MB`;
}

function formatTime(seconds: number): string {
  if (seconds < 60) return `${Math.round(seconds)}ث`;
  if (seconds < 3600) return `${Math.round(seconds / 60)}د`;
  return `${Math.round(seconds / 3600)}س`;
}
