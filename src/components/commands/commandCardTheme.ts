import { tv } from '@/lib/tv';

/** Shared chrome for CommandItem + CommandSubmenu cards. */
export const commandCardTv = tv({
  slots: {
    root: [
      'border border-slate-200 rounded-md p-3 space-y-3 shadow-sm',
      'hover:shadow-md transition-shadow duration-200',
    ],
    header:
      'flex items-center justify-between pb-2.5 border-b border-slate-200 -mx-3 px-3',
    headerLeft: 'flex items-center gap-2',
    iconWrap:
      'w-7 h-7 bg-blue-100 flex items-center justify-center rounded-md',
    icon: 'w-4 h-4 text-blue-600',
    title: 'font-semibold text-sm',
    actions: 'flex items-center gap-0.5',
    fieldsGrid: 'grid gap-3 grid-cols-1',
    fieldsRow: 'flex items-start gap-3',
    iconField: 'w-16',
    nameField: 'flex-1',
    section: 'space-y-3',
    sectionInner: 'space-y-2.5',
    sectionHeader: 'flex items-center justify-between',
    sectionHeaderLeft: 'flex items-center gap-2',
    sectionLabel: 'block text-sm font-semibold',
    commandList: 'space-y-2',
    commandRow: 'flex items-start gap-2',
    commandField: 'flex-1',
    shrinkBtn: 'flex-shrink-0',
    checkboxRow: 'flex items-center gap-2',
    checkbox:
      'w-4 h-4 text-blue-600 bg-gray-100 border-gray-300 rounded focus:ring-blue-500 focus:ring-2',
    checkboxLabel: 'text-sm font-medium text-slate-700',
    hint: 'text-xs text-slate-500',
    divider: 'border-t border-slate-200 my-3 -mx-3',
    fieldStack: 'space-y-1',
    fieldLabel: 'block text-sm font-medium text-slate-700',
    inputsList: 'space-y-1.5',
    inputsHeader: 'flex items-center gap-2 py-0.5 px-1 rounded-md',
    inputsCol: 'flex-1',
    inputsColLabel:
      'text-xs font-semibold text-slate-700 uppercase tracking-wide',
    inputsSpacer: 'w-8',
    inputsRow: 'flex items-center gap-2 py-0.5',
    removeBtn: 'flex-shrink-0 w-8',
    ghostAccent:
      'text-blue-600 hover:text-blue-700 hover:bg-blue-50',
    ghostAccentStrong:
      'text-blue-600 hover:text-blue-700 hover:bg-blue-100',
    dangerGhost: 'text-red-600 hover:text-red-700 hover:bg-red-50',
    inputCompact:
      'border border-slate-300 bg-white rounded px-2 py-1 focus:border-blue-400 focus:ring-0',
  },
  variants: {
    tone: {
      command: {
        root: 'bg-white',
        title: 'text-slate-900',
        sectionLabel: 'text-slate-700',
        fieldsGrid: 'md:grid-cols-2',
      },
      group: {
        root: 'bg-blue-50/50',
        title: 'text-blue-900',
        sectionLabel: 'text-blue-700',
      },
    },
  },
  defaultVariants: {
    tone: 'command',
  },
});
