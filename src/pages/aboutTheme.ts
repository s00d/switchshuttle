import { tv } from '@/lib/tv';
import { cardTv } from '@/components/ui/themes';

export const aboutTv = tv({
  slots: {
    root: 'h-full overflow-hidden bg-slate-50 text-sm flex flex-col',
    main: 'flex-1 flex items-center justify-center p-4',
    card: [
      cardTv({ hover: false, padding: 'lg' }),
      'w-full max-w-md space-y-4',
    ].join(' '),
    brand: 'flex items-center gap-3',
    logo: 'w-12 h-12 rounded-xl shadow-sm bg-white',
    brandText: 'min-w-0',
    title: 'text-base font-semibold text-slate-900 leading-tight',
    meta: 'text-xs text-slate-500 mt-0.5',
    pitch: 'text-xs text-slate-600 leading-relaxed',
    chips: 'flex flex-wrap gap-1.5',
    chip: [
      'inline-flex items-center gap-1.5 h-7 px-2 rounded-md',
      'border border-slate-200 bg-white text-[11px] font-medium text-slate-700',
    ].join(' '),
    chipDot: 'w-1.5 h-1.5 rounded-full',
    stack: 'flex flex-wrap items-center gap-x-2 gap-y-1 text-[11px] text-slate-500',
    stackLabel: 'font-medium text-slate-700',
    stackSep: 'text-slate-300',
    actions: 'flex flex-wrap gap-1.5 pt-1',
    actionIcon: 'w-3.5 h-3.5',
  },
  variants: {
    tone: {
      blue: { chipDot: 'bg-blue-500' },
      green: { chipDot: 'bg-emerald-500' },
      amber: { chipDot: 'bg-amber-500' },
      slate: { chipDot: 'bg-slate-500' },
    },
  },
});
