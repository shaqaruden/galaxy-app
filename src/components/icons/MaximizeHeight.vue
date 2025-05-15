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
    <g id="maximize_height">
      <path :fill="props.background" :fill-opacity="props.backgroundOpacity"
        d="M1920.83,185.114l0,835.145c0,100.829 -81.859,182.688 -182.688,182.688l-1555.46,0c-100.828,0 -182.688,-81.859 -182.688,-182.688l0,-835.145c0,-100.828 81.86,-182.688 182.688,-182.688l1555.46,0c100.829,0 182.688,81.86 182.688,182.688Z" />
      <path :fill="props.foreground"
        d="M1058.33,190.696l0,823.981c0,57.916 -47.02,104.937 -104.936,104.937l-479.71,-0c-57.916,-0 -104.937,-47.021 -104.937,-104.937l-0,-823.981c-0,-57.916 47.021,-104.936 104.937,-104.936l479.71,-0c57.916,-0 104.936,47.02 104.936,104.936Z" />
    </g>
  </svg>
</template>
