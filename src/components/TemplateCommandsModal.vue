<template>
  <Modal :is-open="isOpen" @close="$emit('close')">
    <template #header>
      <h2 class="text-xl font-semibold text-slate-900">Choose Commands from Templates</h2>
    </template>
    
    <div class="space-y-6">
      <!-- Search and Filters -->
      <div class="flex flex-col md:flex-row items-center space-y-4 md:space-y-0 md:space-x-4">
        <!-- Search -->
        <div class="flex-1">
          <div class="relative">
            <input
              v-model="searchQuery"
              type="text"
              placeholder="Search commands..."
              class="w-full h-10 px-3 py-2 pl-10 border border-slate-300 rounded-md focus:outline-none focus:ring-2 focus:ring-blue-500 focus:border-blue-500"
            />
            <svg class="w-5 h-5 text-slate-400 absolute left-3 top-1/2 transform -translate-y-1/2" fill="none" stroke="currentColor" viewBox="0 0 24 24">
              <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M21 21l-6-6m2-5a7 7 0 11-14 0 7 7 0 0114 0z" />
            </svg>
          </div>
        </div>

        <!-- Category Filter -->
        <div class="w-64 flex-shrink-0" style="margin: 0">
          <CustomSelect
            v-model="selectedCategory"
            :options="categoryOptions"
            placeholder="Select Category"
          />
        </div>
      </div>

      <!-- Commands List -->
      <div v-if="groupedCommands.length === 0" class="text-center py-12">
        <div class="w-16 h-16 bg-slate-100 flex items-center justify-center mx-auto mb-4 rounded-lg">
          <svg class="w-8 h-8 text-slate-400" fill="none" stroke="currentColor" viewBox="0 0 24 24">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 12h6m-6 4h6m2 5H7a2 2 0 01-2-2V5a2 2 0 012-2h5.586a1 1 0 01.707.293l5.414 5.414a1 1 0 01.293.707V19a2 2 0 01-2 2z" />
          </svg>
        </div>
        <p class="text-slate-500 mb-2">
          {{ searchQuery || selectedCategory ? 'No commands found' : 'No commands available' }}
        </p>
        <Button @click="clearFilters" v-if="searchQuery || selectedCategory">
          Clear filters
        </Button>
      </div>

      <div v-else class="space-y-6">
        <!-- Group by Template -->
        <div v-for="template in groupedCommands" :key="template.id" class="space-y-4">
          <!-- Template Header -->
          <div class="bg-gradient-to-r from-slate-50 to-blue-50 border border-slate-200 rounded-xl p-4">
            <div class="flex items-center space-x-4">
              <div class="text-3xl bg-white p-3 rounded-lg shadow-sm">{{ template.icon }}</div>
              <div class="flex-1">
                <h3 class="font-bold text-slate-900 text-lg">{{ template.name }}</h3>
                <p class="text-sm text-slate-600">{{ template.category }}</p>
                <div class="flex items-center space-x-2 mt-1">
                  <span class="text-xs text-slate-500">{{ template.description }}</span>
                </div>
                <div class="flex flex-wrap gap-1 mt-2">
                  <span 
                    v-for="tag in template.tags" 
                    :key="tag"
                    class="px-2 py-1 bg-blue-100 text-blue-700 text-xs rounded-full font-medium"
                  >
                    {{ tag }}
                  </span>
                </div>
              </div>
            </div>
          </div>
          
          <!-- Commands in this template -->
          <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-3 ml-4">
            <div 
              v-for="command in template.commands" 
              :key="command.id"
              class="bg-white border-2 border-slate-200 rounded-lg p-4 hover:border-blue-300 hover:shadow-md transition-all duration-200 group relative flex flex-col"
            >

              
              <!-- Command Header -->
              <div class="flex items-start justify-between mb-3">
                <div class="flex-1 min-w-0">
                  <div class="flex items-center justify-between mb-1">
                    <h4 class="font-semibold text-slate-900 text-sm truncate">{{ command.name }}</h4>
                    <span v-if="command.hotkey" class="px-2 py-1 bg-gradient-to-r from-blue-500 to-blue-600 text-white text-xs rounded-md font-medium flex-shrink-0 shadow-sm">
                      {{ command.hotkey }}
                    </span>
                  </div>
                  <div class="flex items-center space-x-1 text-xs text-slate-500 mb-1">
                    <span class="text-slate-500 truncate">{{ template.name }}</span>
                    <span class="w-1 h-1 bg-slate-300 rounded-full flex-shrink-0"></span>
                    <span class="flex items-center space-x-1">
                      <svg class="w-3 h-3" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 12l2 2 4-4m6 2a9 9 0 11-18 0 9 9 0 0118 0z" />
                      </svg>
                      <span class="truncate">{{ template.category }}</span>
                    </span>
                  </div>
                </div>
              </div>
              
              <!-- Command Content -->
              <div class="space-y-2 flex-1">
                <div v-if="command.command" class="bg-slate-100 p-2 rounded text-xs">
                  <div class="text-xs text-slate-500 mb-1 font-medium">Command:</div>
                  <div class="text-xs font-mono text-slate-800 break-all line-clamp-2">{{ command.command }}</div>
                </div>
                
                <div v-if="command.commands" class="space-y-1">
                  <div class="text-xs text-slate-500 font-medium">Multiple Commands:</div>
                  <div 
                    v-for="(cmd, index) in command.commands.slice(0, 2)" 
                    :key="index"
                    class="bg-slate-100 p-2 rounded text-xs font-mono text-slate-800 line-clamp-1"
                  >
                    {{ cmd }}
                  </div>
                  <div v-if="command.commands.length > 2" class="text-xs text-slate-500">
                    +{{ command.commands.length - 2 }} more commands
                  </div>
                </div>
                
                <div v-if="command.inputs" class="space-y-1">
                  <div class="text-xs text-slate-500 font-medium">Inputs</div>
                  <div class="space-y-1">
                    <div 
                      v-for="(prompt, key) in command.inputs" 
                      :key="key"
                      class="bg-blue-50 border border-blue-200 text-blue-800 px-2 py-1 rounded text-xs line-clamp-1"
                    >
                      <span class="font-medium">{{ key }}</span> - {{ prompt.replace(':', '') }}
                    </div>
                  </div>
                </div>
              </div>
              
              <!-- Action Button -->
              <div class="mt-3 pt-2 border-t border-slate-200">
                <Button 
                  @click="selectCommand(command, template)"
                  variant="primary"
                  size="sm"
                  class="w-full text-xs py-1"
                >
                  Select Command
                </Button>
              </div>

            </div>
          </div>
        </div>
      </div>
    </div>

    <!-- Actions -->
    <template #footer>
      <div class="flex items-center justify-end space-x-3">
        <Button @click="$emit('close')" variant="ghost" size="sm">
          Cancel
        </Button>
      </div>
    </template>
  </Modal>
</template>

<script setup lang="ts">
import { ref, computed } from 'vue';
import Button from './Button.vue';
import Modal from './Modal.vue';
import CustomSelect from './CustomSelect.vue';
import { templates, searchTemplates, getTemplatesByCategory } from '../lib/templates';
import type { Template } from '../lib/templates';
import type { Command } from '../types';

interface Props {
  isOpen: boolean;
}

defineProps<Props>();

// State
const searchQuery = ref('');
const selectedCategory = ref('');

// Computed
const filteredTemplates = computed(() => {
  let filtered = templates;
  
  if (selectedCategory.value && selectedCategory.value !== '') {
    filtered = getTemplatesByCategory(selectedCategory.value);
  }
  
  if (searchQuery.value) {
    filtered = searchTemplates(searchQuery.value);
  }
  
  return filtered;
});

const categoryOptions = computed(() => {
  // Get unique categories from templates
  const uniqueCategories = [...new Set(templates.map(template => template.category))];
  
  return [
    { value: '', label: 'All Categories', icon: '📂' },
    ...uniqueCategories.map(category => ({
      value: category.toLowerCase().replace(' ', '-'),
      label: category,
      icon: templates.find(t => t.category === category)?.icon || '📁'
    }))
  ];
});

const groupedCommands = computed(() => {
  return filteredTemplates.value.map(template => ({
    ...template,
    commands: template.commands.filter((command: any) => {
      if (!searchQuery.value) return true;
      
      const query = searchQuery.value.toLowerCase();
      return command.name.toLowerCase().includes(query) ||
             (command.command && command.command.toLowerCase().includes(query)) ||
             (command.commands && command.commands.some((cmd: any) => cmd.toLowerCase().includes(query)));
    })
  })).filter(template => template.commands.length > 0);
});

// Methods
function selectCommand(command: Command, template: Template) {
  // Add command immediately and close modal
  emit('commandsSelected', [command]);
}

function clearFilters() {
  searchQuery.value = '';
  selectedCategory.value = '';
}

const emit = defineEmits<{
  close: [];
  commandsSelected: [commands: Command[]];
}>();
</script> 