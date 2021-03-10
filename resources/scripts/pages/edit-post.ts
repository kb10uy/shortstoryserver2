import Vue from 'vue';
import TagEditor from './../components/TagEditor.vue';

const app = new Vue({
  components: {
    TagEditor,
  },
});

app.$mount('#app');

window.addEventListener('submit', (e) => {
  const form = e.target as HTMLFormElement;

  e.preventDefault();
  form.submit();
});
