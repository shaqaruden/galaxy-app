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
    <g id="make_larger">
      <path :fill="props.background" :fill-opacity="props.backgroundOpacity"
        d="M1920.83,185.101l0,835.145c0,100.828 -81.859,182.688 -182.688,182.688l-1555.46,0c-100.828,0 -182.688,-81.86 -182.688,-182.688l0,-835.145c0,-100.828 81.86,-182.688 182.688,-182.688l1555.46,0c100.829,0 182.688,81.86 182.688,182.688Z" />
      <g>
        <path :fill="props.foreground"
          d="M1284.38,596.053l0,13.241c0,14.355 -11.654,26.01 -26.009,26.01l-595.898,-0c-14.355,-0 -26.01,-11.655 -26.01,-26.01l0,-13.241c0,-14.355 11.655,-26.009 26.01,-26.009l595.898,-0c14.355,-0 26.009,11.654 26.009,26.009Z" />
        <path :fill="props.foreground"
          d="M967.037,926.632l-13.241,-0c-14.355,-0 -26.009,-11.654 -26.009,-26.009l-0,-595.898c-0,-14.355 11.654,-26.01 26.009,-26.01l13.241,0c14.355,0 26.01,11.655 26.01,26.01l-0,595.898c-0,14.355 -11.655,26.009 -26.01,26.009Z" />
      </g>
    </g>
  </svg>
</template>
