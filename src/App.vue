<script setup>
import { ref } from 'vue';
import { getCurrentWindow } from '@tauri-apps/api/window';
import { invoke } from '@tauri-apps/api/core';
import ShortcutInput from './components/ShortcutInput.vue';

// Icons
import MoveLeftDisplay from './components/icons/MoveLeftDisplay.vue';
import MoveRightDisplay from './components/icons/MoveRightDisplay.vue';
import Maximize from './components/icons/Maximize.vue';
import AlmostMaximize from './components/icons/AlmostMaximize.vue';

// Reactive state for shortcuts
const shortcuts = ref({
  moveMonitorLeft: 'Shift+Control+Alt+ArrowLeft',
  moveMonitorRight: 'Shift+Control+Alt+ArrowRight',
  maximizeWindow: 'Control+Alt+Enter',
  almostMaximizeWindow: 'Shift+Control+Alt+Enter',
});

// Handle window close
async function closeWindow() {
  const window = await getCurrentWindow();
  await window.close();
}

// Handle shortcut save
const handleShortcutSave = async ({ id, shortcut }) => {
  console.log(`Shortcut ${id} updated to:`, shortcut);
  try {
    await invoke('update_shortcut', {
      shortcutId: id,
      newShortcut: shortcut,
    });
  } catch (error) {
    console.error('Failed to update shortcut:', error);
  }
};
</script>

<template>
  <v-layout style="height: 25px">
    <v-system-bar style="overflow: none;" class="bg-grey-darken-4" data-tauri-drag-region>
      <v-icon style="user-select: none; pointer-events: none;" class="me-2" icon="mdi-message"></v-icon>
      <span style="user-select: none; pointer-events: none;">Galaxy Window Manager</span>
      <v-spacer></v-spacer>
      <v-btn size="small" icon="mdi-close" variant="text" @click="closeWindow"></v-btn>
    </v-system-bar>
  </v-layout>
  <v-app>
    <v-main>
      <h2 class="px-4">Settings</h2>
      <v-container class="pa-6 pt-2" style="display: flex; flex-direction: column; gap: 16px;">
        <v-row style="gap: 16px;" class="py-2">
          <v-col style="padding: 0; display: flex; flex-direction: column; gap: 8px;">
            <ShortcutInput v-model="shortcuts.moveMonitorLeft" label="Move to Left Monitor"
              shortcut-id="moveMonitorLeft" @save="handleShortcutSave">
              <template #icon>
                <MoveLeftDisplay />
              </template>
            </ShortcutInput>

            <ShortcutInput v-model="shortcuts.moveMonitorRight" label="Move to Right Monitor"
              shortcut-id="moveMonitorRight" @save="handleShortcutSave">
              <template #icon>
                <MoveRightDisplay />
              </template>
            </ShortcutInput>

            <ShortcutInput v-model="shortcuts.maximizeWindow" label="Maximize Window" shortcut-id="maximizeWindow"
              @save="handleShortcutSave">
              <template #icon>
                <Maximize />
              </template>
            </ShortcutInput>

            <ShortcutInput v-model="shortcuts.almostMaximizeWindow" label="Almost Maximize Window"
              shortcut-id="almostMaximizeWindow" @save="handleShortcutSave">
              <template #icon>
                <AlmostMaximize />
              </template>
            </ShortcutInput>
          </v-col>
        </v-row>

        <!-- <v-row style="gap: 16px;" class="py-2">
          <v-col style="padding: 0; display: flex; flex-direction: column; gap: 8px;">
            <v-text-field variant="outlined" density="compact" label="First Third" placeholder="Record Shortcut"
              hide-details>
              <template v-slot:prepend-inner>
                <FirstThird />
              </template>
            </v-text-field>
            <v-text-field variant="outlined" density="compact" label="Center Third" placeholder="Record Shortcut"
              hide-details>
              <template v-slot:prepend-inner>
                <CenterThird />
              </template>
            </v-text-field>
            <v-text-field variant="outlined" density="compact" label="Last Third" placeholder="Record Shortcut"
              hide-details>
              <template v-slot:prepend-inner>
                <LastThird />
              </template>
            </v-text-field>
          </v-col>

          <v-col style="padding: 0; display: flex; flex-direction: column; gap: 8px;">
            <v-text-field variant="outlined" density="compact" label="Left Half" placeholder="Record Shortcut"
              hide-details></v-text-field>
            <v-text-field variant="outlined" density="compact" label="Center Half" placeholder="Record Shortcut"
              hide-details></v-text-field>
            <v-text-field variant="outlined" density="compact" label="Right Half" placeholder="Record Shortcut"
              hide-details></v-text-field>
          </v-col>
        </v-row> -->
      </v-container>
    </v-main>
  </v-app>
</template>
