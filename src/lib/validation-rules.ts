import type { ValidationRule } from '../components/ui/ValidatedField.vue';

export const validationRules = {
  required: (message = 'This field is required'): ValidationRule => ({
    test: (value: any) => value !== null && value !== undefined && value.toString().trim() !== '',
    message
  }),

  maxLength: (maxLength: number, message?: string): ValidationRule => ({
    test: (value: string) => !value || value.length <= maxLength,
    message: message || `Maximum length is ${maxLength} characters`
  }),

  minLength: (minLength: number, message?: string): ValidationRule => ({
    test: (value: string) => !value || value.length >= minLength,
    message: message || `Minimum length is ${minLength} characters`
  }),

  email: (message = 'Invalid email format'): ValidationRule => ({
    test: (value: string) => !value || /^[^\s@]+@[^\s@]+\.[^\s@]+$/.test(value),
    message
  }),

  url: (message = 'Invalid URL format'): ValidationRule => ({
    test: (value: string) => !value || /^https?:\/\/.+/.test(value),
    message
  }),

  hotkey: (message = 'Invalid hotkey format'): ValidationRule => ({
    test: (value: string) => !value || /^[A-Za-z0-9+\-]+$/.test(value),
    message
  }),

  launchOption: (message = 'Invalid launch option'): ValidationRule => ({
    test: (value: string) => ['current', 'new_tab', 'new_window'].includes(value),
    message
  }),

  cronExpression: (message = 'Invalid cron expression format'): ValidationRule => ({
    test: (value: string) => !value || /^(\*|[0-9,\-*/]+)\s+(\*|[0-9,\-*/]+)\s+(\*|[0-9,\-*/]+)\s+(\*|[0-9,\-*/]+)\s+(\*|[0-9,\-*/]+)$/.test(value),
    message
  }),

  notEmpty: (message = 'Field cannot be empty'): ValidationRule => ({
    test: (value: any[]) => value && value.length > 0,
    message
  }),

  custom: (test: (value: any) => boolean, message: string): ValidationRule => ({
    test,
    message
  })
};

// Готовые наборы правил для часто используемых полей
export const fieldRules = {
  title: [
    validationRules.required('Title is required'),
    validationRules.maxLength(200, 'Title is too long (maximum 100 characters)')
  ],

  terminal: [
    validationRules.required('Terminal is required')
  ],

  launchIn: [
    validationRules.required('Launch method is required'),
    validationRules.launchOption()
  ],

  hotkey: [
    validationRules.hotkey()
  ],

  commands: [
    validationRules.notEmpty('Configuration must contain at least one command')
  ],

  cron: [
    validationRules.cronExpression()
  ],

  commandName: [
    validationRules.required('Command name is required'),
    validationRules.maxLength(50, 'Command name is too long (maximum 50 characters)')
  ],

  commandInput: [
    validationRules.custom(
      (value: string) => !value || value.trim().length > 0,
      'Command cannot be empty'
    ),
    validationRules.custom(
      (value: string) => !value || value.length <= 500,
      'Command is too long (maximum 500 characters)'
    )
  ],

  inputKey: [
    validationRules.required('Input key is required'),
    validationRules.custom(
      (value: string) => !value || /^[a-zA-Z0-9_]+$/.test(value),
      'Input key can only contain letters, numbers, and underscores'
    ),
    validationRules.maxLength(30, 'Input key is too long (maximum 30 characters)')
  ],

  inputValue: [
    validationRules.maxLength(200, 'Input value is too long (maximum 200 characters)')
  ]
}; 