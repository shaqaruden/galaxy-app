<template>
  <v-text-field :model-value="displayValue" variant="outlined" density="compact" :label="label"
    :placeholder="isRecording ? 'Press a key combination...' : 'Record Shortcut'" :readonly="!isRecording" hide-details
    @keydown.prevent="handleKeyDown" @keyup="handleKeyUp" @blur="handleBlur" append-inner-icon="mdi-keyboard"
    @click:append-inner.stop="toggleRecording" ref="inputRef">
    <template v-slot:prepend-inner>
      <slot name="icon"></slot>
    </template>
    <!-- <template v-slot:append-inner>
      <v-btn :icon="isRecording ? 'mdi-check' : 'mdi-keyboard'" :color="isRecording ? 'success' : undefined"
        @click.stop="toggleRecording" :loading="isSaving" :disabled="isSaving"></v-btn>
    </template> -->
  </v-text-field>
</template>

<script setup>
import { ref, computed, onMounted, onUnmounted, nextTick } from 'vue';
import { invoke } from '@tauri-apps/api/core';

const props = defineProps({
  modelValue: {
    type: String,
    default: '',
  },
  label: {
    type: String,
    required: true,
  },
  shortcutId: {
    type: String,
    required: true,
  },
});

const emit = defineEmits(['update:modelValue', 'save']);

const inputRef = ref(null);
const isRecording = ref(false);
const isSaving = ref(false);
const pressedKeys = new Set();

// Format the display value with proper capitalization
const displayValue = computed(() => {
  if (!props.modelValue) return '';
  return props.modelValue
    .split('+')
    .map(part => part.charAt(0).toUpperCase() + part.slice(1).toLowerCase())
    .join('+');
});

const toggleRecording = () => {
  if (isRecording.value) {
    saveShortcut();
  } else {
    startRecording();
  }
};

const startRecording = () => {
  isRecording.value = true;
  pressedKeys.clear();
  // Focus the input when starting recording
  nextTick(() => {
    if (inputRef.value) {
      inputRef.value.focus();
    }
  });
};

const stopRecording = () => {
  if (!isRecording.value) return;
  isRecording.value = false;

  // If we have a value and it's not just a single modifier, save it
  if (props.modelValue && !['Control', 'Alt', 'Shift', 'Command'].includes(props.modelValue)) {
    saveShortcut();
  }
};

const handleBlur = () => {
  if (isRecording.value) {
    stopRecording();
  }
};

const handleKeyDown = (event) => {
  if (!isRecording.value) return;

  event.preventDefault();
  event.stopPropagation();

  // Don't process the Escape key here, let it be handled by the keyup
  if (event.key === 'Escape') {
    return;
  }

  // Get the key combination
  const keys = [];
  if (event.ctrlKey) keys.push('Control');
  if (event.altKey) keys.push('Alt');
  if (event.shiftKey) keys.push('Shift');
  if (event.metaKey) keys.push('Command');

  // Add the main key (if it's not a modifier)
  if (!['Control', 'Alt', 'Shift', 'Meta', 'OS'].includes(event.key)) {
    // Handle special keys
    const key = event.key.length === 1 ? event.key.toUpperCase() : event.key;
    keys.push(key);
  }

  // If we have at least one non-modifier key, update the shortcut
  if (keys.length > 0) {
    const shortcut = [...new Set(keys)].join('+');
    emit('update:modelValue', shortcut);
    pressedKeys.add(shortcut);
  }
};

const handleKeyUp = (event) => {
  if (!isRecording.value) return;

  event.preventDefault();
  event.stopPropagation();

  // Handle Escape key to cancel recording
  if (event.key === 'Escape') {
    isRecording.value = false;
    return;
  }

  // If we have keys pressed and this is a keyup for a non-modifier key, stop recording
  if (pressedKeys.size > 0 && !['Control', 'Alt', 'Shift', 'Meta', 'OS'].includes(event.key)) {
    setTimeout(() => {
      if (isRecording.value) {
        stopRecording();
      }
    }, 50);
  }
};

const saveShortcut = async () => {
  if (!props.modelValue) return;

  isSaving.value = true;

  try {
    await invoke('update_shortcut', {
      shortcutId: props.shortcutId,
      newShortcut: props.modelValue,
    });

    emit('save', { id: props.shortcutId, shortcut: props.modelValue });
  } catch (error) {
    console.error('Failed to update shortcut:', error);
  } finally {
    isSaving.value = false;
  }
};

// Handle click outside to stop recording
const handleClickOutside = (event) => {
  if (isRecording.value && inputRef.value && !inputRef.value.$el.contains(event.target)) {
    stopRecording();
  }
};

onMounted(() => {
  document.addEventListener('mousedown', handleClickOutside);
  document.addEventListener('touchstart', handleClickOutside);
});

onUnmounted(() => {
  document.removeEventListener('mousedown', handleClickOutside);
  document.removeEventListener('touchstart', handleClickOutside);
});
</script>
