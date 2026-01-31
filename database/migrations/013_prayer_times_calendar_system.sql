-- Migration: Prayer Times and Calendar System
-- Description: Comprehensive prayer times calculation and Hijri calendar system

-- Prayer calculation methods enum
CREATE TYPE prayer_calculation_method AS ENUM (
    'muslim_world_league',
    'islamic_society_north_america',
    'egyptian_general_authority',
    'umm_al_qura_makkah',
    'university_islamic_sciences_karachi',
    'institute_geophysics_tehran',
    'shia',
    'custom'
);

-- Prayer names enum (already exists in 009_smart_notification_system.sql)
-- CREATE TYPE prayer_name AS ENUM (
--     'fajr',
--     'dhuhr',
--     'asr',
--     'maghrib',
--     'isha'
-- );

-- Locations table for prayer time calculations
CREATE TABLE locations (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    name VARCHAR(255) NOT NULL,
    city VARCHAR(255),
    country VARCHAR(255),
    latitude DECIMAL(10, 8) NOT NULL,
    longitude DECIMAL(11, 8) NOT NULL,
    timezone VARCHAR(100) NOT NULL,
    elevation_meters INTEGER DEFAULT 0,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT NOW()
);

-- Prayer times calculation settings
CREATE TABLE prayer_calculation_settings (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    user_id UUID REFERENCES users(id) ON DELETE CASCADE,
    location_id UUID REFERENCES locations(id) ON DELETE SET NULL,
    calculation_method prayer_calculation_method NOT NULL DEFAULT 'muslim_world_league',
    
    -- Custom angles for custom method
    fajr_angle DECIMAL(5, 2),
    maghrib_angle DECIMAL(5, 2),
    isha_angle DECIMAL(5, 2),
    
    -- Adjustments in minutes
    fajr_adjustment INTEGER DEFAULT 0,
    dhuhr_adjustment INTEGER DEFAULT 0,
    asr_adjustment INTEGER DEFAULT 0,
    maghrib_adjustment INTEGER DEFAULT 0,
    isha_adjustment INTEGER DEFAULT 0,
    
    -- Asr calculation method (1 = Shafi/Maliki/Hanbali, 2 = Hanafi)
    asr_method INTEGER DEFAULT 1,
    
    -- High latitude adjustment method
    high_latitude_adjustment VARCHAR(50) DEFAULT 'middle_of_night',
    
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    
    UNIQUE(user_id)
);

-- Daily prayer times cache
CREATE TABLE daily_prayer_times (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    location_id UUID NOT NULL REFERENCES locations(id) ON DELETE CASCADE,
    calculation_method prayer_calculation_method NOT NULL,
    date DATE NOT NULL,
    
    -- Prayer times in UTC
    fajr_time TIMESTAMP WITH TIME ZONE NOT NULL,
    sunrise_time TIMESTAMP WITH TIME ZONE NOT NULL,
    dhuhr_time TIMESTAMP WITH TIME ZONE NOT NULL,
    asr_time TIMESTAMP WITH TIME ZONE NOT NULL,
    maghrib_time TIMESTAMP WITH TIME ZONE NOT NULL,
    isha_time TIMESTAMP WITH TIME ZONE NOT NULL,
    
    -- Qibla direction in degrees from North
    qibla_direction DECIMAL(6, 3) NOT NULL,
    
    -- Calculation metadata
    fajr_angle DECIMAL(5, 2),
    maghrib_angle DECIMAL(5, 2),
    isha_angle DECIMAL(5, 2),
    asr_method INTEGER DEFAULT 1,
    
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    
    UNIQUE(location_id, calculation_method, date)
);

-- Hijri calendar months
CREATE TABLE hijri_months (
    month_number INTEGER PRIMARY KEY CHECK (month_number >= 1 AND month_number <= 12),
    name_arabic VARCHAR(50) NOT NULL,
    name_english VARCHAR(50) NOT NULL,
    name_transliteration VARCHAR(50) NOT NULL
);

-- Insert Hijri month names
INSERT INTO hijri_months (month_number, name_arabic, name_english, name_transliteration) VALUES
(1, 'مُحَرَّم', 'Muharram', 'Muharram'),
(2, 'صَفَر', 'Safar', 'Safar'),
(3, 'رَبِيع الأَوَّل', 'Rabi al-Awwal', 'Rabi al-Awwal'),
(4, 'رَبِيع الآخِر', 'Rabi al-Thani', 'Rabi al-Thani'),
(5, 'جُمَادَى الأُولَى', 'Jumada al-Awwal', 'Jumada al-Awwal'),
(6, 'جُمَادَى الآخِرَة', 'Jumada al-Thani', 'Jumada al-Thani'),
(7, 'رَجَب', 'Rajab', 'Rajab'),
(8, 'شَعْبَان', 'Shaban', 'Shaban'),
(9, 'رَمَضَان', 'Ramadan', 'Ramadan'),
(10, 'شَوَّال', 'Shawwal', 'Shawwal'),
(11, 'ذُو القَعْدَة', 'Dhu al-Qadah', 'Dhu al-Qadah'),
(12, 'ذُو الحِجَّة', 'Dhu al-Hijjah', 'Dhu al-Hijjah');

-- Islamic events and occasions
CREATE TABLE islamic_events (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    name_arabic VARCHAR(255) NOT NULL,
    name_english VARCHAR(255) NOT NULL,
    description_arabic TEXT,
    description_english TEXT,
    
    -- Hijri date (for recurring events)
    hijri_month INTEGER REFERENCES hijri_months(month_number),
    hijri_day INTEGER CHECK (hijri_day >= 1 AND hijri_day <= 30),
    
    -- For events spanning multiple days
    hijri_end_month INTEGER REFERENCES hijri_months(month_number),
    hijri_end_day INTEGER CHECK (hijri_end_day >= 1 AND hijri_end_day <= 30),
    
    -- Event type
    event_type VARCHAR(50) NOT NULL,
    
    -- Importance level (1-5, 5 being most important)
    importance_level INTEGER DEFAULT 3 CHECK (importance_level >= 1 AND importance_level <= 5),
    
    -- Whether this event should trigger notifications
    notification_enabled BOOLEAN DEFAULT TRUE,
    
    -- Special rules or calculations (e.g., "last_friday_of_ramadan")
    special_calculation VARCHAR(255),
    
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT NOW()
);

-- Insert major Islamic events
INSERT INTO islamic_events (
    name_arabic, name_english, description_arabic, description_english,
    hijri_month, hijri_day, event_type, importance_level
) VALUES
-- Muharram events
('رأس السنة الهجرية', 'Islamic New Year', 'بداية السنة الهجرية الجديدة', 'Beginning of the new Hijri year', 1, 1, 'new_year', 4),
('يوم عاشوراء', 'Day of Ashura', 'اليوم العاشر من محرم، يوم صيام مستحب', 'The tenth day of Muharram, a recommended fasting day', 1, 10, 'fasting_day', 5),

-- Rabi al-Awwal events
('المولد النبوي الشريف', 'Prophet Muhammad Birthday', 'ذكرى مولد النبي محمد صلى الله عليه وسلم', 'Birthday of Prophet Muhammad (peace be upon him)', 3, 12, 'prophet_birthday', 5),

-- Rajab events
('الإسراء والمعراج', 'Isra and Miraj', 'ذكرى رحلة الإسراء والمعراج', 'Night Journey and Ascension of Prophet Muhammad', 7, 27, 'holy_night', 4),

-- Ramadan events
('بداية شهر رمضان', 'Beginning of Ramadan', 'بداية شهر الصيام المبارك', 'Beginning of the blessed month of fasting', 9, 1, 'holy_month_start', 5),
('ليلة القدر', 'Laylat al-Qadr', 'ليلة القدر خير من ألف شهر', 'The Night of Power, better than a thousand months', 9, 27, 'holy_night', 5),

-- Shawwal events
('عيد الفطر', 'Eid al-Fitr', 'عيد الفطر المبارك', 'The blessed Eid al-Fitr', 10, 1, 'eid', 5),

-- Dhu al-Hijjah events
('يوم عرفة', 'Day of Arafah', 'يوم عرفة، يوم الحج الأكبر', 'Day of Arafah, the greatest day of Hajj', 12, 9, 'hajj_day', 5),
('عيد الأضحى', 'Eid al-Adha', 'عيد الأضحى المبارك', 'The blessed Eid al-Adha', 12, 10, 'eid', 5);

-- Hijri date conversion cache
CREATE TABLE hijri_gregorian_conversion (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    gregorian_date DATE NOT NULL,
    hijri_year INTEGER NOT NULL,
    hijri_month INTEGER NOT NULL REFERENCES hijri_months(month_number),
    hijri_day INTEGER NOT NULL CHECK (hijri_day >= 1 AND hijri_day <= 30),
    
    -- Julian day number for accurate conversion
    julian_day_number INTEGER NOT NULL,
    
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    
    UNIQUE(gregorian_date),
    UNIQUE(hijri_year, hijri_month, hijri_day)
);

-- User prayer time preferences
CREATE TABLE user_prayer_preferences (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    
    -- Notification preferences for each prayer
    fajr_notification_enabled BOOLEAN DEFAULT TRUE,
    fajr_notification_minutes INTEGER DEFAULT 15,
    
    dhuhr_notification_enabled BOOLEAN DEFAULT TRUE,
    dhuhr_notification_minutes INTEGER DEFAULT 15,
    
    asr_notification_enabled BOOLEAN DEFAULT TRUE,
    asr_notification_minutes INTEGER DEFAULT 15,
    
    maghrib_notification_enabled BOOLEAN DEFAULT TRUE,
    maghrib_notification_minutes INTEGER DEFAULT 15,
    
    isha_notification_enabled BOOLEAN DEFAULT TRUE,
    isha_notification_minutes INTEGER DEFAULT 15,
    
    -- Additional preferences
    sunrise_notification_enabled BOOLEAN DEFAULT FALSE,
    sunrise_notification_minutes INTEGER DEFAULT 15,
    
    -- Graduated notifications
    graduated_notifications_enabled BOOLEAN DEFAULT TRUE,
    graduated_intervals INTEGER[] DEFAULT '{30, 15, 5}',
    
    -- Qibla compass preferences
    show_qibla_direction BOOLEAN DEFAULT TRUE,
    qibla_compass_style VARCHAR(50) DEFAULT 'traditional',
    
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    
    UNIQUE(user_id)
);

-- Prayer time history for analytics
CREATE TABLE prayer_time_history (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    prayer_name prayer_name NOT NULL,
    scheduled_time TIMESTAMP WITH TIME ZONE NOT NULL,
    actual_prayer_time TIMESTAMP WITH TIME ZONE,
    location_id UUID REFERENCES locations(id),
    
    -- Prayer completion status
    prayer_completed BOOLEAN DEFAULT FALSE,
    completion_method VARCHAR(50), -- 'on_time', 'early', 'late', 'qada'
    
    -- Congregation info
    prayed_in_congregation BOOLEAN DEFAULT FALSE,
    mosque_name VARCHAR(255),
    
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW()
);

-- Indexes for performance
CREATE INDEX idx_locations_coordinates ON locations(latitude, longitude);
CREATE INDEX idx_daily_prayer_times_date ON daily_prayer_times(date);
CREATE INDEX idx_daily_prayer_times_location_date ON daily_prayer_times(location_id, date);
CREATE INDEX idx_islamic_events_hijri_date ON islamic_events(hijri_month, hijri_day);
CREATE INDEX idx_hijri_conversion_gregorian ON hijri_gregorian_conversion(gregorian_date);
CREATE INDEX idx_hijri_conversion_hijri ON hijri_gregorian_conversion(hijri_year, hijri_month, hijri_day);
CREATE INDEX idx_prayer_history_user_date ON prayer_time_history(user_id, scheduled_time);
CREATE INDEX idx_prayer_history_prayer_name ON prayer_time_history(prayer_name);

-- Functions for prayer time calculations

-- Function to calculate Qibla direction
CREATE OR REPLACE FUNCTION calculate_qibla_direction(
    lat DECIMAL(10, 8),
    lng DECIMAL(11, 8)
) RETURNS DECIMAL(6, 3) AS $$
DECLARE
    -- Kaaba coordinates
    kaaba_lat CONSTANT DECIMAL(10, 8) := 21.4224779;
    kaaba_lng CONSTANT DECIMAL(11, 8) := 39.8251832;
    
    lat_rad DECIMAL;
    lng_rad DECIMAL;
    kaaba_lat_rad DECIMAL;
    kaaba_lng_rad DECIMAL;
    
    delta_lng DECIMAL;
    y DECIMAL;
    x DECIMAL;
    bearing DECIMAL;
BEGIN
    -- Convert to radians
    lat_rad := RADIANS(lat);
    lng_rad := RADIANS(lng);
    kaaba_lat_rad := RADIANS(kaaba_lat);
    kaaba_lng_rad := RADIANS(kaaba_lng);
    
    delta_lng := kaaba_lng_rad - lng_rad;
    
    -- Calculate bearing using spherical trigonometry
    y := SIN(delta_lng) * COS(kaaba_lat_rad);
    x := COS(lat_rad) * SIN(kaaba_lat_rad) - SIN(lat_rad) * COS(kaaba_lat_rad) * COS(delta_lng);
    
    bearing := DEGREES(ATAN2(y, x));
    
    -- Normalize to 0-360 degrees
    IF bearing < 0 THEN
        bearing := bearing + 360;
    END IF;
    
    RETURN bearing;
END;
$$ LANGUAGE plpgsql IMMUTABLE;

-- Function to get Islamic events for a specific Hijri date
CREATE OR REPLACE FUNCTION get_islamic_events_for_hijri_date(
    hijri_year INTEGER,
    hijri_month INTEGER,
    hijri_day INTEGER
) RETURNS TABLE(
    event_id UUID,
    name_arabic VARCHAR(255),
    name_english VARCHAR(255),
    description_arabic TEXT,
    description_english TEXT,
    event_type VARCHAR(50),
    importance_level INTEGER
) AS $$
BEGIN
    RETURN QUERY
    SELECT 
        ie.id,
        ie.name_arabic,
        ie.name_english,
        ie.description_arabic,
        ie.description_english,
        ie.event_type,
        ie.importance_level
    FROM islamic_events ie
    WHERE ie.hijri_month = $2 
      AND ie.hijri_day = $3
      AND ie.notification_enabled = TRUE;
END;
$$ LANGUAGE plpgsql STABLE;

-- Function to convert Gregorian to Hijri (simplified algorithm)
CREATE OR REPLACE FUNCTION gregorian_to_hijri(gregorian_date DATE)
RETURNS TABLE(hijri_year INTEGER, hijri_month INTEGER, hijri_day INTEGER) AS $$
DECLARE
    -- Hijri epoch: July 16, 622 CE (Julian day 1948439)
    hijri_epoch CONSTANT INTEGER := 1948439;
    julian_day INTEGER;
    days_since_epoch INTEGER;
    hijri_years INTEGER;
    remaining_days INTEGER;
    hijri_months INTEGER;
    result_year INTEGER;
    result_month INTEGER;
    result_day INTEGER;
BEGIN
    -- Calculate Julian day number
    julian_day := EXTRACT(JULIAN FROM gregorian_date);
    days_since_epoch := julian_day - hijri_epoch;
    
    -- Approximate conversion (354.367 days per Hijri year)
    hijri_years := FLOOR(days_since_epoch / 354.367);
    remaining_days := days_since_epoch - FLOOR(hijri_years * 354.367);
    
    -- Calculate months (approximately 29.53 days per month)
    hijri_months := FLOOR(remaining_days / 29.53);
    remaining_days := remaining_days - FLOOR(hijri_months * 29.53);
    
    result_year := hijri_years + 1;
    result_month := LEAST(hijri_months + 1, 12);
    result_day := GREATEST(remaining_days + 1, 1);
    
    -- Ensure day is within valid range
    IF result_day > 30 THEN
        result_day := 30;
    END IF;
    
    RETURN QUERY SELECT result_year, result_month, result_day;
END;
$$ LANGUAGE plpgsql IMMUTABLE;

-- Function to convert Hijri to Gregorian (simplified algorithm)
CREATE OR REPLACE FUNCTION hijri_to_gregorian(
    hijri_year INTEGER,
    hijri_month INTEGER,
    hijri_day INTEGER
) RETURNS DATE AS $$
DECLARE
    hijri_epoch CONSTANT INTEGER := 1948439;
    total_days INTEGER;
    julian_day INTEGER;
BEGIN
    -- Calculate total days since Hijri epoch
    total_days := FLOOR((hijri_year - 1) * 354.367) + 
                  FLOOR((hijri_month - 1) * 29.53) + 
                  (hijri_day - 1);
    
    julian_day := hijri_epoch + total_days;
    
    -- Convert Julian day to Gregorian date
    RETURN (DATE '1900-01-01' + (julian_day - 2415021));
END;
$$ LANGUAGE plpgsql IMMUTABLE;

-- Insert some default locations
INSERT INTO locations (name, city, country, latitude, longitude, timezone) VALUES
('Makkah', 'Makkah', 'Saudi Arabia', 21.4224779, 39.8251832, 'Asia/Riyadh'),
('Madinah', 'Madinah', 'Saudi Arabia', 24.4686, 39.6142, 'Asia/Riyadh'),
('Cairo', 'Cairo', 'Egypt', 30.0444, 31.2357, 'Africa/Cairo'),
('Istanbul', 'Istanbul', 'Turkey', 41.0082, 28.9784, 'Europe/Istanbul'),
('London', 'London', 'United Kingdom', 51.5074, -0.1278, 'Europe/London'),
('New York', 'New York', 'United States', 40.7128, -74.0060, 'America/New_York'),
('Jakarta', 'Jakarta', 'Indonesia', -6.2088, 106.8456, 'Asia/Jakarta'),
('Kuala Lumpur', 'Kuala Lumpur', 'Malaysia', 3.1390, 101.6869, 'Asia/Kuala_Lumpur');

-- Populate some Hijri-Gregorian conversion data for current years
-- This would typically be populated by a more accurate astronomical calculation
INSERT INTO hijri_gregorian_conversion (gregorian_date, hijri_year, hijri_month, hijri_day, julian_day_number)
SELECT 
    date_series,
    EXTRACT(YEAR FROM (SELECT hijri_year FROM gregorian_to_hijri(date_series))),
    EXTRACT(MONTH FROM (SELECT hijri_month FROM gregorian_to_hijri(date_series))),
    EXTRACT(DAY FROM (SELECT hijri_day FROM gregorian_to_hijri(date_series))),
    EXTRACT(JULIAN FROM date_series)
FROM generate_series('2024-01-01'::date, '2025-12-31'::date, '1 day'::interval) AS date_series
ON CONFLICT (gregorian_date) DO NOTHING;

-- Create trigger to update timestamps
CREATE OR REPLACE FUNCTION update_updated_at_column()
RETURNS TRIGGER AS $$
BEGIN
    NEW.updated_at = NOW();
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

CREATE TRIGGER update_prayer_calculation_settings_updated_at
    BEFORE UPDATE ON prayer_calculation_settings
    FOR EACH ROW
    EXECUTE FUNCTION update_updated_at_column();

CREATE TRIGGER update_islamic_events_updated_at
    BEFORE UPDATE ON islamic_events
    FOR EACH ROW
    EXECUTE FUNCTION update_updated_at_column();

CREATE TRIGGER update_user_prayer_preferences_updated_at
    BEFORE UPDATE ON user_prayer_preferences
    FOR EACH ROW
    EXECUTE FUNCTION update_updated_at_column();