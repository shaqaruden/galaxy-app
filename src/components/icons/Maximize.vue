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
  <svg :width="props.size" :height="props.size" viewBox="0 0 1925 1205" version="1.1" xmlns="http://www.w3.org/2000/svg"
    xmlns:xlink="http://www.w3.org/1999/xlink" xml:space="preserve" xmlns:serif="http://www.serif.com/"
    style="fill-rule:evenodd;clip-rule:evenodd;stroke-linejoin:round;stroke-miterlimit:2;">
    <g id="maximize">
      <path :fill="props.background" :fill-opacity="props.backgroundOpacity"
        d="M1924.6,182.688l-0,835.145c-0,100.828 -81.86,182.688 -182.688,182.688l-1555.46,-0c-100.828,-0 -182.688,-81.86 -182.688,-182.688l0,-835.145c0,-100.828 81.86,-182.688 182.688,-182.688l1555.46,0c100.828,0 182.688,81.86 182.688,182.688Z" />
      <path :fill="props.foreground"
        d="M1841.67,203.2l-0,794.12c-0,66.157 -53.711,119.868 -119.867,119.868l-1514.43,-0c-66.156,-0 -119.867,-53.711 -119.867,-119.868l0,-794.12c0,-66.156 53.711,-119.867 119.867,-119.867l1514.43,0c66.156,0 119.867,53.711 119.867,119.867Z" />
    </g>
  </svg>
</template>
