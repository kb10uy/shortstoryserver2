<template>
  <div>
    <input type="text" name="composition">
    <ul class="tags"><li class="tag">うんち</li></ul>
  </div>
</template>

<style lang="scss" scoped>
ul.tags {
  display: flex;
  grid-row: 3;
  grid-column: 1;
  padding: 0;
  margin: 0;
  gap: 10px;
}
</style>

<script lang="ts">
import Vue from 'vue';
export default Vue.extend({});
</script>

