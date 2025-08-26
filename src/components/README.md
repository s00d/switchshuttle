# Components Structure

This directory contains all Vue components organized by functionality:

## 📁 ui/
Basic UI components used throughout the application:
- `CustomButton.vue` - Reusable button component
- `Card.vue` - Card container component
- `CollapsibleSection.vue` - Collapsible content section
- `Input.vue` - Basic input field component
- `Modal.vue` - Modal dialog component
- `Toggle.vue` - Toggle switch component
- `ValidatedField.vue` - Input field with validation

## 📁 commands/
Components related to command management:
- `CommandInput.vue` - Command input field with validation
- `CommandItem.vue` - Individual command item display
- `CommandSubmenu.vue` - Submenu for grouped commands
- `CommandsTable.vue` - Table displaying multiple commands

## 📁 forms/
Form-related components and inputs:
- `CustomSelect.vue` - Custom select dropdown
- `HotkeyInput.vue` - Hotkey input with validation
- `IconSelector.vue` - Icon selection component
- `SchedulerInput.vue` - Scheduler/cron input component
- `SchedulerSelect.vue` - Scheduler selection component

## 📁 config/
Configuration management components:
- `ConfigEditor.vue` - Configuration editor interface
- `ConfigSelector.vue` - Configuration selection component

## 📁 modals/
Modal dialog components:
- `ConfirmModal.vue` - Confirmation dialog
- `TemplateCommandsModal.vue` - Template commands selection modal

## 📁 performance/
Performance monitoring components:
- `PerformanceMonitor.vue` - Performance monitoring interface

## 📁 icons/
Icon components used throughout the application.

## Usage

Import components using the new path structure:

```vue
<script setup>
// UI components
import CustomButton from '../components/ui/CustomButton.vue'
import Card from '../components/ui/Card.vue'

// Command components
import CommandItem from '../components/commands/CommandItem.vue'

// Form components
import CustomSelect from '../components/forms/CustomSelect.vue'

// Config components
import ConfigEditor from '../components/config/ConfigEditor.vue'

// Modal components
import ConfirmModal from '../components/modals/ConfirmModal.vue'

// Performance components
import PerformanceMonitor from '../components/performance/PerformanceMonitor.vue'
</script>
``` 