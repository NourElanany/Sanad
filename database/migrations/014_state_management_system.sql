-- Migration for State Management System with CRDTs
-- This migration creates tables for advanced state management and synchronization

-- User personal data table (stores CRDT data as JSONB)
CREATE TABLE IF NOT EXISTS user_personal_data (
    user_id UUID PRIMARY KEY,
    data JSONB NOT NULL,
    last_updated TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW()
);

-- Index for faster queries
CREATE INDEX IF NOT EXISTS idx_user_personal_data_last_updated 
ON user_personal_data(last_updated);

-- Sync operations queue
CREATE TABLE IF NOT EXISTS sync_operations (
    id UUID PRIMARY KEY,
    operation_type TEXT NOT NULL,
    data BYTEA NOT NULL,
    priority TEXT NOT NULL,
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
    retry_count INTEGER NOT NULL DEFAULT 0,
    last_retry TIMESTAMP WITH TIME ZONE,
    error_message TEXT
);

-- Indexes for sync operations
CREATE INDEX IF NOT EXISTS idx_sync_operations_priority_created 
ON sync_operations(priority DESC, created_at ASC);

CREATE INDEX IF NOT EXISTS idx_sync_operations_retry_count 
ON sync_operations(retry_count) WHERE retry_count < 3;

-- Content metadata for smart storage management
CREATE TABLE IF NOT EXISTS content_metadata (
    id UUID PRIMARY KEY,
    content_type TEXT NOT NULL,
    size_bytes BIGINT NOT NULL,
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
    last_accessed TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
    access_count INTEGER NOT NULL DEFAULT 0,
    priority TEXT NOT NULL,
    compressed BOOLEAN NOT NULL DEFAULT FALSE,
    checksum TEXT NOT NULL
);

-- Indexes for content metadata
CREATE INDEX IF NOT EXISTS idx_content_metadata_priority 
ON content_metadata(priority);

CREATE INDEX IF NOT EXISTS idx_content_metadata_last_accessed 
ON content_metadata(last_accessed);

CREATE INDEX IF NOT EXISTS idx_content_metadata_size 
ON content_metadata(size_bytes DESC);

-- Device information for CRDT synchronization
CREATE TABLE IF NOT EXISTS devices (
    device_id TEXT PRIMARY KEY,
    user_id UUID REFERENCES users(id) ON DELETE CASCADE,
    device_name TEXT NOT NULL,
    device_type TEXT NOT NULL, -- 'mobile', 'tablet', 'desktop', 'web'
    last_sync TIMESTAMP WITH TIME ZONE,
    sync_version BIGINT NOT NULL DEFAULT 0,
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
    is_active BOOLEAN NOT NULL DEFAULT TRUE
);

-- Index for device queries
CREATE INDEX IF NOT EXISTS idx_devices_user_id 
ON devices(user_id);

CREATE INDEX IF NOT EXISTS idx_devices_last_sync 
ON devices(last_sync);

-- Sync conflicts log for debugging and resolution
CREATE TABLE IF NOT EXISTS sync_conflicts (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id UUID NOT NULL,
    device_id_1 TEXT NOT NULL,
    device_id_2 TEXT NOT NULL,
    conflict_type TEXT NOT NULL,
    local_data JSONB NOT NULL,
    remote_data JSONB NOT NULL,
    resolved_data JSONB,
    resolution_strategy TEXT NOT NULL,
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
    resolved_at TIMESTAMP WITH TIME ZONE
);

-- Index for conflict queries
CREATE INDEX IF NOT EXISTS idx_sync_conflicts_user_id 
ON sync_conflicts(user_id);

CREATE INDEX IF NOT EXISTS idx_sync_conflicts_created_at 
ON sync_conflicts(created_at);

-- Storage statistics for monitoring
CREATE TABLE IF NOT EXISTS storage_statistics (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    total_size_bytes BIGINT NOT NULL,
    available_space_bytes BIGINT NOT NULL,
    items_count INTEGER NOT NULL,
    compression_ratio DECIMAL(4,2) NOT NULL DEFAULT 1.0,
    last_cleanup TIMESTAMP WITH TIME ZONE,
    recorded_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW()
);

-- Index for storage statistics
CREATE INDEX IF NOT EXISTS idx_storage_statistics_recorded_at 
ON storage_statistics(recorded_at);

-- Version vectors for CRDT synchronization
CREATE TABLE IF NOT EXISTS version_vectors (
    user_id UUID NOT NULL,
    device_id TEXT NOT NULL,
    data_type TEXT NOT NULL, -- 'bookmarks', 'progress', 'notes', 'preferences'
    version BIGINT NOT NULL DEFAULT 0,
    last_updated TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
    PRIMARY KEY (user_id, device_id, data_type)
);

-- Index for version vector queries
CREATE INDEX IF NOT EXISTS idx_version_vectors_user_data_type 
ON version_vectors(user_id, data_type);

-- Bookmarks table (for efficient querying, data also stored in CRDT)
CREATE TABLE IF NOT EXISTS user_bookmarks (
    id UUID PRIMARY KEY,
    user_id UUID NOT NULL,
    content_type TEXT NOT NULL,
    content_id UUID NOT NULL,
    title TEXT NOT NULL,
    description TEXT,
    tags TEXT[] DEFAULT '{}',
    folder TEXT,
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
    device_id TEXT NOT NULL,
    is_deleted BOOLEAN NOT NULL DEFAULT FALSE
);

-- Indexes for bookmarks
CREATE INDEX IF NOT EXISTS idx_user_bookmarks_user_id 
ON user_bookmarks(user_id) WHERE NOT is_deleted;

CREATE INDEX IF NOT EXISTS idx_user_bookmarks_content 
ON user_bookmarks(content_type, content_id) WHERE NOT is_deleted;

CREATE INDEX IF NOT EXISTS idx_user_bookmarks_tags 
ON user_bookmarks USING GIN(tags) WHERE NOT is_deleted;

-- Reading progress table (for efficient querying)
CREATE TABLE IF NOT EXISTS reading_progress (
    user_id UUID NOT NULL,
    surah_number SMALLINT NOT NULL,
    last_ayah_read SMALLINT NOT NULL,
    completion_percentage DECIMAL(5,2) NOT NULL DEFAULT 0.0,
    last_read_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
    device_id TEXT NOT NULL,
    PRIMARY KEY (user_id, surah_number)
);

-- Index for reading progress
CREATE INDEX IF NOT EXISTS idx_reading_progress_last_read 
ON reading_progress(last_read_at);

-- Khatma progress table
CREATE TABLE IF NOT EXISTS khatma_progress (
    user_id UUID NOT NULL,
    khatma_id UUID NOT NULL,
    completed_portions INTEGER NOT NULL DEFAULT 0,
    total_portions INTEGER NOT NULL,
    last_read_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
    device_id TEXT NOT NULL,
    PRIMARY KEY (user_id, khatma_id)
);

-- Index for khatma progress
CREATE INDEX IF NOT EXISTS idx_khatma_progress_user_id 
ON khatma_progress(user_id);

-- Personal notes table
CREATE TABLE IF NOT EXISTS personal_notes (
    id UUID PRIMARY KEY,
    user_id UUID NOT NULL,
    content_type TEXT NOT NULL,
    content_id UUID NOT NULL,
    text TEXT NOT NULL,
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
    device_id TEXT NOT NULL,
    is_deleted BOOLEAN NOT NULL DEFAULT FALSE
);

-- Indexes for personal notes
CREATE INDEX IF NOT EXISTS idx_personal_notes_user_id 
ON personal_notes(user_id) WHERE NOT is_deleted;

CREATE INDEX IF NOT EXISTS idx_personal_notes_content 
ON personal_notes(content_type, content_id) WHERE NOT is_deleted;

CREATE INDEX IF NOT EXISTS idx_personal_notes_updated_at 
ON personal_notes(updated_at);

-- Functions for CRDT operations

-- Function to increment version vector
CREATE OR REPLACE FUNCTION increment_version_vector(
    p_user_id UUID,
    p_device_id TEXT,
    p_data_type TEXT
) RETURNS BIGINT AS $$
DECLARE
    new_version BIGINT;
BEGIN
    INSERT INTO version_vectors (user_id, device_id, data_type, version, last_updated)
    VALUES (p_user_id, p_device_id, p_data_type, 1, NOW())
    ON CONFLICT (user_id, device_id, data_type)
    DO UPDATE SET 
        version = version_vectors.version + 1,
        last_updated = NOW()
    RETURNING version INTO new_version;
    
    RETURN new_version;
END;
$$ LANGUAGE plpgsql;

-- Function to get max version for data type across all devices
CREATE OR REPLACE FUNCTION get_max_version(
    p_user_id UUID,
    p_data_type TEXT
) RETURNS BIGINT AS $$
DECLARE
    max_version BIGINT;
BEGIN
    SELECT COALESCE(MAX(version), 0) INTO max_version
    FROM version_vectors
    WHERE user_id = p_user_id AND data_type = p_data_type;
    
    RETURN max_version;
END;
$$ LANGUAGE plpgsql;

-- Trigger to update version vectors when data changes
CREATE OR REPLACE FUNCTION update_version_vector_trigger()
RETURNS TRIGGER AS $$
BEGIN
    -- Determine data type based on table name
    CASE TG_TABLE_NAME
        WHEN 'user_bookmarks' THEN
            PERFORM increment_version_vector(NEW.user_id, NEW.device_id, 'bookmarks');
        WHEN 'reading_progress' THEN
            PERFORM increment_version_vector(NEW.user_id, NEW.device_id, 'progress');
        WHEN 'personal_notes' THEN
            PERFORM increment_version_vector(NEW.user_id, NEW.device_id, 'notes');
    END CASE;
    
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

-- Create triggers for version vector updates
DROP TRIGGER IF EXISTS trigger_bookmarks_version ON user_bookmarks;
CREATE TRIGGER trigger_bookmarks_version
    AFTER INSERT OR UPDATE ON user_bookmarks
    FOR EACH ROW EXECUTE FUNCTION update_version_vector_trigger();

DROP TRIGGER IF EXISTS trigger_progress_version ON reading_progress;
CREATE TRIGGER trigger_progress_version
    AFTER INSERT OR UPDATE ON reading_progress
    FOR EACH ROW EXECUTE FUNCTION update_version_vector_trigger();

DROP TRIGGER IF EXISTS trigger_notes_version ON personal_notes;
CREATE TRIGGER trigger_notes_version
    AFTER INSERT OR UPDATE ON personal_notes
    FOR EACH ROW EXECUTE FUNCTION update_version_vector_trigger();

-- Views for easier querying

-- View for user data summary
CREATE OR REPLACE VIEW user_data_summary AS
SELECT 
    u.id as user_id,
    u.username,
    COUNT(DISTINCT b.id) as bookmarks_count,
    COUNT(DISTINCT rp.surah_number) as surahs_with_progress,
    COUNT(DISTINCT kp.khatma_id) as active_khatmas,
    COUNT(DISTINCT n.id) as notes_count,
    MAX(upd.last_updated) as last_data_update
FROM users u
LEFT JOIN user_bookmarks b ON u.id = b.user_id AND NOT b.is_deleted
LEFT JOIN reading_progress rp ON u.id = rp.user_id
LEFT JOIN khatma_progress kp ON u.id = kp.user_id
LEFT JOIN personal_notes n ON u.id = n.user_id AND NOT n.is_deleted
LEFT JOIN user_personal_data upd ON u.id = upd.user_id
GROUP BY u.id, u.username;

-- View for sync status
CREATE OR REPLACE VIEW sync_status AS
SELECT 
    d.user_id,
    d.device_id,
    d.device_name,
    d.last_sync,
    d.sync_version,
    COUNT(so.id) as pending_operations,
    MAX(vv.last_updated) as last_version_update
FROM devices d
LEFT JOIN sync_operations so ON so.data LIKE '%' || d.device_id || '%'
LEFT JOIN version_vectors vv ON d.user_id = vv.user_id AND d.device_id = vv.device_id
GROUP BY d.user_id, d.device_id, d.device_name, d.last_sync, d.sync_version;

-- Comments for documentation
COMMENT ON TABLE user_personal_data IS 'Stores user personal data using CRDT structures for conflict-free synchronization';
COMMENT ON TABLE sync_operations IS 'Queue for synchronization operations with priority and retry logic';
COMMENT ON TABLE content_metadata IS 'Metadata for smart local storage management and cleanup';
COMMENT ON TABLE devices IS 'Device information for multi-device synchronization';
COMMENT ON TABLE sync_conflicts IS 'Log of synchronization conflicts and their resolution';
COMMENT ON TABLE version_vectors IS 'Version vectors for CRDT synchronization tracking';

-- Grant permissions (adjust as needed for your setup)
-- GRANT SELECT, INSERT, UPDATE, DELETE ON ALL TABLES IN SCHEMA public TO sanad_app;
-- GRANT USAGE, SELECT ON ALL SEQUENCES IN SCHEMA public TO sanad_app;