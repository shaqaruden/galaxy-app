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
    <g id="bottom_half">
      <path :fill="props.background" :fill-opacity="props.backgroundOpacity"
        d="M1920.83,184.976l0,835.145c0,100.828 -81.859,182.688 -182.688,182.688l-1555.46,-0c-100.828,-0 -182.688,-81.86 -182.688,-182.688l0,-835.145c0,-100.828 81.86,-182.688 182.688,-182.688l1555.46,-0c100.829,-0 182.688,81.86 182.688,182.688Z" />
      <path :fill="props.foreground"
        d="M1741.67,668.53l-0,301.63c-0,36.417 -29.566,65.982 -65.982,65.982l-1443.04,-0c-36.416,-0 -65.981,-29.565 -65.981,-65.982l-0,-301.63c-0,-36.416 29.565,-65.982 65.981,-65.982l1443.04,0c36.416,0 65.982,29.566 65.982,65.982Z" />
    </g>
  </svg>
</template>
