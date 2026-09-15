<template>
  <Modal :is-open="isOpen" @close="$emit('close')">
    <template #header>
      <h2 :class="ui.modalTitle()">Choose Commands from Templates</h2>
    </template>

    <div :class="ui.root()">
      <div :class="ui.filters()">
        <div :class="ui.searchWrap()">
          <div :class="ui.searchInner()">
            <input
              v-model="searchQuery"
              type="text"
              placeholder="Search commands..."
              :class="ui.searchInput()"
            />
            <SearchIcon :class="ui.searchIcon()" />
          </div>
        </div>

        <div :class="ui.filterSelect()">
          <CustomSelect
            v-model="selectedCategory"
            :options="categoryOptions"
            placeholder="Select Category"
          />
        </div>

        <div :class="ui.filterSelect()">
          <CustomSelect
            v-model="selectedTag"
            :options="tagOptions"
            placeholder="Select Tag"
          />
        </div>
      </div>

      <div v-if="groupedCommands.length === 0" :class="ui.empty()">
        <div :class="ui.emptyIcon()">
          <DocumentIcon :class="ui.emptyIconInner()" />
        </div>
        <p :class="ui.emptyTitle()">
          {{
            searchQuery || selectedCategory || selectedTag
              ? 'No commands found'
              : 'No commands available'
          }}
        </p>
        <CustomButton
          v-if="searchQuery || selectedCategory || selectedTag"
          @click="clearFilters"
        >
          Clear filters
        </CustomButton>
      </div>

      <div v-else :class="ui.groups()">
        <div
          v-for="template in groupedCommands"
          :key="template.id"
          :class="ui.group()"
        >
          <div :class="ui.templateHeader()">
            <div :class="ui.templateHeaderInner()">
              <div :class="ui.templateEmoji()">
                {{ template.icon }}
              </div>
              <div :class="ui.templateMeta()">
                <h3 :class="ui.templateName()">
                  {{ template.name }}
                </h3>
                <p :class="ui.templateCategory()">{{ template.category }}</p>
                <div :class="ui.templateDescRow()">
                  <span :class="ui.templateDesc()">{{
                    template.description
                  }}</span>
                </div>
                <div :class="ui.tags()">
                  <span
                    v-for="tag in template.tags"
                    :key="tag"
                    :class="ui.tag()"
                  >
                    {{ tag }}
                  </span>
                </div>
              </div>
            </div>
          </div>

          <div :class="ui.commandGrid()">
            <div
              v-for="command in template.commands"
              :key="command.id"
              :class="ui.commandCard()"
            >
              <div :class="ui.commandCardHeader()">
                <div :class="ui.commandCardHeaderInner()">
                  <div :class="ui.commandTitleRow()">
                    <div :class="ui.commandTitleLeft()">
                      <span
                        v-if="command.icon"
                        :class="ui.commandEmoji()"
                        >{{ command.icon }}</span
                      >
                      <h4 :class="ui.commandName()">
                        {{ command.name }}
                      </h4>
                    </div>
                    <span
                      v-if="command.hotkey"
                      :class="ui.hotkey()"
                    >
                      {{ command.hotkey }}
                    </span>
                  </div>
                  <div :class="ui.commandMeta()">
                    <span :class="ui.commandMetaText()">{{
                      template.name
                    }}</span>
                    <span :class="ui.metaDot()"></span>
                    <span :class="ui.metaCheck()">
                      <CheckIcon :class="ui.metaCheckIcon()" />
                      <span :class="ui.metaCheckText()">{{
                        template.category
                      }}</span>
                    </span>
                  </div>
                  <div v-if="command.description" :class="ui.commandDescWrap()">
                    <div :class="ui.commandDesc()">
                      {{ command.description }}
                    </div>
                  </div>
                </div>
              </div>

              <div :class="ui.commandBody()">
                <div v-if="command.command" :class="ui.codeBlock()">
                  <div :class="ui.codeLabel()">Command:</div>
                  <div :class="ui.codeText()">
                    {{ command.command }}
                  </div>
                </div>

                <div v-if="command.commands" :class="ui.multiCmds()">
                  <div :class="ui.codeLabel()">Multiple Commands:</div>
                  <div
                    v-for="(cmd, index) in command.commands.slice(0, 2)"
                    :key="index"
                    :class="ui.multiCmdLine()"
                  >
                    {{ cmd }}
                  </div>
                  <div
                    v-if="command.commands.length > 2"
                    :class="ui.moreCmds()"
                  >
                    +{{ command.commands.length - 2 }} more commands
                  </div>
                </div>

                <div v-if="command.switch" :class="ui.switchBlock()">
                  <div :class="ui.switchLabel()">
                    <CheckIcon :class="ui.inlineIcon()" />
                    Switch Command:
                  </div>
                  <div :class="ui.switchText()">
                    {{ command.switch }}
                  </div>
                </div>

                <div v-if="command.monitor" :class="ui.monitorBlock()">
                  <div :class="ui.monitorLabel()">
                    <ChartIcon :class="ui.inlineIcon()" />
                    Monitor Command:
                  </div>
                  <div :class="ui.monitorText()">
                    {{ command.monitor }}
                  </div>
                </div>

                <div v-if="command.inputs" :class="ui.inputsBlock()">
                  <div :class="ui.codeLabel()">Inputs</div>
                  <div :class="ui.inputsList()">
                    <div
                      v-for="(default_value, key) in command.inputs"
                      :key="key"
                      :class="ui.inputChip()"
                    >
                      <span :class="ui.inputKey()">{{ key }}</span> -
                      {{ default_value }}
                    </div>
                  </div>
                </div>
              </div>

              <div :class="ui.commandFooter()">
                <CustomButton
                  variant="primary"
                  size="sm"
                  :class="ui.selectBtn()"
                  @click="selectCommand(command)"
                >
                  Select Command
                </CustomButton>
              </div>
            </div>
          </div>
        </div>
      </div>
    </div>

    <template #footer>
      <div :class="ui.modalFooter()">
        <CustomButton variant="ghost" size="sm" @click="$emit('close')">
          Cancel
        </CustomButton>
      </div>
    </template>
  </Modal>
</template>

<script setup lang="ts">
import { ref, computed } from 'vue';
import { tv } from '@/lib/tv';
import CustomButton from '../ui/CustomButton.vue';
import Modal from '../ui/Modal.vue';
import CustomSelect from '../forms/CustomSelect.vue';
import SearchIcon from '../icons/SearchIcon.vue';
import DocumentIcon from '../icons/DocumentIcon.vue';
import CheckIcon from '../icons/CheckIcon.vue';
import ChartIcon from '../icons/ChartIcon.vue';
import {
  templates,
  searchTemplates,
  getTemplatesByCategory,
} from '../../lib/templates';
import type { Command } from '../../types';

defineOptions({ name: 'TemplateCommandsModal' });

interface Props {
  isOpen: boolean;
}

defineProps<Props>();

const searchQuery = ref('');
const selectedCategory = ref('');
const selectedTag = ref('');

const templateCommandsModalTv = tv({
  slots: {
    modalTitle: 'text-base font-semibold text-slate-900',
    root: 'space-y-3',
    filters:
      'flex flex-col md:flex-row items-stretch md:items-center gap-2.5',
    searchWrap: 'flex-1',
    searchInner: 'relative',
    searchInput: [
      'w-full h-8 px-2.5 py-1.5 pl-9 border border-slate-300 rounded-md text-sm',
      'focus:outline-none focus:ring-2 focus:ring-blue-500 focus:border-blue-500',
    ],
    searchIcon:
      'w-4 h-4 text-slate-400 absolute left-2.5 top-1/2 -translate-y-1/2',
    filterSelect: 'w-full md:w-44 flex-shrink-0',
    empty: 'text-center py-8',
    emptyIcon:
      'w-12 h-12 bg-slate-100 flex items-center justify-center mx-auto mb-3 rounded-md',
    emptyIconInner: 'w-6 h-6 text-slate-400',
    emptyTitle: 'text-sm text-slate-500 mb-1',
    groups: 'space-y-3',
    group: 'space-y-2.5',
    templateHeader:
      'bg-gradient-to-r from-slate-50 to-blue-50 border border-slate-200 rounded-md p-3',
    templateHeaderInner: 'flex items-center gap-3',
    templateEmoji: 'text-2xl bg-white p-2 rounded-md shadow-sm',
    templateMeta: 'flex-1 min-w-0',
    templateName: 'font-semibold text-slate-900 text-sm',
    templateCategory: 'text-xs text-slate-600',
    templateDescRow: 'flex items-center gap-2 mt-0.5',
    templateDesc: 'text-xs text-slate-500',
    tags: 'flex flex-wrap gap-1 mt-1.5',
    tag: 'px-1.5 py-0.5 bg-blue-100 text-blue-700 text-xs rounded-md font-medium',
    commandGrid: 'grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-2 ml-2',
    commandCard: [
      'bg-white border border-slate-200 rounded-md p-3',
      'hover:border-blue-300 hover:shadow-sm transition-all duration-200',
      'group relative flex flex-col',
    ],
    commandCardHeader: 'flex items-start justify-between mb-2',
    commandCardHeaderInner: 'flex-1 min-w-0',
    commandTitleRow: 'flex items-center justify-between mb-1 gap-1.5',
    commandTitleLeft: 'flex items-center gap-1.5 flex-1 min-w-0',
    commandEmoji: 'text-base flex-shrink-0',
    commandName: 'font-semibold text-slate-900 text-sm truncate',
    hotkey: [
      'px-1.5 py-0.5 bg-gradient-to-r from-blue-500 to-blue-600 text-white',
      'text-xs rounded-md font-medium flex-shrink-0 shadow-sm',
    ],
    commandMeta: 'flex items-center gap-1 text-xs text-slate-500 mb-1',
    commandMetaText: 'text-slate-500 truncate',
    metaDot: 'w-1 h-1 bg-slate-300 rounded-full flex-shrink-0',
    metaCheck: 'flex items-center gap-1',
    metaCheckIcon: 'w-3 h-3',
    metaCheckText: 'truncate',
    commandDescWrap: 'mb-1.5',
    commandDesc: 'text-xs text-slate-600 line-clamp-2 leading-relaxed',
    commandBody: 'space-y-1.5 flex-1',
    codeBlock: 'bg-slate-100 p-1.5 rounded-md text-xs',
    codeLabel: 'text-xs text-slate-500 mb-0.5 font-medium',
    codeText: 'text-xs font-mono text-slate-800 break-all line-clamp-2',
    multiCmds: 'space-y-1',
    multiCmdLine:
      'bg-slate-100 p-1.5 rounded-md text-xs font-mono text-slate-800 line-clamp-1',
    moreCmds: 'text-xs text-slate-500',
    switchBlock: 'bg-green-50 border border-green-200 p-1.5 rounded-md text-xs',
    switchLabel:
      'text-xs text-green-700 mb-0.5 font-medium flex items-center',
    switchText: 'text-xs font-mono text-green-800 break-all line-clamp-2',
    monitorBlock: 'bg-blue-50 border border-blue-200 p-1.5 rounded-md text-xs',
    monitorLabel:
      'text-xs text-blue-700 mb-0.5 font-medium flex items-center',
    monitorText: 'text-xs font-mono text-blue-800 break-all line-clamp-2',
    inlineIcon: 'w-3 h-3 mr-1',
    inputsBlock: 'space-y-1',
    inputsList: 'space-y-1',
    inputChip:
      'bg-blue-50 border border-blue-200 text-blue-800 px-1.5 py-0.5 rounded-md text-xs line-clamp-1',
    inputKey: 'font-medium',
    commandFooter: 'mt-2 pt-1.5 border-t border-slate-200',
    selectBtn: 'w-full text-xs',
    modalFooter: 'flex items-center justify-end gap-1.5',
  },
});

const ui = templateCommandsModalTv();

const filteredTemplates = computed(() => {
  let filtered = templates;

  if (selectedCategory.value && selectedCategory.value !== '') {
    filtered = getTemplatesByCategory(selectedCategory.value);
  }

  if (selectedTag.value && selectedTag.value !== '') {
    filtered = filtered.filter(template =>
      template.tags.includes(selectedTag.value)
    );
  }

  if (searchQuery.value) {
    filtered = searchTemplates(searchQuery.value);
  }

  return filtered;
});

const categoryOptions = computed(() => {
  const uniqueCategories = [
    ...new Set(templates.map(template => template.category)),
  ];

  return [
    { value: '', label: 'All Categories', icon: '📂' },
    ...uniqueCategories.map(category => ({
      value: category.toLowerCase().replace(' ', '-'),
      label: category,
      icon: templates.find(t => t.category === category)?.icon || '📁',
    })),
  ];
});

const tagOptions = computed(() => {
  const allTags = templates.flatMap(template => template.tags);
  const uniqueTags = [...new Set(allTags)];

  return [
    { value: '', label: 'All Tags', icon: '🏷️' },
    ...uniqueTags.map(tag => ({
      value: tag,
      label: tag,
      icon: '🏷️',
    })),
  ];
});

const groupedCommands = computed(() => {
  return filteredTemplates.value
    .map(template => ({
      ...template,
      commands: template.commands.filter((command: any) => {
        if (!searchQuery.value) return true;

        const query = searchQuery.value.toLowerCase();
        return (
          command.name.toLowerCase().includes(query) ||
          (command.command && command.command.toLowerCase().includes(query)) ||
          (command.commands &&
            command.commands.some((cmd: any) =>
              cmd.toLowerCase().includes(query)
            )) ||
          (command.switch && command.switch.toLowerCase().includes(query)) ||
          (command.monitor && command.monitor.toLowerCase().includes(query))
        );
      }),
    }))
    .filter(template => template.commands.length > 0);
});

function selectCommand(command: Command) {
  emit('commandsSelected', [command]);
}

function clearFilters() {
  searchQuery.value = '';
  selectedCategory.value = '';
  selectedTag.value = '';
}

const emit = defineEmits<{
  close: [];
  commandsSelected: [commands: Command[]];
}>();
</script>
