<script setup lang="ts">
import { ref, watch, onMounted, onUnmounted, computed } from 'vue'
import { t } from '@shared/utils/i18n';
import { useNavigationStore } from '@/store/navigation'
import type { PlaylistInfo } from '@shared/domains/playlist/types'
import type { Session } from '@shared/domains/session/types'
import type { Song } from '@shared/domains/song/types'
import { nav, NavigationPath } from '@/utils/navigationTree'
import { formatDuration } from '@shared/utils/format'
import { getSessions } from '@shared/domains/session/endpoints'
import { getSongs } from '@shared/domains/song/endpoints'
import { updatePlaylist, getPlaylist } from '@shared/domains/playlist/endpoints'

const navStore = useNavigationStore()
const uid = computed((): string => navStore.options.uid as string)

const loading = ref<boolean>(true)
const error = ref<string | null>(null)
const info = ref<PlaylistInfo | null>(null)
const saving = ref<boolean>(false)
const saveError = ref<string | null>(null)

// Popup state for adding a session
const showAddPopup = ref<boolean>(false)
const loadingSessions = ref<boolean>(false)
const selectedSession = ref<string>('')
const allSessions = ref<Session[]>([])
const allSongs = ref<Song[]>([])

async function loadSessionsAndSongs(): Promise<void> {
  loadingSessions.value = true
  try {
    const [sessionResult, songResult] = await Promise.all([
      getSessions(),
      getSongs()
    ])
    if (sessionResult?.success) {
      allSessions.value = sessionResult.data?.sessions || []
    }
    if (songResult?.success) {
      allSongs.value = songResult.data?.songs || []
    }
  } catch (e) {
    // TODO: Show error to user if needed
    console.error('Error loading sessions or songs:', e)
  } finally {
    loadingSessions.value = false
  }
}

function getSessionByUid(uid: string): Session | undefined {
  return allSessions.value.find((s: Session) => s.uid === uid)
}
function getSongByUid(uid: string): Song | undefined {
  return allSongs.value.find((s: Song) => s.uid === uid)
}
function getSessionName(uid: string): string {
  return getSessionByUid(uid)?.info.name || uid
}
function getSessionDuration(uid: string): number | null {
  const session = getSessionByUid(uid)
  if (!session) return null
  const song = getSongByUid(session.info.song_uid)
  return song?.info.duration ?? null
}

async function load(): Promise<void> {
  if (!uid.value) return
  loading.value = true
  error.value = null
  try {
    const result = await getPlaylist(uid.value)
    if (result?.success) {
      info.value = result.data?.playlist?.info as PlaylistInfo
    } else {
      error.value = result?.error || t('unknownError')
    }
  } catch (e) {
    error.value = (e as Error).message
  } finally {
    loading.value = false
  }
}

async function save(): Promise<void> {
  if (!info.value || !uid.value) return
  saving.value = true
  saveError.value = null
  try {
    // Compute total duration
    let total = 0
    for (const sessionUid of info.value.sessions) {
      const session = getSessionByUid(sessionUid)
      if (session) {
        const song = getSongByUid(session.info.song_uid)
        if (song && typeof song.info.duration === 'number') {
          total += song.info.duration
        }
      }
    }
    info.value.duration = total
    const payload = {
      uid: uid.value,
      info: JSON.parse(JSON.stringify(info.value))
    }
    const result = await updatePlaylist(payload)
    if (result?.success) {
      navStore.navigateTo(nav.playlists.list as NavigationPath)
    } else {
      saveError.value = result?.error || t('unknownError')
    }
  } catch (e) {
    saveError.value = (e as Error).message
  } finally {
    saving.value = false
  }
}

function openAddPopup(): void {
  showAddPopup.value = true
  loadSessionsAndSongs()
}

function addSession(sessionUid: string): void {
  if (!info.value) return
  if (!sessionUid) return // Prevent adding empty session
  info.value.sessions.push(sessionUid)
  showAddPopup.value = false
  selectedSession.value = ''
}

function removeSession(index: number): void {
  if (!info.value) return
  info.value.sessions.splice(index, 1)
}

// Simple drag and drop state
const dragState = ref<{
  isDragging: boolean
  dragIndex: number | null
  dragOverIndex: number | null
  startY: number
}>({
  isDragging: false,
  dragIndex: null,
  dragOverIndex: null,
  startY: 0,
})

function startDrag(event: MouseEvent, index: number): void {
  dragState.value = {
    isDragging: true,
    dragIndex: index,
    dragOverIndex: null,
    startY: event.clientY,
  }
  
  // Add global mouse event listeners
  document.addEventListener('mousemove', onDragMove)
  document.addEventListener('mouseup', onDragEnd)
  
  // Prevent text selection
  event.preventDefault()
}

function onDragMove(event: MouseEvent): void {
  if (!dragState.value.isDragging || dragState.value.dragIndex === null) return
  
  // Find which session we're hovering over
  const elements = document.querySelectorAll('[data-session-index]')
  let hoverIndex: number | null = null
  
  for (const el of elements) {
    const rect = el.getBoundingClientRect()
    if (event.clientY >= rect.top && event.clientY <= rect.bottom) {
      hoverIndex = parseInt((el as HTMLElement).dataset.sessionIndex || '-1')
      break
    }
  }
  
  dragState.value.dragOverIndex = hoverIndex
}

function onDragEnd(): void {
  if (!dragState.value.isDragging) return
  
  const { dragIndex, dragOverIndex } = dragState.value
  
  // Perform the move if we have valid indices
  if (dragIndex !== null && dragOverIndex !== null && dragIndex !== dragOverIndex && info.value) {
    const sessions = info.value.sessions
    const [movedSession] = sessions.splice(dragIndex, 1)
    sessions.splice(dragOverIndex, 0, movedSession)
  }
  
  // Reset drag state
  dragState.value = {
    isDragging: false,
    dragIndex: null,
    dragOverIndex: null,
    startY: 0,
  }
  
  // Remove global listeners
  document.removeEventListener('mousemove', onDragMove)
  document.removeEventListener('mouseup', onDragEnd)
}

onMounted(() => {
  load()
  loadSessionsAndSongs()
})

onUnmounted(() => {
  // Clean up event listeners if component unmounts during drag
  document.removeEventListener('mousemove', onDragMove)
  document.removeEventListener('mouseup', onDragEnd)
})

watch(uid, () => {
  load()
  loadSessionsAndSongs()
})
</script>

<template>
  <div class="bg-brand-50 rounded-xl p-6 pt-16 shadow-lg relative">
    <div class="absolute top-4 right-4 text-xs text-brand-300 font-mono opacity-70 select-all z-10">
      UID: {{ uid }}
    </div>
    <button @click="navStore.navigateTo(nav.playlists.list as NavigationPath)" class="absolute top-4 left-4 bg-brand-200 hover:bg-brand-300 text-brand-700 rounded-full px-4 py-2 font-bold shadow transition flex items-center gap-2">
      <svg class="w-5 h-5" fill="none" stroke="currentColor" stroke-width="2" viewBox="0 0 24 24"><path d="M15 19l-7-7 7-7"/></svg>
      {{ t('back') }}
    </button>
    <div v-if="loading" class="text-center py-8 text-brand-400">{{ t('loading') }}</div>
    <div v-else-if="error" class="text-center py-8 text-red-500">{{ error }}</div>
    <div v-else-if="info">
      <div class="mb-4">
        <label class="block mb-2 font-bold text-brand-700">{{ t('playlistName') }}
          <input v-model="info.name" class="rounded-full px-4 py-2 border border-brand-200 focus:ring-2 focus:ring-brand-400 outline-none w-full mt-1" />
        </label>
        <label class="flex items-center gap-2 mb-2">
          <input v-model="info.repeat" type="checkbox" class="accent-brand-500" />
          {{ t('repeat') }}
        </label>
      </div>
      <div class="mb-4">
        <div class="font-bold text-brand-700 mb-2">{{ t('sessions') }}</div>
        <ul class="space-y-2">
          <li v-for="(sessionUid, idx) in info.sessions" :key="idx"
              :data-session-index="idx"
              :class="[
                'flex items-center gap-3 bg-brand-100 rounded-lg px-3 py-2 shadow-sm transition-all duration-200',
                dragState.isDragging && dragState.dragIndex === idx ? 'opacity-50 scale-95' : '',
                dragState.dragOverIndex === idx ? 'ring-2 ring-brand-400 bg-brand-200' : '',
                'hover:bg-brand-150'
              ]">
            <!-- Drag handle -->
            <div 
              @mousedown="startDrag($event, idx)"
              class="cursor-move text-brand-400 hover:text-brand-600 p-1 -ml-1 rounded"
              title="Drag to reorder"
            >
              <svg class="w-4 h-4" fill="currentColor" viewBox="0 0 24 24">
                <path d="M9 3h2v2H9V3zm0 4h2v2H9V7zm0 4h2v2H9v-2zm0 4h2v2H9v-2zm0 4h2v2H9v-2zm4-16h2v2h-2V3zm0 4h2v2h-2V7zm0 4h2v2h-2v-2zm0 4h2v2h-2v-2zm0 4h2v2h-2v-2z"/>
              </svg>
            </div>
            
            <span class="font-bold text-brand-400 w-6 text-right">{{ (idx+1).toString().padStart(2, '0') }}</span>
            <span class="flex-1 font-semibold text-brand-700 truncate">{{ getSessionName(sessionUid) }}</span>
            <span class="text-xs text-brand-400 w-12 text-right">{{ formatDuration(getSessionDuration(sessionUid)) }}</span>
            
            <button @click="removeSession(idx)" class="text-red-500 hover:text-red-700 ml-2">{{ t('remove') }}</button>
          </li>
        </ul>
        <button @click="openAddPopup" class="mt-2 bg-brand-200 hover:bg-brand-300 text-brand-700 rounded-full px-4 py-2 font-bold shadow transition">{{ t('addSession') }}</button>
      </div>
      <div v-if="saveError" class="text-center text-red-500 mb-2">{{ saveError }}</div>
      <div class="flex gap-2 mt-4">
        <button @click="save" :disabled="saving" class="btn rounded-full px-6 py-2 shadow bg-brand-500 text-white hover:bg-brand-600 transition font-bold tracking-wide uppercase">{{ t('save') }}</button>
        <button @click="navStore.navigateTo(nav.playlists.list as NavigationPath)" class="btn rounded-full px-6 py-2 shadow bg-brand-200 text-brand-700 hover:bg-brand-300 transition font-bold tracking-wide uppercase">{{ t('cancel') }}</button>
      </div>
    </div>

    <!-- Add session popup -->
    <div v-if="showAddPopup" class="fixed inset-0 bg-black bg-opacity-50 flex items-center justify-center z-50">
      <div class="bg-white rounded-2xl p-6 w-full max-w-md shadow-2xl">
        <h2 class="text-2xl font-bold text-brand-700 mb-4">{{ t('addSession') }}</h2>
        <div class="mb-4">
          <label class="block text-sm font-bold text-brand-700 mb-2">{{ t('selectSession') }}</label>
          <select v-model="selectedSession" class="w-full px-4 py-2 border border-brand-200 rounded-lg focus:ring-2 focus:ring-brand-400 outline-none">
            <option value="">{{ t('selectSession') }}</option>
            <option v-for="session in allSessions" :key="session.uid" :value="session.uid">{{ session.info.name }}</option>
          </select>
        </div>
        <div class="flex justify-end gap-2">
          <button @click="showAddPopup = false" class="px-4 py-2 bg-brand-200 text-brand-700 rounded-lg hover:bg-brand-300 transition">{{ t('cancel') }}</button>
          <button @click="addSession(selectedSession)" class="px-4 py-2 bg-brand-500 text-white rounded-lg hover:bg-brand-600 transition">{{ t('add') }}</button>
        </div>
      </div>
    </div>
  </div>
</template>