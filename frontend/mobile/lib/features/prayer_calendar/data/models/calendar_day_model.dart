/// Model for a single day in the prayer times calendar
class CalendarDayModel {
  final DateTime gregorianDate;
  final HijriDateModel hijriDate;
  final PrayerTimesModel prayerTimes;
  final List<IslamicEventModel> events;
  final bool isFriday;
  final bool isWeekend;
  final bool isToday;

  CalendarDayModel({
    required this.gregorianDate,
    required this.hijriDate,
    required this.prayerTimes,
    required this.events,
    required this.isFriday,
    required this.isWeekend,
    required this.isToday,
  });

  factory CalendarDayModel.fromJson(Map<String, dynamic> json) {
    return CalendarDayModel(
      gregorianDate: DateTime.parse(json['gregorian_date']),
      hijriDate: HijriDateModel.fromJson(json['hijri_date']),
      prayerTimes: PrayerTimesModel.fromJson(json['prayer_times']),
      events: (json['events'] as List?)
              ?.map((e) => IslamicEventModel.fromJson(e))
              .toList() ??
          [],
      isFriday: json['is_friday'] ?? false,
      isWeekend: json['is_weekend'] ?? false,
      isToday: json['is_today'] ?? false,
    );
  }

  Map<String, dynamic> toJson() {
    return {
      'gregorian_date': gregorianDate.toIso8601String(),
      'hijri_date': hijriDate.toJson(),
      'prayer_times': prayerTimes.toJson(),
      'events': events.map((e) => e.toJson()).toList(),
      'is_friday': isFriday,
      'is_weekend': isWeekend,
      'is_today': isToday,
    };
  }

  bool get hasEvents => events.isNotEmpty;
  bool get isSpecialDay => isFriday || hasEvents;
}

/// Model for Hijri date
class HijriDateModel {
  final int day;
  final int month;
  final int year;
  final String monthNameArabic;
  final String monthNameEnglish;
  final String weekdayArabic;

  HijriDateModel({
    required this.day,
    required this.month,
    required this.year,
    required this.monthNameArabic,
    required this.monthNameEnglish,
    required this.weekdayArabic,
  });

  factory HijriDateModel.fromJson(Map<String, dynamic> json) {
    return HijriDateModel(
      day: json['day'] ?? 1,
      month: json['month'] ?? 1,
      year: json['year'] ?? 1445,
      monthNameArabic: json['month_name_arabic'] ?? '',
      monthNameEnglish: json['month_name_english'] ?? '',
      weekdayArabic: json['weekday_arabic'] ?? '',
    );
  }

  Map<String, dynamic> toJson() {
    return {
      'day': day,
      'month': month,
      'year': year,
      'month_name_arabic': monthNameArabic,
      'month_name_english': monthNameEnglish,
      'weekday_arabic': weekdayArabic,
    };
  }

  String get formatted => '$weekdayArabic، $day $monthNameArabic $year هـ';
}

/// Model for prayer times
class PrayerTimesModel {
  final String fajr;
  final String sunrise;
  final String dhuhr;
  final String asr;
  final String maghrib;
  final String isha;

  PrayerTimesModel({
    required this.fajr,
    required this.sunrise,
    required this.dhuhr,
    required this.asr,
    required this.maghrib,
    required this.isha,
  });

  factory PrayerTimesModel.fromJson(Map<String, dynamic> json) {
    return PrayerTimesModel(
      fajr: json['fajr'] ?? '',
      sunrise: json['sunrise'] ?? '',
      dhuhr: json['dhuhr'] ?? '',
      asr: json['asr'] ?? '',
      maghrib: json['maghrib'] ?? '',
      isha: json['isha'] ?? '',
    );
  }

  Map<String, dynamic> toJson() {
    return {
      'fajr': fajr,
      'sunrise': sunrise,
      'dhuhr': dhuhr,
      'asr': asr,
      'maghrib': maghrib,
      'isha': isha,
    };
  }

  List<Map<String, String>> get allPrayers => [
        {'name': 'الفجر', 'time': fajr},
        {'name': 'الشروق', 'time': sunrise},
        {'name': 'الظهر', 'time': dhuhr},
        {'name': 'العصر', 'time': asr},
        {'name': 'المغرب', 'time': maghrib},
        {'name': 'العشاء', 'time': isha},
      ];
}

/// Model for Islamic events
class IslamicEventModel {
  final String id;
  final String nameArabic;
  final String nameEnglish;
  final String? descriptionArabic;
  final String? descriptionEnglish;
  final String eventType;
  final int importanceLevel;

  IslamicEventModel({
    required this.id,
    required this.nameArabic,
    required this.nameEnglish,
    this.descriptionArabic,
    this.descriptionEnglish,
    required this.eventType,
    required this.importanceLevel,
  });

  factory IslamicEventModel.fromJson(Map<String, dynamic> json) {
    return IslamicEventModel(
      id: json['id'] ?? '',
      nameArabic: json['name_arabic'] ?? '',
      nameEnglish: json['name_english'] ?? '',
      descriptionArabic: json['description_arabic'],
      descriptionEnglish: json['description_english'],
      eventType: json['event_type'] ?? '',
      importanceLevel: json['importance_level'] ?? 3,
    );
  }

  Map<String, dynamic> toJson() {
    return {
      'id': id,
      'name_arabic': nameArabic,
      'name_english': nameEnglish,
      'description_arabic': descriptionArabic,
      'description_english': descriptionEnglish,
      'event_type': eventType,
      'importance_level': importanceLevel,
    };
  }

  bool get isEid => eventType == 'eid';
  bool get isHolyNight => eventType == 'holy_night';
  bool get isHighImportance => importanceLevel >= 4;
}

/// Model for monthly calendar response
class MonthlyCalendarModel {
  final HijriMonthModel hijriMonth;
  final int hijriYear;
  final List<CalendarDayModel> days;
  final List<IslamicEventModel> events;

  MonthlyCalendarModel({
    required this.hijriMonth,
    required this.hijriYear,
    required this.days,
    required this.events,
  });

  factory MonthlyCalendarModel.fromJson(Map<String, dynamic> json) {
    return MonthlyCalendarModel(
      hijriMonth: HijriMonthModel.fromJson(json['hijri_month']),
      hijriYear: json['hijri_year'] ?? 1445,
      days: (json['days'] as List?)
              ?.map((d) => CalendarDayModel.fromJson(d))
              .toList() ??
          [],
      events: (json['events'] as List?)
              ?.map((e) => IslamicEventModel.fromJson(e))
              .toList() ??
          [],
    );
  }

  Map<String, dynamic> toJson() {
    return {
      'hijri_month': hijriMonth.toJson(),
      'hijri_year': hijriYear,
      'days': days.map((d) => d.toJson()).toList(),
      'events': events.map((e) => e.toJson()).toList(),
    };
  }
}

/// Model for Hijri month
class HijriMonthModel {
  final int monthNumber;
  final String nameArabic;
  final String nameEnglish;
  final String nameTransliteration;

  HijriMonthModel({
    required this.monthNumber,
    required this.nameArabic,
    required this.nameEnglish,
    required this.nameTransliteration,
  });

  factory HijriMonthModel.fromJson(Map<String, dynamic> json) {
    return HijriMonthModel(
      monthNumber: json['month_number'] ?? 1,
      nameArabic: json['name_arabic'] ?? '',
      nameEnglish: json['name_english'] ?? '',
      nameTransliteration: json['name_transliteration'] ?? '',
    );
  }

  Map<String, dynamic> toJson() {
    return {
      'month_number': monthNumber,
      'name_arabic': nameArabic,
      'name_english': nameEnglish,
      'name_transliteration': nameTransliteration,
    };
  }
}
