import { tv } from '../../lib/tv';

export const buttonTv = tv({
  base: [
    'inline-flex items-center justify-center gap-1.5 font-medium transition-all duration-200',
    'border cursor-pointer rounded-md',
    'disabled:opacity-50 disabled:cursor-not-allowed',
    'enabled:hover:shadow-sm enabled:active:scale-[0.98]',
  ],
  variants: {
    variant: {
      primary:
        'bg-blue-600 border-blue-600 text-white hover:bg-blue-700 hover:border-blue-700 focus:outline-none focus:ring-2 focus:ring-blue-500 focus:ring-offset-1',
      secondary:
        'bg-white border-slate-300 text-slate-700 hover:bg-slate-50 hover:border-slate-400 focus:outline-none focus:ring-2 focus:ring-slate-400 focus:ring-offset-1',
      danger:
        'bg-red-600 border-red-600 text-white hover:bg-red-700 hover:border-red-700 focus:outline-none focus:ring-2 focus:ring-red-500 focus:ring-offset-1',
      success:
        'bg-green-600 border-green-600 text-white hover:bg-green-700 hover:border-green-700 focus:outline-none focus:ring-2 focus:ring-green-500 focus:ring-offset-1',
      ghost:
        'bg-transparent border-transparent text-slate-600 hover:bg-slate-100 hover:border-slate-200 focus:outline-none focus:ring-2 focus:ring-slate-400 focus:ring-offset-1',
    },
    size: {
      sm: 'px-2.5 py-1 text-xs h-7',
      md: 'px-3 py-1.5 text-sm h-8',
      lg: 'px-4 py-2 text-sm h-9',
    },
  },
  defaultVariants: {
    variant: 'primary',
    size: 'md',
  },
});

export const cardTv = tv({
  base: 'bg-white border border-slate-200 rounded-md transition-all duration-200',
  variants: {
    hover: {
      true: 'hover:border-slate-300 hover:shadow-sm',
      false: '',
    },
    padding: {
      none: 'p-0',
      sm: 'p-2.5',
      md: 'p-3',
      lg: 'p-4',
    },
  },
  defaultVariants: {
    hover: true,
    padding: 'md',
  },
});

export const formControlBase = tv({
  slots: {
    root: 'space-y-1',
    label: 'block text-sm font-medium text-slate-700',
    controlWrap: 'relative',
    control: [
      'w-full border border-slate-300 text-sm rounded-md transition-all duration-200 bg-white',
      'focus:outline-none focus:ring-2 focus:ring-blue-500 focus:border-blue-500',
      'disabled:bg-slate-100 disabled:text-slate-500 disabled:cursor-not-allowed',
    ],
    error: 'text-xs text-red-600',
    hint: 'text-xs text-slate-500',
  },
  variants: {
    size: {
      sm: { control: 'px-2.5 py-1 h-7 text-xs' },
      md: { control: 'px-2.5 py-1.5 h-8' },
      lg: { control: 'px-3 py-2 h-9' },
    },
    isError: {
      true: {
        control:
          'border-red-500 focus:ring-red-500 focus:border-red-500',
      },
      false: {},
    },
  },
  defaultVariants: {
    size: 'md',
    isError: false,
  },
});

export const inputTv = tv({
  extend: formControlBase,
});

export const toggleTv = tv({
  slots: {
    root: 'flex items-start justify-between gap-3',
    labelCol: 'flex flex-col flex-1 min-w-0',
    label: 'text-sm font-medium text-slate-700 break-words',
    description: 'text-xs text-slate-500 mt-0.5 break-words',
    track: [
      'relative inline-flex h-5 w-9 items-center rounded-full transition-colors flex-shrink-0',
      'focus:outline-none focus:ring-2 focus:ring-blue-500 focus:ring-offset-1',
      'disabled:opacity-50 disabled:cursor-not-allowed',
    ],
    thumb:
      'inline-block h-3.5 w-3.5 transform rounded-full bg-white transition-transform shadow-sm',
  },
  variants: {
    isOn: {
      true: {
        track: 'bg-blue-600',
        thumb: 'translate-x-[18px]',
      },
      false: {
        track: 'bg-slate-200',
        thumb: 'translate-x-1',
      },
    },
  },
  defaultVariants: {
    isOn: false,
  },
});

export const modalTv = tv({
  slots: {
    overlay:
      'fixed inset-0 z-50 flex items-center justify-center p-3',
    backdrop: 'absolute inset-0 bg-black/50 backdrop-blur-sm',
    panel: [
      'relative bg-white border border-slate-200 shadow-xl w-full rounded-md',
      'max-h-[calc(95vh-1rem)] overflow-hidden flex flex-col',
    ],
    header:
      'flex items-center justify-between px-3 py-2.5 border-b border-slate-200 flex-shrink-0',
    title: 'text-base font-semibold text-slate-900',
    body: 'flex-1 overflow-y-auto p-3',
    footer:
      'flex items-center justify-end gap-2 px-3 py-2.5 border-t border-slate-200 flex-shrink-0',
  },
  variants: {
    size: {
      sm: { panel: 'max-w-md' },
      md: { panel: 'max-w-2xl' },
      lg: { panel: 'max-w-4xl' },
      xl: { panel: 'max-w-6xl' },
    },
  },
  defaultVariants: {
    size: 'xl',
  },
});

export const collapsibleTv = tv({
  slots: {
    root: 'space-y-2',
    trigger: 'flex items-center justify-between w-full p-2.5 text-left',
    triggerInner: 'flex items-center gap-2',
    title: 'font-medium text-slate-700 text-sm',
    summary: 'text-xs text-slate-500',
    panel:
      'space-y-3 p-3 bg-slate-50 rounded-md border border-slate-200',
  },
});

export const validatedFieldTv = tv({
  slots: {
    root: 'validated-field',
    error: 'text-red-500 text-xs mt-1',
  },
});

export const pageShellTv = tv({
  slots: {
    root: 'min-h-full bg-slate-50 text-sm',
    main: 'mx-auto px-4 py-4',
    content: 'mx-auto space-y-4',
  },
  variants: {
    width: {
      sm: { main: 'max-w-3xl', content: 'max-w-3xl' },
      md: { main: 'max-w-4xl', content: 'max-w-4xl' },
      lg: { main: 'max-w-6xl', content: 'max-w-6xl' },
      full: { main: 'max-w-none', content: 'max-w-none' },
    },
  },
  defaultVariants: {
    width: 'md',
  },
});

export const pageHeaderTv = tv({
  slots: {
    root: 'flex items-center justify-between gap-3 sticky top-0 z-10 -mx-4 px-4 py-2.5 bg-slate-50/95 backdrop-blur border-b border-slate-200 mb-1',
    titleBlock: 'min-w-0',
    title: 'text-lg font-semibold text-slate-900 truncate',
    subtitle: 'text-xs text-slate-500 mt-0.5',
    actions: 'flex items-center gap-1.5 flex-shrink-0',
  },
});

export const dropdownTv = tv({
  slots: {
    trigger: [
      'w-full flex items-center justify-between gap-2 h-8 px-2.5 text-sm',
      'border border-slate-300 rounded-md bg-white text-left',
      'hover:border-slate-400 focus:outline-none focus:ring-2 focus:ring-blue-500',
    ],
    menu: [
      'absolute z-40 mt-1 w-full max-h-60 overflow-auto',
      'bg-white border border-slate-200 rounded-md shadow-lg py-1',
    ],
    option:
      'px-2.5 py-1.5 text-sm cursor-pointer hover:bg-slate-50 text-slate-700',
    optionActive: 'bg-blue-50 text-blue-700',
  },
});

export const emptyStateTv = tv({
  slots: {
    root: 'text-center py-8',
    iconWrap:
      'w-12 h-12 bg-slate-100 flex items-center justify-center mx-auto mb-3 rounded-md',
    title: 'text-sm text-slate-500 mb-1',
    actions: 'mt-3',
  },
});

export const calloutTv = tv({
  slots: {
    root: 'rounded-md border-l-4 px-3 py-2.5 text-sm',
    title: 'font-semibold mb-1',
    body: 'text-slate-700',
  },
  variants: {
    tone: {
      info: {
        root: 'bg-blue-50 border-blue-500',
        title: 'text-blue-800',
      },
      warning: {
        root: 'bg-yellow-50 border-yellow-500',
        title: 'text-yellow-800',
      },
      success: {
        root: 'bg-green-50 border-green-500',
        title: 'text-green-800',
      },
      danger: {
        root: 'bg-red-50 border-red-500',
        title: 'text-red-800',
      },
    },
  },
  defaultVariants: {
    tone: 'info',
  },
});
