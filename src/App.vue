<script setup>
import { getCurrentWindow } from '@tauri-apps/api/window';
import LeftHalf from './components/icons/LeftHalf.vue';
import RightHalf from './components/icons/RightHalf.vue';
import CenterHalf from './components/icons/CenterHalf.vue';
import TopHalf from './components/icons/TopHalf.vue';
import BottomHalf from './components/icons/BottomHalf.vue';
import FirstThird from './components/icons/FirstThird.vue';
import FirstTwoThirds from './components/icons/FirstTwoThirds.vue';
import LastThird from './components/icons/LastThird.vue';
import LastTwoThirds from './components/icons/LastTwoThirds.vue';
import CenterThird from './components/icons/CenterThird.vue';
import MoveLeftDisplay from './components/icons/MoveLeftDisplay.vue';
import MoveRightDisplay from './components/icons/MoveRightDisplay.vue';
import Maximize from './components/icons/Maximize.vue';
import AlmostMaximize from './components/icons/AlmostMaximize.vue';
import MaximizeHeight from './components/icons/MaximizeHeight.vue';
import MakeSmaller from './components/icons/MakeSmaller.vue';
import MakeLarger from './components/icons/MakeLarger.vue';

async function closeWindow() {
  const window = await getCurrentWindow();
  await window.close();
}
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
            <v-text-field variant="outlined" density="compact" label="Left Half" placeholder="Record Shortcut"
              hide-details>
              <template v-slot:prepend-inner>
                <LeftHalf />
              </template>
            </v-text-field>
            <v-text-field variant="outlined" density="compact" label="Right Half" placeholder="Record Shortcut"
              hide-details>
              <template v-slot:prepend-inner>
                <RightHalf />
              </template>
            </v-text-field>
            <v-text-field variant="outlined" density="compact" label="Center Half" placeholder="Record Shortcut"
              hide-details>
              <template v-slot:prepend-inner>
                <CenterHalf />
              </template>
            </v-text-field>
            <v-text-field variant="outlined" density="compact" label="Top Half" placeholder="Record Shortcut"
              hide-details>
              <template v-slot:prepend-inner>
                <TopHalf />
              </template>
            </v-text-field>
            <v-text-field variant="outlined" density="compact" label="Bottom Half" placeholder="Record Shortcut"
              hide-details>
              <template v-slot:prepend-inner>
                <BottomHalf />
              </template>
            </v-text-field>
          </v-col>

          <v-col style="padding: 0; display: flex; flex-direction: column; gap: 8px;">
            <v-text-field variant="outlined" density="compact" label="Maximize" placeholder="Record Shortcut"
              hide-details>
              <template v-slot:prepend-inner>
                <Maximize />
              </template>
            </v-text-field>
            <v-text-field variant="outlined" density="compact" label="Almost Maximize" placeholder="Record Shortcut"
              hide-details>
              <template v-slot:prepend-inner>
                <AlmostMaximize />
              </template>
            </v-text-field>
            <v-text-field variant="outlined" density="compact" label="Maximize Height" placeholder="Record Shortcut"
              hide-details>
              <template v-slot:prepend-inner>
                <MaximizeHeight />
              </template>
            </v-text-field>
            <v-text-field variant="outlined" density="compact" label="Make Smaller" placeholder="Record Shortcut"
              hide-details>
              <template v-slot:prepend-inner>
                <MakeSmaller />
              </template>
            </v-text-field>
            <v-text-field variant="outlined" density="compact" label="Make Larger" placeholder="Record Shortcut"
              hide-details>
              <template v-slot:prepend-inner>
                <MakeLarger />
              </template>
            </v-text-field>
          </v-col>
        </v-row>

        <v-row style="gap: 16px;" class="py-2">
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
        </v-row>
      </v-container>
    </v-main>
  </v-app>
</template>
