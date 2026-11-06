-- Adiciona permissão de controle remoto e PIN para membros da equipe
ALTER TABLE team_members ADD COLUMN can_control_remote INTEGER NOT NULL DEFAULT 1;
ALTER TABLE team_members ADD COLUMN pin TEXT;
