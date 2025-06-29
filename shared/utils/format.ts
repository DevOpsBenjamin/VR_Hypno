export function formatDuration(seconds?: number | null): string {
  if (seconds == null || seconds < 0) return '00:00.00'
  
  const totalSeconds = Math.abs(seconds)
  const h = Math.floor(totalSeconds / 3600)
  const m = Math.floor((totalSeconds % 3600) / 60)
  const s = Math.floor(totalSeconds % 60)
  const ms = Math.floor((totalSeconds % 1) * 100) // Get first 2 decimal places (10ms precision)
  
  if (h > 0) {
    // Format: HH:MM:SS.mm
    return `${h.toString().padStart(2, '0')}:${m.toString().padStart(2, '0')}:${s.toString().padStart(2, '0')}.${ms.toString().padStart(2, '0')}`
  } else {
    // Format: MM:SS.mm (most common for songs)
    return `${m.toString().padStart(2, '0')}:${s.toString().padStart(2, '0')}.${ms.toString().padStart(2, '0')}`
  }
} 