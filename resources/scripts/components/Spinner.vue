<template>
  <div>
  <transition name="fade">
    <svg class="spinner" viewBox="-1.2 -1.2 2.4 2.4" v-show="loading">
      <path d="M 0.71 0.71 a 1 1 45 1 1 0 -1.42" fill="none" stroke="#0af" stroke-width="0.3"/>
    </svg>
  </transition>
  </div>
</template>

<style lang="scss" scoped>
div {
  text-align: center;
  margin: 10px 0;

  .spinner {
    width: 64px;
    height: 64px;
    animation: spinner-rotation 0.5s linear infinite;
  }
}

@keyframes spinner-rotation {
  0% {
    transform: rotateZ(0deg);
  }
  100% {
    transform: rotateZ(360deg);
  }
}

.fade-leave-active {
  transition: opacity 0.3s;
}

.fade-leave-to {
  opacity: 0;
}
</style>

<script lang="ts">
import Vue from 'vue';

export default Vue.extend({
  props: {
    loading: {
      type: Boolean,
      required: true,
    },
  },
});
</script>
