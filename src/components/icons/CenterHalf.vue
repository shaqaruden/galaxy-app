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
    <g id="center_half">
      <path :fill="props.background" :fill-opacity="props.backgroundOpacity"
        d="M1920.83,185.114l0,835.145c0,100.829 -81.859,182.688 -182.688,182.688l-1555.46,0c-100.828,0 -182.688,-81.859 -182.688,-182.688l0,-835.145c0,-100.828 81.86,-182.688 182.688,-182.688l1555.46,0c100.829,0 182.688,81.86 182.688,182.688Z" />
      <path :fill="props.foreground"
        d="M1356.49,289.881l-0,625.612c-0,66.664 -54.123,120.788 -120.788,120.788l-552.174,-0c-66.665,-0 -120.788,-54.124 -120.788,-120.788l-0,-625.612c-0,-66.665 54.123,-120.788 120.788,-120.788l552.174,0c66.665,0 120.788,54.123 120.788,120.788Z" />
    </g>
  </svg>
</template>
