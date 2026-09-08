CREATE INDEX IF NOT EXISTS idx_policy_versions_document
    ON rag.policy_versions(document_id);

CREATE INDEX IF NOT EXISTS idx_policy_versions_status
    ON rag.policy_versions(status);

CREATE INDEX IF NOT EXISTS idx_policy_versions_effective_date
    ON rag.policy_versions(effective_date);

CREATE INDEX IF NOT EXISTS idx_policy_chunks_version
    ON rag.policy_chunks(version_id);

CREATE INDEX IF NOT EXISTS idx_policy_chunks_page
    ON rag.policy_chunks(page_number);

CREATE INDEX IF NOT EXISTS idx_policy_chunks_metadata
    ON rag.policy_chunks
    USING GIN(metadata);