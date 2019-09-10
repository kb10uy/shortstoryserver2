<template>
<div class="cover">
  <div class="dialog">
    <div class="top">
      <h1><slot name="label"></slot></h1>
      <i class="fas fa-times" @click="onClosed"></i>
    </div>
    <slot></slot>
    <hr>
    <div class="buttons">
      <button class="warning button" @click="onClosed">キャンセル</button>
      <button class="button" @click="onOk" v-if="buttonType === 'ok-cancel'">&nbsp;&nbsp;O K&nbsp;&nbsp;</button>
      <button class="button" @click="onOk" v-if="buttonType === 'submit-cancel'">送信</button>
    </div>
  </div>
</div>
</template>

<style lang="scss" scoped>
@import '../../styles/variables';

.cover {
  position: fixed;
  width: 100%;
  height: 100%;
  top: 0;
  left: 0;
  display: grid;
  grid-template-columns: 1fr max-content 1fr;
  grid-template-rows: 1fr max-content 1fr;
  background-color: rgba(0, 0, 0, 0.3);
  z-index: 100;

  .dialog {
    grid-column: 2;
    grid-row: 2;
    margin: 10px;
    padding: 10px;
    min-width: 200px;
    max-width: 600px;
    background-color: $floating-color-bg;
    border: 1px solid $floating-color-br;
    border-radius: 10px;
    z-index: 101;

    .top {
      display: flex;
      flex-direction: row;
      align-items: center;

      & > h1 {
        flex: 1;
      }

      & > i {
        color: $floating-color-fg;
        font-size: 1.5rem;
        margin-left: 10px;
      }
    }

    .buttons {
      text-align: right;
    }
  }
}
</style>

<script lang="ts">
import Vue from 'vue';

export default Vue.extend({
  props: {
    buttonType: {
      type: String,
      required: false,
      default: 'ok-cancel',
    },
  },

  methods: {
    onClosed() {
      this.$emit('dialog-closed');
    },

    onOk() {
      this.$emit('dialog-ok');
    },
  },
});
</script>
