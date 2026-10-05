import { ref } from 'vue';

export interface ConfirmOptions {
  title?: string;
  message: string;
  confirmText?: string;
  cancelText?: string;
  type?: 'danger' | 'warning' | 'info';
}

const isOpen = ref(false);
const options = ref<ConfirmOptions>({
  title: '¿Confirmar acción?',
  message: '',
  confirmText: 'Confirmar',
  cancelText: 'Cancelar',
  type: 'danger',
});

let resolvePromise: ((value: boolean) => void) | null = null;

export function useConfirm() {
  function confirm(opts: ConfirmOptions | string): Promise<boolean> {
    if (typeof opts === 'string') {
      options.value = {
        title: '¿Confirmar acción?',
        message: opts,
        confirmText: 'Eliminar',
        cancelText: 'Cancelar',
        type: 'danger',
      };
    } else {
      options.value = {
        title: opts.title ?? '¿Confirmar acción?',
        message: opts.message,
        confirmText: opts.confirmText ?? 'Confirmar',
        cancelText: opts.cancelText ?? 'Cancelar',
        type: opts.type ?? 'danger',
      };
    }

    isOpen.value = true;

    return new Promise<boolean>((resolve) => {
      resolvePromise = resolve;
    });
  }

  function handleConfirm() {
    isOpen.value = false;
    if (resolvePromise) {
      resolvePromise(true);
      resolvePromise = null;
    }
  }

  function handleCancel() {
    isOpen.value = false;
    if (resolvePromise) {
      resolvePromise(false);
      resolvePromise = null;
    }
  }

  return {
    isOpen,
    options,
    confirm,
    handleConfirm,
    handleCancel,
  };
}
