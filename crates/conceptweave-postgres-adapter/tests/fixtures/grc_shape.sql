-- Anonymized schema shape only. No application rows or source-system business facts.
CREATE SCHEMA "__SCHEMA__";
CREATE TYPE "__SCHEMA__".risk_level AS ENUM ('low', 'moderate', 'high');
CREATE DOMAIN "__SCHEMA__".impact_score AS integer CHECK (VALUE BETWEEN 0 AND 100);

CREATE TABLE "__SCHEMA__".tenant (
    tenant_id uuid PRIMARY KEY,
    tenant_code text NOT NULL UNIQUE
);
CREATE TABLE "__SCHEMA__".risk_record (
    tenant_id uuid NOT NULL,
    risk_id uuid NOT NULL,
    title text NOT NULL,
    level "__SCHEMA__".risk_level NOT NULL,
    score "__SCHEMA__".impact_score,
    CONSTRAINT risk_record_key PRIMARY KEY (tenant_id, risk_id),
    CONSTRAINT risk_record_tenant_fk FOREIGN KEY (tenant_id)
        REFERENCES "__SCHEMA__".tenant (tenant_id)
);
CREATE TABLE "__SCHEMA__".control_record (
    tenant_id uuid NOT NULL,
    control_id uuid NOT NULL,
    control_code text NOT NULL,
    CONSTRAINT control_record_key PRIMARY KEY (tenant_id, control_id),
    CONSTRAINT control_record_tenant_fk FOREIGN KEY (tenant_id)
        REFERENCES "__SCHEMA__".tenant (tenant_id)
);
CREATE TABLE "__SCHEMA__".risk_control_link (
    tenant_id uuid NOT NULL,
    risk_id uuid NOT NULL,
    control_id uuid NOT NULL,
    CONSTRAINT risk_control_link_key PRIMARY KEY (tenant_id, risk_id, control_id),
    CONSTRAINT risk_control_risk_fk FOREIGN KEY (tenant_id, risk_id)
        REFERENCES "__SCHEMA__".risk_record (tenant_id, risk_id),
    CONSTRAINT risk_control_control_fk FOREIGN KEY (tenant_id, control_id)
        REFERENCES "__SCHEMA__".control_record (tenant_id, control_id)
);
CREATE INDEX risk_record_level_idx ON "__SCHEMA__".risk_record (level);
CREATE INDEX risk_record_title_idx ON "__SCHEMA__".risk_record (title)
    INCLUDE (level) WHERE title IS NOT NULL;
COMMENT ON TABLE "__SCHEMA__".risk_record IS 'Anonymized risk catalog shape';
COMMENT ON COLUMN "__SCHEMA__".risk_record.title IS 'Reviewable risk title';
