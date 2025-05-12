<script setup>
import { ref, watch } from 'vue';
import { VaInput, VaIcon, VaButton } from "vuestic-ui";

const shortcut = defineModel();
const keys = new Set();
const modifierKeys = new Set(['Control', 'Shift', 'Alt', 'Meta']);
const viewMode = ref(true);

const captureShortcut = (event) => {
    event.preventDefault();
    if (!viewMode.value) {
        let key = event.key;
        if (key === ' ') {
            key = 'Space';
        }
        if (!modifierKeys.has(key)) {
            keys.forEach(existingKey => {
                if (!modifierKeys.has(existingKey)) {
                    keys.delete(existingKey);
                }
            });
        }
        keys.add(key);
        const sortedKeys = Array.from(keys).sort((a, b) => {
            if (modifierKeys.has(a) && !modifierKeys.has(b)) return -1;
            if (!modifierKeys.has(a) && modifierKeys.has(b)) return 1;
            return 0;
        });
        shortcut.value = sortedKeys.join(' + ');
    }
};

const clearShortcut = () => {
    keys.clear();
    shortcut.value = '';
};

const toggleMode = () => {
    viewMode.value = !viewMode.value;
};

const props = defineProps({
    label: {
        type: String,
        default: 'Size Window'
    }
});

const inputColor = ref('primary');

watch(shortcut, (newVal) => {
    if (newVal) {
        inputColor.value = 'warning';
    } else {
        inputColor.value = 'primary';
    }
});

</script>

<template>
    <div style="display: flex; align-items: end;">
        <VaInput v-model="shortcut" :readonly="viewMode" type="text" :label="label" placeholder="Assign Shortcut"
            @keydown="captureShortcut" preset="solid" :color="inputColor" inner-label>
            <template #appendInner>
                <VaIcon name="keyboard" size="large" color="primary" @click="toggleMode" />
            </template>
            <template #append>
                <VaIcon name="clear" size="large" class="px-2" color="primary" @click="clearShortcut" />
            </template>
        </VaInput>
    </div>
</template>