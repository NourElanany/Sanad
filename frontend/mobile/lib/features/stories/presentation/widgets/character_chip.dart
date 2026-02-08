import 'package:flutter/material.dart';
import '../../data/models/story_model.dart';

/// Chip for displaying a character in a story
class CharacterChip extends StatelessWidget {
  final CharacterModel character;
  final String role;
  final String importance;

  const CharacterChip({
    Key? key,
    required this.character,
    required this.role,
    required this.importance,
  }) : super(key: key);

  @override
  Widget build(BuildContext context) {
    return Container(
      padding: const EdgeInsets.symmetric(horizontal: 12, vertical: 8),
      decoration: BoxDecoration(
        color: _getImportanceColor().withOpacity(0.1),
        borderRadius: BorderRadius.circular(16),
        border: Border.all(
          color: _getImportanceColor().withOpacity(0.3),
        ),
      ),
      child: Row(
        mainAxisSize: MainAxisSize.min,
        children: [
          Icon(
            _getCharacterIcon(),
            size: 16,
            color: _getImportanceColor(),
          ),
          const SizedBox(width: 6),
          Column(
            crossAxisAlignment: CrossAxisAlignment.start,
            mainAxisSize: MainAxisSize.min,
            children: [
              Text(
                character.arabicName,
                style: TextStyle(
                  fontSize: 13,
                  fontWeight: FontWeight.bold,
                  color: _getImportanceColor(),
                  fontFamily: 'Tajawal',
                ),
              ),
              Text(
                character.characterTypeArabic,
                style: TextStyle(
                  fontSize: 10,
                  color: _getImportanceColor().withOpacity(0.7),
                  fontFamily: 'Tajawal',
                ),
              ),
            ],
          ),
        ],
      ),
    );
  }

  IconData _getCharacterIcon() {
    switch (character.characterType) {
      case CharacterType.prophet:
      case CharacterType.messenger:
        return Icons.star;
      case CharacterType.companion:
        return Icons.people;
      case CharacterType.righteousPerson:
        return Icons.favorite;
      case CharacterType.scholar:
        return Icons.school;
      case CharacterType.ruler:
        return Icons.account_balance;
      case CharacterType.martyr:
        return Icons.military_tech;
      case CharacterType.convert:
        return Icons.person_add;
      case CharacterType.historicalFigure:
        return Icons.history_edu;
      case CharacterType.antagonist:
        return Icons.warning;
    }
  }

  Color _getImportanceColor() {
    switch (importance) {
      case 'primary':
        return Colors.purple;
      case 'secondary':
        return Colors.blue;
      case 'minor':
        return Colors.grey;
      default:
        return Colors.grey;
    }
  }
}
