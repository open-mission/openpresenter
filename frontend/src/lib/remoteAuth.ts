import type { RemoteAuthSession, TeamMember } from '$lib/types';
import { writable } from 'svelte/store';

const STORAGE_KEY = 'openpresenter:remote-auth-session';
const OPERATOR_UNLOCKED_KEY = 'openpresenter:remote-operator-unlocked';

export function getStoredRemoteSession(): RemoteAuthSession | null {
  if (typeof window === 'undefined') return null;
  try {
    const raw = localStorage.getItem(STORAGE_KEY);
    if (!raw) return null;
    return JSON.parse(raw) as RemoteAuthSession;
  } catch {
    return null;
  }
}

export function getStoredOperatorUnlocked(): boolean {
  if (typeof window === 'undefined') return false;
  return localStorage.getItem(OPERATOR_UNLOCKED_KEY) === 'true';
}

export const remoteAuthStore = writable<RemoteAuthSession | null>(getStoredRemoteSession());
export const remoteOperatorUnlockedStore = writable<boolean>(getStoredOperatorUnlocked());

export function saveRemoteSession(member: TeamMember): RemoteAuthSession {
  const session: RemoteAuthSession = {
    member_id: member.id,
    name: member.name,
    role: member.role,
    can_control_remote: member.can_control_remote ?? false,
    color: member.color,
    department_ids: member.department_ids || member.departments || [],
    logged_at: Date.now(),
  };
  if (typeof window !== 'undefined') {
    localStorage.setItem(STORAGE_KEY, JSON.stringify(session));
  }
  remoteAuthStore.set(session);
  return session;
}

export function clearRemoteSession(): void {
  if (typeof window !== 'undefined') {
    localStorage.removeItem(STORAGE_KEY);
    localStorage.removeItem(OPERATOR_UNLOCKED_KEY);
  }
  remoteAuthStore.set(null);
  remoteOperatorUnlockedStore.set(false);
}

export function setOperatorUnlocked(unlocked: boolean): void {
  if (typeof window !== 'undefined') {
    if (unlocked) {
      localStorage.setItem(OPERATOR_UNLOCKED_KEY, 'true');
    } else {
      localStorage.removeItem(OPERATOR_UNLOCKED_KEY);
    }
  }
  remoteOperatorUnlockedStore.set(unlocked);
}
