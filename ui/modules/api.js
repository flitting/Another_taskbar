// Tauri API and invoke functions

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
