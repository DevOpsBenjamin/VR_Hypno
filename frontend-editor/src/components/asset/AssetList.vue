<template>
  <div class="w-full h-full">
    <div class="flex items-center justify-between mb-6">
      <div class="flex items-center gap-3">
        <div class="w-12 h-12 bg-brand-200 rounded-lg flex items-center justify-center shadow-md">
          <span class="text-3xl">{{ DiamondEmoji }}</span>
        </div>
        <div>
          <h1 class="text-3xl font-bold text-brand-700">{{ t('assets') }}</h1>
          <p class="text-brand-400 text-base font-semibold">{{ assets.length }} {{ t('assets') }}</p>
        </div>
      </div>
      <button
        @click="addAssetUI"
        class="bg-brand-600 hover:bg-brand-700 text-white rounded-lg px-6 py-3 flex items-center gap-2 shadow-sm hover:shadow-md transition-all duration-200 text-base font-medium border border-brand-100"
        :title="t('addAsset')"
      >
        <svg class="w-5 h-5" fill="none" stroke="currentColor" stroke-width="2" viewBox="0 0 24 24"><path d="M12 4v16m8-8H4"/></svg>
        <span>{{ t('addAsset') }}</span>
      </button>
    </div>
    <div class="mb-6">
      <div v-if="allTags.length" class="flex flex-wrap items-center gap-2 mb-2">
        <span class="font-bold text-brand-700">Tags:</span>
        <button
          v-for="tag in allTags"
          :key="tag"
          @click="selectedTags = selectedTags.includes(tag) ? selectedTags.filter(t => t !== tag) : [...selectedTags, tag]"
          :class="[
            'px-3 py-1 rounded-full font-bold text-sm border-2 transition',
            selectedTags.includes(tag)
              ? 'bg-brand-500 text-white border-brand-500 shadow'
              : 'bg-brand-100 text-brand-700 border-brand-200 hover:bg-brand-200',
          ]"
        >
          {{ tag }}
        </button>
      </div>
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
          <svg class="w-10 h-10 text-brand-400 mx-auto mb-4" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 8v4m0 4h.01M21 12a9 9 0 11-18 0 9 9 0 0118 0z"/></svg>
          <p class="text-brand-700 font-bold mb-4">{{ error }}</p>
          <button @click="loadAssets" class="bg-brand-500 hover:bg-brand-600 text-white rounded-lg px-4 py-2 flex items-center gap-2 mx-auto transition-colors">
            <svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path d="M4 4v5h.582m15.356 2A8.001 8.001 0 004.582 9m0 0H9m11 11v-5h-.581m0 0a8.003 8.003 0 01-15.357-2m15.357 2H15"/></svg>
            {{ t('retry') }}
          </button>
        </div>
        <div v-else>
          <div v-if="assets.length === 0" class="text-center py-10">
            <div class="w-20 h-20 bg-gradient-to-br from-brand-100 to-brand-300 rounded-full flex items-center justify-center mx-auto mb-4 shadow-xl">
              <span class="text-3xl">{{ DiamondEmoji }}</span>
            </div>
            <h3 class="text-xl font-bold text-brand-700 mb-2">{{ t('noAssets') }}</h3>
            <p class="text-brand-400 mb-6">{{ t('noAssetsDesc') }}</p>
          </div>
          <div v-else class="grid gap-6 md:grid-cols-2 lg:grid-cols-3 xl:grid-cols-4">
            <div v-for="asset in filteredAssets" :key="asset.uid" class="bg-brand-50 rounded-2xl shadow-lg px-6 py-5 mb-4 flex items-center justify-between border-2 border-brand-200">
              <div class="flex flex-col flex-1 min-w-0">
                <span class="block font-extrabold text-lg text-brand-700 whitespace-normal break-words min-w-[120px]">{{ asset.info.name }}</span>
              </div>
              <div class="flex gap-2">
                <button @click="openEditor(asset.uid)" class="bg-brand-200 hover:bg-brand-300 text-brand-700 rounded-full p-2 transition shadow" :title="t('edit')">
                  <span class="text-3xl">{{ PenEmoji }}</span>
                </button>
                <button @click="deleteAssetUI(asset.uid)" class="bg-red-200 hover:bg-red-300 text-red-700 rounded-full p-2 transition shadow" :title="t('delete')">
                  <span class="text-3xl">{{ TrashEmoji }}</span>
                </button>
              </div>
            </div>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted, computed } from 'vue'
import { t } from '@shared/utils/i18n'
import { useNavigationStore } from '@/store/navigation'
import { nav, NavigationPath } from '@/utils/navigationTree'
import {
  getAssets,
  uploadAssetFile,
  deleteAsset
} from '@shared/domains/asset/endpoints'
import type { Asset } from '@shared/domains/asset/types'
import { PenEmoji, TrashEmoji, DiamondEmoji } from '@shared/icons/emoji'

const assets = ref<Asset[]>([])
const loading = ref(true)
const error = ref<string | null>(null)

const navStore = useNavigationStore()

const selectedTags = ref<string[]>([])

const allTags = computed(() => {
  const tags = new Set<string>()
  for (const asset of assets.value) {
    if (asset.info.tags) asset.info.tags.forEach(t => tags.add(t))
  }
  return Array.from(tags).sort()
})

const filteredAssets = computed(() => {
  return assets.value.filter(asset => {
    if (selectedTags.value.length > 0) {
      if (!asset.info.tags || !selectedTags.value.every(tag => asset.info.tags!.includes(tag))) {
        return false
      }
    }
    return true
  })
})

async function loadAssets() {
  loading.value = true
  error.value = null
  try {
    const result = await getAssets()
    if (result?.success) {
      assets.value = result.data?.assets || []
    } else {
      error.value = result?.error || t('unknownError')
    }
  } catch (e) {
    error.value = (e as Error).message
  } finally {
    loading.value = false
  }
}

function openEditor(uid: string) {
  navStore.navigateTo(nav.assets.edit as NavigationPath, { uid })
}

async function addAssetUI() {
  const uploadResult = await uploadAssetFile()
  if (uploadResult?.success) {
    await loadAssets()
  } else {
    error.value = uploadResult?.error || t('unknownError')
  }
}

async function deleteAssetUI(uid: string) {
  if (!confirm(t('confirmDeleteAsset') || 'Delete asset?')) return
  try {
    const result = await deleteAsset(uid)
    if (result?.success) {
      await loadAssets()
    } else {
      error.value = result?.error || t('unknownError')
    }
  } catch (e) {
    error.value = (e as Error).message
  }
}

onMounted(loadAssets)
</script>