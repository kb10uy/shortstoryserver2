<template>
  <section class="post-summary" :class="post.visibility || 'public'">
    <h2>
      <a :href="postLink">{{ post.title }}</a>
      <small v-if="post.visibility === 'hidden'">(非公開)</small>
      <small v-else-if="post.visibility === 'draft'">(下書き)</small>
      <small v-else-if="post.visibility === 'unlisted'">(未収載)</small>
    </h2>
    <p class="summary" v-if="post.description !== null">
      {{ post.description }}
    </p>
    <p class="summary" v-else>(説明はありません)</p>
    <ul class="tags" v-if="post.tags.length > 0">
      <li class="tag" v-for="tag of post.tags" :key="tag.id"><a :href="`/search?query=${encodeURIComponent(tag.name)}&type=tag`">{{ tag.name }}</a></li>
    </ul>
  </section>
</template>

<style lang="scss" scoped>
.unlisted {
  background-color: rgba(200, 255, 216, 0.3);
}

.draft {
  background-color: rgba(255, 255, 200, 0.3);
}

.hidden {
  background-color: rgba(255, 200, 200, 0.3);
}
</style>

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
