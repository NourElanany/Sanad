import 'dart:io';
import 'dart:typed_data';
import 'package:flutter/foundation.dart';
import 'package:flutter/material.dart';
import 'package:cached_network_image/cached_network_image.dart';
import 'package:path_provider/path_provider.dart';
import '../utils/logger.dart';

/// Service for optimizing image loading and caching
/// Implements lazy loading and memory-efficient image handling
class ImageOptimizationService {
  static final ImageOptimizationService _instance = ImageOptimizationService._internal();
  factory ImageOptimizationService() => _instance;
  ImageOptimizationService._internal();

  final _logger = Logger('ImageOptimizationService');
  
  // Cache configuration
  static const int maxCacheSize = 100 * 1024 * 1024; // 100 MB
  static const Duration cacheValidDuration = Duration(days: 7);
  
  // Image quality settings
  static const int defaultQuality = 85;
  static const int thumbnailQuality = 70;
  static const int thumbnailMaxWidth = 300;
  static const int thumbnailMaxHeight = 300;

  /// Get optimized network image widget
  Widget getOptimizedNetworkImage({
    required String imageUrl,
    double? width,
    double? height,
    BoxFit fit = BoxFit.cover,
    Widget? placeholder,
    Widget? errorWidget,
    bool useThumbnail = false,
  }) {
    return CachedNetworkImage(
      imageUrl: imageUrl,
      width: width,
      height: height,
      fit: fit,
      memCacheWidth: useThumbnail ? thumbnailMaxWidth : width?.toInt(),
      memCacheHeight: useThumbnail ? thumbnailMaxHeight : height?.toInt(),
      maxWidthDiskCache: useThumbnail ? thumbnailMaxWidth : null,
      maxHeightDiskCache: useThumbnail ? thumbnailMaxHeight : null,
      placeholder: (context, url) => placeholder ?? _buildPlaceholder(),
      errorWidget: (context, url, error) => errorWidget ?? _buildErrorWidget(),
      fadeInDuration: const Duration(milliseconds: 300),
      fadeOutDuration: const Duration(milliseconds: 100),
    );
  }

  /// Build default placeholder widget
  Widget _buildPlaceholder() {
    return Container(
      color: Colors.grey[200],
      child: const Center(
        child: CircularProgressIndicator(
          strokeWidth: 2,
          valueColor: AlwaysStoppedAnimation<Color>(Color(0xFF1B365D)),
        ),
      ),
    );
  }

  /// Build default error widget
  Widget _buildErrorWidget() {
    return Container(
      color: Colors.grey[200],
      child: const Icon(
        Icons.broken_image,
        color: Colors.grey,
        size: 48,
      ),
    );
  }

  /// Preload images for better performance
  Future<void> preloadImages(
    BuildContext context,
    List<String> imageUrls,
  ) async {
    try {
      await Future.wait(
        imageUrls.map((url) => precacheImage(
          CachedNetworkImageProvider(url),
          context,
        )),
      );
      _logger.info('Preloaded ${imageUrls.length} images');
    } catch (e) {
      _logger.error('Failed to preload images: $e');
    }
  }

  /// Clear image cache
  Future<void> clearCache() async {
    try {
      await CachedNetworkImage.evictFromCache('');
      _logger.info('Image cache cleared');
    } catch (e) {
      _logger.error('Failed to clear image cache: $e');
    }
  }

  /// Get cache size
  Future<int> getCacheSize() async {
    try {
      final cacheDir = await getTemporaryDirectory();
      final cacheFiles = cacheDir.listSync(recursive: true);
      
      int totalSize = 0;
      for (final file in cacheFiles) {
        if (file is File) {
          totalSize += await file.length();
        }
      }
      
      return totalSize;
    } catch (e) {
      _logger.error('Failed to get cache size: $e');
      return 0;
    }
  }

  /// Clean old cache files
  Future<void> cleanOldCache() async {
    try {
      final cacheDir = await getTemporaryDirectory();
      final cacheFiles = cacheDir.listSync(recursive: true);
      final now = DateTime.now();
      
      int deletedCount = 0;
      for (final file in cacheFiles) {
        if (file is File) {
          final stat = await file.stat();
          final age = now.difference(stat.modified);
          
          if (age > cacheValidDuration) {
            await file.delete();
            deletedCount++;
          }
        }
      }
      
      _logger.info('Cleaned $deletedCount old cache files');
    } catch (e) {
      _logger.error('Failed to clean old cache: $e');
    }
  }

  /// Optimize image for display
  Future<Uint8List?> optimizeImage(
    Uint8List imageData, {
    int? maxWidth,
    int? maxHeight,
    int quality = defaultQuality,
  }) async {
    try {
      // In a real implementation, you would use image processing libraries
      // like image package to resize and compress images
      // For now, we'll return the original data
      _logger.info('Image optimization requested (not implemented in this version)');
      return imageData;
    } catch (e) {
      _logger.error('Failed to optimize image: $e');
      return null;
    }
  }

  /// Create thumbnail from image
  Future<Uint8List?> createThumbnail(
    Uint8List imageData,
  ) async {
    return optimizeImage(
      imageData,
      maxWidth: thumbnailMaxWidth,
      maxHeight: thumbnailMaxHeight,
      quality: thumbnailQuality,
    );
  }

  /// Get memory-efficient image provider
  ImageProvider getMemoryEfficientProvider(String imageUrl) {
    return CachedNetworkImageProvider(
      imageUrl,
      maxWidth: thumbnailMaxWidth,
      maxHeight: thumbnailMaxHeight,
    );
  }
}

/// Optimized image widget for Quranic pages
class OptimizedQuranPageImage extends StatelessWidget {
  final String imageUrl;
  final int pageNumber;
  final VoidCallback? onTap;

  const OptimizedQuranPageImage({
    Key? key,
    required this.imageUrl,
    required this.pageNumber,
    this.onTap,
  }) : super(key: key);

  @override
  Widget build(BuildContext context) {
    return GestureDetector(
      onTap: onTap,
      child: Hero(
        tag: 'quran_page_$pageNumber',
        child: ImageOptimizationService().getOptimizedNetworkImage(
          imageUrl: imageUrl,
          fit: BoxFit.contain,
          placeholder: _buildQuranPlaceholder(),
          errorWidget: _buildQuranErrorWidget(),
        ),
      ),
    );
  }

  Widget _buildQuranPlaceholder() {
    return Container(
      color: const Color(0xFFFEFEFE),
      child: const Center(
        child: Column(
          mainAxisAlignment: MainAxisAlignment.center,
          children: [
            CircularProgressIndicator(
              strokeWidth: 2,
              valueColor: AlwaysStoppedAnimation<Color>(Color(0xFF1B365D)),
            ),
            SizedBox(height: 16),
            Text(
              'جاري تحميل الصفحة...',
              style: TextStyle(
                fontFamily: 'Tajawal',
                fontSize: 14,
                color: Color(0xFF666666),
              ),
            ),
          ],
        ),
      ),
    );
  }

  Widget _buildQuranErrorWidget() {
    return Container(
      color: const Color(0xFFFEFEFE),
      child: const Center(
        child: Column(
          mainAxisAlignment: MainAxisAlignment.center,
          children: [
            Icon(
              Icons.error_outline,
              color: Color(0xFFDC3545),
              size: 48,
            ),
            SizedBox(height: 16),
            Text(
              'فشل تحميل الصفحة',
              style: TextStyle(
                fontFamily: 'Tajawal',
                fontSize: 14,
                color: Color(0xFF666666),
              ),
            ),
          ],
        ),
      ),
    );
  }
}
