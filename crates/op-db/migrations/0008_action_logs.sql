-- Histórico auditável de ações do programa e identificação de instâncias
CREATE TABLE IF NOT EXISTS action_logs (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    timestamp TEXT NOT NULL,
    action TEXT NOT NULL,
    category TEXT NOT NULL,
    details TEXT NOT NULL,
    instance_id TEXT,
    client_type TEXT,
    device_info TEXT,
    ip_address TEXT
);

CREATE INDEX IF NOT EXISTS idx_action_logs_timestamp ON action_logs(timestamp DESC);
CREATE INDEX IF NOT EXISTS idx_action_logs_category ON action_logs(category);
