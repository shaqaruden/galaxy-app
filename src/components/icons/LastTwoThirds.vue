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
    <g id="last_two_thirds">
      <path :fill="props.background" :fill-opacity="props.backgroundOpacity"
        d="M1924.6,185.101l-0,835.145c-0,100.828 -81.86,182.688 -182.688,182.688l-1555.46,0c-100.828,0 -182.688,-81.86 -182.688,-182.688l0,-835.145c0,-100.828 81.86,-182.688 182.688,-182.688l1555.46,0c100.828,0 182.688,81.86 182.688,182.688Z" />
      <path :fill="props.foreground"
        d="M1757.93,301.043l-0,603.261c-0,72.833 -59.131,131.963 -131.963,131.963l-796.352,0c-72.832,0 -131.963,-59.13 -131.963,-131.963l0,-603.261c0,-72.832 59.131,-131.963 131.963,-131.963l796.352,-0c72.832,-0 131.963,59.131 131.963,131.963Z" />
    </g>
  </svg>
</template>
