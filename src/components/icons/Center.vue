<script setup>
const props = defineProps({
  size: {
    type: String,
    default: '24px'
  },
  background: {
    type: String,
    default: '#3a77c6'
  },
  foreground: {
    type: String,
    default: '#0058c9'
  },
  backgroundOpacity: {
    type: String,
    default: '0.4'
  }
});
</script>

<template>
  <svg :width="props.size" :height="props.size" viewBox="0 0 1921 1205" version="1.1" xmlns="http://www.w3.org/2000/svg"
    xmlns:xlink="http://www.w3.org/1999/xlink" xml:space="preserve" xmlns:serif="http://www.serif.com/"
    style="fill-rule:evenodd;clip-rule:evenodd;stroke-linejoin:round;stroke-miterlimit:2;">
    <g id="center">
      <path :fill="props.background" :fill-opacity="props.backgroundOpacity"
        d="M1920.83,182.688l0,835.145c0,100.828 -81.859,182.688 -182.688,182.688l-1555.46,-0c-100.828,-0 -182.688,-81.86 -182.688,-182.688l0,-835.145c0,-100.828 81.86,-182.688 182.688,-182.688l1555.46,0c100.829,0 182.688,81.86 182.688,182.688Z" />
      <path :fill="props.foreground"
        d="M1470.83,406.244l0,388.033c0,46.847 -38.034,84.882 -84.882,84.882l-851.87,-0c-46.847,-0 -84.882,-38.035 -84.882,-84.882l0,-388.033c0,-46.847 38.035,-84.882 84.882,-84.882l851.87,0c46.848,0 84.882,38.035 84.882,84.882Z" />
    </g>
  </svg>
</template>
