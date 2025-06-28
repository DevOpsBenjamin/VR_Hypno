<template>
  <div class="w-full h-full">
    <div class="flex items-center justify-between mb-6">
      <div class="flex items-center gap-3">
        <div class="w-12 h-12 bg-brand-200 rounded-lg flex items-center justify-center shadow-md">
          <span class="h-10 items-center text-4xl">{{ BrainEmoji }}</span>
        </div>
        <div>
          <h1 class="text-3xl font-bold text-brand-700">
            {{ t('sessions') }}
          </h1>
          <p class="text-brand-400 text-base font-semibold">{{ sessions.length }} {{ t('sessions') }}</p>
        </div>
      </div>
      <button @click="openAddPopup"
        class="bg-brand-600 hover:bg-brand-700 text-white rounded-lg px-6 py-3 flex items-center gap-2 shadow-sm hover:shadow-md transition-all duration-200 text-base font-medium border border-brand-100"
        :title="t('sessions')">
        <svg class="w-5 h-5" fill="none" stroke="currentColor" stroke-width="2" viewBox="0 0 24 24">
          <path d="M12 4v16m8-8H4" />
        </svg>
        <span>{{ t('sessions') }}</span>
      </button>
    </div>
    <div class="flex-1">
      <div>
        <div v-if="loading" class="flex items-center justify-center py-10">
          <div class="text-center">
            <div class="w-10 h-10 border-4 border-brand-200 border-t-brand-500 rounded-full animate-spin mx-auto mb-4"></div>
            <p class="text-brand-400 font-bold">{{ t('loading') }}</p>
          </div>
        </div>
        <div v-else-if="error" class="bg-brand-50 border-2 border-brand-200 rounded-2xl p-6 text-center shadow-xl">
          <svg class="w-10 h-10 text-brand-400 mx-auto mb-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 8v4m0 4h.01M21 12a9 9 0 11-18 0 9 9 0 0118 0z" />
          </svg>
          <p class="text-brand-700 font-bold mb-4">{{ error }}</p>
          <button @click="loadSessions" class="bg-brand-500 hover:bg-brand-600 text-white rounded-lg px-4 py-2 flex items-center gap-2 mx-auto transition-colors">
            <svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
              <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M4 4v5h.582m15.356 2A8.001 8.001 0 004.582 9m0 0H9m11 11v-5h-.581m0 0a8.003 8.003 0 01-15.357-2m15.357 2H15" />
            </svg>
            {{ t('retry') }}
          </button>
        </div>
        <div v-else>          
          <!-- État vide -->
          <div v-if="sessions.length === 0" class="text-center py-10">
            <div class="w-20 h-20 bg-gradient-to-br from-brand-100 to-brand-300 rounded-full flex items-center justify-center mx-auto mb-4 shadow-xl">
              <span v-html="MusicIcon" class="text-pink-500 w-16 h-16 mx-auto block"></span>
            </div>
            <h3 class="text-xl font-bold text-brand-700 mb-2">{{ t('noSessions') }}</h3>
            <p class="text-brand-400 mb-6">Créez votre première playlist pour commencer</p>
          </div>
          <!-- Grille des sessions -->
          <div v-else class="grid gap-6 md:grid-cols-2 lg:grid-cols-3 xl:grid-cols-4">
            <div v-for="session in sessions" :key="session.uid" class="bg-brand-50 rounded-2xl shadow-lg px-6 py-5 mb-4 flex items-center justify-between border-2 border-brand-200">
              <div class="flex flex-col flex-1 min-w-0">
                <span class="block font-extrabold text-lg text-brand-700 whitespace-normal break-words min-w-[120px]">{{ session.info.name }}</span>
              </div>
              <div class="flex gap-2">
                <button @click="() =>openEditor(session.uid)" class="bg-brand-200 hover:bg-brand-300 text-brand-700 rounded-full p-2 transition shadow" :title="t('edit')">
                  <span class="text-3xl">{{ PenEmoji }}</span>
                </button>
                <button @click="() => deleteSessionUI(session.uid)" class="bg-red-200 hover:bg-red-300 text-red-700 rounded-full p-2 transition shadow" :title="t('delete')">
                  <span class="text-3xl">{{ TrashEmoji }}</span>
                </button>
              </div>
            </div>
          </div>
        </div>
      </div>
    </div>
    <div v-if="showAddPopup" class="fixed inset-0 bg-black bg-opacity-50 flex items-center justify-center z-50">
      <div class="bg-white rounded-2xl p-6 w-full max-w-md shadow-2xl">
        <h2 class="text-2xl font-bold text-brand-700 mb-4">Add Session</h2>
        <div class="mb-4">
          <label class="block text-sm font-bold text-brand-700 mb-2">Session Name</label>
          <input v-model="newSessionName" class="w-full px-4 py-2 border border-brand-200 rounded-lg focus:ring-2 focus:ring-brand-400 outline-none" />
        </div>
        <div class="mb-4">
          <label class="block text-sm font-bold text-brand-700 mb-2">Select Song</label>
          <select v-model="newSessionSongUid" class="w-full px-4 py-2 border border-brand-200 rounded-lg focus:ring-2 focus:ring-brand-400 outline-none">
            <option value="">Select a song</option>
            <option v-for="song in songs" :key="song.uid" :value="song.uid">{{ song.info.name }}</option>
          </select>
        </div>
        <div class="flex justify-end gap-2">
          <button @click="showAddPopup = false" class="px-4 py-2 bg-brand-200 text-brand-700 rounded-lg hover:bg-brand-300 transition">Cancel</button>
          <button @click="addSession" class="px-4 py-2 bg-brand-500 text-white rounded-lg hover:bg-brand-600 transition">Create</button>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted } from 'vue';
import { t } from '@shared/utils/i18n';
import { useNavigationStore } from '@/store/navigation'
import { nav, NavigationPath, SessionEditOption } from '@/utils/navigationTree'
import { getSongs } from '@shared/domains/song/endpoints';
import { getSessions, createSession, deleteSession } from '@shared/domains/session/endpoints';
import type { Session } from '@shared/domains/session/types';
import type { Song } from '@shared/domains/song/types';
import { confirmDialog } from "@shared/utils/confirmDialog";
import { MusicIcon } from '@shared/icons/svg'
import { PenEmoji, TrashEmoji, BrainEmoji } from '@shared/icons/emoji';

const sessions = ref<Session[]>([]);
const loading = ref(true);
const error = ref<string | null>(null);
const navStore = useNavigationStore()

// Popup state
const showAddPopup = ref(false);
const newSessionName = ref('');
const newSessionSongUid = ref('');
const songs = ref<Song[]>([]);
const loadingSongs = ref(false);

async function loadSongs() {
  loadingSongs.value = true;
  try {
    const result = await getSongs();
    if (result?.success) {
      songs.value = result.data?.songs || [];
    }
  } finally {
    loadingSongs.value = false;
  }
}

async function openAddPopup() {
  showAddPopup.value = true;
  loadSongs();
}

async function loadSessions() {
  loading.value = true;
  error.value = null;
  try {
    const result = await getSessions();
    console.log('getSessions result:', result);
    if (result?.success) {
      sessions.value = result.data?.sessions || [];
    } else {
      error.value = result?.error || t('unknownError');
    }
  } catch (e: any) {
    error.value = e.message || t('unknownError');
  } finally {
    loading.value = false;
  }
}

async function addSession() {
  if (!newSessionName.value.trim() || !newSessionSongUid.value) return;
  try {
    const result = await createSession({
      info: {
        name: newSessionName.value.trim(),
        song_uid: newSessionSongUid.value,
      },
    });
    if (result?.success) {
      showAddPopup.value = false;
      newSessionName.value = '';
      newSessionSongUid.value = '';
      await loadSessions();
    } else {
      error.value = result?.error || t('unknownError');
    }
  } catch (e: any) {
    error.value = e.message || t('unknownError');
  }
}

function openEditor(uid: string) {
  navStore.navigateTo(nav.sessions.edit as NavigationPath, { uid } as SessionEditOption)
}

async function deleteSessionUI(uid: string) {
  
  const res = await confirmDialog(t('confirmDeleteSession'), t('confirmTitle'));
  if (!res) return;
  try {
    const result = await deleteSession(uid);
    if (result?.success) {
      await loadSessions();
    } else {
      error.value = result?.error || t('unknownError');
    }
  } catch (e: any) {
    error.value = e.message || t('unknownError');
  }
}

onMounted(() => {
  loadSessions();
});
</script>