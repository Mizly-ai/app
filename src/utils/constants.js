/**
 * Constants and configuration for the stores module
 */

// File handling limits
export const FILE_LIMITS = {
  MAX_HASH_SIZE: 20 * 1024 * 1024, // 20MB - Maximum file size for hash calculation
  DIALOG_CLOSE_DELAY: 500 // Delay after file dialog closes
}

// MIME type mappings for file extensions
export const MIME_TYPES = {
  // Images
  jpg: 'image/jpeg',
  jpeg: 'image/jpeg',
  png: 'image/png',
  gif: 'image/gif',
  webp: 'image/webp',
  svg: 'image/svg+xml',
  // Documents
  pdf: 'application/pdf',
  doc: 'application/msword',
  docx: 'application/vnd.openxmlformats-officedocument.wordprocessingml.document',
  txt: 'text/plain',
  md: 'text/markdown',
  // Code
  js: 'text/javascript',
  ts: 'text/typescript',
  jsx: 'text/javascript',
  tsx: 'text/typescript',
  vue: 'text/x-vue',
  html: 'text/html',
  css: 'text/css',
  json: 'application/json',
  // Archives
  zip: 'application/zip',
  tar: 'application/x-tar',
  gz: 'application/gzip',
  // Video
  mp4: 'video/mp4',
  avi: 'video/x-msvideo',
  mov: 'video/quicktime',
  // Audio
  mp3: 'audio/mpeg',
  wav: 'audio/wav',
  ogg: 'audio/ogg'
}

// Keyboard shortcuts
export const SHORTCUTS = {
  CREATE_NEW: 'n', // Meta+N
  DELETE: 'd', // Meta+D
  CONFIRM_DELETE: 'y', // Meta+Y (confirm deletion)
  OPEN_FILE: 'o', // Meta+O
  ESCAPE: 'Escape'
}

// Store name validation
export const VALIDATION = {
  MIN_NAME_LENGTH: 2
}
