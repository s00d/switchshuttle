import { tv } from '@/lib/tv';
import { cardTv } from '@/components/ui/themes';

export const helpTv = tv({
  slots: {
    panel: `${cardTv({ hover: false, padding: 'md' })} space-y-4`,
    searchWrap: 'relative',
    searchInput: [
      'w-full h-8 px-2.5 pr-9 border border-slate-300 text-sm rounded-md bg-white',
      'focus:outline-none focus:ring-2 focus:ring-blue-500 focus:border-blue-500',
    ],
    searchIcon:
      'h-4 w-4 text-slate-400 absolute right-2.5 top-1/2 -translate-y-1/2 pointer-events-none',
    categories: 'space-y-4',
    category: 'space-y-2',
    categoryTitle: 'text-sm font-semibold text-slate-900',
    items: 'space-y-1.5',
    item: 'border border-slate-200 rounded-md bg-white overflow-hidden',
    itemTrigger: [
      'w-full px-3 py-2 text-left flex justify-between items-center gap-2',
      'hover:bg-slate-50 transition-colors text-sm font-medium text-slate-900',
    ],
    chevron: 'w-4 h-4 text-slate-500 transition-transform flex-shrink-0',
    chevronOpen: 'rotate-180',
    answer: 'px-3 pb-3 text-sm text-slate-700 leading-relaxed',
    footer: 'pt-3 border-t border-slate-200',
    footerText: 'text-xs text-slate-500 text-center',
    footerLink: 'text-blue-600 hover:text-blue-800',
  },
});
