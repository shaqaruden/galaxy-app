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
    <g id="move_left_display">
      <path :fill="props.background" :fill-opacity="props.backgroundOpacity"
        d="M1924.6,185.114l-0,835.145c-0,100.829 -81.86,182.688 -182.688,182.688l-1555.46,0c-100.828,0 -182.688,-81.859 -182.688,-182.688l0,-835.145c0,-100.828 81.86,-182.688 182.688,-182.688l1555.46,0c100.828,0 182.688,81.86 182.688,182.688Z" />
      <path :fill="props.foreground"
        d="M807.398,566.72l399.332,0c10.125,0 18.611,3.425 25.461,10.274c6.849,6.849 10.273,15.336 10.273,25.461c0,10.124 -3.424,18.611 -10.273,25.46c-6.85,6.849 -15.336,10.274 -25.461,10.274l-399.332,-0l175.099,175.098c7.147,7.147 10.571,15.485 10.273,25.014c-0.297,9.529 -4.02,17.868 -11.167,25.014c-7.146,6.552 -15.484,9.976 -25.014,10.274c-9.529,0.298 -17.867,-3.127 -25.014,-10.274l-235.846,-235.846c-3.574,-3.574 -6.105,-7.445 -7.594,-11.614c-1.489,-4.169 -2.233,-8.636 -2.233,-13.4c-0,-4.765 0.744,-9.232 2.233,-13.401c1.489,-4.169 4.02,-8.04 7.594,-11.613l235.846,-235.847c6.552,-6.551 14.741,-9.827 24.568,-9.827c9.827,-0 18.314,3.276 25.46,9.827c7.147,7.147 10.721,15.634 10.721,25.461c-0,9.827 -3.574,18.313 -10.721,25.46l-174.205,174.205Z" />
    </g>
  </svg>
</template>
