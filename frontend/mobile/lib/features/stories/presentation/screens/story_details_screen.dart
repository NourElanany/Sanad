import 'package:flutter/material.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';
import '../../../../core/providers/stories_provider.dart';
import '../../../../core/widgets/islamic_card.dart';
import '../../../../core/widgets/islamic_loading_indicator.dart';
import '../widgets/story_source_card.dart';
import '../widgets/lesson_card.dart';
import '../widgets/character_chip.dart';

/// Story Details Screen with full content and metadata
class StoryDetailsScreen extends ConsumerStatefulWidget {
  final String storyId;

  const StoryDetailsScreen({
    Key? key,
    required this.storyId,
  }) : super(key: key);

  @override
  ConsumerState<StoryDetailsScreen> createState() =>
      _StoryDetailsScreenState();
}

class _StoryDetailsScreenState extends ConsumerState<StoryDetailsScreen> {
  bool _isChildrenMode = false;
  double _fontSize = 18.0;

  @override
  Widget build(BuildContext context) {
    final storyState = ref.watch(storyDetailsProvider(widget.storyId));

    return Scaffold(
      appBar: AppBar(
        title: const Text(
          'تفاصيل القصة',
          style: TextStyle(
            fontFamily: 'Tajawal',
            fontWeight: FontWeight.bold,
          ),
        ),
        centerTitle: true,
        actions: [
          IconButton(
            icon: Icon(_isChildrenMode ? Icons.child_care : Icons.person),
            onPressed: () {
              setState(() {
                _isChildrenMode = !_isChildrenMode;
              });
            },
            tooltip: _isChildrenMode ? 'وضع البالغين' : 'وضع الأطفال',
          ),
          PopupMenuButton<double>(
            icon: const Icon(Icons.text_fields),
            tooltip: 'حجم الخط',
            onSelected: (size) {
              setState(() {
                _fontSize = size;
              });
            },
            itemBuilder: (context) => [
              const PopupMenuItem(value: 14.0, child: Text('صغير')),
              const PopupMenuItem(value: 18.0, child: Text('متوسط')),
              const PopupMenuItem(value: 22.0, child: Text('كبير')),
              const PopupMenuItem(value: 26.0, child: Text('كبير جداً')),
            ],
          ),
        ],
      ),
      body: _buildBody(storyState),
    );
  }

  Widget _buildBody(StoryDetailsState state) {
    if (state.isLoading) {
      return const Center(child: IslamicLoadingIndicator());
    }

    if (state.error != null) {
      return Center(
        child: Column(
          mainAxisAlignment: MainAxisAlignment.center,
          children: [
            const Icon(Icons.error_outline, size: 64, color: Colors.red),
            const SizedBox(height: 16),
            const Text(
              'خطأ في تحميل القصة',
              style: TextStyle(
                fontSize: 18,
                fontWeight: FontWeight.bold,
                fontFamily: 'Tajawal',
              ),
            ),
            const SizedBox(height: 8),
            Text(
              state.error!,
              textAlign: TextAlign.center,
              style: const TextStyle(fontFamily: 'Tajawal'),
            ),
            const SizedBox(height: 16),
            ElevatedButton(
              onPressed: () {
                ref.read(storyDetailsProvider(widget.storyId).notifier).refresh();
              },
              child: const Text('إعادة المحاولة'),
            ),
          ],
        ),
      );
    }

    if (state.story == null) {
      return const Center(
        child: Text(
          'القصة غير موجودة',
          style: TextStyle(
            fontSize: 18,
            fontFamily: 'Tajawal',
          ),
        ),
      );
    }

    final storyDetails = state.story!;
    final story = storyDetails.story;

    return RefreshIndicator(
      onRefresh: () =>
          ref.read(storyDetailsProvider(widget.storyId).notifier).refresh(),
      child: SingleChildScrollView(
        padding: const EdgeInsets.all(16),
        child: Column(
          crossAxisAlignment: CrossAxisAlignment.stretch,
          children: [
            _buildHeader(story),
            const SizedBox(height: 16),
            _buildMetadata(story),
            const SizedBox(height: 24),
            if (story.summary != null) ...[
              _buildSummary(story.summary!),
              const SizedBox(height: 24),
            ],
            _buildContent(story.content),
            const SizedBox(height: 24),
            if (storyDetails.characters.isNotEmpty) ...[
              _buildCharactersSection(storyDetails.characters),
              const SizedBox(height: 24),
            ],
            if (storyDetails.lessons.isNotEmpty) ...[
              _buildLessonsSection(storyDetails.lessons),
              const SizedBox(height: 24),
            ],
            if (storyDetails.sources.isNotEmpty) ...[
              _buildSourcesSection(storyDetails.sources),
              const SizedBox(height: 24),
            ],
            if (story.moralLessons.isNotEmpty) ...[
              _buildMoralLessonsSection(story.moralLessons),
              const SizedBox(height: 24),
            ],
          ],
        ),
      ),
    );
  }

  Widget _buildHeader(story) {
    return IslamicCard(
      child: Column(
        crossAxisAlignment: CrossAxisAlignment.start,
        children: [
          Row(
            children: [
              Text(
                story.category.icon,
                style: const TextStyle(fontSize: 40),
              ),
              const SizedBox(width: 12),
              Expanded(
                child: Column(
                  crossAxisAlignment: CrossAxisAlignment.start,
                  children: [
                    Text(
                      story.arabicTitle,
                      style: const TextStyle(
                        fontSize: 24,
                        fontWeight: FontWeight.bold,
                        fontFamily: 'Tajawal',
                      ),
                    ),
                    if (story.title != story.arabicTitle) ...[
                      const SizedBox(height: 4),
                      Text(
                        story.title,
                        style: const TextStyle(
                          fontSize: 16,
                          color: Colors.grey,
                          fontFamily: 'Tajawal',
                        ),
                      ),
                    ],
                  ],
                ),
              ),
            ],
          ),
          const SizedBox(height: 12),
          Wrap(
            spacing: 8,
            runSpacing: 8,
            children: [
              _buildChip(
                story.categoryArabic,
                Colors.blue,
              ),
              _buildChip(
                story.ageGroupArabic,
                Colors.green,
              ),
              _buildChip(
                story.authenticityArabic,
                _getAuthenticityColor(story.authenticityLevel),
              ),
            ],
          ),
        ],
      ),
    );
  }

  Widget _buildMetadata(story) {
    return IslamicCard(
      child: Column(
        children: [
          _buildMetadataRow(
            Icons.access_time,
            'وقت القراءة',
            '${story.estimatedReadingTime} دقيقة',
          ),
          const Divider(),
          _buildMetadataRow(
            Icons.text_fields,
            'عدد الكلمات',
            '${story.wordCount} كلمة',
          ),
          if (story.timePeriod != null) ...[
            const Divider(),
            _buildMetadataRow(
              Icons.calendar_today,
              'الفترة الزمنية',
              story.timePeriod!.arabicName,
            ),
          ],
          if (story.location != null) ...[
            const Divider(),
            _buildMetadataRow(
              Icons.location_on,
              'المكان',
              story.location!,
            ),
          ],
        ],
      ),
    );
  }

  Widget _buildMetadataRow(IconData icon, String label, String value) {
    return Padding(
      padding: const EdgeInsets.symmetric(vertical: 8),
      child: Row(
        children: [
          Icon(icon, size: 20, color: Colors.grey),
          const SizedBox(width: 12),
          Text(
            label,
            style: const TextStyle(
              fontWeight: FontWeight.bold,
              fontFamily: 'Tajawal',
            ),
          ),
          const Spacer(),
          Text(
            value,
            style: const TextStyle(
              color: Colors.grey,
              fontFamily: 'Tajawal',
            ),
          ),
        ],
      ),
    );
  }

  Widget _buildSummary(String summary) {
    return IslamicCard(
      child: Column(
        crossAxisAlignment: CrossAxisAlignment.start,
        children: [
          const Row(
            children: [
              Icon(Icons.summarize, color: Colors.blue),
              SizedBox(width: 8),
              Text(
                'ملخص القصة',
                style: TextStyle(
                  fontSize: 18,
                  fontWeight: FontWeight.bold,
                  fontFamily: 'Tajawal',
                ),
              ),
            ],
          ),
          const SizedBox(height: 12),
          Text(
            summary,
            style: TextStyle(
              fontSize: _fontSize,
              height: 1.8,
              fontFamily: 'Tajawal',
            ),
            textDirection: TextDirection.rtl,
          ),
        ],
      ),
    );
  }

  Widget _buildContent(String content) {
    return IslamicCard(
      child: Column(
        crossAxisAlignment: CrossAxisAlignment.start,
        children: [
          Row(
            children: [
              const Icon(Icons.menu_book, color: Colors.green),
              const SizedBox(width: 8),
              const Text(
                'القصة الكاملة',
                style: TextStyle(
                  fontSize: 18,
                  fontWeight: FontWeight.bold,
                  fontFamily: 'Tajawal',
                ),
              ),
              const Spacer(),
              if (_isChildrenMode)
                Container(
                  padding: const EdgeInsets.symmetric(
                    horizontal: 12,
                    vertical: 4,
                  ),
                  decoration: BoxDecoration(
                    color: Colors.orange.withOpacity(0.2),
                    borderRadius: BorderRadius.circular(12),
                  ),
                  child: const Row(
                    children: [
                      Icon(Icons.child_care, size: 16, color: Colors.orange),
                      SizedBox(width: 4),
                      Text(
                        'وضع الأطفال',
                        style: TextStyle(
                          fontSize: 12,
                          color: Colors.orange,
                          fontFamily: 'Tajawal',
                        ),
                      ),
                    ],
                  ),
                ),
            ],
          ),
          const SizedBox(height: 16),
          Text(
            content,
            style: TextStyle(
              fontSize: _isChildrenMode ? _fontSize + 2 : _fontSize,
              height: _isChildrenMode ? 2.0 : 1.8,
              fontFamily: 'Tajawal',
              letterSpacing: _isChildrenMode ? 0.5 : 0,
            ),
            textDirection: TextDirection.rtl,
          ),
        ],
      ),
    );
  }

  Widget _buildCharactersSection(List characters) {
    return Column(
      crossAxisAlignment: CrossAxisAlignment.start,
      children: [
        const Row(
          children: [
            Icon(Icons.people, color: Colors.purple),
            SizedBox(width: 8),
            Text(
              'الشخصيات',
              style: TextStyle(
                fontSize: 18,
                fontWeight: FontWeight.bold,
                fontFamily: 'Tajawal',
              ),
            ),
          ],
        ),
        const SizedBox(height: 12),
        Wrap(
          spacing: 8,
          runSpacing: 8,
          children: characters.map((charInStory) {
            return CharacterChip(
              character: charInStory.character,
              role: charInStory.roleInStory,
              importance: charInStory.importanceLevel,
            );
          }).toList(),
        ),
      ],
    );
  }

  Widget _buildLessonsSection(List lessons) {
    return Column(
      crossAxisAlignment: CrossAxisAlignment.start,
      children: [
        const Row(
          children: [
            Icon(Icons.lightbulb, color: Colors.amber),
            SizedBox(width: 8),
            Text(
              'الدروس المستفادة',
              style: TextStyle(
                fontSize: 18,
                fontWeight: FontWeight.bold,
                fontFamily: 'Tajawal',
              ),
            ),
          ],
        ),
        const SizedBox(height: 12),
        ...lessons.map((lessonInStory) {
          return Padding(
            padding: const EdgeInsets.only(bottom: 12),
            child: LessonCard(
              lesson: lessonInStory.lesson,
              relevanceScore: lessonInStory.relevanceScore,
              explanation: lessonInStory.explanation,
            ),
          );
        }).toList(),
      ],
    );
  }

  Widget _buildSourcesSection(List sources) {
    return Column(
      crossAxisAlignment: CrossAxisAlignment.start,
      children: [
        const Row(
          children: [
            Icon(Icons.library_books, color: Colors.teal),
            SizedBox(width: 8),
            Text(
              'المصادر والمراجع',
              style: TextStyle(
                fontSize: 18,
                fontWeight: FontWeight.bold,
                fontFamily: 'Tajawal',
              ),
            ),
          ],
        ),
        const SizedBox(height: 12),
        ...sources.map((source) {
          return Padding(
            padding: const EdgeInsets.only(bottom: 12),
            child: StorySourceCard(source: source),
          );
        }).toList(),
      ],
    );
  }

  Widget _buildMoralLessonsSection(List<String> moralLessons) {
    return IslamicCard(
      child: Column(
        crossAxisAlignment: CrossAxisAlignment.start,
        children: [
          const Row(
            children: [
              Icon(Icons.star, color: Colors.amber),
              SizedBox(width: 8),
              Text(
                'العبر الأخلاقية',
                style: TextStyle(
                  fontSize: 18,
                  fontWeight: FontWeight.bold,
                  fontFamily: 'Tajawal',
                ),
              ),
            ],
          ),
          const SizedBox(height: 12),
          ...moralLessons.map((lesson) {
            return Padding(
              padding: const EdgeInsets.symmetric(vertical: 4),
              child: Row(
                crossAxisAlignment: CrossAxisAlignment.start,
                children: [
                  const Text('• ', style: TextStyle(fontSize: 20)),
                  Expanded(
                    child: Text(
                      lesson,
                      style: const TextStyle(
                        fontSize: 16,
                        fontFamily: 'Tajawal',
                      ),
                    ),
                  ),
                ],
              ),
            );
          }).toList(),
        ],
      ),
    );
  }

  Widget _buildChip(String label, Color color) {
    return Container(
      padding: const EdgeInsets.symmetric(horizontal: 12, vertical: 6),
      decoration: BoxDecoration(
        color: color.withOpacity(0.1),
        borderRadius: BorderRadius.circular(16),
        border: Border.all(color: color.withOpacity(0.3)),
      ),
      child: Text(
        label,
        style: TextStyle(
          color: color.withOpacity(0.9),
          fontSize: 12,
          fontWeight: FontWeight.bold,
          fontFamily: 'Tajawal',
        ),
      ),
    );
  }

  Color _getAuthenticityColor(authenticityLevel) {
    return Color(int.parse(
      authenticityLevel.colorCode.replaceFirst('#', '0xFF'),
    ));
  }
}
