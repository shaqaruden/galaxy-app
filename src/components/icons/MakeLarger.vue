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
      <path
        d="M1920.83,185.101l0,835.145c0,100.828 -81.859,182.688 -182.688,182.688l-1555.46,0c-100.828,0 -182.688,-81.86 -182.688,-182.688l0,-835.145c0,-100.828 81.86,-182.688 182.688,-182.688l1555.46,0c100.829,0 182.688,81.86 182.688,182.688Z"
        :style="{ fill: props.background, fillOpacity: props.backgroundOpacity }" />
      <g :style="{ fill: props.foreground }">
        <path
          d="M1284.38,592.106l0,21.135c0,22.913 -18.603,41.516 -41.516,41.516l-564.885,-0c-22.913,-0 -41.516,-18.603 -41.516,-41.516l0,-21.135c0,-22.913 18.603,-41.516 41.516,-41.516l564.885,0c22.913,0 41.516,18.603 41.516,41.516Z" />
        <path
          d="M970.984,926.632l-21.135,-0c-22.913,-0 -41.516,-18.603 -41.516,-41.516l0,-564.885c0,-22.913 18.603,-41.516 41.516,-41.516l21.135,0c22.913,0 41.516,18.603 41.516,41.516l0,564.885c0,22.913 -18.603,41.516 -41.516,41.516Z" />
      </g>
    </g>
  </svg>
</template>
