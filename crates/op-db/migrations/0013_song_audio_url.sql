-- Migração 0013: Adiciona coluna audio_url na tabela songs para vincular trilhas de playback a músicas
ALTER TABLE songs ADD COLUMN audio_url TEXT;
