<template>
  <Modal :is-open="isOpen" @close="$emit('close')">
    <template #header>
      <h2 class="text-xl font-semibold text-slate-900">{{ title }}</h2>
    </template>
    
    <div class="space-y-4">
      <div class="flex items-center space-x-3">
        <div class="w-10 h-10 bg-red-100 flex items-center justify-center rounded-full">
          <WarningIcon class="w-5 h-5 text-red-600" />
        </div>
        <div>
          <p class="text-slate-900 font-medium">{{ message }}</p>
          <p class="text-slate-600 text-sm">{{ description }}</p>
        </div>
      </div>
      
      <div v-if="details" class="bg-slate-50 p-3 rounded-lg">
        <div v-for="(value, key) in details" :key="key" class="mb-2 last:mb-0">
          <p class="text-slate-700">
            <span class="font-medium">{{ formatKey(key) }}:</span> {{ value }}
          </p>
        </div>
      </div>
    </div>
    
    <template #footer>
      <div class="flex items-center justify-end space-x-2">
        <CustomButton
          variant="ghost" 
          size="sm" 
          :disabled="loading" 
          @click="$emit('close')"
        >
          {{ cancelText }}
        </CustomButton>
        <CustomButton
          :variant="confirmVariant" 
          size="sm" 
          :disabled="loading" 
          @click="$emit('confirm')"
        >
          <SpinnerIcon v-if="loading" class="w-4 h-4 animate-spin" />
          <component v-else :is="confirmIcon" class="w-4 h-4" />
          {{ loading ? loadingText : confirmText }}
        </CustomButton>
      </div>
    </template>
  </Modal>
</template>

<script setup lang="ts">
import Modal from '../ui/Modal.vue'
import CustomButton from '../ui/CustomButton.vue'
import WarningIcon from '../icons/WarningIcon.vue'
import SpinnerIcon from '../icons/SpinnerIcon.vue'
import TrashIcon from '../icons/TrashIcon.vue'

interface Props {
  isOpen: boolean
  title?: string
  message: string
  description?: string
  details?: Record<string, string>
  confirmText?: string
  cancelText?: string
  loadingText?: string
  loading?: boolean
  confirmVariant?: 'danger' | 'primary' | 'secondary'
  confirmIcon?: any
}

withDefaults(defineProps<Props>(), {
  title: 'Confirm Action',
  description: 'This action cannot be undone',
  confirmText: 'Confirm',
  cancelText: 'Cancel',
  loadingText: 'Loading...',
  loading: false,
  confirmVariant: 'danger',
  confirmIcon: TrashIcon
})

defineEmits<{
  close: []
  confirm: []
}>()

function formatKey(key: string): string {
  return key.charAt(0).toUpperCase() + key.slice(1).replace(/([A-Z])/g, ' $1')
}
</script> 