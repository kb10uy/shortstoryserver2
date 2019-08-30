<template>
<div class="user-profile">
  <div class="avatar-box">
    <div class="avatar-content">
      <img :src="avatar" alt="avatar">
      <h2>{{ name }}</h2>
    </div>
  </div>
  <div class="counts-box">
    <div class="count activated">
      <span id="user-profile-posts" class="number"><slot name="posts-count" /></span>
      <span class="label" for="user-profile-posts">投稿</span>
    </div>
    <div class="count">
      <span id="user-profile-series" class="number"><slot name="series-count" /></span>
      <span class="label" for="user-profile-series">シリーズ</span>
    </div>
    <div class="count">
      <span id="user-profile-bookmarks" class="number"><slot name="bookmarks-count" /></span>
      <span class="label" for="user-profile-bookmarks">ブックマーク</span>
    </div>
  </div>
</div>
</template>

<style lang="scss" scoped>
@import '../../styles/variables';

.user-profile {
  color: $floating-color-fg;

  .avatar-box {
    display: grid;
    grid-template-columns: 1fr minmax(100px, 400px) 1fr;
    grid-template-rows: max-content;
    background-color: $floating-color-bg;
    border: 1px solid $floating-color-br;
    border-radius: 10px 10px 0 0;

    .avatar-content {
      grid-column: 2;
      text-align: center;

      img {
        max-height: 300px;
        border-radius: 50%;
      }

      h2 {
        margin-top: 10px;
        font-size: 1.6rem;
        font-weight: bold;
      }
    }
  }

  .counts-box {
    display: flex;
    background-color: $floating-color-bg;
    border: 1px solid $floating-color-br;
    border-top: 0;
    cursor: pointer;

    .count {
      flex: 1;
      display: grid;
      position: relative;
      grid-template-rows: max-content max-content;
      text-align: center;
      transition: 0.2s background-color;

      &.activated::after {
        position: absolute;
        display: block;
        width: 100%;
        height: 5px;
        bottom: 0;
        background-color: $tag-color-bg;
        content: '';
      }

      span.number {
        grid-row: 1;
        font-size: 1.3rem;
        font-weight: bold;
      }

      span.label {
        grid-row: 2;
        margin-bottom: 5px;
      }
    }
  }
}
</style>

<script lang="ts">
import Vue from 'vue';

interface UserProfileData {
  posts: any[] | false;
  series: any[] | false;
  bookmarks: any[] | false;
}

export default Vue.extend({
  props: {
    name: {
      type: String,
      required: true,
    },
    avatar: {
      type: String,
      required: true,
    },
  },

  data(): UserProfileData {
    return {
      posts: false,
      series: false,
      bookmarks: false,
    };
  },

  methods: {

  },
});
</script>
