import { ref, watch, onMounted } from "vue";
import { updateSession, getSession } from "@shared/domains/session/endpoints";
import { Session } from "@shared/domains/session/types";

export function useSessionEditor(sessionUid: string) {
  const loading = ref(true);
  const error = ref<string | null>(null);
  const saving = ref(false);
  //
  const session = ref<Session | null>(null);

  async function load() {
    if (!sessionUid) return;
    loading.value = true;
    error.value = null;
    try {
      const result = await getSession(sessionUid);
      if (result.success && result.data) {
        session.value = result.data.session;
      } else {
        throw new Error(result.error || "Failed to load session");
      }
    } catch (e) {
      error.value = (e as Error).message;
    } finally {
      loading.value = false;
    }
  }

  async function save() {
    if (!session.value) return;
    saving.value = true;
    error.value = null;
    try {
      const result = await updateSession(session.value);
      if (!result.success) {
        throw new Error(result.error || "Failed to save session");
      }
    } catch (e) {
      error.value = (e as Error).message;
    } finally {
      saving.value = false;
    }
  }

  onMounted(load);
  watch(() => sessionUid, load);

  return { loading, error, session, saving, load, save };
}
