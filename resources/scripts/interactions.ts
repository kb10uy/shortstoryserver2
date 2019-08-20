/**
 * ドロップダウンメニューの動作を設定する
 *
 * @export
 */
export function enableDropdownMenu(): void {
  const dropdownSources = document.querySelectorAll('[data-dropdown]');
  for (const dropdownSource of dropdownSources) {
    if (!(dropdownSource instanceof HTMLElement)) continue;
    const targetId = dropdownSource.dataset['dropdown'];
    if (!targetId) continue;

    const target = document.getElementById(targetId);
    if (!target || !(target instanceof HTMLElement)) continue;

    if (target.dataset.dropdownMerge === '') {
      const parent = target.parentNode || document.body;
      parent.removeChild(target);
      dropdownSource.appendChild(target);
      target.style.top = '32px';
      target.style.left = '0px';
    }

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
}

/**
 * チェックボックスの動作を設定する
 *
 * @export
 */
export function enableCheckboxes(): void {
  const checkboxPairs = document.querySelectorAll('.checkbox');
  for (const checkboxPair of checkboxPairs) {
    if (!(checkboxPair instanceof HTMLElement)) continue;
    const label: HTMLLabelElement | null = checkboxPair.querySelector('label');
    const checkbox: HTMLInputElement | null = checkboxPair.querySelector('input[type=checkbox]');
    if (!label || !checkbox) continue;

    if (checkbox.checked) {
      label.classList.add('checked');
    }

    checkboxPair.addEventListener('click', (e): void => {
      checkbox.checked = label.classList.toggle('checked');
      checkbox.focus();
      e.preventDefault();
    });
    checkbox.addEventListener('focus', (): void => {
      label.classList.add('focused');
    });
    checkbox.addEventListener('blur', (): void => {
      label.classList.remove('focused');
    });
  }
}
