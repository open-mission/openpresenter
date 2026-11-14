import type { ScheduleEvent } from '$lib/types';
import type { SchedulePresetItem } from './types';

export function getDefaultEvents(): ScheduleEvent[] {
  return [
    {
      id: `ev-${Date.now()}-1`,
      dayOrDate: 'Quarta-feira',
      time: '19h30',
      title: 'Culto de Oração e Doutrina',
      details: 'Templo Central',
    },
    {
      id: `ev-${Date.now()}-2`,
      dayOrDate: 'Sábado',
      time: '19h30',
      title: 'Culto de Jovens e Adolescentes',
      details: 'Anexo de Eventos',
    },
    {
      id: `ev-${Date.now()}-3`,
      dayOrDate: 'Domingo',
      time: '09h00 e 18h30',
      title: 'Culto da Família e Celebração',
      details: 'Com Escola Bíblica e Santa Ceia',
    },
  ];
}

export const PRESET_SCHEDULE_DAYS: SchedulePresetItem[] = [
  {
    dayOrDate: 'Domingo',
    time: '09h00 e 18h30',
    title: 'Culto de Celebração e Família',
  },
  {
    dayOrDate: 'Quarta-feira',
    time: '19h30',
    title: 'Culto de Ensino da Palavra',
  },
  {
    dayOrDate: 'Sábado',
    time: '19h30',
    title: 'Culto de Jovens',
  },
  {
    dayOrDate: 'Terça-feira',
    time: '19h30',
    title: 'Culto de Oração e Intercessão',
  },
  {
    dayOrDate: 'Segunda-feira',
    time: '19h30',
    title: 'Reunião de Líderes',
  },
  {
    dayOrDate: 'Quinta-feira',
    time: '19h30',
    title: 'Culto nos Lares / PG',
  },
  {
    dayOrDate: 'Sexta-feira',
    time: '20h00',
    title: 'Vigília de Oração',
  },
  {
    dayOrDate: '15 a 17 de Novembro',
    time: '19h00',
    title: 'Conferência Anual da Igreja',
    details: 'Festividade Especial',
  },
];
