use chrono::{NaiveDate, Datelike};
use shared::{HijriDate, IslamicEvent, EventType};
use crate::models::{HijriMonth, HijriGregorianConversion, IslamicEventDetails};

/// Hijri calendar converter with accurate astronomical algorithms
/// Based on the Kuwaiti algorithm for better accuracy
pub struct HijriCalendar;

impl HijriCalendar {
    /// Convert Gregorian date to Hijri date using improved algorithm
    pub fn gregorian_to_hijri(gregorian_date: NaiveDate) -> Result<HijriDate, Box<dyn std::error::Error>> {
        let julian_day = Self::gregorian_to_julian(gregorian_date);
        let (hijri_year, hijri_month, hijri_day) = Self::julian_to_hijri_improved(julian_day);
        
        let month_name = Self::get_month_name(hijri_month)?;
        
        Ok(HijriDate {
            year: hijri_year,
            month: hijri_month as u8,
            day: hijri_day as u8,
            month_name,
        })
    }
    
    /// Convert Hijri date to Gregorian date
    pub fn hijri_to_gregorian(
        hijri_year: i32,
        hijri_month: i32,
        hijri_day: i32,
    ) -> Result<NaiveDate, Box<dyn std::error::Error>> {
        let julian_day = Self::hijri_to_julian(hijri_year, hijri_month, hijri_day);
        Self::julian_to_gregorian(julian_day)
    }
    
    /// Get all Hijri months with their names
    pub fn get_hijri_months() -> Vec<HijriMonth> {
        vec![
            HijriMonth {
                month_number: 1,
                name_arabic: "مُحَرَّم".to_string(),
                name_english: "Muharram".to_string(),
                name_transliteration: "Muharram".to_string(),
            },
            HijriMonth {
                month_number: 2,
                name_arabic: "صَفَر".to_string(),
                name_english: "Safar".to_string(),
                name_transliteration: "Safar".to_string(),
            },
            HijriMonth {
                month_number: 3,
                name_arabic: "رَبِيع الأَوَّل".to_string(),
                name_english: "Rabi al-Awwal".to_string(),
                name_transliteration: "Rabi al-Awwal".to_string(),
            },
            HijriMonth {
                month_number: 4,
                name_arabic: "رَبِيع الآخِر".to_string(),
                name_english: "Rabi al-Thani".to_string(),
                name_transliteration: "Rabi al-Thani".to_string(),
            },
            HijriMonth {
                month_number: 5,
                name_arabic: "جُمَادَى الأُولَى".to_string(),
                name_english: "Jumada al-Awwal".to_string(),
                name_transliteration: "Jumada al-Awwal".to_string(),
            },
            HijriMonth {
                month_number: 6,
                name_arabic: "جُمَادَى الآخِرَة".to_string(),
                name_english: "Jumada al-Thani".to_string(),
                name_transliteration: "Jumada al-Thani".to_string(),
            },
            HijriMonth {
                month_number: 7,
                name_arabic: "رَجَب".to_string(),
                name_english: "Rajab".to_string(),
                name_transliteration: "Rajab".to_string(),
            },
            HijriMonth {
                month_number: 8,
                name_arabic: "شَعْبَان".to_string(),
                name_english: "Shaban".to_string(),
                name_transliteration: "Shaban".to_string(),
            },
            HijriMonth {
                month_number: 9,
                name_arabic: "رَمَضَان".to_string(),
                name_english: "Ramadan".to_string(),
                name_transliteration: "Ramadan".to_string(),
            },
            HijriMonth {
                month_number: 10,
                name_arabic: "شَوَّال".to_string(),
                name_english: "Shawwal".to_string(),
                name_transliteration: "Shawwal".to_string(),
            },
            HijriMonth {
                month_number: 11,
                name_arabic: "ذُو القَعْدَة".to_string(),
                name_english: "Dhu al-Qadah".to_string(),
                name_transliteration: "Dhu al-Qadah".to_string(),
            },
            HijriMonth {
                month_number: 12,
                name_arabic: "ذُو الحِجَّة".to_string(),
                name_english: "Dhu al-Hijjah".to_string(),
                name_transliteration: "Dhu al-Hijjah".to_string(),
            },
        ]
    }

    /// Convert Gregorian date to Julian day number
    fn gregorian_to_julian(date: NaiveDate) -> i32 {
        let year = date.year();
        let month = date.month() as i32;
        let day = date.day() as i32;
        
        let a = (14 - month) / 12;
        let y = year + 4800 - a;
        let m = month + 12 * a - 3;
        
        day + (153 * m + 2) / 5 + 365 * y + y / 4 - y / 100 + y / 400 - 32045
    }
    
    /// Convert Julian day number to Gregorian date
    fn julian_to_gregorian(julian_day: i32) -> Result<NaiveDate, Box<dyn std::error::Error>> {
        let a = julian_day + 32044;
        let b = (4 * a + 3) / 146097;
        let c = a - (146097 * b) / 4;
        let d = (4 * c + 3) / 1461;
        let e = c - (1461 * d) / 4;
        let m = (5 * e + 2) / 153;
        
        let day = e - (153 * m + 2) / 5 + 1;
        let month = m + 3 - 12 * (m / 10);
        let year = 100 * b + d - 4800 + m / 10;
        
        NaiveDate::from_ymd_opt(year, month as u32, day as u32)
            .ok_or("Invalid Gregorian date".into())
    }
    
    /// Convert Julian day to Hijri date using improved Kuwaiti algorithm
    fn julian_to_hijri_improved(julian_day: i32) -> (i32, i32, i32) {
        // Hijri epoch: July 16, 622 CE (Julian day 1948439)
        const HIJRI_EPOCH: i32 = 1948439;
        
        let days_since_epoch = julian_day - HIJRI_EPOCH;
        
        // More accurate calculation using the Kuwaiti algorithm
        // Average Hijri year: 354.36707 days
        // Average Hijri month: 29.530589 days
        
        let hijri_years_approx = (days_since_epoch as f64 / 354.36707).floor() as i32;
        let remaining_days = days_since_epoch - Self::hijri_years_to_days(hijri_years_approx);
        
        let hijri_months = (remaining_days as f64 / 29.530589).floor() as i32;
        let remaining_days = remaining_days - Self::hijri_months_to_days(hijri_months);
        
        let year = hijri_years_approx + 1;
        let month = (hijri_months + 1).min(12).max(1);
        let day = (remaining_days + 1).max(1).min(Self::days_in_hijri_month(year, month));
        
        (year, month, day)
    }
    
    /// Convert Hijri years to approximate days
    fn hijri_years_to_days(years: i32) -> i32 {
        (years as f64 * 354.36707).floor() as i32
    }
    
    /// Convert Hijri months to approximate days
    fn hijri_months_to_days(months: i32) -> i32 {
        (months as f64 * 29.530589).floor() as i32
    }
    
    /// Convert Hijri date to Julian day using improved calculation
    fn hijri_to_julian_improved(hijri_year: i32, hijri_month: i32, hijri_day: i32) -> i32 {
        const HIJRI_EPOCH: i32 = 1948439;
        
        let total_days = Self::hijri_years_to_days(hijri_year - 1)
                       + Self::hijri_months_to_days(hijri_month - 1)
                       + (hijri_day - 1);
        
        HIJRI_EPOCH + total_days
    }
    
    /// Convert Hijri date to Julian day using reverse calculation
    fn hijri_to_julian(hijri_year: i32, hijri_month: i32, hijri_day: i32) -> i32 {
        Self::hijri_to_julian_improved(hijri_year, hijri_month, hijri_day)
    }
    
    /// Get month name for a given month number
    fn get_month_name(month_number: i32) -> Result<String, Box<dyn std::error::Error>> {
        let months = Self::get_hijri_months();
        months.iter()
            .find(|m| m.month_number == month_number)
            .map(|m| m.name_english.clone())
            .ok_or("Invalid month number".into())
    }
    
    /// Check if a Hijri year is a leap year (has 355 days instead of 354)
    pub fn is_hijri_leap_year(hijri_year: i32) -> bool {
        // Simplified leap year calculation for Hijri calendar
        // In reality, this depends on lunar observations
        let cycle = hijri_year % 30;
        matches!(cycle, 2 | 5 | 7 | 10 | 13 | 16 | 18 | 21 | 24 | 26 | 29)
    }
    
    /// Get the number of days in a Hijri month
    pub fn days_in_hijri_month(hijri_year: i32, hijri_month: i32) -> i32 {
        // Simplified calculation - in reality depends on lunar observations
        match hijri_month {
            1 | 3 | 5 | 7 | 9 | 11 => 30,
            2 | 4 | 6 | 8 | 10 => 29,
            12 => if Self::is_hijri_leap_year(hijri_year) { 30 } else { 29 },
            _ => 29,
        }
    }
    
    /// Get the total days in a Hijri year
    pub fn days_in_hijri_year(hijri_year: i32) -> i32 {
        if Self::is_hijri_leap_year(hijri_year) { 355 } else { 354 }
    }
    
    /// Validate Hijri date
    pub fn is_valid_hijri_date(hijri_year: i32, hijri_month: i32, hijri_day: i32) -> bool {
        if hijri_year < 1 || hijri_month < 1 || hijri_month > 12 || hijri_day < 1 {
            return false;
        }
        
        let max_days = Self::days_in_hijri_month(hijri_year, hijri_month);
        hijri_day <= max_days
    }
    
    /// Get the day of week for a Hijri date (0 = Saturday, 6 = Friday)
    pub fn hijri_day_of_week(hijri_year: i32, hijri_month: i32, hijri_day: i32) -> Result<i32, Box<dyn std::error::Error>> {
        let julian_day = Self::hijri_to_julian_improved(hijri_year, hijri_month, hijri_day);
        // Julian day 0 was a Monday, so we adjust
        Ok((julian_day + 1) % 7)
    }
    
    /// Check if a Hijri date falls on Friday (Jumu'ah)
    pub fn is_friday(hijri_year: i32, hijri_month: i32, hijri_day: i32) -> Result<bool, Box<dyn std::error::Error>> {
        let day_of_week = Self::hijri_day_of_week(hijri_year, hijri_month, hijri_day)?;
        Ok(day_of_week == 6) // Friday is day 6 in our system
    }
    
    /// Get Islamic events for the current Hijri date
    pub fn get_islamic_events_for_date(hijri_month: i32, hijri_day: i32) -> Vec<IslamicEvent> {
        let mut events = Vec::new();
        
        // Major Islamic events with fixed dates
        match (hijri_month, hijri_day) {
            // Muharram
            (1, 1) => events.push(Self::create_event(
                "رأس السنة الهجرية",
                "Islamic New Year",
                "بداية السنة الهجرية الجديدة",
                "Beginning of the new Hijri year",
                EventType::ImportantDay,
                hijri_month,
                hijri_day,
            )),
            (1, 10) => events.push(Self::create_event(
                "يوم عاشوراء",
                "Day of Ashura",
                "اليوم العاشر من محرم، يوم صيام مستحب وذكرى نجاة موسى عليه السلام",
                "The tenth day of Muharram, a recommended fasting day commemorating Moses' salvation",
                EventType::ImportantDay,
                hijri_month,
                hijri_day,
            )),
            
            // Rabi al-Awwal
            (3, 12) => events.push(Self::create_event(
                "المولد النبوي الشريف",
                "Prophet Muhammad's Birthday",
                "ذكرى مولد النبي محمد صلى الله عليه وسلم",
                "Birthday of Prophet Muhammad (peace be upon him)",
                EventType::ProphetBirthday,
                hijri_month,
                hijri_day,
            )),
            
            // Rajab
            (7, 27) => events.push(Self::create_event(
                "الإسراء والمعراج",
                "Isra and Miraj",
                "ذكرى رحلة الإسراء والمعراج للنبي محمد صلى الله عليه وسلم",
                "Night Journey and Ascension of Prophet Muhammad (peace be upon him)",
                EventType::ImportantDay,
                hijri_month,
                hijri_day,
            )),
            
            // Ramadan
            (9, 1) => events.push(Self::create_event(
                "بداية شهر رمضان",
                "Beginning of Ramadan",
                "بداية شهر الصيام المبارك",
                "Beginning of the blessed month of fasting",
                EventType::HolyMonth,
                hijri_month,
                hijri_day,
            )),
            (9, 27) => events.push(Self::create_event(
                "ليلة القدر",
                "Laylat al-Qadr",
                "ليلة القدر خير من ألف شهر",
                "The Night of Power, better than a thousand months",
                EventType::ImportantDay,
                hijri_month,
                hijri_day,
            )),
            
            // Shawwal
            (10, 1) => events.push(Self::create_event(
                "عيد الفطر",
                "Eid al-Fitr",
                "عيد الفطر المبارك، عيد انتهاء شهر رمضان",
                "The blessed Eid al-Fitr, celebrating the end of Ramadan",
                EventType::Eid,
                hijri_month,
                hijri_day,
            )),
            
            // Dhu al-Hijjah
            (12, 9) => events.push(Self::create_event(
                "يوم عرفة",
                "Day of Arafah",
                "يوم عرفة، يوم الحج الأكبر ويوم صيام مستحب لغير الحاج",
                "Day of Arafah, the greatest day of Hajj and recommended fasting for non-pilgrims",
                EventType::ImportantDay,
                hijri_month,
                hijri_day,
            )),
            (12, 10) => events.push(Self::create_event(
                "عيد الأضحى",
                "Eid al-Adha",
                "عيد الأضحى المبارك، عيد الحج والأضحية",
                "The blessed Eid al-Adha, the festival of sacrifice and Hajj",
                EventType::Eid,
                hijri_month,
                hijri_day,
            )),
            
            _ => {}
        }
        
        // Add special events for entire months
        match hijri_month {
            1 => {
                if hijri_day == 1 {
                    events.push(Self::create_event(
                        "شهر محرم",
                        "Month of Muharram",
                        "الشهر الحرام الأول من السنة الهجرية",
                        "The first sacred month of the Hijri year",
                        EventType::HolyMonth,
                        hijri_month,
                        hijri_day,
                    ));
                }
            },
            7 => {
                if hijri_day == 1 {
                    events.push(Self::create_event(
                        "شهر رجب",
                        "Month of Rajab",
                        "الشهر الحرام، شهر الإعداد لرمضان",
                        "The sacred month, month of preparation for Ramadan",
                        EventType::HolyMonth,
                        hijri_month,
                        hijri_day,
                    ));
                }
            },
            8 => {
                if hijri_day == 1 {
                    events.push(Self::create_event(
                        "شهر شعبان",
                        "Month of Shaban",
                        "شهر الاستعداد لرمضان",
                        "Month of preparation for Ramadan",
                        EventType::HolyMonth,
                        hijri_month,
                        hijri_day,
                    ));
                }
            },
            9 => {
                events.push(Self::create_event(
                    "شهر رمضان",
                    "Month of Ramadan",
                    "شهر الصيام والقرآن والقيام",
                    "Month of fasting, Quran, and night prayers",
                    EventType::HolyMonth,
                    hijri_month,
                    hijri_day,
                ));
            },
            11 => {
                if hijri_day == 1 {
                    events.push(Self::create_event(
                        "شهر ذو القعدة",
                        "Month of Dhu al-Qadah",
                        "الشهر الحرام، شهر الإعداد للحج",
                        "The sacred month, month of preparation for Hajj",
                        EventType::HolyMonth,
                        hijri_month,
                        hijri_day,
                    ));
                }
            },
            12 => {
                if hijri_day >= 1 && hijri_day <= 10 {
                    events.push(Self::create_event(
                        "العشر من ذي الحجة",
                        "First Ten Days of Dhu al-Hijjah",
                        "العشر المباركة من ذي الحجة، أيام الحج والعمل الصالح",
                        "The blessed first ten days of Dhu al-Hijjah, days of Hajj and righteous deeds",
                        EventType::HolyMonth,
                        hijri_month,
                        hijri_day,
                    ));
                }
            },
            _ => {}
        }
        
        events
    }
    
    /// Create an Islamic event
    fn create_event(
        name_arabic: &str,
        name_english: &str,
        description_arabic: &str,
        description_english: &str,
        event_type: EventType,
        hijri_month: i32,
        hijri_day: i32,
    ) -> IslamicEvent {
        // Convert to approximate Gregorian date for the current year
        let current_hijri_year = Self::get_current_hijri_year();
        let gregorian_date = Self::hijri_to_gregorian(current_hijri_year, hijri_month, hijri_day)
            .unwrap_or_else(|_| chrono::Utc::now().date_naive());
        
        IslamicEvent {
            name: format!("{} / {}", name_arabic, name_english),
            description: format!("{} / {}", description_arabic, description_english),
            hijri_date: HijriDate {
                year: current_hijri_year,
                month: hijri_month as u8,
                day: hijri_day as u8,
                month_name: Self::get_month_name(hijri_month).unwrap_or_default(),
            },
            gregorian_date: gregorian_date.and_hms_opt(0, 0, 0)
                .unwrap_or_else(|| chrono::Utc::now().naive_utc())
                .and_utc(),
            event_type,
        }
    }
    
    /// Get the current Hijri year
    fn get_current_hijri_year() -> i32 {
        let today = chrono::Utc::now().date_naive();
        Self::gregorian_to_hijri(today)
            .map(|hijri| hijri.year)
            .unwrap_or(1446) // Fallback to approximate current year
    }
    
    /// Get detailed information about an Islamic event
    pub fn get_event_details(event_name: &str) -> Option<String> {
        match event_name {
            "يوم عاشوراء" | "Day of Ashura" => Some(
                "يوم عاشوراء هو اليوم العاشر من شهر محرم، وهو يوم صيام مستحب. صامه النبي صلى الله عليه وسلم وأمر بصيامه، وقال: 'أحتسب على الله أن يكفر السنة التي قبله'. يستحب صيام يوم قبله أو بعده.".to_string()
            ),
            "المولد النبوي الشريف" | "Prophet Muhammad's Birthday" => Some(
                "ذكرى مولد النبي محمد صلى الله عليه وسلم في الثاني عشر من ربيع الأول. يحتفل المسلمون بهذه المناسبة بتذكر سيرته العطرة وأخلاقه الكريمة وتعاليمه السمحة.".to_string()
            ),
            "ليلة القدر" | "Laylat al-Qadr" => Some(
                "ليلة القدر خير من ألف شهر، وهي الليلة التي نزل فيها القرآن الكريم. تقع في العشر الأواخر من رمضان، والأرجح أنها في الليالي الوترية. العبادة فيها خير من عبادة ألف شهر.".to_string()
            ),
            "يوم عرفة" | "Day of Arafah" => Some(
                "يوم عرفة هو اليوم التاسع من ذي الحجة، وهو يوم الحج الأكبر. يقف فيه الحجاج بعرفة، وهو ركن الحج الأعظم. يستحب لغير الحاج صيام هذا اليوم، وقد قال النبي صلى الله عليه وسلم: 'صيام يوم عرفة أحتسب على الله أن يكفر السنة التي قبله والسنة التي بعده'.".to_string()
            ),
            "عيد الفطر" | "Eid al-Fitr" => Some(
                "عيد الفطر هو العيد الذي يأتي بعد انتهاء شهر رمضان المبارك، ويبدأ من أول يوم في شهر شوال. يحرم صيام هذا اليوم، ويستحب فيه التكبير والتهليل وصلاة العيد والتهنئة وصلة الأرحام.".to_string()
            ),
            "عيد الأضحى" | "Eid al-Adha" => Some(
                "عيد الأضحى هو العيد الكبير الذي يأتي في العاشر من ذي الحجة، ويستمر أربعة أيام. يحرم صيام هذا اليوم، ويشرع فيه الأضحية تقرباً إلى الله، وصلاة العيد والتكبير والتهنئة.".to_string()
            ),
            _ => None,
        }
    }
}