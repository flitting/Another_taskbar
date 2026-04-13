const invoke =
  window.__TAURI__?.core?.invoke ||
  window.__TAURI_INTERNALS__?.invoke ||
  window.__TAURI_INVOKE__;

if (!invoke) {
  document.body.innerHTML =
    "<p style='padding:16px'>Tauri invoke API is not available.</p>";
  throw new Error("Tauri invoke API is not available");
}

const els = {
  errorBox: document.getElementById("error-box"),
  errorText: document.getElementById("error-text"),
  errorCloseBtn: document.getElementById("error-close-btn"),
  taskBoard: document.getElementById("task-board"),
  taskList: document.getElementById("task-list"),
  searchInput: document.getElementById("search-input"),
  searchClearBtn: document.getElementById("search-clear-btn"),
  toolbarSort: document.getElementById("toolbar-sort"),
  settingsBtn: document.getElementById("settings-btn"),
  filterBtn: document.getElementById("filter-btn"),
  undoBtn: document.getElementById("undo-btn"),
  newTaskBtn: document.getElementById("new-task-btn"),

  taskModal: document.getElementById("task-modal"),
  taskModalTitle: document.getElementById("task-modal-title"),
  taskModalClose: document.getElementById("task-modal-close"),
  taskModalSave: document.getElementById("task-modal-save"),
  taskModalCancel: document.getElementById("task-modal-cancel"),
  taskModalDelete: document.getElementById("task-modal-delete"),
  taskFormName: document.getElementById("task-form-name"),
  taskFormState: document.getElementById("task-form-state"),
  taskFormPinned: document.getElementById("task-form-pinned"),
  taskFormDescription: document.getElementById("task-form-description"),
  taskFormUrgency: document.getElementById("task-form-urgency"),
  taskFormImportance: document.getElementById("task-form-importance"),
  taskFormDue: document.getElementById("task-form-due"),
  taskFormDueOpen: document.getElementById("task-form-due-open"),
  taskFormDueClear: document.getElementById("task-form-due-clear"),
  taskFormCompleted: document.getElementById("task-form-completed"),
  taskFormCompletedOpen: document.getElementById("task-form-completed-open"),
  taskFormCompletedClear: document.getElementById("task-form-completed-clear"),
  taskFormRecurringToggle: document.getElementById(
    "task-form-recurring-toggle",
  ),
  taskFormRecurringRow: document.getElementById("task-form-recurring-row"),
  taskFormRecurringFrequency: document.getElementById(
    "task-form-recurring-frequency",
  ),
  taskFormRecurringHour: document.getElementById("task-form-recurring-hour"),
  taskFormRecurringMinute: document.getElementById(
    "task-form-recurring-minute",
  ),
  taskFormRecurringCustomBtn: document.getElementById(
    "task-form-recurring-custom-btn",
  ),
  taskFormTagInput: document.getElementById("task-form-tag-input"),
  taskFormTagAdd: document.getElementById("task-form-tag-add"),
  taskFormTagsList: document.getElementById("task-form-tags-list"),
  taskFormQuickTags: document.getElementById("task-form-quick-tags"),
  quickTagsLabel: document.getElementById("quick-tags-label"),
  taskFormTimes: document.getElementById("task-form-times"),

  summaryModal: document.getElementById("summary-modal"),
  summaryTitle: document.getElementById("summary-title"),
  summaryStateRow: document.getElementById("summary-state-row"),
  summaryPriorityRow: document.getElementById("summary-priority-row"),
  summaryDescriptionRow: document.getElementById("summary-description-row"),
  summaryTagsRow: document.getElementById("summary-tags-row"),
  summaryTimesRow: document.getElementById("summary-times-row"),
  summaryCloseBtn: document.getElementById("summary-close-btn"),
  summaryOpenDetailBtn: document.getElementById("summary-open-detail-btn"),
  summaryCloseFooterBtn: document.getElementById("summary-close-footer-btn"),

  settingsModal: document.getElementById("settings-modal"),
  settingsModalClose: document.getElementById("settings-modal-close"),
  settingsTheme: document.getElementById("settings-theme"),
  settingsLanguage: document.getElementById("settings-language"),
  settingsOptionalStates: document.getElementById("settings-optional-states"),
  settingsAutoCompleteParents: document.getElementById(
    "settings-auto-complete-parents",
  ),
  settingsTaskFontSize: document.getElementById("settings-task-font-size"),
  settingsTaskFontSizeValue: document.getElementById(
    "settings-task-font-size-value",
  ),
  settingsThemePath: document.getElementById("settings-theme-path"),
  settingsThemeDefaultPath: document.getElementById(
    "settings-theme-default-path",
  ),
  settingsDataDir: document.getElementById("settings-data-dir"),
  settingsImportBtn: document.getElementById("settings-import-btn"),
  settingsLoadTaskbarBtn: document.getElementById("settings-load-taskbar-btn"),
  settingsSaveBtn: document.getElementById("settings-save-btn"),
  settingsDeleteDataBtn: document.getElementById("settings-delete-data-btn"),
  settingsUiScale: document.getElementById("settings-ui-scale"),
  settingsUiScaleValue: document.getElementById("settings-ui-scale-value"),

  filterModal: document.getElementById("filter-modal"),
  filterModalClose: document.getElementById("filter-modal-close"),
  filterImportanceButtons: document.getElementById("filter-importance-buttons"),
  filterUrgencyButtons: document.getElementById("filter-urgency-buttons"),
  filterStateButtons: document.getElementById("filter-state-buttons"),
  filterPinnedButtons: document.getElementById("filter-pinned-buttons"),
  filterTags: document.getElementById("filter-tags"),
  filterClearBtn: document.getElementById("filter-clear-btn"),
  filterApplyBtn: document.getElementById("filter-apply-btn"),

  timeModal: document.getElementById("time-modal"),
  timeModalTitle: document.getElementById("time-modal-title"),
  timeModalClose: document.getElementById("time-modal-close"),
  timeYear: document.getElementById("time-year"),
  timeMonth: document.getElementById("time-month"),
  timeDay: document.getElementById("time-day"),
  timeHour: document.getElementById("time-hour"),
  timeMinute: document.getElementById("time-minute"),
  timeSuggestTonight: document.getElementById("time-suggest-tonight"),
  timeSuggestNow: document.getElementById("time-suggest-now"),
  timeSuggestPlus15: document.getElementById("time-suggest-plus15"),
  timeSuggestTomorrow: document.getElementById("time-suggest-tomorrow"),
  timeModalSave: document.getElementById("time-modal-save"),
  timeModalCancel: document.getElementById("time-modal-cancel"),
  recurrenceModal: document.getElementById("recurrence-modal"),
  recurrenceModalClose: document.getElementById("recurrence-modal-close"),
  recurrenceEvery: document.getElementById("recurrence-every"),
  recurrenceUnit: document.getElementById("recurrence-unit"),
  recurrenceEndMode: document.getElementById("recurrence-end-mode"),
  recurrenceEndDate: document.getElementById("recurrence-end-date"),
  recurrenceEndCount: document.getElementById("recurrence-end-count"),
  recurrenceModalSave: document.getElementById("recurrence-modal-save"),
  recurrenceModalCancel: document.getElementById("recurrence-modal-cancel"),
  confirmModal: document.getElementById("confirm-modal"),
  confirmTitle: document.getElementById("confirm-title"),
  confirmMessage: document.getElementById("confirm-message"),
  confirmCancelBtn: document.getElementById("confirm-cancel-btn"),
  confirmOkBtn: document.getElementById("confirm-ok-btn"),
  hoverTip: document.getElementById("hover-tip"),
  dragZones: document.getElementById("drag-zones"),
  dragDeleteZone: document.getElementById("drag-delete-zone"),
  dragCancelZone: document.getElementById("drag-cancel-zone"),
};

const app = {
  snapshot: null,
  selectedId: null,
  collapsed: new Set(),
  modalMode: "create",
  modalTaskId: null,
  modalParentId: 0,
  summaryTaskId: null,
  modalTags: [],
  appliedFilter: {
    search: "",
    importance: new Set(),
    urgency: new Set(),
    state: new Set(),
    pinned: new Set(),
    tags: new Set(),
  },
  draftFilter: {
    importance: new Set(),
    urgency: new Set(),
    state: new Set(),
    pinned: new Set(),
    tags: new Set(),
  },
  timeEditingField: null,
  recurrenceEnabled: false,
  recurrenceDraft: null,
  drag: {
    active: false,
    taskId: null,
    sourceLi: null,
    placeholder: null,
    pointerOffsetX: 0,
    pointerOffsetY: 0,
    proposal: null,
    zone: null,
    ghost: null,
    autoScrollTimer: null,
  },
  strings: {},
};

function setError(message) {
  const text = (message || "").trim();
  els.errorText.textContent = text;
  els.errorBox.style.display = text ? "flex" : "none";
}

function localizeInvokeError(command, text) {
  if (command === "undo_last_change" && text.includes("Nothing to undo")) {
    return t("error_nothing_to_undo", "Nothing to undo.");
  }
  if (
    command === "reload_taskbar_file" &&
    text.includes("Failed to read taskbar file")
  ) {
    return t(
      "error_load_taskbar_json",
      "Failed to load taskbar.json from the configured directory.",
    );
  }
  return text;
}

async function safeInvoke(command, payload) {
  try {
    setError("");
    console.log("[invoke][js] calling:", command, payload);
    const result = await invoke(command, payload);
    console.log("[invoke][js] success:", command, result);
    return result;
  } catch (error) {
    console.error("[invoke][js] failed:", command, error);
    const text = typeof error === "string" ? error : JSON.stringify(error);
    console.error("[invoke][js] failed text:", text);
    setError(localizeInvokeError(command, text));
    throw error;
  }
}

function applyUiScale(value) {
  const numeric = Number(value);
  if (!Number.isFinite(numeric)) return;
  const clamped = Math.max(0.8, Math.min(1.4, numeric));
  document.body.style.zoom = String(clamped);
  if (els.settingsUiScale) {
    els.settingsUiScale.value = String(clamped);
  }
  if (els.settingsUiScaleValue) {
    els.settingsUiScaleValue.textContent = `${Math.round(clamped * 100)}%`;
  }
}

function askConfirmation(title, message, confirmLabel = null) {
  return new Promise((resolve) => {
    console.log("[confirm] open", { title, message, confirmLabel });

    const modal = els.confirmModal;
    const okBtn = els.confirmOkBtn;
    const cancelBtn = els.confirmCancelBtn;

    okBtn.type = "button";
    cancelBtn.type = "button";

    // 理论上不该还开着；如果开着，先关掉并清空返回值
    if (modal.open) {
      console.log("[confirm] modal already open, force close before reuse");
      modal.close();
    }
    modal.returnValue = "";

    els.confirmTitle.textContent = title;
    els.confirmMessage.textContent = message;
    okBtn.textContent = confirmLabel || t("confirm", "Confirm");

    let settled = false;

    const cleanup = () => {
      console.log("[confirm] cleanup listeners");
      okBtn.removeEventListener("click", onConfirm);
      cancelBtn.removeEventListener("click", onCancelClick);
      modal.removeEventListener("cancel", onCancelEvent);
      modal.removeEventListener("close", onClose);
    };

    const finish = (result, source) => {
      if (settled) {
        console.log("[confirm] finish ignored", { result, source });
        return;
      }
      settled = true;
      console.log("[confirm] finish", { result, source });
      cleanup();
      resolve(result);
    };

    const onConfirm = (event) => {
      console.log("[confirm] confirm button clicked");
      event.preventDefault();
      event.stopPropagation();

      // 不在这里 resolve，统一交给 onClose
      if (modal.open) {
        modal.close("confirm");
      }
    };

    const onCancelClick = (event) => {
      console.log("[confirm] cancel button clicked");
      event.preventDefault();
      event.stopPropagation();

      // 不在这里 resolve，统一交给 onClose
      if (modal.open) {
        modal.close("cancel");
      }
    };

    const onCancelEvent = (event) => {
      console.log("[confirm] dialog cancel event fired");
      event.preventDefault();

      // Esc / 原生取消行为，也统一走 close
      if (modal.open) {
        modal.close("cancel");
      }
    };

    const onClose = () => {
      const rv = modal.returnValue;
      console.log("[confirm] dialog close event fired", { returnValue: rv });

      if (rv === "confirm") {
        finish(true, "close-confirm");
      } else {
        finish(false, "close-cancel");
      }
    };

    okBtn.addEventListener("click", onConfirm);
    cancelBtn.addEventListener("click", onCancelClick);
    modal.addEventListener("cancel", onCancelEvent);
    modal.addEventListener("close", onClose);

    console.log("[confirm] showModal()");
    modal.showModal();
  });
}
async function invokeCreateTask(parentId, draft) {
  try {
    return await safeInvoke("create_task", { parentId, draft });
  } catch {
    return await safeInvoke("create_task", { parent_id: parentId, draft });
  }
}

async function invokeSetTaskState(id, state) {
  try {
    return await safeInvoke("set_task_state", { id, taskState: state });
  } catch {
    return await safeInvoke("set_task_state", { id, task_state: state });
  }
}

async function invokeSetTaskStateWithOptions(id, state, cascadeDescendants) {
  try {
    return await safeInvoke("set_task_state", {
      id,
      taskState: state,
      cascadeDescendants,
    });
  } catch {
    return await safeInvoke("set_task_state", {
      id,
      task_state: state,
      cascade_descendants: cascadeDescendants,
    });
  }
}

async function invokeUpdateTaskWithOptions(id, draft, cascadeDescendants) {
  try {
    return await safeInvoke("update_task_with_options", {
      id,
      draft,
      cascadeDescendants,
    });
  } catch {
    return await safeInvoke("update_task_with_options", {
      id,
      draft,
      cascade_descendants: cascadeDescendants,
    });
  }
}

async function invokeSetTheme(name) {
  try {
    return await safeInvoke("set_theme", { payload: { themeName: name } });
  } catch {
    return await safeInvoke("set_theme", { payload: { theme_name: name } });
  }
}

async function invokeMoveTask(taskId, targetId, relation) {
  try {
    return await safeInvoke("move_task", { taskId, targetId, relation });
  } catch {
    return await safeInvoke("move_task", {
      task_id: taskId,
      target_id: targetId,
      relation,
    });
  }
}

function valueOrNull(value) {
  return value === "" ? null : value;
}

function optionalStateSettings() {
  return new Set(app.snapshot?.settings?.enabled_optional_states || []);
}

function isStateSelectable(value) {
  if (value === "Todo" || value === "Completed") return true;
  return optionalStateSettings().has(value);
}

function availableSelectableTaskStates() {
  return ["Todo", "InProgress", "Blocked", "Completed", "Archived"].filter(
    isStateSelectable,
  );
}

function localFromIso(iso) {
  if (!iso) return "";
  const date = new Date(iso);
  if (Number.isNaN(date.getTime())) return "";
  const local = new Date(date.getTime() - date.getTimezoneOffset() * 60000);
  return local.toISOString().slice(0, 16);
}

function isoFromLocal(local) {
  if (!local) return null;
  const date = new Date(local);
  if (Number.isNaN(date.getTime())) return null;
  return date.toISOString();
}

function formatTimeDisplay(iso) {
  if (!iso) return t("none", "No time selected");
  const date = new Date(iso);
  if (Number.isNaN(date.getTime())) return t("none", "No time selected");
  return date.toLocaleString(undefined, {
    year: "numeric",
    month: "2-digit",
    day: "2-digit",
    hour: "2-digit",
    minute: "2-digit",
  });
}

function updateTimeButtons() {
  const due = els.taskFormDue.value || "";
  const completed = els.taskFormCompleted.value || "";

  els.taskFormDueOpen.textContent = due
    ? formatTimeDisplay(due)
    : t("none", "No time selected");
  els.taskFormDueOpen.classList.toggle("empty", !due);

  els.taskFormCompletedOpen.textContent = completed
    ? formatTimeDisplay(completed)
    : t("none", "No time selected");
  els.taskFormCompletedOpen.classList.toggle("empty", !completed);
}

function updateSearchClearButton() {
  const hasText = !!els.searchInput.value.trim();
  els.searchClearBtn.style.opacity = hasText ? "0.9" : "0.35";
}

function updateUndoButtonState() {
  const canUndo = !!app.snapshot?.can_undo;
  els.undoBtn.disabled = !canUndo;
  els.undoBtn.style.opacity = canUndo ? "1" : "0.45";
}

function hasActiveDragBlockingFilters() {
  const f = app.appliedFilter;
  return (
    !!f.search.trim() ||
    f.importance.size > 0 ||
    f.urgency.size > 0 ||
    f.state.size > 0 ||
    f.pinned.size > 0 ||
    f.tags.size > 0
  );
}

function daysInMonth(year, month1to12) {
  return new Date(year, month1to12, 0).getDate();
}

function buildRange(start, end, pad = 0) {
  const values = [];
  for (let i = start; i <= end; i += 1) {
    values.push(pad > 0 ? String(i).padStart(pad, "0") : String(i));
  }
  return values;
}

function clampDayForCurrentMonth() {
  const year = Number(getSelectListValue(els.timeYear));
  const month = Number(getSelectListValue(els.timeMonth));
  const maxDay = daysInMonth(year, month);
  const current = Number(getSelectListValue(els.timeDay) || "1");
  renderSelectList(
    els.timeDay,
    buildRange(1, maxDay),
    String(Math.min(current, maxDay)),
  );
}

function setTimeModalFromDate(date) {
  const nowYear = new Date().getFullYear();
  renderSelectList(
    els.timeYear,
    buildRange(nowYear - 2, nowYear + 8),
    String(date.getFullYear()),
    (value) => value,
    () => clampDayForCurrentMonth(),
  );
  renderSelectList(
    els.timeMonth,
    buildRange(1, 12),
    String(date.getMonth() + 1),
    (value) => value,
    () => clampDayForCurrentMonth(),
  );
  renderSelectList(
    els.timeHour,
    buildRange(0, 23, 2),
    String(date.getHours()).padStart(2, "0"),
  );
  renderSelectList(
    els.timeMinute,
    buildRange(0, 59, 2),
    String(date.getMinutes()).padStart(2, "0"),
  );
  const maxDay = daysInMonth(date.getFullYear(), date.getMonth() + 1);
  renderSelectList(
    els.timeDay,
    buildRange(1, maxDay),
    String(Math.min(date.getDate(), maxDay)),
  );
}

function selectedDateFromTimeModal() {
  const year = Number(getSelectListValue(els.timeYear));
  const month = Number(getSelectListValue(els.timeMonth));
  const day = Number(getSelectListValue(els.timeDay));
  const hour = Number(getSelectListValue(els.timeHour));
  const minute = Number(getSelectListValue(els.timeMinute));
  return new Date(year, month - 1, day, hour, minute, 0, 0);
}

function openTimeModal(field) {
  app.timeEditingField = field;
  const iso =
    field === "due" ? els.taskFormDue.value : els.taskFormCompleted.value;
  const base = iso ? new Date(iso) : new Date();
  setTimeModalFromDate(Number.isNaN(base.getTime()) ? new Date() : base);
  els.timeModalTitle.textContent =
    field === "due"
      ? t("set_due_time", "Set Due Time")
      : t("set_completed_time", "Set Completed Time");
  els.timeModal.showModal();
}

function applySuggestion(kind) {
  const now = new Date();
  let target = new Date(now);
  if (kind === "now") {
    target = now;
  } else if (kind === "tonight") {
    target.setHours(21, 0, 0, 0);
    if (target < now) {
      target.setDate(target.getDate() + 1);
    }
  } else if (kind === "plus15") {
    target = new Date(now.getTime() + 15 * 60 * 1000);
  } else if (kind === "tomorrow") {
    target.setDate(target.getDate() + 1);
    target.setHours(9, 0, 0, 0);
  }
  setTimeModalFromDate(target);
}

function formatDateTime(value) {
  if (!value) return "None";
  const d = new Date(value);
  if (Number.isNaN(d.getTime())) return value;
  return d.toLocaleString(undefined, {
    year: "numeric",
    month: "2-digit",
    day: "2-digit",
    hour: "2-digit",
    minute: "2-digit",
  });
}

function formatDueShort(value) {
  if (!value) return "";
  const d = new Date(value);
  if (Number.isNaN(d.getTime())) return "";
  return d.toLocaleString(undefined, {
    month: "short",
    day: "2-digit",
    hour: "2-digit",
    minute: "2-digit",
  });
}

function importanceValueLabel(value) {
  if (value === "High") return t("high", "High");
  if (value === "Low") return t("low", "Low");
  return t("none", "None");
}

function truncateText(value, maxLength = 56) {
  const text = (value || "").trim();
  if (!text) return "";
  if (text.length <= maxLength) return text;
  return `${text.slice(0, Math.max(0, maxLength - 3))}...`;
}

function normalizeTag(tag) {
  return tag.trim();
}

function t(key, fallback) {
  return app.strings?.[key] ?? fallback ?? key;
}

function setTip(element, text) {
  if (!element) return;
  if (!text) {
    element.removeAttribute("title");
    element.removeAttribute("data-tip");
    return;
  }
  element.title = text;
  element.dataset.tip = text;
}

function setSegmentedValue(groupEl, value) {
  const selected = value ?? "";
  for (const btn of groupEl.querySelectorAll("button[data-value]")) {
    btn.classList.toggle("active", btn.dataset.value === selected);
  }
}

function renderTaskStateSegmentButtons(selectedValue = "Todo") {
  const selected = isStateSelectable(selectedValue) ? selectedValue : "Todo";
  const states = availableSelectableTaskStates();
  const options = states.length ? states : ["Todo", "Completed"];
  els.taskFormState.innerHTML = "";
  for (const value of options) {
    const btn = document.createElement("button");
    btn.type = "button";
    btn.dataset.value = value;
    btn.textContent = `${stateSymbol(value)} ${stateLabel(value)}`;
    els.taskFormState.append(btn);
  }
  wireSegmentedGroup(els.taskFormState);
  setSegmentedValue(els.taskFormState, selected);
}

function setPinnedValue(value) {
  const pinned = !!value;
  els.taskFormPinned.dataset.value = pinned ? "true" : "false";
  els.taskFormPinned.classList.toggle("active", pinned);
  els.taskFormPinned.textContent = pinned
    ? `★ ${t("pinned", "Pinned")}`
    : `☆ ${t("unpinned", "Unpinned")}`;
}

function getPinnedValue() {
  return els.taskFormPinned.dataset.value === "true";
}

function defaultRecurrenceDraft() {
  return {
    frequency: "DoesNotRepeat",
    due_hour: String(new Date().getHours()).padStart(2, "0"),
    due_minute: String(new Date().getMinutes()).padStart(2, "0"),
    custom: {
      every: 1,
      unit: "Day",
      end: { mode: "Never", on_date: "", after_count: 1 },
    },
  };
}

function parseEndObject(endObj) {
  if (!endObj || typeof endObj !== "object") {
    return { mode: "Never", on_date: "", after_count: 1 };
  }
  if (Object.prototype.hasOwnProperty.call(endObj, "OnDate")) {
    const dt = new Date(endObj.OnDate);
    const yyyy = dt.getUTCFullYear();
    const mm = String(dt.getUTCMonth() + 1).padStart(2, "0");
    const dd = String(dt.getUTCDate()).padStart(2, "0");
    return {
      mode: "OnDate",
      on_date: `${yyyy}-${mm}-${dd}`,
      after_count: 1,
    };
  }
  if (Object.prototype.hasOwnProperty.call(endObj, "AfterOccurrences")) {
    return {
      mode: "AfterOccurrences",
      on_date: "",
      after_count: Number(endObj.AfterOccurrences || 1),
    };
  }
  return { mode: "Never", on_date: "", after_count: 1 };
}

function setRecurrenceDraftFromTask(taskRecurrence) {
  if (!taskRecurrence) {
    app.recurrenceEnabled = false;
    app.recurrenceDraft = defaultRecurrenceDraft();
    return;
  }
  app.recurrenceEnabled = true;
  const custom = taskRecurrence.custom || {};
  app.recurrenceDraft = {
    frequency: taskRecurrence.frequency || "DoesNotRepeat",
    due_hour: String(taskRecurrence.due_hour ?? 0).padStart(2, "0"),
    due_minute: String(taskRecurrence.due_minute ?? 0).padStart(2, "0"),
    custom: {
      every: Number(custom.every || 1),
      unit: custom.unit || "Day",
      end: parseEndObject(custom.end),
    },
  };
}

function applyDueTimeToRecurrenceDraft() {
  if (!app.recurrenceDraft) app.recurrenceDraft = defaultRecurrenceDraft();
  const dueIso = els.taskFormDue?.value || "";
  if (!dueIso) return;
  const due = new Date(dueIso);
  if (Number.isNaN(due.getTime())) return;
  app.recurrenceDraft.due_hour = String(due.getHours()).padStart(2, "0");
  app.recurrenceDraft.due_minute = String(due.getMinutes()).padStart(2, "0");
}

function toRecurrencePayload() {
  if (!app.recurrenceEnabled) return null;
  const draft = app.recurrenceDraft || defaultRecurrenceDraft();
  draft.frequency =
    getSelectListValue(els.taskFormRecurringFrequency) || draft.frequency;
  draft.due_hour =
    getSelectListValue(els.taskFormRecurringHour) || draft.due_hour;
  draft.due_minute =
    getSelectListValue(els.taskFormRecurringMinute) || draft.due_minute;
  if (draft.frequency === "DoesNotRepeat") return null;
  const payload = {
    frequency: draft.frequency,
    due_hour: Number(draft.due_hour || "0"),
    due_minute: Number(draft.due_minute || "0"),
    custom: null,
    occurrences_done: 0,
  };
  if (draft.frequency === "Custom") {
    const end = draft.custom?.end || {
      mode: "Never",
      on_date: "",
      after_count: 1,
    };
    let endValue = "Never";
    if (end.mode === "OnDate" && end.on_date) {
      const [y, m, d] = end.on_date.split("-").map((part) => Number(part));
      const onDateIso = new Date(
        Date.UTC(y, (m || 1) - 1, d || 1, 0, 0, 0),
      ).toISOString();
      endValue = { OnDate: onDateIso };
    } else if (end.mode === "AfterOccurrences") {
      endValue = { AfterOccurrences: Number(end.after_count || 1) };
    }
    payload.custom = {
      every: Number(draft.custom?.every || 1),
      unit: draft.custom?.unit || "Day",
      end: endValue,
    };
  }
  return payload;
}

function renderRecurrenceControls() {
  const draft = app.recurrenceDraft || defaultRecurrenceDraft();
  const enabledLabel = app.recurrenceEnabled
    ? t("recurring_on", "Recurring: On")
    : t("recurring_off", "Does not repeat");
  els.taskFormRecurringToggle.classList.toggle("active", app.recurrenceEnabled);
  els.taskFormRecurringToggle.textContent = enabledLabel;
  els.taskFormRecurringRow.style.display = app.recurrenceEnabled ? "" : "none";
  if (!app.recurrenceEnabled) return;

  const frequencies = [
    "DoesNotRepeat",
    "Daily",
    "Weekly",
    "Biweekly",
    "Monthly",
    "Yearly",
    "Custom",
  ];
  renderSelectList(
    els.taskFormRecurringFrequency,
    frequencies,
    draft.frequency,
    (value) => {
      if (value === "DoesNotRepeat")
        return t("recurring_off", "Does not repeat");
      if (value === "Daily") return t("recurrence_daily", "Daily");
      if (value === "Weekly") return t("recurrence_weekly", "Weekly");
      if (value === "Biweekly") return t("recurrence_biweekly", "Biweekly");
      if (value === "Monthly") return t("recurrence_monthly", "Monthly");
      if (value === "Yearly") return t("recurrence_yearly", "Yearly");
      return t("recurring_custom", "Custom");
    },
    (value) => {
      app.recurrenceDraft.frequency = value;
      els.taskFormRecurringCustomBtn.style.display =
        value === "Custom" ? "" : "none";
      if (value === "Custom") {
        openRecurrenceModal();
      }
    },
  );
  renderSelectList(
    els.taskFormRecurringHour,
    buildRange(0, 23, 2),
    draft.due_hour,
  );
  renderSelectList(
    els.taskFormRecurringMinute,
    buildRange(0, 59, 2),
    draft.due_minute,
  );
  els.taskFormRecurringCustomBtn.style.display =
    draft.frequency === "Custom" ? "" : "none";
}

function openRecurrenceModal() {
  if (!app.recurrenceDraft) app.recurrenceDraft = defaultRecurrenceDraft();
  const custom = app.recurrenceDraft.custom || defaultRecurrenceDraft().custom;
  els.recurrenceEvery.value = String(Math.max(1, Number(custom.every || 1)));
  renderSelectList(
    els.recurrenceUnit,
    ["Day", "Week", "Month", "Year"],
    custom.unit || "Day",
    (value) => {
      if (value === "Day") return t("recurrence_unit_day", "Day(s)");
      if (value === "Week") return t("recurrence_unit_week", "Week(s)");
      if (value === "Month") return t("recurrence_unit_month", "Month(s)");
      return t("recurrence_unit_year", "Year(s)");
    },
  );
  setSegmentedValue(els.recurrenceEndMode, custom.end?.mode || "Never");
  els.recurrenceEndDate.value = custom.end?.on_date || "";
  els.recurrenceEndCount.value = String(
    Math.max(1, Number(custom.end?.after_count || 1)),
  );
  syncRecurrenceEndInputs();
  els.recurrenceModal.showModal();
}

function syncRecurrenceEndInputs() {
  const mode = getSegmentedValue(els.recurrenceEndMode) || "Never";
  els.recurrenceEndDate.disabled = mode !== "OnDate";
  els.recurrenceEndCount.disabled = mode !== "AfterOccurrences";
}

function getSegmentedValue(groupEl) {
  const active = groupEl.querySelector("button.active");
  return active ? active.dataset.value : "";
}

function wireSegmentedGroup(groupEl) {
  for (const btn of groupEl.querySelectorAll("button[data-value]")) {
    btn.addEventListener("click", () =>
      setSegmentedValue(groupEl, btn.dataset.value),
    );
  }
}

function renderSegmentedOptions(groupEl, options, selected) {
  groupEl.innerHTML = "";
  for (const value of options) {
    const btn = document.createElement("button");
    btn.type = "button";
    btn.dataset.value = value;
    btn.textContent = value;
    groupEl.append(btn);
  }
  wireSegmentedGroup(groupEl);
  const fallback = options[0] ?? "";
  setSegmentedValue(groupEl, selected ?? fallback);
}

function stateLabel(value) {
  switch (value) {
    case "Todo":
      return t("state_todo", "Todo");
    case "InProgress":
      return t("state_in_progress", "In Progress");
    case "Blocked":
      return t("state_blocked", "Blocked");
    case "Completed":
      return t("state_completed", "Completed");
    case "Archived":
      return t("state_archived", "Archived");
    default:
      return value;
  }
}

function renderSelectList(
  controlEl,
  options,
  selected,
  formatLabel = (value) => value,
  onChange = null,
  formatTriggerLabel = null,
) {
  controlEl.innerHTML = "";
  controlEl.classList.remove("open");
  const trigger = document.createElement("button");
  trigger.type = "button";
  trigger.className = "select-trigger";
  const menu = document.createElement("div");
  menu.className = "select-menu";

  controlEl.append(trigger, menu);

  let chosen = selected;
  if (!options.includes(chosen)) {
    chosen = options[0] ?? "";
  }
  controlEl.dataset.value = chosen;
  trigger.textContent = (formatTriggerLabel || formatLabel)(chosen);

  for (const value of options) {
    const option = document.createElement("button");
    option.type = "button";
    option.className = "select-option";
    option.dataset.value = value;
    option.textContent = formatLabel(value);
    option.classList.toggle("active", value === chosen);
    option.addEventListener("click", () => {
      controlEl.dataset.value = value;
      trigger.textContent = (formatTriggerLabel || formatLabel)(value);
      for (const item of menu.querySelectorAll(".select-option")) {
        item.classList.toggle("active", item.dataset.value === value);
      }
      controlEl.classList.remove("open");
      if (typeof onChange === "function") {
        onChange(value);
      }
    });
    menu.append(option);
  }

  trigger.addEventListener("click", () => {
    const opening = !controlEl.classList.contains("open");
    document
      .querySelectorAll(".select-list.open")
      .forEach((node) => node.classList.remove("open"));
    if (opening) controlEl.classList.add("open");
  });
}

function getSelectListValue(controlEl) {
  return controlEl.dataset.value || "";
}

function setupHoverTips() {
  const tip = els.hoverTip;
  if (!tip) return;

  function pickTipHost(target) {
    const insideDialog = target?.closest("dialog[open]");
    return insideDialog || document.body;
  }

  function showAt(target, x, y, text) {
    const host = pickTipHost(target);
    if (tip.parentElement !== host) {
      host.append(tip);
    }
    tip.textContent = text;
    tip.style.display = "block";
    const pad = 10;
    const rect = tip.getBoundingClientRect();
    let left = x + 12;
    let top = y + 14;
    if (left + rect.width + pad > window.innerWidth) {
      left = x - rect.width - 12;
    }
    if (top + rect.height + pad > window.innerHeight) {
      top = y - rect.height - 12;
    }
    if (left < pad) left = pad;
    if (top < pad) top = pad;
    tip.style.left = `${left}px`;
    tip.style.top = `${top}px`;
  }

  function hide() {
    tip.style.display = "none";
  }

  document.addEventListener("mousemove", (event) => {
    const target =
      event.target instanceof Element
        ? event.target.closest("[data-tip]")
        : null;
    if (!target) {
      hide();
      return;
    }
    const text = target.getAttribute("data-tip");
    if (!text) {
      hide();
      return;
    }
    showAt(target, event.clientX, event.clientY, text);
  });

  document.addEventListener("focusin", (event) => {
    const target =
      event.target instanceof Element
        ? event.target.closest("[data-tip]")
        : null;
    if (!target) {
      hide();
      return;
    }
    const text = target.getAttribute("data-tip");
    if (!text) {
      hide();
      return;
    }
    const rect = target.getBoundingClientRect();
    showAt(target, rect.left + 6, rect.bottom + 4, text);
  });

  document.addEventListener("focusout", hide);
  document.addEventListener("mouseleave", hide);
}

function applyTaskFontSize(value) {
  const size = Number(value);
  if (!Number.isFinite(size)) return;
  const clamped = Math.max(11, Math.min(28, Math.round(size)));
  const defaultSize = 14;
  document.documentElement.style.setProperty(
    "--task-font-size",
    `${clamped}px`,
  );
  document.documentElement.style.setProperty(
    "--modal-font-size",
    `${clamped}px`,
  );
  if (els.settingsTaskFontSize) {
    els.settingsTaskFontSize.value = String(clamped);
  }
  if (els.settingsTaskFontSizeValue) {
    els.settingsTaskFontSizeValue.textContent = `${clamped}px (${t("default_value", "default")}: ${defaultSize}px)`;
  }
}

function sortModeLabel(value) {
  if (value === "TaskName") return t("task_sort_name", "Task name");
  if (value === "CreateFirst") return t("task_sort_create", "Create first");
  if (value === "UpdateFirst") return t("task_sort_update", "Update first");
  if (value === "CompleteFirst")
    return t("task_sort_complete", "Complete first");
  return t("task_sort_custom", "Custom");
}

function sortDisplayLabel(value) {
  return `${t("sort_in_prefix", "Sort in")} : ${sortModeLabel(value)}`;
}

function shouldConfirmCascade(task, nextState) {
  if (!task || !(task.subtasks || []).length) return false;
  if (nextState === "Completed") return true;
  if (nextState === "Blocked" && isStateSelectable("Blocked")) return true;
  return false;
}

function confirmCascadeMessage(nextState) {
  if (nextState === "Blocked") {
    return t(
      "confirm_block_descendants",
      "Set all subtasks and nested subtasks to Blocked too?",
    );
  }
  return t(
    "confirm_complete_descendants",
    "Set all subtasks and nested subtasks to Completed too?",
  );
}

async function saveSortMode(value) {
  const current = app.snapshot?.settings;
  if (!current || current.task_sort_mode === value) return;
  const settings = {
    selected_theme: current.selected_theme,
    task_font_size: current.task_font_size,
    selected_language: current.selected_language,
    task_sort_mode: value,
    enabled_optional_states: current.enabled_optional_states || [],
    auto_complete_parent_tasks: current.auto_complete_parent_tasks !== false,
    task_data_directory: current.task_data_directory || "",
    ui_scale: current.ui_scale ?? 1.0,
  };
  await safeInvoke("save_gui_settings_cmd", { settings });
  await refresh();
}

function renderToolbarSortMode() {
  const current = app.snapshot?.settings?.task_sort_mode || "Custom";
  renderSelectList(
    els.toolbarSort,
    ["Custom", "TaskName", "CreateFirst", "UpdateFirst", "CompleteFirst"],
    current,
    sortModeLabel,
    (value) => {
      saveSortMode(value);
    },
    sortDisplayLabel,
  );
  setTip(
    els.toolbarSort.querySelector(".select-trigger"),
    t("task_sort", "Task Sort"),
  );
}

function addModalTag(tag) {
  const normalized = normalizeTag(tag);
  if (!normalized) return;
  if (!app.modalTags.includes(normalized)) {
    app.modalTags.push(normalized);
  }
  renderModalTags();
}

function removeModalTag(tag) {
  app.modalTags = app.modalTags.filter((item) => item !== tag);
  renderModalTags();
}

function renderModalTags() {
  els.taskFormTagsList.innerHTML = "";
  if (!app.modalTags.length) {
    const muted = document.createElement("span");
    muted.className = "muted";
    muted.textContent = t("no_tags", "No tags");
    els.taskFormTagsList.append(muted);
  } else {
    for (const tag of app.modalTags) {
      const chip = document.createElement("button");
      chip.className = "chip tag-remove";
      chip.innerHTML = `<span>${tag}</span><span>x</span>`;
      chip.addEventListener("click", () => removeModalTag(tag));
      els.taskFormTagsList.append(chip);
    }
  }

  els.taskFormQuickTags.innerHTML = "";
  const quickTags = app.snapshot?.common_tags || [];
  els.quickTagsLabel.style.display = quickTags.length ? "" : "none";
  els.taskFormQuickTags.style.display = quickTags.length ? "flex" : "none";
  for (const tag of quickTags) {
    const chip = document.createElement("button");
    chip.className = "chip";
    chip.textContent = tag;
    chip.addEventListener("click", () => addModalTag(tag));
    els.taskFormQuickTags.append(chip);
  }
}

function collectAllTasks(tasks, depth = 0, out = []) {
  for (const task of tasks) {
    out.push({ task, depth });
    if (task.subtasks?.length) {
      collectAllTasks(task.subtasks, depth + 1, out);
    }
  }
  return out;
}

function findTaskById(tasks, id) {
  if (id == null) return null;
  for (const task of tasks) {
    if (task.id === id) return task;
    const nested = findTaskById(task.subtasks || [], id);
    if (nested) return nested;
  }
  return null;
}

function collectAllTags(tasks, set = new Set()) {
  for (const task of tasks) {
    for (const tag of task.tags || []) set.add(tag);
    collectAllTags(task.subtasks || [], set);
  }
  return [...set].sort((a, b) => a.localeCompare(b));
}

function matchesEnumSet(value, selectedSet) {
  if (!selectedSet || selectedSet.size === 0) return true;
  const normalized = value == null ? "None" : value;
  return selectedSet.has(normalized);
}

function matchesPinnedSet(value, selectedSet) {
  if (!selectedSet || selectedSet.size === 0) return true;
  return selectedSet.has(value ? "Pinned" : "Unpinned");
}

function matchesTask(task) {
  const f = app.appliedFilter;
  const q = f.search.trim().toLowerCase();
  const searchOK =
    !q ||
    task.name.toLowerCase().includes(q) ||
    task.description.toLowerCase().includes(q);

  return (
    searchOK &&
    matchesEnumSet(task.state, f.state) &&
    matchesEnumSet(task.importance, f.importance) &&
    matchesEnumSet(task.urgency, f.urgency) &&
    matchesPinnedSet(task.pinned, f.pinned) &&
    (f.tags.size === 0 || (task.tags || []).some((tag) => f.tags.has(tag)))
  );
}

function filterTree(task) {
  const sub = (task.subtasks || []).map(filterTree).filter(Boolean);
  if (matchesTask(task) || sub.length > 0) {
    return { ...task, subtasks: sub };
  }
  return null;
}

function filteredRootTasks() {
  return (app.snapshot?.tasks || []).map(filterTree).filter(Boolean);
}

function applyTheme(theme) {
  if (!theme) return;
  const root = document.documentElement;
  root.style.setProperty("--bg", theme.primary_bg);
  root.style.setProperty("--panel", theme.secondary_bg);
  root.style.setProperty("--panel-soft", theme.secondary_bg);
  root.style.setProperty("--row", theme.tertiary_bg);
  root.style.setProperty("--row-inner", theme.highlight_bg);
  root.style.setProperty("--border", theme.accent_color);
  root.style.setProperty("--text", theme.text_primary);
  root.style.setProperty("--muted", theme.text_secondary);
  root.style.setProperty("--accent", theme.selection_bg);
  root.style.setProperty("--accent-2", theme.accent_color);
  root.style.setProperty("--danger", theme.blocked_color);
  root.style.setProperty("--input-bg", theme.input_bg);
  root.style.setProperty("--chip-bg", theme.tag_bg || theme.highlight_bg);
  root.style.setProperty(
    "--chip-active",
    theme.tag_active_bg || theme.selection_bg,
  );

  const bg = (theme.primary_bg || "").replace("#", "");
  if (bg.length === 6) {
    const r = parseInt(bg.slice(0, 2), 16);
    const g = parseInt(bg.slice(2, 4), 16);
    const b = parseInt(bg.slice(4, 6), 16);
    const luminance = (0.2126 * r + 0.7152 * g + 0.0722 * b) / 255;
    root.style.colorScheme = luminance > 0.6 ? "light" : "dark";
  }
}

function stateSymbol(taskState) {
  switch (taskState) {
    case "Todo":
      return "○";
    case "InProgress":
      return "◐";
    case "Blocked":
      return "△";
    case "Completed":
      return "✓";
    case "Archived":
      return "⇧";
    default:
      return "○";
  }
}

function stateColor(taskState) {
  const palette = app.snapshot?.active_theme;
  switch (taskState) {
    case "Todo":
      return palette?.todo_color || "#7387BA";
    case "InProgress":
      return palette?.in_progress_color || "#E0AC47";
    case "Blocked":
      return palette?.blocked_color || "#D47272";
    case "Completed":
      return palette?.completed_color || "#74C787";
    case "Archived":
      return palette?.archived_color || "#7D8092";
    default:
      return app.snapshot?.active_theme?.text_primary || "#F3F3FA";
  }
}

function updateTaskStateSegmentLabels() {
  renderTaskStateSegmentButtons(getSegmentedValue(els.taskFormState) || "Todo");
}

function applyTextInputAttributes() {
  const activeLang =
    app.snapshot?.settings?.selected_language === "zh-CN" ? "zh-CN" : "en";
  document.querySelectorAll("input[type='text'], textarea").forEach((field) => {
    field.setAttribute("lang", activeLang);
    field.setAttribute("inputmode", "text");
    field.setAttribute("autocapitalize", "off");
    field.setAttribute("autocorrect", "off");
    field.setAttribute("autocomplete", "off");
    field.spellcheck = false;
  });
}

function applyLocalizedText() {
  const duePrefix = t("due_none", "Due: None").split(/[:：]/)[0];
  const completedPrefix = t("completed_none", "Completed: None").split(
    /[:：]/,
  )[0];
  const activeLang =
    app.snapshot?.settings?.selected_language === "zh-CN" ? "zh-CN" : "en";
  document.documentElement.lang = activeLang;
  applyTextInputAttributes();
  document.getElementById("title-tasks").textContent = t("tasks", "Tasks");
  els.searchInput.placeholder = t("search_tasks", "Search tasks");
  els.searchClearBtn.textContent = "✕";
  els.filterBtn.textContent = t("filter", "Filter");
  els.undoBtn.textContent = t("undo", "Undo");
  setTip(els.newTaskBtn, t("create_root_help", "Create a new top-level task."));
  setTip(els.settingsBtn, t("open_settings_help", "Open settings."));
  setTip(els.searchClearBtn, t("clear_search_help", "Clear search."));
  setTip(els.filterBtn, t("open_filter_help", "Open filters."));
  setTip(els.undoBtn, t("undo_help", "Undo latest change."));

  document.getElementById("task-name-label").childNodes[0].textContent =
    `${t("task_name", "Task name")}\n`;
  document.getElementById("state-label").textContent = t("state", "State");
  document.getElementById("urgency-label").textContent = t(
    "urgency",
    "Urgency",
  );
  document.getElementById("importance-label").textContent = t(
    "importance",
    "Importance",
  );
  document.getElementById("pinned-label").textContent = t("pinned", "Pinned");
  setPinnedValue(getPinnedValue());
  document.getElementById("description-label").childNodes[0].textContent =
    `${t("description", "Description")}\n`;
  document.getElementById("due-label").childNodes[0].textContent =
    `${duePrefix}\n`;
  document.getElementById("completed-label").childNodes[0].textContent =
    `${completedPrefix}\n`;
  document.getElementById("tags-label").textContent = t("tags", "Tags");
  document.getElementById("quick-tags-label").textContent = t(
    "quick_add",
    "Quick Add",
  );
  els.taskFormTagInput.placeholder = t("new_tag", "New tag");
  if (!els.taskFormDue.value) {
    els.taskFormDueOpen.textContent = t("none", "No time selected");
  }
  if (!els.taskFormCompleted.value) {
    els.taskFormCompletedOpen.textContent = t("none", "No time selected");
  }
  els.taskModalSave.textContent = t("save", "Save");
  els.taskModalCancel.textContent = t("close", "Close");
  els.taskModalDelete.textContent = t("delete", "Delete");
  setTip(els.taskModalSave, t("save_detail_help", "Save"));
  setTip(els.taskModalCancel, t("close_detail_help", "Close"));
  setTip(els.taskModalDelete, t("delete_help", "Delete task"));
  setTip(els.taskModalClose, t("close_popup_help", "Close"));
  els.summaryOpenDetailBtn.textContent = t("open_detail", "Open details");
  els.summaryCloseFooterBtn.textContent = t("close", "Close");
  setTip(els.summaryCloseBtn, t("close_popup_help", "Close"));

  document.getElementById("settings-title").textContent = t(
    "settings",
    "Settings",
  );
  document.getElementById("settings-theme-label").textContent = t(
    "theme",
    "Theme",
  );
  document.getElementById("settings-language-label").textContent = t(
    "language",
    "Language",
  );
  document.getElementById("settings-optional-states-label").textContent = t(
    "optional_task_states",
    "Optional task states",
  );
  document.getElementById("settings-auto-complete-parents-label").textContent =
    t("auto_complete_parent_tasks", "Auto complete parent tasks");
  document.getElementById(
    "settings-task-font-size-label",
  ).childNodes[0].textContent = `${t("task_font_size", "Task Font Size")}\n`;
  document.getElementById(
    "settings-theme-path-label",
  ).childNodes[0].textContent =
    `${t("import_theme_path", "Import Theme Path")}\n`;
  els.settingsThemeDefaultPath.textContent = `${t("theme_default_path", "Theme path")}: ${app.snapshot?.theme_dir_path || "-"}`;
  document.getElementById("settings-data-dir-label").childNodes[0].textContent =
    `${t("data_directory_path", "Data Directory Path")}\n`;
  document.getElementById("settings-ui-scale-label").childNodes[0].textContent =
    `${t("ui_scale", "UI Scale")}\n`;
  els.settingsImportBtn.textContent = t("import_theme", "Import Theme");
  els.settingsLoadTaskbarBtn.textContent = t(
    "load_taskbar_json",
    "Load taskbar.json",
  );
  els.settingsSaveBtn.textContent = t("save", "Save");
  els.settingsDeleteDataBtn.textContent = t(
    "delete_all_data_exit",
    "Delete all data and exit",
  );
  setTip(els.settingsModalClose, t("close_settings_help", "Close settings"));
  setTip(
    els.settingsImportBtn,
    t("import_theme_help", "Import theme from a TOML file path."),
  );
  setTip(
    els.settingsLoadTaskbarBtn,
    t(
      "load_taskbar_json_help",
      "Reload taskbar.json from the configured data directory.",
    ),
  );
  setTip(els.settingsUiScale, t("ui_scale_help", "Adjust overall UI zoom."));
  setTip(els.settingsSaveBtn, t("save_settings_help", "Save settings"));
  setTip(
    els.settingsDeleteDataBtn,
    t("delete_all_data_exit_help", "Delete all app data and quit immediately."),
  );
  els.dragDeleteZone.textContent = t("delete", "Delete");
  els.dragCancelZone.textContent = t("cancel_drag", "Cancel Drag");

  document.getElementById("filter-title").textContent = t(
    "filter_title",
    "Filter",
  );
  document.getElementById("filter-importance-label").textContent = t(
    "importance",
    "Importance",
  );
  document.getElementById("filter-urgency-label").textContent = t(
    "urgency",
    "Urgency",
  );
  document.getElementById("filter-state-label").textContent = t(
    "state",
    "State",
  );
  document.getElementById("filter-pinned-label").textContent = t(
    "pinned",
    "Pinned",
  );
  document.getElementById("filter-tags-label").textContent = t("tags", "Tags");
  els.filterClearBtn.textContent = t("clear_all_filters", "Clear All");
  els.filterApplyBtn.textContent = t("confirm", "Confirm");
  setTip(els.filterModalClose, t("filter_close_help", "Close filter"));
  setTip(els.filterClearBtn, t("clear_all_filters_help", "Clear filters"));
  setTip(els.filterApplyBtn, t("confirm_filters_help", "Apply filters"));
  setTip(els.taskFormDueOpen, t("set_due_time", "Set Due Time"));
  setTip(els.taskFormDueClear, t("clear", "Clear"));
  setTip(
    els.taskFormCompletedOpen,
    t("set_completed_time", "Set Completed Time"),
  );
  setTip(els.taskFormCompletedClear, t("clear", "Clear"));

  document.getElementById("time-modal-title").textContent = t(
    "set_due_time",
    "Set Time",
  );
  document.getElementById("time-year-label").textContent = t("year", "Year");
  document.getElementById("time-month-label").textContent = t("month", "Month");
  document.getElementById("time-day-label").textContent = t("day", "Day");
  document.getElementById("time-hour-label").textContent = t("hour", "Hour");
  document.getElementById("time-minute-label").textContent = t("mins", "Mins");
  els.timeModalSave.textContent = t("save", "Save");
  els.timeModalCancel.textContent = t("cancel", "Cancel");
  setTip(els.timeModalClose, t("close_popup_help", "Close"));
  els.timeSuggestNow.textContent = t("time_now", "Now");
  els.timeSuggestTonight.textContent = t("time_tonight", "Tonight");
  els.timeSuggestPlus15.textContent = t("time_plus_15", "15 mins later");
  els.timeSuggestTomorrow.textContent = t("time_tomorrow", "Tomorrow 9:00");
  setTip(els.timeSuggestNow, t("time_now_help", "Set to current time"));
  setTip(
    els.timeSuggestTonight,
    t("time_tonight_help", "Set 21:00 (today/tomorrow)"),
  );
  setTip(els.timeSuggestPlus15, t("time_plus_15_help", "Set now + 15 minutes"));
  setTip(
    els.timeSuggestTomorrow,
    t("time_tomorrow_help", "Set tomorrow at 09:00"),
  );
  document.getElementById("recurring-label").textContent = t(
    "recurring",
    "Recurring",
  );
  document.getElementById("recurring-repeat-label").childNodes[0].textContent =
    `${t("recurring_repeat", "Repeat")}\n`;
  document.getElementById(
    "recurring-due-time-label",
  ).childNodes[0].textContent = `${t("recurring_due_time", "Due time")}\n`;
  els.taskFormRecurringCustomBtn.textContent = t("recurring_custom", "Custom");
  document.getElementById("recurrence-custom-title").textContent = t(
    "recurrence_custom_title",
    "Custom Recurrence",
  );
  document.getElementById("recurrence-repeats-every-label").textContent = t(
    "recurrence_repeats_every",
    "Repeats every",
  );
  document.getElementById("recurrence-unit-label").childNodes[0].textContent =
    `${t("recurrence_unit", "Unit")}\n`;
  document.getElementById("recurrence-ends-label").textContent = t(
    "recurrence_ends",
    "Ends",
  );
  document.getElementById("recurrence-end-never-btn").textContent = t(
    "recurrence_end_never",
    "Never",
  );
  document.getElementById("recurrence-end-on-btn").textContent = t(
    "recurrence_end_on",
    "On",
  );
  document.getElementById("recurrence-end-after-btn").textContent = t(
    "recurrence_end_after",
    "After",
  );
  document.getElementById(
    "recurrence-on-date-label",
  ).childNodes[0].textContent = `${t("recurrence_on_date", "On date")}\n`;
  document.getElementById(
    "recurrence-occurrences-label",
  ).childNodes[0].textContent =
    `${t("recurrence_occurrences", "Occurrences")}\n`;

  updateTaskStateSegmentLabels();
}

function openTaskModalCreate(parentId) {
  app.modalMode = "create";
  app.modalTaskId = null;
  app.modalParentId = parentId ?? 0;
  app.modalTags = [];

  els.taskModalTitle.textContent = "";
  els.taskModalDelete.style.display = "none";

  els.taskFormName.value = "Untitled";
  renderTaskStateSegmentButtons("Todo");
  setPinnedValue(false);
  els.taskFormDescription.value = "";
  setSegmentedValue(els.taskFormUrgency, "");
  setSegmentedValue(els.taskFormImportance, "");
  els.taskFormDue.value = "";
  els.taskFormCompleted.value = "";
  setRecurrenceDraftFromTask(null);
  renderRecurrenceControls();
  updateTimeButtons();
  els.taskFormTagInput.value = "";
  els.taskFormTimes.textContent = "";

  renderModalTags();
  els.taskModal.showModal();
}

function openTaskModalEdit(taskId) {
  const task = findTaskById(app.snapshot?.tasks || [], taskId);
  if (!task) return;

  app.modalMode = "edit";
  app.modalTaskId = taskId;
  app.modalParentId = 0;
  app.modalTags = [...(task.tags || [])];

  els.taskModalTitle.textContent = "";
  els.taskModalDelete.style.display = "inline-block";

  els.taskFormName.value = task.name || "Untitled";
  renderTaskStateSegmentButtons(task.state);
  setPinnedValue(!!task.pinned);
  els.taskFormDescription.value = task.description;
  setSegmentedValue(els.taskFormUrgency, task.urgency || "");
  setSegmentedValue(els.taskFormImportance, task.importance || "");
  els.taskFormDue.value = task.times?.due_date || "";
  els.taskFormCompleted.value = task.times?.completed_at || "";
  setRecurrenceDraftFromTask(task.recurrence || null);
  renderRecurrenceControls();
  updateTimeButtons();
  els.taskFormTagInput.value = "";
  els.taskFormTimes.textContent = `${t("created_at", "Created at")}: ${formatDateTime(task.times?.created_at)} | ${t("updated_at", "Updated at")}: ${formatDateTime(task.times?.updated_at)}`;

  renderModalTags();
  els.taskModal.showModal();
}

function openTaskSummaryModal(taskId) {
  const task = findTaskById(app.snapshot?.tasks || [], taskId);
  if (!task) return;
  app.summaryTaskId = taskId;
  els.summaryTitle.textContent = task.name || "Untitled";
  els.summaryStateRow.textContent = `${stateSymbol(task.state)} ${stateLabel(task.state)}`;
  const urgencyLabel = importanceValueLabel(task.urgency);
  const importanceLabel = importanceValueLabel(task.importance);
  els.summaryPriorityRow.textContent = `${t("urgency", "Urgency")}: ${urgencyLabel} | ${t("importance", "Importance")}: ${importanceLabel}`;
  els.summaryDescriptionRow.textContent =
    task.description?.trim() || t("no_description", "No description");
  const tags = (task.tags || []).join(", ");
  els.summaryTagsRow.textContent = tags
    ? `${t("tags", "Tags")}: ${tags}`
    : `${t("tags", "Tags")}: ${t("none", "None")}`;
  els.summaryTimesRow.textContent = `${t("created_at", "Created at")}: ${formatDateTime(task.times?.created_at)} | ${t("updated_at", "Updated at")}: ${formatDateTime(task.times?.updated_at)}`;
  els.summaryModal.showModal();
}

async function saveTaskFromModal() {
  const name = els.taskFormName.value.trim() || "Untitled";
  const nextState = getSegmentedValue(els.taskFormState) || "Todo";
  let cascadeDescendants = false;
  if (app.modalMode === "edit" && app.modalTaskId != null) {
    const existingTask = findTaskById(
      app.snapshot?.tasks || [],
      app.modalTaskId,
    );
    if (
      existingTask &&
      existingTask.state !== nextState &&
      shouldConfirmCascade(existingTask, nextState)
    ) {
      cascadeDescendants = window.confirm(confirmCascadeMessage(nextState));
    }
  }

  const draft = {
    name,
    description: els.taskFormDescription.value,
    state: nextState,
    urgency: valueOrNull(getSegmentedValue(els.taskFormUrgency)),
    importance: valueOrNull(getSegmentedValue(els.taskFormImportance)),
    tags: [...app.modalTags],
    pinned: getPinnedValue(),
    due_date: valueOrNull(els.taskFormDue.value),
    completed_at: valueOrNull(els.taskFormCompleted.value),
    recurrence: toRecurrencePayload(),
  };

  if (app.modalMode === "create") {
    const id = await invokeCreateTask(app.modalParentId, draft);
    app.selectedId = id;
  } else if (app.modalMode === "edit" && app.modalTaskId != null) {
    await invokeUpdateTaskWithOptions(
      app.modalTaskId,
      draft,
      cascadeDescendants,
    );
    app.selectedId = app.modalTaskId;
  }

  els.taskModal.close();
  await refresh();
}

async function deleteTaskFromModal() {
  if (app.modalTaskId == null) return;

  const task = findTaskById(app.snapshot?.tasks || [], app.modalTaskId);
  if (!task) return;
  const confirmed = await askConfirmation(
    t("delete", "Delete"),
    t("delete_task_confirm_named", `Delete '${task.name}'?`),
  );
  if (!confirmed) return;

  await safeInvoke("delete_task", { id: app.modalTaskId });
  app.selectedId = null;
  els.taskModal.close();
  await refresh();
}

function renderFilterTagChips() {
  els.filterTags.innerHTML = "";
  for (const tag of collectAllTags(app.snapshot?.tasks || [])) {
    const chip = document.createElement("button");
    chip.className = `chip ${app.draftFilter.tags.has(tag) ? "active" : ""}`;
    chip.textContent = tag;
    chip.addEventListener("click", () => {
      if (app.draftFilter.tags.has(tag)) {
        app.draftFilter.tags.delete(tag);
      } else {
        app.draftFilter.tags.add(tag);
      }
      renderFilterTagChips();
    });
    els.filterTags.append(chip);
  }
}

function renderFilterOptionButtons(container, options, selectedSet) {
  container.innerHTML = "";
  for (const option of options) {
    const btn = document.createElement("button");
    btn.type = "button";
    btn.className = `filter-chip ${selectedSet.has(option.value) ? "active" : ""}`;
    btn.textContent = option.label;
    setTip(btn, option.tip || option.label);
    btn.addEventListener("click", () => {
      if (selectedSet.has(option.value)) {
        selectedSet.delete(option.value);
      } else {
        selectedSet.add(option.value);
      }
      renderFilterButtons();
    });
    container.append(btn);
  }
}

function renderFilterButtons() {
  renderFilterOptionButtons(
    els.filterImportanceButtons,
    [
      {
        value: "High",
        label: t("high", "High"),
        tip: t("filter_high_importance", "High importance"),
      },
      {
        value: "Low",
        label: t("low", "Low"),
        tip: t("filter_low_importance", "Low importance"),
      },
      {
        value: "None",
        label: t("none", "None"),
        tip: t("filter_no_importance", "No importance"),
      },
    ],
    app.draftFilter.importance,
  );

  renderFilterOptionButtons(
    els.filterUrgencyButtons,
    [
      {
        value: "High",
        label: t("high", "High"),
        tip: t("filter_high_urgency", "High urgency"),
      },
      {
        value: "Low",
        label: t("low", "Low"),
        tip: t("filter_low_urgency", "Low urgency"),
      },
      {
        value: "None",
        label: t("none", "None"),
        tip: t("filter_no_urgency", "No urgency"),
      },
    ],
    app.draftFilter.urgency,
  );

  renderFilterOptionButtons(
    els.filterStateButtons,
    availableSelectableTaskStates().map((value) => ({
      value,
      label: stateLabel(value),
    })),
    app.draftFilter.state,
  );

  renderFilterOptionButtons(
    els.filterPinnedButtons,
    [
      {
        value: "Pinned",
        label: t("pinned_filter", "Pinned"),
        tip: t("filter_pinned", "Pinned only"),
      },
      {
        value: "Unpinned",
        label: t("unpinned", "Unpinned"),
        tip: t("filter_unpinned", "Unpinned only"),
      },
    ],
    app.draftFilter.pinned,
  );
}

function openFilterModal() {
  app.draftFilter.importance = new Set(app.appliedFilter.importance);
  app.draftFilter.urgency = new Set(app.appliedFilter.urgency);
  app.draftFilter.state = new Set(app.appliedFilter.state);
  app.draftFilter.pinned = new Set(app.appliedFilter.pinned);
  app.draftFilter.tags = new Set(app.appliedFilter.tags);

  renderFilterButtons();
  renderFilterTagChips();

  els.filterModal.showModal();
}

function clearDraftFilters() {
  app.draftFilter.importance = new Set();
  app.draftFilter.urgency = new Set();
  app.draftFilter.state = new Set();
  app.draftFilter.pinned = new Set();
  app.draftFilter.tags = new Set();

  renderFilterButtons();
  renderFilterTagChips();
}

function applyFilters() {
  app.appliedFilter.importance = new Set(app.draftFilter.importance);
  app.appliedFilter.urgency = new Set(app.draftFilter.urgency);
  app.appliedFilter.state = new Set(app.draftFilter.state);
  app.appliedFilter.pinned = new Set(app.draftFilter.pinned);
  app.appliedFilter.tags = new Set(app.draftFilter.tags);

  els.filterModal.close();
  renderTaskList();
}

function openSettingsModal() {
  const settings = app.snapshot?.settings;
  if (!settings) return;

  const themes = app.snapshot.available_themes || [];
  const languages = app.snapshot.available_languages || [];

  renderSelectList(
    els.settingsTheme,
    themes,
    settings.selected_theme,
    (value) => {
      if (value === "dark") return t("theme_dark", "Dark");
      if (value === "light") return t("theme_light", "Light");
      return value;
    },
  );
  renderSelectList(
    els.settingsLanguage,
    languages.map((item) => item.code),
    settings.selected_language,
    (code) => languages.find((item) => item.code === code)?.label || code,
  );
  const optionalStates = new Set(settings.enabled_optional_states || []);
  els.settingsOptionalStates.innerHTML = "";
  for (const value of ["InProgress", "Blocked", "Archived"]) {
    const btn = document.createElement("button");
    btn.type = "button";
    btn.dataset.value = value;
    btn.textContent = `${stateSymbol(value)} ${stateLabel(value)}`;
    btn.classList.toggle("active", optionalStates.has(value));
    btn.addEventListener("click", () => {
      btn.classList.toggle("active");
    });
    els.settingsOptionalStates.append(btn);
  }
  els.settingsAutoCompleteParents.checked =
    settings.auto_complete_parent_tasks !== false;
  els.settingsTaskFontSize.value = String(settings.task_font_size ?? 14);
  applyTaskFontSize(settings.task_font_size ?? 14);
  applyUiScale(settings.ui_scale ?? 1.0);
  els.settingsThemePath.value = "";
  els.settingsDataDir.value = settings.task_data_directory || "";

  els.settingsModal.showModal();
}

async function saveSettings() {
  const enabledOptionalStates = [
    ...els.settingsOptionalStates.querySelectorAll("button.active"),
  ]
    .map((btn) => btn.dataset.value)
    .filter(Boolean);
  const settings = {
    selected_theme:
      getSelectListValue(els.settingsTheme) ||
      app.snapshot?.settings?.selected_theme,
    task_font_size: Number(els.settingsTaskFontSize.value || 14),
    selected_language:
      getSelectListValue(els.settingsLanguage) ||
      app.snapshot?.settings?.selected_language,
    task_sort_mode: app.snapshot?.settings?.task_sort_mode || "Custom",
    enabled_optional_states: enabledOptionalStates,
    auto_complete_parent_tasks: els.settingsAutoCompleteParents.checked,
    task_data_directory:
      (els.settingsDataDir.value || "").trim() ||
      app.snapshot?.settings?.task_data_directory ||
      "",
    ui_scale: Number(els.settingsUiScale.value || 1),
  };

  await safeInvoke("save_gui_settings_cmd", { settings });
  await invokeSetTheme(settings.selected_theme);

  els.settingsModal.close();
  await refresh();
}

async function importTheme() {
  const path = els.settingsThemePath.value.trim();
  if (!path) {
    setError("Theme path is required.");
    return;
  }

  await safeInvoke("import_theme_file_cmd", { path });
  await refresh();
  openSettingsModal();
}

async function loadTaskbarFromCurrentDirectory() {
  await safeInvoke("reload_taskbar_file");
  await refresh();
}
/*
async function deleteAllDataAndExit() {
  const firstConfirm = await askConfirmation(
    t(
      "confirm_delete_data_title",
      "Are you sure you want to delete all application data?",
    ),
    t(
      "confirm_delete_data_hint",
      "This will remove settings, tasks, themes, and cache.",
    ),
  );
  if (!firstConfirm) return;

  const secondConfirm = await askConfirmation(
    t("confirm_delete_data_second_title", "Final confirmation"),
    t("confirm_delete_data_body", "This action cannot be undone."),
    t("confirm_delete_data_btn", "Delete all and exit"),
  );
  if (!secondConfirm) return;

  await safeInvoke("delete_all_data_and_exit");
}
*/
async function deleteAllDataAndExit() {
  console.log("[cleanup][js] entered deleteAllDataAndExit");

  const firstConfirm = await askConfirmation(
    t(
      "confirm_delete_data_title",
      "Are you sure you want to delete all application data?",
    ),
    t(
      "confirm_delete_data_hint",
      "This will remove settings, tasks, themes, and cache.",
    ),
  );
  console.log("[cleanup][js] firstConfirm =", firstConfirm);
  if (!firstConfirm) return;

  const secondConfirm = await askConfirmation(
    t("confirm_delete_data_second_title", "Final confirmation"),
    t("confirm_delete_data_body", "This action cannot be undone."),
    t("confirm_delete_data_btn", "Delete all and exit"),
  );
  console.log("[cleanup][js] secondConfirm =", secondConfirm);
  if (!secondConfirm) return;

  console.log("[cleanup][js] before safeInvoke(delete_all_data_and_exit)");
  try {
    const result = await safeInvoke("delete_all_data_and_exit");
    console.log("[cleanup][js] safeInvoke success:", result);
  } catch (err) {
    console.error("[cleanup][js] safeInvoke failed:", err);
  }
  console.log("[cleanup][js] after safeInvoke(delete_all_data_and_exit)");
}
function createStripe(color, tip = "") {
  const stripe = document.createElement("div");
  stripe.className = "stripe";
  stripe.style.background = color;
  setTip(stripe, tip);
  return stripe;
}

function closeAllStateMenus() {
  document
    .querySelectorAll(".state-wrap.open")
    .forEach((el) => el.classList.remove("open"));
}

function renderTaskRow(task, depth, parentId = 0) {
  const depthLevel = Math.min(depth, 4);
  const li = document.createElement("li");
  li.className = [
    "task-item",
    `task-level-${depthLevel}`,
    app.selectedId === task.id ? "selected" : "",
    task.pinned ? "pinned" : "",
  ]
    .filter(Boolean)
    .join(" ");
  li.dataset.depth = String(depthLevel);
  li.dataset.id = String(task.id);
  li.dataset.parentId = String(parentId);

  const main = document.createElement("div");
  main.className = "task-main";

  const hasChildren = (task.subtasks || []).length > 0;

  const handle = document.createElement("button");
  handle.className = "drag-handle";
  handle.textContent = "☰";
  handle.draggable = true;
  setTip(handle, t("drag_task", "Drag task"));

  const caret = document.createElement("button");
  caret.className = "caret";
  if (hasChildren) {
    caret.textContent = app.collapsed.has(task.id) ? "▸" : "▾";
  } else if (depthLevel > 0) {
    caret.textContent = "▾";
    caret.classList.add("subtask-caret");
  } else {
    caret.textContent = "";
  }
  if (hasChildren) {
    setTip(caret, t("expand_collapse", "Expand / Collapse"));
  }
  caret.addEventListener("click", () => {
    if (!hasChildren) return;
    if (app.collapsed.has(task.id)) {
      app.collapsed.delete(task.id);
    } else {
      app.collapsed.add(task.id);
    }
    renderTaskList();
  });

  const stateWrap = document.createElement("div");
  stateWrap.className = "state-wrap";

  const circle = document.createElement("button");
  circle.className = "circle";
  circle.textContent = stateSymbol(task.state);
  circle.style.color = stateColor(task.state);
  setTip(circle, `${t("state", "State")}: ${stateLabel(task.state)}`);

  const stateMenu = document.createElement("div");
  stateMenu.className = "state-menu";
  const stateCurrent = document.createElement("div");
  stateCurrent.className = "state-current";
  stateCurrent.textContent = stateSymbol(task.state);
  stateCurrent.style.color = stateColor(task.state);
  stateMenu.append(stateCurrent);
  const divider = document.createElement("hr");
  divider.className = "state-divider";
  stateMenu.append(divider);
  const stateItems = availableSelectableTaskStates().map((value) => [
    value,
    stateLabel(value),
  ]);

  for (const [value, label] of stateItems) {
    const item = document.createElement("button");
    item.type = "button";
    item.className = "state-choice";
    item.innerHTML = `<span class="glyph">${stateSymbol(value)}</span><span>${label}</span>`;
    const glyph = item.querySelector(".glyph");
    if (glyph) glyph.style.color = stateColor(value);
    setTip(item, label);
    item.addEventListener("click", async (event) => {
      event.stopPropagation();
      stateWrap.classList.remove("open");
      if (task.state === value) return;
      const cascadeDescendants = shouldConfirmCascade(task, value)
        ? window.confirm(confirmCascadeMessage(value))
        : false;
      await invokeSetTaskStateWithOptions(task.id, value, cascadeDescendants);
      await refresh();
    });
    stateMenu.append(item);
  }

  circle.addEventListener("click", (event) => {
    event.stopPropagation();
    const opening = !stateWrap.classList.contains("open");
    closeAllStateMenus();
    if (opening) stateWrap.classList.add("open");
  });
  stateWrap.append(circle, stateMenu);

  const stripes = document.createElement("div");
  stripes.className = "stripes";
  const palette = app.snapshot?.active_theme;
  if (task.importance === "High") {
    stripes.append(
      createStripe(
        palette?.importance_high_stripe || "#DB4B4B",
        t("stripe_importance_high", "High importance"),
      ),
    );
  } else if (task.importance === "Low") {
    stripes.append(
      createStripe(
        palette?.importance_low_stripe || "#48BF63",
        t("stripe_importance_low", "Low importance"),
      ),
    );
  }
  if (task.urgency === "High") {
    stripes.append(
      createStripe(
        palette?.urgency_high_stripe || "#EA8A2B",
        t("stripe_urgency_high", "High urgency"),
      ),
    );
  } else if (task.urgency === "Low") {
    stripes.append(
      createStripe(
        palette?.urgency_low_stripe || "#6AD3FA",
        t("stripe_urgency_low", "Low urgency"),
      ),
    );
  }

  const content = document.createElement("div");
  content.className = "task-content";
  const meta = document.createElement("div");
  meta.className = "task-meta";

  const title = document.createElement("div");
  title.className = "task-title";
  if (task.state === "Completed" || task.state === "Blocked") {
    title.classList.add("state-crossed");
    title.style.setProperty("--strike-color", stateColor(task.state));
  }
  const nameSpan = document.createElement("span");
  nameSpan.className = "task-name-text";
  nameSpan.textContent = task.name || "Untitled";
  title.append(nameSpan);

  const descPreview = truncateText(task.description, 52);
  if (descPreview) {
    const descSpan = document.createElement("span");
    descSpan.className = "task-desc-preview";
    descSpan.textContent = descPreview;
    title.append(descSpan);
  }

  if (
    task.state !== "Completed" &&
    task.state !== "Blocked" &&
    task.times?.due_date
  ) {
    const dueSpan = document.createElement("span");
    dueSpan.className = "task-due";
    dueSpan.textContent = formatDueShort(task.times.due_date);
    setTip(
      dueSpan,
      `${t("due_none", "Due").split(/[:：]/)[0]}: ${formatDateTime(task.times.due_date)}`,
    );
    title.append(dueSpan);
  }
  title.addEventListener("click", () => {
    app.selectedId = task.id;
    openTaskSummaryModal(task.id);
  });

  const quick = document.createElement("div");
  quick.className = "quick";

  const detailBtn = document.createElement("button");
  detailBtn.textContent = "⚙";
  setTip(detailBtn, t("open_detail", "Open details"));
  detailBtn.addEventListener("click", () => {
    app.selectedId = task.id;
    openTaskModalEdit(task.id);
  });

  const star = document.createElement("button");
  star.textContent = task.pinned ? "★" : "☆";
  setTip(
    star,
    task.pinned ? t("filter_unpinned", "Unpin") : t("filter_pinned", "Pin"),
  );
  star.addEventListener("click", async () => {
    await safeInvoke("toggle_task_pinned", { id: task.id });
    await refresh();
  });

  const plus = document.createElement("button");
  plus.textContent = "+";
  setTip(plus, t("create_root_help", "Create subtask"));
  plus.addEventListener("click", () => {
    app.selectedId = task.id;
    openTaskModalCreate(task.id);
  });

  quick.append(detailBtn, star, plus);
  content.append(title);
  if (stripes.childElementCount > 0) {
    if (stripes.childElementCount === 1) {
      stripes.classList.add("single");
    }
    meta.append(stripes);
  }
  meta.append(quick);
  main.append(handle, caret, stateWrap, content, meta);
  li.append(main);

  if (hasChildren && !app.collapsed.has(task.id)) {
    const sub = document.createElement("ul");
    sub.className = "subtasks";
    for (const child of task.subtasks || []) {
      sub.append(renderTaskRow(child, depth + 1, task.id));
    }
    li.append(sub);
  }

  wireDragForTaskItem(li, handle, task.id);

  return li;
}

function renderTaskList() {
  const roots = filteredRootTasks();
  els.taskList.innerHTML = "";

  if (!roots.length) {
    const empty = document.createElement("li");
    empty.className = "task-item";
    empty.textContent = "No tasks";
    els.taskList.append(empty);
    return;
  }

  for (const task of roots) {
    els.taskList.append(renderTaskRow(task, 0, 0));
  }
}

function clearDragHighlights() {
  document
    .querySelectorAll(
      ".task-item.drop-before, .task-item.drop-after, .task-item.drop-child",
    )
    .forEach((node) => {
      node.classList.remove("drop-before", "drop-after", "drop-child");
    });
  els.taskList.classList.remove("drop-root");
}

function setDragZonesVisible(visible) {
  els.dragZones.style.display = visible ? "flex" : "none";
  els.taskBoard.classList.toggle("drag-mode", visible);
}

function stopAutoScroll() {
  if (app.drag.autoScrollTimer) {
    clearInterval(app.drag.autoScrollTimer);
    app.drag.autoScrollTimer = null;
  }
}

function updateAutoScroll(clientY) {
  const rect = els.taskBoard.getBoundingClientRect();
  const threshold = 44;
  let delta = 0;
  if (clientY < rect.top + threshold) delta = -14;
  if (clientY > rect.bottom - threshold) delta = 14;
  stopAutoScroll();
  if (!delta) return;
  app.drag.autoScrollTimer = setInterval(() => {
    const max = els.taskBoard.scrollHeight - els.taskBoard.clientHeight;
    if (max > 0) {
      els.taskBoard.scrollTop = Math.max(
        0,
        Math.min(max, els.taskBoard.scrollTop + delta),
      );
    } else {
      window.scrollBy(0, delta);
    }
  }, 16);
}

function finishDragState() {
  stopAutoScroll();
  clearDragHighlights();
  setDragZonesVisible(false);
  els.dragDeleteZone.classList.remove("active");
  els.dragCancelZone.classList.remove("active");
  if (app.drag.sourceLi) {
    app.drag.sourceLi.classList.remove("dragging");
    app.drag.sourceLi.style.display = "";
  }
  if (app.drag.placeholder) {
    app.drag.placeholder.remove();
  }
  if (app.drag.ghost) {
    app.drag.ghost.remove();
    app.drag.ghost = null;
  }
  app.drag.active = false;
  app.drag.taskId = null;
  app.drag.sourceLi = null;
  app.drag.placeholder = null;
  app.drag.pointerOffsetX = 0;
  app.drag.pointerOffsetY = 0;
  app.drag.proposal = null;
  app.drag.zone = null;
}

async function commitDrop() {
  if (!app.drag.active || !app.drag.taskId) return;
  try {
    if (app.drag.zone === "delete") {
      await safeInvoke("delete_task", { id: app.drag.taskId });
    } else if (app.drag.zone === "cancel") {
      return;
    } else if (app.drag.proposal) {
      await invokeMoveTask(
        app.drag.taskId,
        app.drag.proposal.targetId,
        app.drag.proposal.relation,
      );
    }
  } finally {
    finishDragState();
    await refresh();
  }
}

function positionDragGhost(clientX, clientY) {
  if (!app.drag.ghost) return;
  const left = clientX - app.drag.pointerOffsetX;
  const top = clientY - app.drag.pointerOffsetY;
  app.drag.ghost.style.left = `${left}px`;
  app.drag.ghost.style.top = `${top}px`;
}

function updateDropProposalFromPointer(clientX, clientY) {
  if (!app.drag.active) return;
  updateAutoScroll(clientY);
  clearDragHighlights();
  app.drag.zone = null;
  app.drag.proposal = null;

  const pointed = document.elementFromPoint(clientX, clientY);
  const zone = pointed?.closest(".drag-zone");
  if (zone) {
    if (zone.id === "drag-delete-zone") app.drag.zone = "delete";
    if (zone.id === "drag-cancel-zone") app.drag.zone = "cancel";
    zone.classList.add("active");
    return;
  }
  els.dragDeleteZone.classList.remove("active");
  els.dragCancelZone.classList.remove("active");

  const targetItem = pointed?.closest(".task-item");
  if (!targetItem) {
    app.drag.proposal = { targetId: 0, relation: "append_root" };
    els.taskList.classList.add("drop-root");
    return;
  }

  if (targetItem === app.drag.sourceLi || targetItem === app.drag.placeholder) {
    return;
  }
  const targetId = Number(targetItem.dataset.id || "0");
  if (!targetId || targetId === app.drag.taskId) return;

  const rect = targetItem.getBoundingClientRect();
  const yRatio = (clientY - rect.top) / Math.max(rect.height, 1);
  let relation = "as_subtask";
  if (yRatio < 0.28) relation = "before";
  else if (yRatio > 0.72) relation = "after";

  app.drag.proposal = { targetId, relation };
  if (relation === "before") targetItem.classList.add("drop-before");
  if (relation === "after") targetItem.classList.add("drop-after");
  if (relation === "as_subtask") targetItem.classList.add("drop-child");
}

function createDragGhostFromTaskItem(li) {
  const ghost = li.cloneNode(true);
  ghost.classList.remove(
    "dragging",
    "selected",
    "drop-before",
    "drop-after",
    "drop-child",
  );
  ghost.classList.add("drag-ghost");
  ghost
    .querySelectorAll(".state-wrap.open, .select-list.open")
    .forEach((node) => {
      node.classList.remove("open");
    });
  return ghost;
}

function wireDragForTaskItem(li, handle, taskId) {
  handle.draggable = false;
  handle.addEventListener("dragstart", (event) => {
    event.preventDefault();
  });
  handle.addEventListener("pointerdown", (event) => {
    if (event.button !== 0) return;
    if (hasActiveDragBlockingFilters()) {
      setError("Dragging is disabled while filters or search are active.");
      window.setTimeout(() => {
        if (hasActiveDragBlockingFilters()) return;
        setError("");
      }, 1800);
      return;
    }
    setError("");
    event.preventDefault();

    const rect = li.getBoundingClientRect();
    app.drag.active = true;
    app.drag.taskId = taskId;
    app.drag.sourceLi = li;
    app.drag.proposal = null;
    app.drag.zone = null;
    app.drag.pointerOffsetX = event.clientX - rect.left;
    app.drag.pointerOffsetY = event.clientY - rect.top;
    li.classList.add("dragging");

    const ghost = createDragGhostFromTaskItem(li);
    ghost.style.width = `${rect.width}px`;
    document.body.append(ghost);
    app.drag.ghost = ghost;
    positionDragGhost(event.clientX, event.clientY);

    const placeholder = document.createElement("li");
    placeholder.className = "task-item drag-placeholder";
    placeholder.style.height = `${rect.height}px`;
    li.parentNode?.insertBefore(placeholder, li.nextSibling);
    app.drag.placeholder = placeholder;
    li.style.display = "none";
    setDragZonesVisible(true);

    const onMove = (moveEvent) => {
      if (!app.drag.active) return;
      moveEvent.preventDefault();
      positionDragGhost(moveEvent.clientX, moveEvent.clientY);
      updateDropProposalFromPointer(moveEvent.clientX, moveEvent.clientY);
    };

    const onUp = async () => {
      document.removeEventListener("pointermove", onMove, true);
      document.removeEventListener("pointerup", onUp, true);
      if (!app.drag.active) return;
      await commitDrop();
    };

    document.addEventListener("pointermove", onMove, true);
    document.addEventListener("pointerup", onUp, true);
  });
}

function setupDatePickerGuard() {
  document.addEventListener("keydown", (event) => {
    if (event.key !== "Escape") return;
    const active = document.activeElement;
    if (
      active &&
      active.tagName === "INPUT" &&
      active.type === "datetime-local"
    ) {
      active.blur();
    }
  });

  document.addEventListener(
    "pointerdown",
    (event) => {
      const active = document.activeElement;
      if (
        active &&
        active.tagName === "INPUT" &&
        active.type === "datetime-local" &&
        event.target !== active
      ) {
        active.blur();
      }
    },
    true,
  );

  document.addEventListener(
    "change",
    (event) => {
      const target = event.target;
      if (
        target &&
        target.tagName === "INPUT" &&
        target.type === "datetime-local"
      ) {
        target.blur();
      }
    },
    true,
  );
}

async function refresh() {
  app.snapshot = await safeInvoke("load_app_state");
  app.strings = app.snapshot?.strings || {};
  applyTheme(app.snapshot.active_theme);
  applyTaskFontSize(app.snapshot?.settings?.task_font_size ?? 14);
  applyUiScale(app.snapshot?.settings?.ui_scale ?? 1.0);
  applyLocalizedText();

  if (!findTaskById(app.snapshot.tasks || [], app.selectedId)) {
    app.selectedId = null;
  }

  renderTaskList();
  renderToolbarSortMode();
  updateSearchClearButton();
  updateUndoButtonState();
}

document.addEventListener("pointerdown", (event) => {
  if (
    !(event.target instanceof Element) ||
    !event.target.closest(".state-wrap")
  ) {
    closeAllStateMenus();
  }
  if (!(event.target instanceof Element)) {
    return;
  }
  if (!event.target.closest(".select-list")) {
    document
      .querySelectorAll(".select-list.open")
      .forEach((node) => node.classList.remove("open"));
  }
});

els.taskBoard.addEventListener("mousemove", (event) => {
  const rect = els.taskBoard.getBoundingClientRect();
  const rightZone = rect.width - 180;
  const x = event.clientX - rect.left;
  els.taskList.classList.toggle("show-quick-all", x >= rightZone);
});

els.taskBoard.addEventListener("mouseleave", () => {
  els.taskList.classList.remove("show-quick-all");
});

els.searchInput.addEventListener("input", () => {
  app.appliedFilter.search = els.searchInput.value;
  renderTaskList();
  updateSearchClearButton();
});

els.searchClearBtn.addEventListener("click", () => {
  app.appliedFilter.search = "";
  els.searchInput.value = "";
  renderTaskList();
  updateSearchClearButton();
  els.searchInput.focus();
});

els.newTaskBtn.addEventListener("click", () => openTaskModalCreate(0));
els.undoBtn.addEventListener("click", async () => {
  if (els.undoBtn.disabled) return;
  await safeInvoke("undo_last_change");
  await refresh();
});
els.settingsBtn.addEventListener("click", openSettingsModal);
els.filterBtn.addEventListener("click", openFilterModal);

els.taskModalClose.addEventListener("click", () => els.taskModal.close());
els.taskModalCancel.addEventListener("click", () => els.taskModal.close());
els.taskModalSave.addEventListener("click", saveTaskFromModal);
els.taskModalDelete.addEventListener("click", deleteTaskFromModal);
els.summaryCloseBtn.addEventListener("click", () => els.summaryModal.close());
els.summaryCloseFooterBtn.addEventListener("click", () =>
  els.summaryModal.close(),
);
els.summaryOpenDetailBtn.addEventListener("click", () => {
  if (app.summaryTaskId == null) return;
  const id = app.summaryTaskId;
  els.summaryModal.close();
  openTaskModalEdit(id);
});
els.summaryModal.addEventListener("close", () => {
  app.summaryTaskId = null;
});

els.taskFormTagAdd.addEventListener("click", () => {
  addModalTag(els.taskFormTagInput.value);
  els.taskFormTagInput.value = "";
  els.taskFormTagInput.focus();
});

els.taskFormTagInput.addEventListener("keydown", (event) => {
  if (event.isComposing) {
    return;
  }
  if (event.key === "Enter") {
    event.preventDefault();
    addModalTag(els.taskFormTagInput.value);
    els.taskFormTagInput.value = "";
  }
});

els.taskFormDueOpen.addEventListener("click", () => openTimeModal("due"));
els.taskFormDueClear.addEventListener("click", () => {
  els.taskFormDue.value = "";
  updateTimeButtons();
});
els.taskFormCompletedOpen.addEventListener("click", () =>
  openTimeModal("completed"),
);
els.taskFormCompletedClear.addEventListener("click", () => {
  els.taskFormCompleted.value = "";
  updateTimeButtons();
});
els.taskFormRecurringToggle.addEventListener("click", () => {
  app.recurrenceEnabled = !app.recurrenceEnabled;
  if (!app.recurrenceDraft) app.recurrenceDraft = defaultRecurrenceDraft();
  if (app.recurrenceEnabled) {
    applyDueTimeToRecurrenceDraft();
  }
  renderRecurrenceControls();
});
els.taskFormRecurringCustomBtn.addEventListener("click", () => {
  if (!app.recurrenceDraft) app.recurrenceDraft = defaultRecurrenceDraft();
  app.recurrenceDraft.frequency =
    getSelectListValue(els.taskFormRecurringFrequency) || "Custom";
  openRecurrenceModal();
});

els.timeSuggestNow.addEventListener("click", () => applySuggestion("now"));
els.timeSuggestTonight.addEventListener("click", () =>
  applySuggestion("tonight"),
);
els.timeSuggestPlus15.addEventListener("click", () =>
  applySuggestion("plus15"),
);
els.timeSuggestTomorrow.addEventListener("click", () =>
  applySuggestion("tomorrow"),
);
els.timeModalClose.addEventListener("click", () => els.timeModal.close());
els.timeModalCancel.addEventListener("click", () => els.timeModal.close());
els.timeModalSave.addEventListener("click", () => {
  if (!app.timeEditingField) {
    els.timeModal.close();
    return;
  }
  const selected = selectedDateFromTimeModal();
  const iso = Number.isNaN(selected.getTime()) ? "" : selected.toISOString();
  if (app.timeEditingField === "due") {
    els.taskFormDue.value = iso;
  } else {
    els.taskFormCompleted.value = iso;
  }
  updateTimeButtons();
  els.timeModal.close();
  app.timeEditingField = null;
});
els.timeModal.addEventListener("close", () => {
  app.timeEditingField = null;
});

els.recurrenceModalClose.addEventListener("click", () =>
  els.recurrenceModal.close(),
);
els.recurrenceModalCancel.addEventListener("click", () =>
  els.recurrenceModal.close(),
);
els.recurrenceModalSave.addEventListener("click", () => {
  if (!app.recurrenceDraft) app.recurrenceDraft = defaultRecurrenceDraft();
  const mode = getSegmentedValue(els.recurrenceEndMode) || "Never";
  app.recurrenceDraft.custom = {
    every: Math.max(1, Number(els.recurrenceEvery.value || 1)),
    unit: getSelectListValue(els.recurrenceUnit) || "Day",
    end: {
      mode,
      on_date: els.recurrenceEndDate.value || "",
      after_count: Math.max(1, Number(els.recurrenceEndCount.value || 1)),
    },
  };
  els.recurrenceModal.close();
});

els.settingsModalClose.addEventListener("click", () =>
  els.settingsModal.close(),
);
els.settingsSaveBtn.addEventListener("click", saveSettings);
els.settingsImportBtn.addEventListener("click", importTheme);
els.settingsLoadTaskbarBtn.addEventListener(
  "click",
  loadTaskbarFromCurrentDirectory,
);

els.settingsDeleteDataBtn.addEventListener("click", deleteAllDataAndExit);

els.settingsTaskFontSize.addEventListener("input", () => {
  applyTaskFontSize(els.settingsTaskFontSize.value);
});
els.settingsUiScale.addEventListener("input", () => {
  applyUiScale(els.settingsUiScale.value);
});

els.confirmCancelBtn.addEventListener("click", () => els.confirmModal.close());
els.errorCloseBtn.addEventListener("click", () => setError(""));

els.filterModalClose.addEventListener("click", () => els.filterModal.close());
els.filterClearBtn.addEventListener("click", clearDraftFilters);
els.filterApplyBtn.addEventListener("click", applyFilters);

wireSegmentedGroup(els.taskFormState);
wireSegmentedGroup(els.taskFormUrgency);
wireSegmentedGroup(els.taskFormImportance);
wireSegmentedGroup(els.recurrenceEndMode);
for (const btn of els.recurrenceEndMode.querySelectorAll(
  "button[data-value]",
)) {
  btn.addEventListener("click", syncRecurrenceEndInputs);
}
els.taskFormPinned.addEventListener("click", () => {
  setPinnedValue(!getPinnedValue());
});

setupHoverTips();
setupDatePickerGuard();
refresh();
