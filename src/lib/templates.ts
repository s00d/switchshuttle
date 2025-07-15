import type { Template } from '../types';

// Импортируем все JSON файлы из папки templates
import development from './templates/development.json';
import devops from './templates/devops.json';
import frontend from './templates/frontend.json';
import backend from './templates/backend.json';
import database from './templates/database.json';
import cloud from './templates/cloud.json';
import security from './templates/security.json';
import testing from './templates/testing.json';
import utility from './templates/utility.json';
import switchesMacos from './templates/switches-macos.json';
import switchesWindows from './templates/switches-windows.json';
import switchesLinux from './templates/switches-linux.json';
import monitoring from './templates/monitoring.json';
import scheduler from './templates/scheduler.json';

// Функция для преобразования JSON данных в Template
function createTemplate(jsonData: any, id: string): Template {
    // Автоматически создаем Template из JSON данных
    const template: Template = {
        id,
        ...jsonData // копируем все поля из JSON
    };
    
    // Устанавливаем дефолтные значения только если их нет
    if (!template.icon) template.icon = '📋';
    if (!template.category) template.category = jsonData.name.toLowerCase().replace(/\s+/g, '-');
    if (!template.tags) template.tags = [jsonData.name.toLowerCase()];
    
    return template;
}

export const templates: Template[] = [
    createTemplate(development, 'development'),
    createTemplate(devops, 'devops'),
    createTemplate(frontend, 'frontend'),
    createTemplate(backend, 'backend'),
    createTemplate(database, 'database'),
    createTemplate(cloud, 'cloud'),
    createTemplate(security, 'security'),
    createTemplate(testing, 'testing'),
    createTemplate(utility, 'utility'),
    createTemplate(switchesMacos, 'switches-macos'),
    createTemplate(switchesWindows, 'switches-windows'),
    createTemplate(switchesLinux, 'switches-linux'),
    createTemplate(monitoring, 'monitoring'),
    createTemplate(scheduler, 'scheduler')
];

export function getTemplatesByCategory(category?: string): Template[] {
  if (!category) return templates;
  return templates.filter(template => template.category.toLowerCase().replace(' ', '-') === category);
}

export function searchTemplates(query: string): Template[] {
  const lowercaseQuery = query.toLowerCase();
  return templates.filter(template => {
    // Поиск по названию шаблона, описанию и тегам
    const templateMatch = template.name.toLowerCase().includes(lowercaseQuery) ||
                         template.description.toLowerCase().includes(lowercaseQuery) ||
                         template.tags.some(tag => tag.toLowerCase().includes(lowercaseQuery));
    
    // Поиск по названиям команд
    const commandMatch = template.commands.some(command => 
      command.name.toLowerCase().includes(lowercaseQuery)
    );
    
    return templateMatch || commandMatch;
  });
}

export function getTemplateById(id: string): Template | undefined {
  return templates.find(template => template.id === id);
} 