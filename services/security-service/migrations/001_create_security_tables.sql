-- Migration for Security Service Database Tables
-- This creates tables for digital authentication, content integrity, and encrypted backups

-- Content signatures table for digital authentication
CREATE TABLE IF NOT EXISTS content_signatures (
    content_id UUID PRIMARY KEY,
    content_type VARCHAR(50) NOT NULL,
    sha256_hash VARCHAR(64) NOT NULL,
    sha512_hash VARCHAR(128) NOT NULL,
    digital_signature TEXT NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW(),
    version INTEGER NOT NULL DEFAULT 1,
    authority VARCHAR(255) NOT NULL,
    metadata JSONB DEFAULT '{}'::jsonb
);

-- Create indexes for content signatures
CREATE INDEX IF NOT EXISTS idx_content_signatures_type ON content_signatures(content_type);
CREATE INDEX IF NOT EXISTS idx_content_signatures_authority ON content_signatures(authority);
CREATE INDEX IF NOT EXISTS idx_content_signatures_created_at ON content_signatures(created_at);
CREATE INDEX IF NOT EXISTS idx_content_signatures_sha256 ON content_signatures(sha256_hash);

-- Encrypted backups table
CREATE TABLE IF NOT EXISTS encrypted_backups (
    id UUID PRIMARY KEY,
    backup_type VARCHAR(20) NOT NULL CHECK (backup_type IN ('full', 'incremental', 'differential')),
    encryption_level VARCHAR(20) NOT NULL CHECK (encryption_level IN ('standard', 'high', 'maximum')),
    encrypted_data BYTEA NOT NULL,
    encryption_hash VARCHAR(64) NOT NULL,
    content_manifest JSONB NOT NULL, -- Array of content IDs
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    metadata JSONB DEFAULT '{}'::jsonb
);

-- Create indexes for encrypted backups
CREATE INDEX IF NOT EXISTS idx_encrypted_backups_type ON encrypted_backups(backup_type);
CREATE INDEX IF NOT EXISTS idx_encrypted_backups_created_at ON encrypted_backups(created_at);
CREATE INDEX IF NOT EXISTS idx_encrypted_backups_encryption_level ON encrypted_backups(encryption_level);

-- Security audit logs table
CREATE TABLE IF NOT EXISTS security_audit_logs (
    id UUID PRIMARY KEY,
    event_type VARCHAR(50) NOT NULL,
    content_id UUID,
    user_id UUID,
    ip_address INET,
    details JSONB DEFAULT '{}'::jsonb,
    severity VARCHAR(20) NOT NULL CHECK (severity IN ('low', 'medium', 'high', 'critical')),
    timestamp TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- Create indexes for security audit logs
CREATE INDEX IF NOT EXISTS idx_security_logs_event_type ON security_audit_logs(event_type);
CREATE INDEX IF NOT EXISTS idx_security_logs_severity ON security_audit_logs(severity);
CREATE INDEX IF NOT EXISTS idx_security_logs_timestamp ON security_audit_logs(timestamp);
CREATE INDEX IF NOT EXISTS idx_security_logs_content_id ON security_audit_logs(content_id);
CREATE INDEX IF NOT EXISTS idx_security_logs_user_id ON security_audit_logs(user_id);

-- Content integrity records table
CREATE TABLE IF NOT EXISTS content_integrity_records (
    content_id UUID PRIMARY KEY,
    content_type VARCHAR(50) NOT NULL,
    sha256_hash VARCHAR(64) NOT NULL,
    sha512_hash VARCHAR(128) NOT NULL,
    digital_signature TEXT NOT NULL,
    authority VARCHAR(255) NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    last_verified TIMESTAMPTZ,
    verification_count INTEGER DEFAULT 0,
    is_trusted BOOLEAN DEFAULT TRUE
);

-- Create indexes for content integrity records
CREATE INDEX IF NOT EXISTS idx_content_integrity_type ON content_integrity_records(content_type);
CREATE INDEX IF NOT EXISTS idx_content_integrity_authority ON content_integrity_records(authority);
CREATE INDEX IF NOT EXISTS idx_content_integrity_trusted ON content_integrity_records(is_trusted);
CREATE INDEX IF NOT EXISTS idx_content_integrity_last_verified ON content_integrity_records(last_verified);

-- Reference database table for trusted content hashes
CREATE TABLE IF NOT EXISTS reference_content_database (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    content_hash VARCHAR(64) NOT NULL UNIQUE,
    content_type VARCHAR(50) NOT NULL,
    authority VARCHAR(255) NOT NULL,
    signature_id UUID REFERENCES content_signatures(content_id),
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    is_active BOOLEAN DEFAULT TRUE
);

-- Create indexes for reference database
CREATE INDEX IF NOT EXISTS idx_reference_db_hash ON reference_content_database(content_hash);
CREATE INDEX IF NOT EXISTS idx_reference_db_type ON reference_content_database(content_type);
CREATE INDEX IF NOT EXISTS idx_reference_db_authority ON reference_content_database(authority);
CREATE INDEX IF NOT EXISTS idx_reference_db_active ON reference_content_database(is_active);

-- Backup verification history table
CREATE TABLE IF NOT EXISTS backup_verification_history (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    backup_id UUID REFERENCES encrypted_backups(id),
    verification_time TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    is_valid BOOLEAN NOT NULL,
    integrity_score DECIMAL(3,2) NOT NULL CHECK (integrity_score >= 0.0 AND integrity_score <= 1.0),
    errors_found INTEGER DEFAULT 0,
    warnings_found INTEGER DEFAULT 0,
    content_items_verified INTEGER DEFAULT 0,
    corrupted_items JSONB DEFAULT '[]'::jsonb, -- Array of corrupted content IDs
    verification_details JSONB DEFAULT '{}'::jsonb
);

-- Create indexes for backup verification history
CREATE INDEX IF NOT EXISTS idx_backup_verification_backup_id ON backup_verification_history(backup_id);
CREATE INDEX IF NOT EXISTS idx_backup_verification_time ON backup_verification_history(verification_time);
CREATE INDEX IF NOT EXISTS idx_backup_verification_valid ON backup_verification_history(is_valid);

-- Create a function to automatically update the updated_at timestamp
CREATE OR REPLACE FUNCTION update_updated_at_column()
RETURNS TRIGGER AS $$
BEGIN
    NEW.updated_at = NOW();
    RETURN NEW;
END;
$$ language 'plpgsql';

-- Create trigger for content_signatures table
CREATE TRIGGER update_content_signatures_updated_at 
    BEFORE UPDATE ON content_signatures 
    FOR EACH ROW 
    EXECUTE FUNCTION update_updated_at_column();

-- Insert some initial trusted authorities
INSERT INTO reference_content_database (content_hash, content_type, authority, created_at, is_active)
VALUES 
    ('placeholder_hash_1', 'system', 'Sanad System', NOW(), TRUE),
    ('placeholder_hash_2', 'system', 'Islamic Foundation', NOW(), TRUE),
    ('placeholder_hash_3', 'system', 'Al-Azhar University', NOW(), TRUE),
    ('placeholder_hash_4', 'system', 'King Fahd Complex', NOW(), TRUE)
ON CONFLICT (content_hash) DO NOTHING;

-- Create a view for security dashboard
CREATE OR REPLACE VIEW security_dashboard AS
SELECT 
    'content_signatures' as table_name,
    COUNT(*) as total_records,
    COUNT(CASE WHEN created_at > NOW() - INTERVAL '24 hours' THEN 1 END) as last_24h,
    COUNT(CASE WHEN created_at > NOW() - INTERVAL '7 days' THEN 1 END) as last_7d
FROM content_signatures
UNION ALL
SELECT 
    'encrypted_backups' as table_name,
    COUNT(*) as total_records,
    COUNT(CASE WHEN created_at > NOW() - INTERVAL '24 hours' THEN 1 END) as last_24h,
    COUNT(CASE WHEN created_at > NOW() - INTERVAL '7 days' THEN 1 END) as last_7d
FROM encrypted_backups
UNION ALL
SELECT 
    'security_audit_logs' as table_name,
    COUNT(*) as total_records,
    COUNT(CASE WHEN timestamp > NOW() - INTERVAL '24 hours' THEN 1 END) as last_24h,
    COUNT(CASE WHEN timestamp > NOW() - INTERVAL '7 days' THEN 1 END) as last_7d
FROM security_audit_logs;

-- Create a view for security alerts (high and critical severity events)
CREATE OR REPLACE VIEW security_alerts AS
SELECT 
    id,
    event_type,
    content_id,
    user_id,
    ip_address,
    severity,
    timestamp,
    details
FROM security_audit_logs 
WHERE severity IN ('high', 'critical')
ORDER BY timestamp DESC;

COMMENT ON TABLE content_signatures IS 'Digital signatures for Islamic content authentication';
COMMENT ON TABLE encrypted_backups IS 'Encrypted backups of Islamic content with integrity verification';
COMMENT ON TABLE security_audit_logs IS 'Security events and audit trail for the system';
COMMENT ON TABLE content_integrity_records IS 'Content integrity tracking and verification history';
COMMENT ON TABLE reference_content_database IS 'Reference database of trusted content hashes';
COMMENT ON TABLE backup_verification_history IS 'History of backup integrity verification results';