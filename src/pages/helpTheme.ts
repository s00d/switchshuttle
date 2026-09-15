import { tv } from '@/lib/tv';
import { cardTv } from '@/components/ui/themes';

export const helpTv = tv({
  slots: {
    layout: 'space-y-3',
    searchWrap: 'relative',
    searchInput: [
      'w-full h-8 px-2.5 pr-9 border border-slate-300 text-sm rounded-md bg-white',
      'focus:outline-none focus:ring-2 focus:ring-blue-500 focus:border-blue-500',
    ],
    searchIcon:
      'h-4 w-4 text-slate-400 absolute right-2.5 top-1/2 -translate-y-1/2 pointer-events-none',
    nav: 'flex flex-wrap gap-1.5',
    navBtn: [
      'h-7 px-2.5 text-xs font-medium rounded-md border transition-colors',
      'border-slate-200 bg-white text-slate-600 hover:border-slate-300 hover:bg-slate-50',
    ],
    navBtnActive: 'border-blue-500 bg-blue-50 text-blue-700',
    section: `${cardTv({ hover: false, padding: 'md' })} space-y-3`,
    sectionTitle: 'text-sm font-semibold text-slate-900',
    sectionLead: 'text-xs text-slate-500 leading-relaxed',
    steps: 'space-y-1.5 text-sm text-slate-700 list-decimal list-inside',
    grid: 'grid gap-2 sm:grid-cols-2',
    item: 'rounded-md border border-slate-200 bg-slate-50/80 px-2.5 py-2',
    itemTitle: 'text-xs font-semibold text-slate-900',
    itemBody: 'mt-0.5 text-xs text-slate-600 leading-relaxed',
    code: [
      'block w-full overflow-x-auto rounded-md bg-slate-900 text-slate-100',
      'px-2.5 py-2 text-[11px] leading-relaxed font-mono',
    ],
    inlineCode:
      'px-1 py-0.5 rounded bg-slate-200 text-[11px] font-mono text-slate-800',
    list: 'space-y-1.5 text-sm text-slate-700',
    listRow: 'flex gap-2',
    listKey: 'shrink-0 w-28 text-xs font-semibold text-slate-900 pt-0.5',
    listVal: 'text-xs text-slate-600 leading-relaxed min-w-0',
    empty: 'text-sm text-slate-500 py-6 text-center',
    footer: 'text-xs text-slate-500 text-center pt-1',
    footerLink: 'text-blue-600 hover:text-blue-800',
  },
});
