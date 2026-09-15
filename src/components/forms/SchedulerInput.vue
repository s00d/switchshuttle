<template>
  <div :class="ui.root()">
    <div :class="ui.header()">
      <label :class="ui.title()">Scheduler</label>
      <div :class="ui.actions()">
        <CustomButton
          variant="ghost"
          size="sm"
          :class="ui.modeBtn()"
          @click="toggleEditMode"
        >
          <EditIcon :class="ui.actionIcon()" />
          {{ editMode ? 'Visual Editor' : 'Text Editor' }}
        </CustomButton>
        <CustomButton
          variant="ghost"
          size="sm"
          :class="ui.enableBtn()"
          @click="toggleScheduler"
        >
          <ClockIcon :class="ui.actionIcon()" />
          {{ schedulerEnabled ? 'Enabled' : 'Disabled' }}
        </CustomButton>
      </div>
    </div>

    <div v-if="schedulerEnabled" :class="ui.panel()">
      <!-- Text Editor Mode -->
      <div v-if="editMode" :class="ui.section()">
        <Input
          v-model="cronExpression"
          label="Cron Expression"
          placeholder="* * * * * * (every second)"
          required
        />

        <div :class="ui.help()">
          <h4 :class="ui.helpTitle()">Cron Expression Format</h4>
          <div :class="ui.helpBody()">
            <p><strong>Format:</strong> second minute hour day month weekday</p>
            <p><strong>Examples:</strong></p>
            <ul :class="ui.helpList()">
              <li><code>* * * * * *</code> - Every second</li>
              <li><code>0 * * * * *</code> - Every minute</li>
              <li><code>0 0 * * * *</code> - Every hour</li>
              <li><code>0 0 0 * * *</code> - Every day at midnight</li>
              <li><code>0 0 0 * * 0</code> - Every Sunday at midnight</li>
              <li><code>0 0 9 * * 1-5</code> - Weekdays at 9 AM</li>
              <li><code>0 */15 * * * *</code> - Every 15 minutes</li>
            </ul>
          </div>
        </div>
      </div>

      <!-- Visual Editor Mode -->
      <div v-else :class="ui.section()">
        <div :class="ui.fieldsGrid()">
          <div :class="ui.field()">
            <label :class="ui.fieldLabel()">Second</label>
            <SchedulerSelect
              v-model="cronFields.second"
              :options="secondOptions"
              placeholder="0-59"
              @update:modelValue="
                value => {
                  cronFields.second = value;
                  updateCronExpression();
                }
              "
            />
          </div>

          <div :class="ui.field()">
            <label :class="ui.fieldLabel()">Minute</label>
            <SchedulerSelect
              v-model="cronFields.minute"
              :options="minuteOptions"
              placeholder="0-59"
              @update:modelValue="
                value => {
                  cronFields.minute = value;
                  updateCronExpression();
                }
              "
            />
          </div>

          <div :class="ui.field()">
            <label :class="ui.fieldLabel()">Hour</label>
            <SchedulerSelect
              v-model="cronFields.hour"
              :options="hourOptions"
              placeholder="0-23"
              @update:modelValue="
                value => {
                  cronFields.hour = value;
                  updateCronExpression();
                }
              "
            />
          </div>

          <div :class="ui.field()">
            <label :class="ui.fieldLabel()">Day</label>
            <SchedulerSelect
              v-model="cronFields.day"
              :options="dayOptions"
              placeholder="1-31"
              @update:modelValue="
                value => {
                  cronFields.day = value;
                  updateCronExpression();
                }
              "
            />
          </div>

          <div :class="ui.field()">
            <label :class="ui.fieldLabel()">Month</label>
            <SchedulerSelect
              v-model="cronFields.month"
              :options="monthOptions"
              placeholder="1-12"
              @update:modelValue="
                value => {
                  cronFields.month = value;
                  updateCronExpression();
                }
              "
            />
          </div>

          <div :class="ui.field()">
            <label :class="ui.fieldLabel()">Weekday</label>
            <SchedulerSelect
              v-model="cronFields.weekday"
              :options="weekdayOptions"
              placeholder="0-6"
              @update:modelValue="
                value => {
                  cronFields.weekday = value;
                  updateCronExpression();
                }
              "
            />
          </div>
        </div>

        <div :class="ui.presets()">
          <label :class="ui.fieldLabel()">Quick Presets</label>
          <div :class="ui.presetRow()">
            <CustomButton
              variant="ghost"
              size="sm"
              :class="ui.presetBtn()"
              @click="setPreset('0 * * * * *')"
            >
              Every Minute
            </CustomButton>
            <CustomButton
              variant="ghost"
              size="sm"
              :class="ui.presetBtn()"
              @click="setPreset('0 0 * * * *')"
            >
              Every Hour
            </CustomButton>
            <CustomButton
              variant="ghost"
              size="sm"
              :class="ui.presetBtn()"
              @click="setPreset('0 0 0 * * *')"
            >
              Daily
            </CustomButton>
            <CustomButton
              variant="ghost"
              size="sm"
              :class="ui.presetBtn()"
              @click="setPreset('0 0 0 * * 0')"
            >
              Weekly
            </CustomButton>
            <CustomButton
              variant="ghost"
              size="sm"
              :class="ui.presetBtn()"
              @click="setPreset('0 0 0 1 * *')"
            >
              Monthly
            </CustomButton>
            <CustomButton
              variant="ghost"
              size="sm"
              :class="ui.presetBtn()"
              @click="setPreset('0 0 9 * * 1-5')"
            >
              Weekdays 9AM
            </CustomButton>
          </div>
        </div>

        <div :class="ui.expression()">
          <label :class="ui.expressionLabel()">Current Expression</label>
          <code :class="ui.expressionCode()">{{ cronExpression }}</code>
        </div>
      </div>
    </div>
  </div>
</template>

<script lang="ts" setup>
import { computed, ref, watch } from 'vue';
import { tv } from '@/lib/tv';
import Input from '../ui/Input.vue';
import CustomButton from '../ui/CustomButton.vue';
import SchedulerSelect from './SchedulerSelect.vue';
import EditIcon from '../icons/EditIcon.vue';
import ClockIcon from '../icons/ClockIcon.vue';

defineOptions({ name: 'SchedulerInput' });

const props = defineProps<{
  modelValue?: string;
}>();

const emit = defineEmits<{
  (e: 'update:modelValue', value: string | undefined): void;
}>();

const cronExpression = ref<string>('* * * * * *');
const editMode = ref(false);
const schedulerEnabled = computed(
  () => cronExpression.value && cronExpression.value.trim() !== ''
);

// Cron fields for visual editor
const cronFields = ref({
  second: '*',
  minute: '*',
  hour: '*',
  day: '*',
  month: '*',
  weekday: '*',
});

// Custom input fields are now handled by SchedulerSelect component

// Options for SchedulerSelect components
const secondOptions = [
  { value: '*', label: 'Every' },
  { value: '0', label: '0' },
  { value: '15', label: '15' },
  { value: '30', label: '30' },
  { value: '45', label: '45' },
  { value: 'custom', label: 'Custom' },
];

const minuteOptions = [
  { value: '*', label: 'Every' },
  { value: '0', label: '0' },
  { value: '15', label: '15' },
  { value: '30', label: '30' },
  { value: '45', label: '45' },
  { value: 'custom', label: 'Custom' },
];

const hourOptions = [
  { value: '*', label: 'Every' },
  { value: '0', label: 'Midnight' },
  { value: '6', label: '6 AM' },
  { value: '9', label: '9 AM' },
  { value: '12', label: 'Noon' },
  { value: '18', label: '6 PM' },
  { value: 'custom', label: 'Custom' },
];

const dayOptions = [
  { value: '*', label: 'Every' },
  { value: '1', label: '1st' },
  { value: '15', label: '15th' },
  { value: 'custom', label: 'Custom' },
];

const monthOptions = [
  { value: '*', label: 'Every' },
  { value: '1', label: 'Jan' },
  { value: '2', label: 'Feb' },
  { value: '3', label: 'Mar' },
  { value: '4', label: 'Apr' },
  { value: '5', label: 'May' },
  { value: '6', label: 'Jun' },
  { value: '7', label: 'Jul' },
  { value: '8', label: 'Aug' },
  { value: '9', label: 'Sep' },
  { value: '10', label: 'Oct' },
  { value: '11', label: 'Nov' },
  { value: '12', label: 'Dec' },
  { value: 'custom', label: 'Custom' },
];

const weekdayOptions = [
  { value: '*', label: 'Every' },
  { value: '0', label: 'Sun' },
  { value: '1', label: 'Mon' },
  { value: '2', label: 'Tue' },
  { value: '3', label: 'Wed' },
  { value: '4', label: 'Thu' },
  { value: '5', label: 'Fri' },
  { value: '6', label: 'Sat' },
  { value: '1-5', label: 'Weekdays' },
  { value: '0,6', label: 'Weekends' },
  { value: 'custom', label: 'Custom' },
];

const schedulerTv = tv({
  slots: {
    root: 'space-y-3',
    header: 'flex items-center justify-between',
    title: 'block text-sm font-semibold text-slate-700',
    actions: 'flex items-center gap-2',
    modeBtn: 'text-slate-600',
    enableBtn: 'text-slate-600',
    actionIcon: 'w-4 h-4 mr-1',
    panel: 'space-y-3 p-3 bg-slate-50 rounded-md',
    section: 'space-y-3',
    help: 'bg-blue-50 p-2.5 rounded-md',
    helpTitle: 'text-sm font-semibold text-blue-900 mb-1.5',
    helpBody: 'text-xs text-blue-800 space-y-1',
    helpList: 'list-disc list-inside space-y-1 ml-2',
    fieldsGrid: 'grid grid-cols-6 gap-1.5',
    field: 'space-y-1',
    fieldLabel: 'block text-xs font-medium text-slate-700',
    presets: 'space-y-2',
    presetRow: 'flex flex-wrap gap-1.5',
    presetBtn: 'text-xs',
    expression: 'bg-white p-2.5 rounded-md border border-slate-200',
    expressionLabel: 'block text-xs font-medium text-slate-700 mb-1',
    expressionCode: 'text-sm bg-slate-100 px-2 py-1 rounded-md',
  },
  variants: {
    editMode: {
      true: { modeBtn: 'text-blue-600' },
      false: { modeBtn: 'text-slate-600' },
    },
    schedulerEnabled: {
      true: { enableBtn: 'text-green-600' },
      false: { enableBtn: 'text-slate-600' },
    },
  },
});

const ui = computed(() =>
  schedulerTv({
    editMode: editMode.value,
    schedulerEnabled: !!schedulerEnabled.value,
  }),
);

// Функция для парсинга cron выражения
const parseCronExpression = () => {
  const parts = cronExpression.value.split(' ');
  if (parts.length >= 6) {
    cronFields.value = {
      second: parts[0] || '*',
      minute: parts[1] || '*',
      hour: parts[2] || '*',
      day: parts[3] || '*',
      month: parts[4] || '*',
      weekday: parts[5] || '*',
    };
  }
};

// Инициализируем из props
if (props.modelValue) {
  cronExpression.value = props.modelValue;
  parseCronExpression();
}

const toggleScheduler = () => {
  if (schedulerEnabled.value) {
    cronExpression.value = '';
  } else {
    cronExpression.value = '* * * * * *';
    parseCronExpression();
  }
  updateScheduler();
};

const toggleEditMode = () => {
  editMode.value = !editMode.value;
  if (editMode.value) {
    // Switching to text mode
  } else {
    // Switching to visual mode
    parseCronExpression();
  }
};

const updateCronExpression = () => {
  const second = cronFields.value.second;
  const minute = cronFields.value.minute;
  const hour = cronFields.value.hour;
  const day = cronFields.value.day;
  const month = cronFields.value.month;
  const weekday = cronFields.value.weekday;

  cronExpression.value = `${second} ${minute} ${hour} ${day} ${month} ${weekday}`;
  updateScheduler();
};

const setPreset = (preset: string) => {
  cronExpression.value = preset;
  parseCronExpression();
  updateScheduler();
};

const updateScheduler = () => {
  emit(
    'update:modelValue',
    schedulerEnabled.value ? cronExpression.value : undefined
  );
};

// Watch for changes
watch(cronExpression, () => {
  if (!editMode.value) {
    parseCronExpression();
  }
  updateScheduler();
});
</script>
