// Utility functions for formatting, localization, and UI helpers

function valueOrNull(value) {
  return value === "" ? null : value;
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

function importanceValueLabel(value) {
  if (value === "High") return t("high", "High");
  if (value === "Low") return t("low", "Low");
  return t("none", "None");
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
