<script setup>
import { ref, onMounted } from 'vue';
import { getCurrentWindow } from '@tauri-apps/api/window';
import { invoke } from '@tauri-apps/api/core';
import ShortcutInput from './components/ShortcutInput.vue';
import shortcutsConfig from './shortcuts.json';

// Icons
import MoveLeftDisplay from './components/icons/MoveLeftDisplay.vue';
import MoveRightDisplay from './components/icons/MoveRightDisplay.vue';
import Maximize from './components/icons/Maximize.vue';
import AlmostMaximize from './components/icons/AlmostMaximize.vue';
import MaximizeHeight from './components/icons/MaximizeHeight.vue';
import LeftHalf from './components/icons/LeftHalf.vue';
import RightHalf from './components/icons/RightHalf.vue';
import TopHalf from './components/icons/TopHalf.vue';
import BottomHalf from './components/icons/BottomHalf.vue';
import TopLeft from './components/icons/TopLeft.vue';
import TopRight from './components/icons/TopRight.vue';
import BottomLeft from './components/icons/BottomLeft.vue';
import BottomRight from './components/icons/BottomRight.vue';
import MakeSmaller from './components/icons/MakeSmaller.vue';
import MakeLarger from './components/icons/MakeLarger.vue';
import Center from './components/icons/Center.vue';
import FirstThird from './components/icons/FirstThird.vue';
import CenterThird from './components/icons/CenterThird.vue';
import LastThird from './components/icons/LastThird.vue';
import FirstTwoThirds from './components/icons/FirstTwoThirds.vue';
import LastTwoThirds from './components/icons/LastTwoThirds.vue';

// Reactive state for shortcuts - initialize with values from JSON
const shortcuts = ref({});

// Load shortcuts from JSON config
onMounted(() => {
  const shortcutsFromConfig = {};
  for (const [key, config] of Object.entries(shortcutsConfig.shortcuts)) {
    shortcutsFromConfig[key] = config.defaultShortcut;
  }
  shortcuts.value = shortcutsFromConfig;
});

// Handle window close - hide instead of closing to keep app in tray
async function closeWindow() {
  try {
    console.log('Close button clicked - hiding window');
    const window = await getCurrentWindow();
    await window.hide();
    console.log('Window hidden successfully');
  } catch (error) {
    console.error('Error hiding window:', error);
    // Fallback: try using the invoke command
    try {
      await invoke('toggle_window');
    } catch (invokeError) {
      console.error('Error with toggle_window:', invokeError);
    }
  }
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
          <v-col style="padding: 0; display: flex; flex-direction: column; justify-content: start; gap: 16px;">
            <ShortcutInput v-model="shortcuts.leftHalf" label="Left Half" shortcut-id="leftHalf"
              @save="handleShortcutSave">
              <template #icon>
                <LeftHalf />
              </template>
            </ShortcutInput>

            <ShortcutInput v-model="shortcuts.rightHalf" label="Right Half" shortcut-id="rightHalf" @save="
              handleShortcutSave">
              <template #icon>
                <RightHalf />
              </template>
            </ShortcutInput>

            <ShortcutInput v-model="shortcuts.topHalf" label="Top Half" shortcut-id="topHalf" @save="
              handleShortcutSave">
              <template #icon>
                <TopHalf />
              </template>
            </ShortcutInput>

            <ShortcutInput v-model="shortcuts.bottomHalf" label="Bottom Half" shortcut-id="bottomHalf" @save="
              handleShortcutSave">
              <template #icon>
                <BottomHalf />
              </template>
            </ShortcutInput>

            <v-divider opacity="0" thickness="40" />

            <ShortcutInput v-model="shortcuts.topLeft" label="Top Left" shortcut-id="topLeft" @save="
              handleShortcutSave">
              <template #icon>
                <TopLeft />
              </template>
            </ShortcutInput>

            <ShortcutInput v-model="shortcuts.topRight" label="Top Right" shortcut-id="topRight" @save="
              handleShortcutSave">
              <template #icon>
                <TopRight />
              </template>
            </ShortcutInput>

            <ShortcutInput v-model="shortcuts.bottomLeft" label="Bottom Left" shortcut-id="bottomLeft" @save="
              handleShortcutSave">
              <template #icon>
                <BottomLeft />
              </template>
            </ShortcutInput>

            <ShortcutInput v-model="shortcuts.bottomRight" label="Bottom Right" shortcut-id="bottomRight" @save="
              handleShortcutSave">
              <template #icon>
                <BottomRight />
              </template>
            </ShortcutInput>

          </v-col>

          <v-col style="padding: 0; display: flex; flex-direction: column; gap: 16px;">
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

            <ShortcutInput v-model="shortcuts.maximizeHeight" label="Maximize Height" shortcut-id="maximizeHeight"
              @save="handleShortcutSave">
              <template #icon>
                <MaximizeHeight />
              </template>
            </ShortcutInput>

            <ShortcutInput v-model="shortcuts.makeSmaller" label="Make Smaller" shortcut-id="makeSmaller"
              @save="handleShortcutSave">
              <template #icon>
                <MakeSmaller />
              </template>
            </ShortcutInput>

            <ShortcutInput v-model="shortcuts.makeLarger" label="Make Larger" shortcut-id="makeLarger"
              @save="handleShortcutSave">
              <template #icon>
                <MakeLarger />
              </template>
            </ShortcutInput>

            <ShortcutInput v-model="shortcuts.center" label="Center" shortcut-id="center" @save="handleShortcutSave">
              <template #icon>
                <Center />
              </template>
            </ShortcutInput>


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
          </v-col>
        </v-row>

        <v-divider style="margin: 8px 0 16px;" />

        <v-row>
          <v-col style="padding: 0; display: flex; flex-direction: column; gap: 16px;">
            <ShortcutInput v-model="shortcuts.firstThird" label="First Third" shortcut-id="firstThird"
              @save="handleShortcutSave">
              <template #icon>
                <FirstThird />
              </template>
            </ShortcutInput>

            <ShortcutInput v-model="shortcuts.centerThird" label="Center Third" shortcut-id="centerThird"
              @save="handleShortcutSave">
              <template #icon>
                <CenterThird />
              </template>
            </ShortcutInput>

            <ShortcutInput v-model="shortcuts.lastThird" label="Last Third" shortcut-id="lastThird"
              @save="handleShortcutSave">
              <template #icon>
                <LastThird />
              </template>
            </ShortcutInput>

            <ShortcutInput v-model="shortcuts.firstTwoThirds" label="First Two Thirds" shortcut-id="firstTwoThirds"
              @save="handleShortcutSave">
              <template #icon>
                <FirstTwoThirds />
              </template>
            </ShortcutInput>

            <ShortcutInput v-model="shortcuts.lastTwoThirds" label="Last Two Thirds" shortcut-id="lastTwoThirds"
              @save="handleShortcutSave">
              <template #icon>
                <LastTwoThirds />
              </template>
            </ShortcutInput>
          </v-col>

          <v-col style="padding: 0; display: flex; flex-direction: column; gap: 16px;">

          </v-col>
        </v-row>
      </v-container>
    </v-main>
  </v-app>
</template>
