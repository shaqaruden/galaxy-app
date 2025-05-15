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
    <g id="move_right_display">
      <path :fill="props.background" :fill-opacity="props.backgroundOpacity"
        d="M1920.83,185.114l0,835.145c0,100.829 -81.859,182.688 -182.688,182.688l-1555.46,0c-100.828,0 -182.688,-81.859 -182.688,-182.688l0,-835.145c0,-100.828 81.86,-182.688 182.688,-182.688l1555.46,0c100.829,0 182.688,81.86 182.688,182.688Z" />
      <path :fill="props.foreground"
        d="M1117.2,638.654l-399.331,-0c-10.125,-0 -18.612,-3.425 -25.461,-10.274c-6.849,-6.849 -10.274,-15.336 -10.274,-25.461c0,-10.125 3.425,-18.611 10.274,-25.461c6.849,-6.849 15.336,-10.273 25.461,-10.273l399.331,-0l-175.098,-175.098c-7.147,-7.147 -10.572,-15.485 -10.274,-25.015c0.298,-9.529 4.02,-17.867 11.167,-25.014c7.147,-6.551 15.485,-9.975 25.014,-10.273c9.529,-0.298 17.867,3.127 25.014,10.273l235.847,235.847c3.573,3.574 6.105,7.445 7.594,11.614c1.488,4.169 2.233,8.636 2.233,13.4c-0,4.765 -0.745,9.232 -2.233,13.401c-1.489,4.169 -4.021,8.04 -7.594,11.613l-235.847,235.847c-6.551,6.551 -14.74,9.827 -24.567,9.827c-9.827,-0 -18.314,-3.276 -25.461,-9.827c-7.147,-7.147 -10.72,-15.634 -10.72,-25.461c-0,-9.827 3.573,-18.314 10.72,-25.461l174.205,-174.204Z" />
    </g>
  </svg>
</template>
