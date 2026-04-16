// State management and UI state helpers

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

function setSegmentedValue(groupEl, value) {
  const selected = value ?? "";
  for (const btn of groupEl.querySelectorAll("button[data-value]")) {
    btn.classList.toggle("active", btn.dataset.value === selected);
  }
}

function getSegmentedValue(groupEl) {
  const active = groupEl.querySelector("button.active");
  return active ? active.dataset.value : "";
}

function getSelectListValue(controlEl) {
  return controlEl.dataset.value || "";
}

function wireSegmentedGroup(groupEl) {
  for (const btn of groupEl.querySelectorAll("button[data-value]")) {
    btn.addEventListener("click", () =>
      setSegmentedValue(groupEl, btn.dataset.value),
    );
  }
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

function findTaskById(tasks, id) {
  if (id == null) return null;
  for (const task of tasks) {
    if (task.id === id) return task;
    const nested = findTaskById(task.subtasks || [], id);
    if (nested) return nested;
  }
  return null;
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
