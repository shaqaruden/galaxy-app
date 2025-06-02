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
  <svg :width="size" :height="size" viewBox="0 0 1921 1205" xmlns="http://www.w3.org/2000/svg" style="fill-rule:evenodd;clip-rule:evenodd;stroke-linejoin:round;stroke-miterlimit:2;">
    <g id="Top-Right">
      <g id="left_half">
        <path d="M1920.83,182.688l0,835.145c0,100.828 -81.859,182.688 -182.688,182.688l-1555.46,-0c-100.828,-0 -182.688,-81.86 -182.688,-182.688l0,-835.145c0,-100.828 81.86,-182.688 182.688,-182.688l1555.46,0c100.829,0 182.688,81.86 182.688,182.688Z" :style="{ fill: background, fillOpacity: backgroundOpacity }"/>
        <path d="M1754.17,232.648l-0,301.631c-0,36.416 -29.566,65.981 -65.982,65.981l-661.787,0c-36.416,0 -65.981,-29.565 -65.981,-65.981l-0,-301.631c-0,-36.416 29.565,-65.981 65.981,-65.981l661.787,-0c36.416,-0 65.982,29.565 65.982,65.981Z" :fill="foreground"/>
      </g>
    </g>
  </svg>
</template>
