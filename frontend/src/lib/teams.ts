import type {
  RosterStatus,
  ServiceRoster,
  ServiceRosterItem,
  ServiceRosterSummary,
  TeamDepartment,
  TeamMember,
  UpsertServiceRoster,
  UpsertServiceRosterItem,
  UpsertTeamDepartment,
  UpsertTeamMember,
} from './types';

// ==========================================
// API REST CLIENT
// ==========================================

/**
 * Converte respostas HTTP de erro em `Error` com a mensagem do backend.
 * O corpo pode ser texto puro ou JSON (`error` / `message` / `detail`).
 */
async function ensureOk(res: Response, fallbackMessage: string): Promise<void> {
  if (res.ok) return;
  const body = await res.text().catch(() => '');
  const trimmed = body.trim();
  let detail = '';
  if (trimmed) {
    try {
      const parsed = JSON.parse(trimmed) as Record<string, unknown>;
      const raw = parsed.error ?? parsed.message ?? parsed.detail;
      if (typeof raw === 'string') detail = raw.trim();
    } catch {
      // Corpo não é JSON: usa o texto puro abaixo.
    }
  }
  throw new Error(detail || trimmed || fallbackMessage);
}

export async function fetchDepartments(signal?: AbortSignal): Promise<TeamDepartment[]> {
  const res = await fetch('/api/teams/departments', { signal });
  await ensureOk(res, 'Falha ao carregar departamentos');
  return res.json();
}

export async function saveDepartment(
  id: string,
  data: UpsertTeamDepartment
): Promise<TeamDepartment> {
  const isNew = !id;
  const url = isNew ? '/api/teams/departments' : `/api/teams/departments/${id}`;
  const res = await fetch(url, {
    method: isNew ? 'POST' : 'PUT',
    headers: { 'Content-Type': 'application/json' },
    body: JSON.stringify(data),
  });
  await ensureOk(res, 'Falha ao salvar departamento');
  return res.json();
}

export async function deleteDepartment(id: string): Promise<boolean> {
  const res = await fetch(`/api/teams/departments/${id}`, { method: 'DELETE' });
  await ensureOk(res, 'Falha ao excluir departamento');
  return true;
}

export async function fetchMembers(
  includeInactive = false,
  signal?: AbortSignal
): Promise<TeamMember[]> {
  const res = await fetch(`/api/teams/members?include_inactive=${includeInactive}`, { signal });
  await ensureOk(res, 'Falha ao carregar membros');
  return res.json();
}

export async function saveMember(
  id: string | null,
  data: UpsertTeamMember
): Promise<TeamMember> {
  const url = id ? `/api/teams/members/${id}` : '/api/teams/members';
  const res = await fetch(url, {
    method: id ? 'PUT' : 'POST',
    headers: { 'Content-Type': 'application/json' },
    body: JSON.stringify(data),
  });
  await ensureOk(res, 'Falha ao salvar membro');
  return res.json();
}

export async function deleteMember(id: string): Promise<boolean> {
  const res = await fetch(`/api/teams/members/${id}`, { method: 'DELETE' });
  await ensureOk(res, 'Falha ao excluir membro');
  return true;
}

export async function fetchRosters(signal?: AbortSignal): Promise<ServiceRosterSummary[]> {
  const res = await fetch('/api/teams/rosters', { signal });
  await ensureOk(res, 'Falha ao carregar escalas');
  return res.json();
}

export async function fetchActiveRoster(signal?: AbortSignal): Promise<ServiceRoster | null> {
  const res = await fetch('/api/teams/rosters/active', { signal });
  if (res.status === 404) return null;
  await ensureOk(res, 'Falha ao carregar escala ativa');
  return res.json();
}

export async function fetchRoster(id: string, signal?: AbortSignal): Promise<ServiceRoster> {
  const res = await fetch(`/api/teams/rosters/${id}`, { signal });
  await ensureOk(res, 'Falha ao carregar escala');
  return res.json();
}

export async function saveRoster(
  id: string | null,
  data: UpsertServiceRoster
): Promise<ServiceRoster> {
  const url = id ? `/api/teams/rosters/${id}` : '/api/teams/rosters';
  const res = await fetch(url, {
    method: id ? 'PUT' : 'POST',
    headers: { 'Content-Type': 'application/json' },
    body: JSON.stringify(data),
  });
  await ensureOk(res, 'Falha ao salvar escala');
  return res.json();
}

export async function activateRoster(id: string): Promise<boolean> {
  const res = await fetch(`/api/teams/rosters/${id}/active`, { method: 'POST' });
  await ensureOk(res, 'Falha ao ativar escala');
  return true;
}

export async function deleteRoster(id: string): Promise<boolean> {
  const res = await fetch(`/api/teams/rosters/${id}`, { method: 'DELETE' });
  await ensureOk(res, 'Falha ao excluir escala');
  return true;
}

export async function addRosterItem(
  rosterId: string,
  item: UpsertServiceRosterItem
): Promise<ServiceRosterItem> {
  const res = await fetch(`/api/teams/rosters/${rosterId}/items`, {
    method: 'POST',
    headers: { 'Content-Type': 'application/json' },
    body: JSON.stringify(item),
  });
  await ensureOk(res, 'Falha ao alocar voluntário');
  return res.json();
}

export async function removeRosterItem(rosterId: string, itemId: string): Promise<boolean> {
  const res = await fetch(`/api/teams/rosters/${rosterId}/items/${itemId}`, {
    method: 'DELETE',
  });
  await ensureOk(res, 'Falha ao remover voluntário da escala');
  return true;
}

export async function updateRosterItemStatus(
  rosterId: string,
  itemId: string,
  status: RosterStatus
): Promise<boolean> {
  const res = await fetch(`/api/teams/rosters/${rosterId}/items/${itemId}`, {
    method: 'PATCH',
    headers: { 'Content-Type': 'application/json' },
    body: JSON.stringify({ status }),
  });
  await ensureOk(res, 'Falha ao atualizar presença');
  return true;
}

// ==========================================
// UTILITÁRIOS DE WHATSAPP E FORMATAÇÃO
// ==========================================

export function cleanPhoneForWhatsApp(phone?: string | null): string {
  if (!phone) return '';
  const digits = phone.replace(/\D/g, '');
  if (!digits) return '';
  if (digits.length === 10 || digits.length === 11) {
    return `55${digits}`;
  }
  return digits;
}

export function getWhatsAppLink(phone?: string | null, message?: string): string {
  const clean = cleanPhoneForWhatsApp(phone);
  if (!clean) return '';
  const text = message ? `?text=${encodeURIComponent(message)}` : '';
  return `https://wa.me/${clean}${text}`;
}

export const getWhatsAppUrl = getWhatsAppLink;

export function getDepartmentEmoji(deptName: string): string {
  const lower = deptName.toLowerCase();
  if (lower.includes('proje') || lower.includes('tel')) return '🖥️';
  if (lower.includes('foto')) return '📸';
  if (lower.includes('stori') || lower.includes('rede') || lower.includes('insta')) return '📱';
  if (
    lower.includes('transm') ||
    lower.includes('live') ||
    lower.includes('obs') ||
    lower.includes('vídeo') ||
    lower.includes('video')
  )
    return '🎥';
  if (lower.includes('som') || lower.includes('áudio') || lower.includes('audio')) return '🔊';
  if (lower.includes('ilu') || lower.includes('luz')) return '💡';
  return '👥';
}

export function isRosterLeadRole(role?: string | null): boolean {
  return role === 'lead' || role === 'titular';
}

export function formatRosterRoleLabel(role?: string | null): string {
  if (isRosterLeadRole(role)) return 'Titular';
  if (role === 'backup') return 'Apoio';
  return 'Auxiliar';
}

export function formatMemberRoleLabel(role?: string | null): string {
  if (role === 'leader') return 'Líder da Equipe';
  if (role === 'vice_leader') return 'Vice-Líder';
  return 'Voluntário';
}

export function formatRosterWhatsApp(
  roster: ServiceRoster,
  departments: TeamDepartment[],
  members: TeamMember[]
): string {
  const memberMap = new Map<string, TeamMember>();
  for (const m of members) {
    memberMap.set(m.id, m);
  }

  const lines: string[] = [];
  lines.push(`📋 *Escala de Mídia — ${roster.title}*`);

  // Data e horário
  const parts: string[] = [];
  if (roster.service_date) {
    const [y, m, d] = roster.service_date.split('-');
    if (d && m && y) {
      parts.push(`📅 ${d}/${m}/${y}`);
    } else {
      parts.push(`📅 ${roster.service_date}`);
    }
  }
  if (roster.service_time) {
    parts.push(`às ${roster.service_time}`);
  }
  if (parts.length > 0) {
    lines.push(parts.join(' '));
  }
  lines.push('');

  // Agrupa itens por departamento
  const itemsByDept = new Map<string, ServiceRosterItem[]>();
  for (const item of roster.items) {
    const list = itemsByDept.get(item.department_id) || [];
    list.push(item);
    itemsByDept.set(item.department_id, list);
  }

  for (const dept of departments) {
    const items = itemsByDept.get(dept.id);
    if (!items || items.length === 0) continue;

    const emoji = getDepartmentEmoji(dept.name);
    lines.push(`${emoji} *${dept.name}:*`);

    for (const item of items) {
      const member = memberMap.get(item.member_id);
      const name = member ? member.name : 'Voluntário';
      const roleLabel = formatRosterRoleLabel(item.role);

      let statusEmoji = '⏳ Pendente';
      if (item.status === 'confirmed') statusEmoji = '✅ Confirmado';
      else if (item.status === 'declined') statusEmoji = '❌ Não comparece';

      lines.push(`  • ${name} (${roleLabel} — ${statusEmoji})`);
    }
    lines.push('');
  }

  lines.push('Equipe, favor confirmar presença no culto! 🙏');
  return lines.join('\n');
}
