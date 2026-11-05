-- Tabela de Departamentos da Equipe de Mídia
CREATE TABLE IF NOT EXISTS team_departments (
    id TEXT PRIMARY KEY,
    name TEXT NOT NULL,
    description TEXT,
    color TEXT NOT NULL DEFAULT '#3b82f6',
    icon TEXT NOT NULL DEFAULT 'Users',
    display_order INTEGER NOT NULL DEFAULT 0,
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL
);

-- Tabela de Membros / Voluntários da Equipe
CREATE TABLE IF NOT EXISTS team_members (
    id TEXT PRIMARY KEY,
    name TEXT NOT NULL,
    phone TEXT,
    email TEXT,
    avatar_url TEXT,
    color TEXT NOT NULL DEFAULT '#10b981',
    notes TEXT,
    is_active INTEGER NOT NULL DEFAULT 1,
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL
);

-- Tabela Associativa N:N (Membros x Departamentos)
CREATE TABLE IF NOT EXISTS team_member_departments (
    member_id TEXT NOT NULL,
    department_id TEXT NOT NULL,
    created_at TEXT NOT NULL,
    PRIMARY KEY (member_id, department_id),
    FOREIGN KEY (member_id) REFERENCES team_members(id) ON DELETE CASCADE,
    FOREIGN KEY (department_id) REFERENCES team_departments(id) ON DELETE CASCADE
);

-- Tabela de Escalas de Culto / Eventos
CREATE TABLE IF NOT EXISTS service_rosters (
    id TEXT PRIMARY KEY,
    title TEXT NOT NULL,
    service_date TEXT NOT NULL,
    service_time TEXT,
    notes TEXT,
    is_active INTEGER NOT NULL DEFAULT 0,
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL
);

-- Tabela de Itens / Atribuições da Escala
CREATE TABLE IF NOT EXISTS service_roster_items (
    id TEXT PRIMARY KEY,
    roster_id TEXT NOT NULL,
    department_id TEXT NOT NULL,
    member_id TEXT NOT NULL,
    role TEXT NOT NULL DEFAULT 'lead',
    status TEXT NOT NULL DEFAULT 'pending',
    role_notes TEXT,
    created_at TEXT NOT NULL,
    FOREIGN KEY (roster_id) REFERENCES service_rosters(id) ON DELETE CASCADE,
    FOREIGN KEY (department_id) REFERENCES team_departments(id) ON DELETE CASCADE,
    FOREIGN KEY (member_id) REFERENCES team_members(id) ON DELETE CASCADE
);

-- Índices para buscas rápidas
CREATE INDEX IF NOT EXISTS idx_team_members_active ON team_members(is_active);
CREATE INDEX IF NOT EXISTS idx_service_rosters_date ON service_rosters(service_date);
CREATE INDEX IF NOT EXISTS idx_service_rosters_active ON service_rosters(is_active);
CREATE INDEX IF NOT EXISTS idx_roster_items_roster ON service_roster_items(roster_id);

-- Departamentos padrão iniciais (sem iluminação)
INSERT OR IGNORE INTO team_departments (id, name, description, color, icon, display_order, created_at, updated_at) VALUES
('projecao', 'Projeção', 'Operação de letras, bíblia e mídias no telão e palcos', '#3b82f6', 'MonitorPlay', 1, datetime('now'), datetime('now')),
('fotografia', 'Fotografia', 'Registro fotográfico dos momentos do culto e membros', '#ec4899', 'Camera', 2, datetime('now'), datetime('now')),
('stories', 'Stories & Redes', 'Cobertura em tempo real para Instagram e mídias sociais', '#f59e0b', 'Smartphone', 3, datetime('now'), datetime('now')),
('transmissao', 'Transmissão (Live)', 'Operação de câmeras, corte, OBS e streaming', '#8b5cf6', 'Video', 4, datetime('now'), datetime('now')),
('som', 'Áudio & Som', 'Mixagem de som do templo e transmissão', '#10b981', 'Volume2', 5, datetime('now'), datetime('now'));
