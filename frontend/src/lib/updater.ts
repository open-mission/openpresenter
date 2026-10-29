export interface UpdateCheckResponse {
  update_available: boolean;
  current_version: string;
  latest_version: string;
  release_name: string;
  release_notes: string;
  html_url: string;
  published_at: string;
  download_url: string | null;
  asset_name: string | null;
  asset_size: number;
}

export type UpdateStage =
  | { type: 'Idle' }
  | { type: 'Checking' }
  | { type: 'Downloading'; percent: number; downloaded_bytes: number; total_bytes: number }
  | { type: 'Extracting' }
  | { type: 'ReadyToRestart'; new_version: string }
  | { type: 'Error'; message: string };

export interface UpdaterStatusPayload {
  stage: UpdateStage;
  last_check: UpdateCheckResponse | null;
}

export async function fetchUpdateCheck(): Promise<UpdateCheckResponse> {
  const resp = await fetch('/api/updater/check');
  if (!resp.ok) {
    const errorText = await resp.text();
    throw new Error(errorText || 'Falha ao consultar atualizações');
  }
  return resp.json();
}

export async function fetchUpdaterStatus(): Promise<UpdaterStatusPayload> {
  const resp = await fetch('/api/updater/status');
  if (!resp.ok) {
    throw new Error('Falha ao obter status do updater');
  }
  return resp.json();
}

export async function triggerInstallUpdate(payload?: {
  download_url?: string;
  asset_name?: string;
  target_version?: string;
}): Promise<void> {
  const resp = await fetch('/api/updater/install', {
    method: 'POST',
    headers: { 'Content-Type': 'application/json' },
    body: JSON.stringify(payload || {})
  });
  if (!resp.ok) {
    const errorText = await resp.text();
    throw new Error(errorText || 'Falha ao iniciar atualização');
  }
}
