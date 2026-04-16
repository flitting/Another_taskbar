# UI Module Structure

This folder contains the refactored UI for Another Taskbar, organized into modular JavaScript and CSS files for better maintainability.

## JavaScript Modules

Located in `ui/modules/`:

- **dom.js** - DOM element references and app state
  - Element selectors for all UI components
  - Global `app` state object
  
- **api.js** - Tauri invoke and API layer
  - `safeInvoke()` - Wrapper for Tauri commands with error handling
  - Task API calls: `invokeCreateTask()`, `invokeSetTaskState()`, etc.
  - Theme API: `invokeSetTheme()`
  - Move API: `invokeMoveTask()`

- **utils.js** - General utility functions
  - Formatting: `formatDateTime()`, `formatDueShort()`, `truncateText()`
  - UI scale and font size: `applyUiScale()`, `applyTaskFontSize()`
  - Theming: `applyTheme()`, `stateLabel()`, `stateSymbol()`, `stateColor()`
  - Localization: `t()` function
  - Tips: `setTip()`, `setupHoverTips()`
  - Sort and labels: `sortModeLabel()`, `sortDisplayLabel()`

- **time.js** - Time and date utilities
  - Date conversion: `localFromIso()`, `isoFromLocal()`
  - Date/time pickers: `setTimeModalFromDate()`, `openTimeModal()`
  - Date suggestions: `applySuggestion()`
  - Utilities: `daysInMonth()`, `buildRange()`, `clampDayForCurrentMonth()`
  - Date picker guard: `setupDatePickerGuard()`

- **state.js** - State management and UI state helpers
  - State selectors: `optionalStateSettings()`, `isStateSelectable()`
  - Task search: `findTaskById()`, `collectAllTasks()`
  - Segmented controls: `getSegmentedValue()`, `setSegmentedValue()`, `wireSegmentedGroup()`
  - Select lists: `getSelectListValue()`
  - UI state: `updateSearchClearButton()`, `updateUndoButtonState()`, `hasActiveDragBlockingFilters()`
  - Task validation: `shouldConfirmCascade()`, `confirmCascadeMessage()`
  - Pinned: `getPinnedValue()`, `setPinnedValue()`

- **app.js** - Main application (currently contains all orchestration and event handlers)
  - Will be refactored to import modular functions
  - Handles all event listeners and modal management
  - Task rendering and filtering logic

## CSS Modules

Located in `ui/styles/`:

- **theme.css** - CSS variables and color scheme
  - Primary, secondary, and tertiary backgrounds
  - Text colors (primary, secondary, muted)
  - Accent colors and danger color
  - Component colors (input, chip backgrounds)
  - Font sizes for tasks and modals

- **base.css** - Typography, fonts, and base styles
  - @font-face declarations for Noto Sans
  - Global styles (*, body)
  - Form elements (button, input, select, textarea)

- **layout.css** - Flexbox and grid layouts
  - Toolbar and toolbar-actions
  - Task board and task list layouts
  - Modal grid layouts
  - Responsive media queries
  - Error display and search layout

- **components.css** - UI components
  - Buttons (search-clear, select-list, select-menu, select-option)
  - Modal layouts (modal, task-modal, settings-modal, etc.)
  - Modal-head and modal-body
  - Segmented controls and checkboxes
  - Chips and filter chips
  - Time and recurrence controls
  - Pill-shaped buttons and badges

- **interactions.css** - Interactive elements and animations
  - Drag and drop (drag-handle, drag-zones, drag-ghost)
  - Task state menu and state choices
  - Task stripes (urgency/importance indicators)
  - Drop zones and drop indicators
  - State-crossed (completed/blocked tasks)
  - Task title and metadata display

## Migration Guide

To fully modularize the JavaScript, follow these steps:

1. **Split app.js** into functional modules:
   - `modals.js` - Modal management functions (askConfirmation, openTaskModal, openSettingsModal, etc.)
   - `rendering.js` - Rendering functions (renderTaskList, renderTaskRow, renderFilterButtons, etc.)
   - `handlers.js` - Event listeners and handlers
   - `recurrence.js` - Recurrence logic and controls
   - `tags.js` - Tag management
   - `filter.js` - Filter and search logic

2. **Create an index file** that imports all modules:
   ```javascript
   import { invoke, els, app } from './modules/dom.js';
   import { safeInvoke, invokeCreateTask, ... } from './modules/api.js';
   import { formatDateTime, applyTheme, ... } from './modules/utils.js';
   // ... etc
   ```

3. **Update index.html** to load modules:
   ```html
   <script type="module" src="app-modular.js"></script>
   ```

4. **Test thoroughly** before removing the original app.js

## File Organization

```
ui/
├── index.html
├── app.js (original - can be kept for reference)
├── styles.css (original - can be kept for reference)
├── modules/
│   ├── dom.js
│   ├── api.js
│   ├── utils.js
│   ├── time.js
│   ├── state.js
│   └── (future: modals.js, rendering.js, handlers.js, etc.)
├── styles/
│   ├── theme.css
│   ├── base.css
│   ├── layout.css
│   ├── components.css
│   └── interactions.css
└── assets/
    └── fonts/
```

## Notes

- The original `app.js` and `styles.css` are still used as fallbacks
- Module files can be imported incrementally during refactoring
- Maintain backward compatibility by keeping the global `app` and `els` objects
- Consider using ES modules with proper bundling for production
