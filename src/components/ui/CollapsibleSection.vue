<template>
  <div :class="ui.root()">
    <CustomButton
      variant="ghost"
      :class="ui.trigger()"
      @click="toggleCollapsed"
    >
      <div :class="ui.triggerInner()">
        <ChevronRightIcon :collapsed="!collapsed" />
        <span :class="ui.title()">{{ title }}</span>
      </div>
      <div :class="ui.summary()">
        {{ summary }}
      </div>
    </CustomButton>

    <div v-if="!collapsed" :class="ui.panel()">
      <slot />
    </div>
  </div>
</template>

<script lang="ts" setup>
import { ref } from 'vue';
import ChevronRightIcon from '../icons/ChevronRightIcon.vue';
import CustomButton from './CustomButton.vue';
import { collapsibleTv } from './themes';

defineOptions({ name: 'CollapsibleSection' });

interface Props {
  title: string;
  summary: string;
  defaultCollapsed?: boolean;
}

const props = withDefaults(defineProps<Props>(), {
  defaultCollapsed: true,
});

const collapsed = ref(props.defaultCollapsed);
const ui = collapsibleTv();

const toggleCollapsed = () => {
  collapsed.value = !collapsed.value;
};
</script>
