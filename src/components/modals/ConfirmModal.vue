<template>
  <Modal :is-open="isOpen" size="sm" @close="$emit('close')">
    <template #header>
      <h2 :class="ui.title()">{{ title }}</h2>
    </template>

    <div :class="ui.body()">
      <div :class="ui.messageRow()">
        <div :class="ui.iconWrap()">
          <WarningIcon :class="ui.icon()" />
        </div>
        <div>
          <p :class="ui.message()">{{ message }}</p>
          <p :class="ui.description()">{{ description }}</p>
        </div>
      </div>

      <div v-if="details" :class="ui.details()">
        <div
          v-for="(value, key) in details"
          :key="key"
          :class="ui.detailRow()"
        >
          <p :class="ui.detailText()">
            <span :class="ui.detailKey()">{{ formatKey(key) }}:</span>
            {{ value }}
          </p>
        </div>
      </div>
    </div>

    <template #footer>
      <div :class="ui.footer()">
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
          <SpinnerIcon v-if="loading" :class="ui.btnIconSpin()" />
          <component :is="confirmIcon" v-else :class="ui.btnIcon()" />
          {{ loading ? loadingText : confirmText }}
        </CustomButton>
      </div>
    </template>
  </Modal>
</template>

<script setup lang="ts">
import { tv } from '@/lib/tv';
import Modal from '../ui/Modal.vue';
import CustomButton from '../ui/CustomButton.vue';
import WarningIcon from '../icons/WarningIcon.vue';
import SpinnerIcon from '../icons/SpinnerIcon.vue';
import TrashIcon from '../icons/TrashIcon.vue';

defineOptions({ name: 'ConfirmModal' });

interface Props {
  isOpen: boolean;
  title?: string;
  message: string;
  description?: string;
  details?: Record<string, string>;
  confirmText?: string;
  cancelText?: string;
  loadingText?: string;
  loading?: boolean;
  confirmVariant?: 'danger' | 'primary' | 'secondary';
  confirmIcon?: any;
}

withDefaults(defineProps<Props>(), {
  title: 'Confirm Action',
  description: 'This action cannot be undone',
  confirmText: 'Confirm',
  cancelText: 'Cancel',
  loadingText: 'Loading...',
  loading: false,
  confirmVariant: 'danger',
  confirmIcon: TrashIcon,
});

defineEmits<{
  close: [];
  confirm: [];
}>();

const confirmModalTv = tv({
  slots: {
    title: 'text-base font-semibold text-slate-900',
    body: 'space-y-3',
    messageRow: 'flex items-center gap-2.5',
    iconWrap:
      'w-9 h-9 bg-red-100 flex items-center justify-center rounded-md flex-shrink-0',
    icon: 'w-5 h-5 text-red-600',
    message: 'text-sm text-slate-900 font-medium',
    description: 'text-xs text-slate-600 mt-0.5',
    details: 'bg-slate-50 p-2.5 rounded-md',
    detailRow: 'mb-1.5 last:mb-0',
    detailText: 'text-sm text-slate-700',
    detailKey: 'font-medium',
    footer: 'flex items-center justify-end gap-1.5',
    btnIcon: 'w-4 h-4',
    btnIconSpin: 'w-4 h-4 animate-spin',
  },
});

const ui = confirmModalTv();

function formatKey(key: string): string {
  return key.charAt(0).toUpperCase() + key.slice(1).replace(/([A-Z])/g, ' $1');
}
</script>
