<template>
  <section class="post-summary">
    <h2><a :href="postLink">{{ post.title }}</a></h2>
    <p class="summary" v-if="post.description !== null">
      {{ post.description }}
    </p>
    <p class="summary" v-else>(説明はありません)</p>
  </section>
</template>

<script lang="ts">
import Vue from 'vue';

export default Vue.extend({
  props: {
    post: {
      type: Object,
      required: true,
    },
  },

  computed: {
    postLink(): string {
      return `/posts/${this.post.id}`;
    },
  },
});
</script>
