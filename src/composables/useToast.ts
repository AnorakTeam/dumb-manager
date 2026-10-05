import { ref } from 'vue';

export type ToastType = 'success' | 'error' | 'info' | 'warning';

export interface ToastItem {
  id: string;
  message: string;
  type: ToastType;
  duration: number;
}

const toasts = ref<ToastItem[]>([]);

function removeToast(id: string) {
  const index = toasts.value.findIndex((t) => t.id === id);
  if (index !== -1) {
    toasts.value.splice(index, 1);
  }
}

function addToast(message: string, type: ToastType = 'info', duration = 3000) {
  const id = Math.random().toString(36).substring(2, 9);
  const item: ToastItem = {
    id,
    message,
    type,
    duration,
  };

  toasts.value.push(item);

  if (duration > 0) {
    setTimeout(() => {
      removeToast(id);
    }, duration);
  }

  return id;
}

export const toast = Object.assign(
  (message: string, type: ToastType = 'info', duration = 3000) => addToast(message, type, duration),
  {
    success: (message: string, duration = 3000) => addToast(message, 'success', duration),
    error: (message: string, duration = 3500) => addToast(message, 'error', duration),
    info: (message: string, duration = 3000) => addToast(message, 'info', duration),
    warning: (message: string, duration = 3500) => addToast(message, 'warning', duration),
    dismiss: (id: string) => removeToast(id),
  }
);

export function useToast() {
  return {
    toasts,
    toast,
    removeToast,
  };
}
