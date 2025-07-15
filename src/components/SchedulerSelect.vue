<template>
  <div class="relative">
    <div
      v-if="!isEditing"
      @click="toggleDropdown"
      class="w-full px-2 py-1 text-xs border border-slate-300 rounded bg-white cursor-pointer hover:border-slate-400 focus:border-blue-500 focus:ring-1 focus:ring-blue-500 transition-colors"
      :class="{ 'border-blue-500 ring-1 ring-blue-500': isOpen }"
    >
      <div class="flex items-center justify-between">
        <span class="text-slate-700 truncate">{{ selectedLabel }}</span>
        <svg
          class="w-3 h-3 text-slate-400 transition-transform flex-shrink-0 ml-1"
          :class="{ 'rotate-180': isOpen }"
          fill="none"
          stroke="currentColor"
          viewBox="0 0 24 24"
        >
          <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M19 9l-7 7-7-7" />
        </svg>
      </div>
    </div>

    <input
      v-else
      v-model="customValue"
      @blur="finishEditing"
      @keyup.enter="finishEditing"
      @keyup.esc="cancelEditing"
      ref="customInput"
      class="w-full px-2 py-1 text-xs border border-blue-500 rounded bg-white focus:outline-none focus:ring-1 focus:ring-blue-500"
      :placeholder="placeholder"
    />

    <div
      v-if="isOpen && !isEditing"
      class="absolute z-50 w-full mt-1 bg-white border border-slate-200 rounded-md shadow-lg max-h-48 overflow-auto"
    >
      <div class="py-1">
        <div
          v-for="option in options"
          :key="option.value"
          @click="selectOption(option)"
          class="px-2 py-1 text-xs cursor-pointer hover:bg-slate-50 transition-colors"
          :class="{ 'bg-blue-50 text-blue-700': modelValue === option.value }"
        >
          {{ option.label }}
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, computed, onMounted, onUnmounted, nextTick } from 'vue'

interface Option {
  value: string
  label: string
}

interface Props {
  modelValue: string
  options: Option[]
  placeholder?: string
}

const props = withDefaults(defineProps<Props>(), {
  placeholder: 'Enter value'
})

const emit = defineEmits<{
  'update:modelValue': [value: string]
}>()

const isOpen = ref(false)
const isEditing = ref(false)
const customValue = ref('')
const customInput = ref<HTMLInputElement>()

const selectedLabel = computed(() => {
  const selected = props.options.find(option => option.value === props.modelValue)
  if (selected) {
    return selected.label
  }
  // Если значение не найдено в опциях, значит это кастомное значение
  // Показываем само значение, а не плейсхолдер
  return props.modelValue || ''
})

const toggleDropdown = () => {
  isOpen.value = !isOpen.value
}

const selectOption = (option: Option) => {
  if (option.value === 'custom') {
    isEditing.value = true
    customValue.value = ''
    isOpen.value = false
    nextTick(() => {
      customInput.value?.focus()
    })
  } else {
    emit('update:modelValue', option.value)
    isOpen.value = false
  }
}

const finishEditing = () => {
  if (customValue.value.trim()) {
    emit('update:modelValue', customValue.value.trim())
  } else {
    // Если значение пустое, возвращаемся к предыдущему значению или '*'
    const currentValue = props.modelValue
    const hasValidValue = props.options.some(option => option.value === currentValue)
    if (!hasValidValue && currentValue) {
      emit('update:modelValue', currentValue)
    }
  }
  isEditing.value = false
  customValue.value = ''
}

const cancelEditing = () => {
  isEditing.value = false
  customValue.value = ''
  // При отмене не меняем значение, оставляем как было
}

const closeDropdown = () => {
  isOpen.value = false
}

onMounted(() => {
  document.addEventListener('click', closeDropdown)
})

onUnmounted(() => {
  document.removeEventListener('click', closeDropdown)
})
</script> 