-- Adiciona coluna role (cargo na equipe) para os membros da equipe de mídia
ALTER TABLE team_members ADD COLUMN role TEXT NOT NULL DEFAULT 'member';
