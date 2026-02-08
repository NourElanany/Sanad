import 'package:flutter_riverpod/flutter_riverpod.dart';
import '../services/tafsir_service.dart';
import '../../features/quran/data/models/tafsir_model.dart';

// Tafsir Sources Provider
final tafsirSourcesProvider =
    StateNotifierProvider<TafsirSourcesNotifier, AsyncValue<List<TafsirSource>>>(
  (ref) => TafsirSourcesNotifier(ref.read(tafsirServiceProvider)),
);

class TafsirSourcesNotifier
    extends StateNotifier<AsyncValue<List<TafsirSource>>> {
  final TafsirService _tafsirService;

  TafsirSourcesNotifier(this._tafsirService)
      : super(const AsyncValue.loading());

  Future<void> loadSources() async {
    state = const AsyncValue.loading();
    try {
      final sources = await _tafsirService.getTafsirSources();
      state = AsyncValue.data(sources);
    } catch (e, stack) {
      state = AsyncValue.error(e, stack);
    }
  }
}

// Tafsir Provider
final tafsirProvider = StateNotifierProvider<TafsirNotifier,
    AsyncValue<List<TafsirWithSource>>>(
  (ref) => TafsirNotifier(ref.read(tafsirServiceProvider)),
);

class TafsirNotifier
    extends StateNotifier<AsyncValue<List<TafsirWithSource>>> {
  final TafsirService _tafsirService;

  TafsirNotifier(this._tafsirService) : super(const AsyncValue.loading());

  Future<void> loadTafsir(
    int surahNumber,
    int ayahNumber,
    List<String> sourceIds,
  ) async {
    if (sourceIds.isEmpty) {
      state = const AsyncValue.data([]);
      return;
    }

    state = const AsyncValue.loading();
    try {
      final tafsirs = await _tafsirService.getTafsirForAyah(
        surahNumber,
        ayahNumber,
        sourceIds: sourceIds,
      );
      state = AsyncValue.data(tafsirs);
    } catch (e, stack) {
      state = AsyncValue.error(e, stack);
    }
  }

  Future<void> downloadForOffline(
    int surahNumber,
    List<String> sourceIds,
  ) async {
    try {
      await _tafsirService.downloadTafsirForOffline(surahNumber, sourceIds);
    } catch (e) {
      rethrow;
    }
  }
}

// Tafsir Comparison Provider
final tafsirComparisonProvider = StateNotifierProvider.family<
    TafsirComparisonNotifier,
    AsyncValue<TafsirComparisonResponse>,
    TafsirComparisonParams>(
  (ref, params) => TafsirComparisonNotifier(
    ref.read(tafsirServiceProvider),
    params,
  ),
);

class TafsirComparisonParams {
  final int surahNumber;
  final int ayahNumber;
  final List<String> sourceIds;
  final List<ComparisonCriteria> criteria;

  TafsirComparisonParams({
    required this.surahNumber,
    required this.ayahNumber,
    required this.sourceIds,
    this.criteria = const [
      ComparisonCriteria.linguistic,
      ComparisonCriteria.thematic,
    ],
  });

  @override
  bool operator ==(Object other) =>
      identical(this, other) ||
      other is TafsirComparisonParams &&
          runtimeType == other.runtimeType &&
          surahNumber == other.surahNumber &&
          ayahNumber == other.ayahNumber &&
          sourceIds.toString() == other.sourceIds.toString() &&
          criteria.toString() == other.criteria.toString();

  @override
  int get hashCode =>
      surahNumber.hashCode ^
      ayahNumber.hashCode ^
      sourceIds.hashCode ^
      criteria.hashCode;
}

class TafsirComparisonNotifier
    extends StateNotifier<AsyncValue<TafsirComparisonResponse>> {
  final TafsirService _tafsirService;
  final TafsirComparisonParams params;

  TafsirComparisonNotifier(this._tafsirService, this.params)
      : super(const AsyncValue.loading()) {
    _loadComparison();
  }

  Future<void> _loadComparison() async {
    state = const AsyncValue.loading();
    try {
      final comparison = await _tafsirService.compareTafsir(
        surahNumber: params.surahNumber,
        ayahNumber: params.ayahNumber,
        sourceIds: params.sourceIds,
        comparisonCriteria: params.criteria,
      );
      state = AsyncValue.data(comparison);
    } catch (e, stack) {
      state = AsyncValue.error(e, stack);
    }
  }

  Future<void> reload() async {
    await _loadComparison();
  }
}

// Tafsir Service Provider
final tafsirServiceProvider = Provider<TafsirService>((ref) {
  throw UnimplementedError('TafsirService must be overridden');
});
