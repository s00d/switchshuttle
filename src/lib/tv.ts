import { type TV, tv as tvBase } from 'tailwind-variants';

export const tv: TV = (options, config) =>
  tvBase(options, {
    ...config,
    twMerge: config?.twMerge ?? true,
    twMergeConfig: {
      ...config?.twMergeConfig,
      theme: { ...config?.twMergeConfig?.theme },
      classGroups: { ...config?.twMergeConfig?.classGroups },
    },
  });
