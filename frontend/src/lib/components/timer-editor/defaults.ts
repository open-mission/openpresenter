import type { TimerPreset } from './types';

export const TIMER_PRESETS: TimerPreset[] = [
  { label: '1 min', mins: 1, secs: 0 },
  { label: '2 min', mins: 2, secs: 0 },
  { label: '3 min', mins: 3, secs: 0 },
  { label: '5 min', mins: 5, secs: 0 },
  { label: '10 min', mins: 10, secs: 0 },
  { label: '15 min', mins: 15, secs: 0 },
  { label: '30 min', mins: 30, secs: 0 },
];

export const DEFAULT_TIMER_TITLE = 'Contagem Regressiva — Início do Culto';
export const DEFAULT_TIMER_SUBTITLE = 'O culto começará em instantes. Prepare o seu coração!';
