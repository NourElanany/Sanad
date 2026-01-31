-- Insert default dhikr content for different categories
-- This provides pre-configured dhikr reminders that users can enable

-- Morning dhikr (أذكار الصباح)
INSERT INTO default_dhikr_content (
    category, title, arabic_text, transliteration, translation_en, translation_ar, 
    reference, repetitions, order_index
) VALUES 
(
    'morning', 
    'أذكار الصباح - الاستعاذة',
    'أَعُوذُ بِاللَّهِ مِنَ الشَّيْطَانِ الرَّجِيمِ',
    'A''udhu billahi min ash-shaytani''r-rajim',
    'I seek refuge in Allah from Satan, the accursed',
    'أعوذ بالله من الشيطان الرجيم',
    'القرآن الكريم',
    3,
    1
),
(
    'morning',
    'أذكار الصباح - آية الكرسي',
    'اللَّهُ لَا إِلَٰهَ إِلَّا هُوَ الْحَيُّ الْقَيُّومُ ۚ لَا تَأْخُذُهُ سِنَةٌ وَلَا نَوْمٌ ۚ لَهُ مَا فِي السَّمَاوَاتِ وَمَا فِي الْأَرْضِ',
    'Allahu la ilaha illa huwa''l-hayyu''l-qayyum...',
    'Allah - there is no deity except Him, the Ever-Living, the Sustainer of existence...',
    'الله لا إله إلا هو الحي القيوم...',
    'سورة البقرة: 255',
    1,
    2
),
(
    'morning',
    'أذكار الصباح - سبحان الله وبحمده',
    'سُبْحَانَ اللَّهِ وَبِحَمْدِهِ',
    'Subhan''allahi wa bihamdihi',
    'Glory is to Allah and praise is to Him',
    'سبحان الله وبحمده',
    'صحيح البخاري',
    100,
    3
),
(
    'morning',
    'أذكار الصباح - لا إله إلا الله وحده',
    'لَا إِلَٰهَ إِلَّا اللَّهُ وَحْدَهُ لَا شَرِيكَ لَهُ، لَهُ الْمُلْكُ وَلَهُ الْحَمْدُ وَهُوَ عَلَىٰ كُلِّ شَيْءٍ قَدِيرٌ',
    'La ilaha illa''llahu wahdahu la sharika lahu, lahu''l-mulku wa lahu''l-hamdu wa huwa ''ala kulli shay''in qadir',
    'There is no god but Allah alone, with no partner. His is the dominion and His is the praise, and He is able to do all things',
    'لا إله إلا الله وحده لا شريك له، له الملك وله الحمد وهو على كل شيء قدير',
    'صحيح البخاري',
    10,
    4
);

-- Evening dhikr (أذكار المساء)
INSERT INTO default_dhikr_content (
    category, title, arabic_text, transliteration, translation_en, translation_ar, 
    reference, repetitions, order_index
) VALUES 
(
    'evening',
    'أذكار المساء - الاستعاذة',
    'أَعُوذُ بِاللَّهِ مِنَ الشَّيْطَانِ الرَّجِيمِ',
    'A''udhu billahi min ash-shaytani''r-rajim',
    'I seek refuge in Allah from Satan, the accursed',
    'أعوذ بالله من الشيطان الرجيم',
    'القرآن الكريم',
    3,
    1
),
(
    'evening',
    'أذكار المساء - آية الكرسي',
    'اللَّهُ لَا إِلَٰهَ إِلَّا هُوَ الْحَيُّ الْقَيُّومُ ۚ لَا تَأْخُذُهُ سِنَةٌ وَلَا نَوْمٌ ۚ لَهُ مَا فِي السَّمَاوَاتِ وَمَا فِي الْأَرْضِ',
    'Allahu la ilaha illa huwa''l-hayyu''l-qayyum...',
    'Allah - there is no deity except Him, the Ever-Living, the Sustainer of existence...',
    'الله لا إله إلا هو الحي القيوم...',
    'سورة البقرة: 255',
    1,
    2
),
(
    'evening',
    'أذكار المساء - أمسينا وأمسى الملك لله',
    'أَمْسَيْنَا وَأَمْسَى الْمُلْكُ لِلَّهِ، وَالْحَمْدُ لِلَّهِ، لَا إِلَٰهَ إِلَّا اللَّهُ وَحْدَهُ لَا شَرِيكَ لَهُ',
    'Amsayna wa amsa''l-mulku lillahi, wa''l-hamdu lillahi, la ilaha illa''llahu wahdahu la sharika lahu',
    'We have entered the evening and the dominion belongs to Allah, and praise is to Allah. There is no god but Allah alone, with no partner',
    'أمسينا وأمسى الملك لله، والحمد لله، لا إله إلا الله وحده لا شريك له',
    'صحيح مسلم',
    1,
    3
);

-- After prayer dhikr (أذكار ما بعد الصلاة)
INSERT INTO default_dhikr_content (
    category, title, arabic_text, transliteration, translation_en, translation_ar, 
    reference, repetitions, order_index
) VALUES 
(
    'after_prayer',
    'أذكار ما بعد الصلاة - الاستغفار',
    'أَسْتَغْفِرُ اللَّهَ',
    'Astaghfiru''llah',
    'I seek forgiveness from Allah',
    'أستغفر الله',
    'صحيح مسلم',
    3,
    1
),
(
    'after_prayer',
    'أذكار ما بعد الصلاة - اللهم أنت السلام',
    'اللَّهُمَّ أَنْتَ السَّلَامُ وَمِنْكَ السَّلَامُ، تَبَارَكْتَ يَا ذَا الْجَلَالِ وَالْإِكْرَامِ',
    'Allahumma anta''s-salamu wa minka''s-salamu, tabarakta ya dha''l-jalali wa''l-ikram',
    'O Allah, You are Peace and from You comes peace. Blessed are You, O Owner of Majesty and Honor',
    'اللهم أنت السلام ومنك السلام، تباركت يا ذا الجلال والإكرام',
    'صحيح مسلم',
    1,
    2
),
(
    'after_prayer',
    'أذكار ما بعد الصلاة - سبحان الله',
    'سُبْحَانَ اللَّهِ',
    'Subhan''allah',
    'Glory is to Allah',
    'سبحان الله',
    'صحيح البخاري ومسلم',
    33,
    3
),
(
    'after_prayer',
    'أذكار ما بعد الصلاة - الحمد لله',
    'الْحَمْدُ لِلَّهِ',
    'Al-hamdu lillah',
    'Praise is to Allah',
    'الحمد لله',
    'صحيح البخاري ومسلم',
    33,
    4
),
(
    'after_prayer',
    'أذكار ما بعد الصلاة - الله أكبر',
    'اللَّهُ أَكْبَرُ',
    'Allahu akbar',
    'Allah is the Greatest',
    'الله أكبر',
    'صحيح البخاري ومسلم',
    34,
    5
);

-- Before sleep dhikr (أذكار النوم)
INSERT INTO default_dhikr_content (
    category, title, arabic_text, transliteration, translation_en, translation_ar, 
    reference, repetitions, order_index
) VALUES 
(
    'before_sleep',
    'أذكار النوم - باسمك اللهم أموت وأحيا',
    'بِاسْمِكَ اللَّهُمَّ أَمُوتُ وَأَحْيَا',
    'Bismika''llahumma amutu wa ahya',
    'In Your name, O Allah, I die and I live',
    'باسمك اللهم أموت وأحيا',
    'صحيح البخاري',
    1,
    1
),
(
    'before_sleep',
    'أذكار النوم - المعوذات',
    'قُلْ هُوَ اللَّهُ أَحَدٌ',
    'Qul huwa''llahu ahad',
    'Say: He is Allah, the One',
    'قل هو الله أحد',
    'سورة الإخلاص',
    3,
    2
);

-- After wudu dhikr (أذكار الوضوء)
INSERT INTO default_dhikr_content (
    category, title, arabic_text, transliteration, translation_en, translation_ar, 
    reference, repetitions, order_index
) VALUES 
(
    'after_wudu',
    'أذكار الوضوء - أشهد أن لا إله إلا الله',
    'أَشْهَدُ أَنْ لَا إِلَٰهَ إِلَّا اللَّهُ وَحْدَهُ لَا شَرِيكَ لَهُ، وَأَشْهَدُ أَنَّ مُحَمَّدًا عَبْدُهُ وَرَسُولُهُ',
    'Ashhadu an la ilaha illa''llahu wahdahu la sharika lahu, wa ashhadu anna Muhammadan ''abduhu wa rasuluh',
    'I bear witness that there is no god but Allah alone, with no partner, and I bear witness that Muhammad is His servant and messenger',
    'أشهد أن لا إله إلا الله وحده لا شريك له، وأشهد أن محمداً عبده ورسوله',
    'صحيح مسلم',
    1,
    1
);

-- Travel dhikr (أذكار السفر)
INSERT INTO default_dhikr_content (
    category, title, arabic_text, transliteration, translation_en, translation_ar, 
    reference, repetitions, order_index
) VALUES 
(
    'travel',
    'أذكار السفر - سبحان الذي سخر لنا هذا',
    'سُبْحَانَ الَّذِي سَخَّرَ لَنَا هَٰذَا وَمَا كُنَّا لَهُ مُقْرِنِينَ',
    'Subhana''lladhi sakhkhara lana hadha wa ma kunna lahu muqrinin',
    'Glory is to Him who has subjected this to us, and we could never have it by our efforts',
    'سبحان الذي سخر لنا هذا وما كنا له مقرنين',
    'سورة الزخرف: 13',
    1,
    1
);

-- General dhikr (أذكار عامة)
INSERT INTO default_dhikr_content (
    category, title, arabic_text, transliteration, translation_en, translation_ar, 
    reference, repetitions, order_index
) VALUES 
(
    'general',
    'أذكار عامة - لا حول ولا قوة إلا بالله',
    'لَا حَوْلَ وَلَا قُوَّةَ إِلَّا بِاللَّهِ',
    'La hawla wa la quwwata illa billah',
    'There is no power and no strength except with Allah',
    'لا حول ولا قوة إلا بالله',
    'صحيح البخاري ومسلم',
    1,
    1
),
(
    'general',
    'أذكار عامة - حسبنا الله ونعم الوكيل',
    'حَسْبُنَا اللَّهُ وَنِعْمَ الْوَكِيلُ',
    'Hasbuna''llahu wa ni''ma''l-wakil',
    'Allah is sufficient for us and He is the best Disposer of affairs',
    'حسبنا الله ونعم الوكيل',
    'صحيح البخاري',
    1,
    2
);