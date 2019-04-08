import './bootstrap';

const dropdownSources = document.querySelectorAll('[data-dropdown]');
for (const dropdownSource of dropdownSources) {
  if (!(dropdownSource instanceof HTMLElement)) continue;
  const targetId = dropdownSource.dataset['dropdown'];
  if (!targetId) continue;

  const target = document.getElementById(targetId);
  if (!target || !(target instanceof HTMLElement)) continue;
  dropdownSource.addEventListener('click', (e): void => {
    target.classList.toggle('show');
    e.stopImmediatePropagation();
  });
  window.addEventListener('click', (e): void => {
    if (!(e.target instanceof HTMLElement)) return;
    if (!target.contains(e.target)) {
      target.classList.remove('show');
    }
  });
}
