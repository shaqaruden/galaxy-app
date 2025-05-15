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
    <g id="last_third">
      <path :fill="props.background" :fill-opacity="props.backgroundOpacity"
        d="M1924.6,185.114l-0,835.145c-0,100.829 -81.86,182.688 -182.688,182.688l-1555.46,0c-100.828,0 -182.688,-81.859 -182.688,-182.688l0,-835.145c0,-100.828 81.86,-182.688 182.688,-182.688l1555.46,0c100.828,0 182.688,81.86 182.688,182.688Z" />
      <path :fill="props.foreground"
        d="M1757.93,249.661l-0,706.052c-0,44.466 -36.101,80.568 -80.568,80.568l-368.309,-0c-44.466,-0 -80.567,-36.102 -80.567,-80.568l-0,-706.052c-0,-44.467 36.101,-80.568 80.567,-80.568l368.309,0c44.467,0 80.568,36.101 80.568,80.568Z" />
    </g>
  </svg>
</template>
