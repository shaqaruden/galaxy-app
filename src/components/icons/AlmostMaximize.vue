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
    <g id="almost_maximize">
      <path :fill="props.background" :fill-opacity="props.backgroundOpacity"
        d="M1920.83,185.114l0,835.145c0,100.829 -81.859,182.688 -182.688,182.688l-1555.46,0c-100.828,0 -182.688,-81.859 -182.688,-182.688l0,-835.145c0,-100.828 81.86,-182.688 182.688,-182.688l1555.46,0c100.829,0 182.688,81.86 182.688,182.688Z" />
      <path :fill="props.foreground"
        d="M1754.17,301.056l-0,603.261c-0,72.833 -59.131,131.964 -131.964,131.964l-1324.37,-0c-72.832,-0 -131.963,-59.131 -131.963,-131.964l-0,-603.261c-0,-72.832 59.131,-131.963 131.963,-131.963l1324.37,0c72.833,0 131.964,59.131 131.964,131.963Z" />
    </g>
  </svg>
</template>
