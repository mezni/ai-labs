-- Create sync scheduler table
CREATE TABLE IF NOT EXISTS sync_scheduler (
    scheduler_id SERIAL PRIMARY KEY,
    job_name VARCHAR(100) UNIQUE NOT NULL,
    enabled BOOLEAN DEFAULT TRUE,
    interval_minutes INTEGER DEFAULT 360, -- 6 hours
    last_run TIMESTAMPTZ,
    next_run TIMESTAMPTZ,
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW()
);

-- Insert OSM sync job
INSERT INTO sync_scheduler (job_name, interval_minutes, next_run) 
VALUES ('osm_sync', 360, NOW() + INTERVAL '1 minute')
ON CONFLICT (job_name) DO NOTHING;

-- Function to check and run scheduled jobs
CREATE OR REPLACE FUNCTION check_and_run_scheduled_jobs()
RETURNS INTEGER AS $$
DECLARE
    v_job RECORD;
    v_run_count INTEGER := 0;
BEGIN
    FOR v_job IN 
        SELECT * FROM sync_scheduler 
        WHERE enabled = TRUE 
        AND (next_run IS NULL OR next_run <= NOW())
    LOOP
        -- Run OSM sync job
        IF v_job.job_name = 'osm_sync' THEN
            CALL scheduled_osm_sync();
        END IF;
        
        -- Update job timestamps
        UPDATE sync_scheduler 
        SET 
            last_run = NOW(),
            next_run = NOW() + (interval_minutes || ' minutes')::INTERVAL,
            updated_at = NOW()
        WHERE scheduler_id = v_job.scheduler_id;
        
        v_run_count := v_run_count + 1;
    END LOOP;
    
    RETURN v_run_count;
END;
$$ LANGUAGE plpgsql;