// Time and date related utilities

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
