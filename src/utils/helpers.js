/**
 * Utility functions for the stores module
 */

import { MIME_TYPES, FILE_LIMITS } from './constants.js'

/**
 * Format file size to human-readable format
 * @param {number} bytes - File size in bytes
 * @returns {string} Formatted file size
 */
export function formatFileSize(bytes) {
  if (!bytes || bytes === 0) return '0 B'
  const units = ['B', 'KB', 'MB', 'GB', 'TB']
  const index = Math.floor(Math.log(bytes) / Math.log(1024))
  return `${(bytes / Math.pow(1024, index)).toFixed(2)} ${units[index]}`
}

/**
 * Get content type from file extension
 * @param {string} fileName - Name of the file
 * @returns {string} MIME type
 */
export function getContentType(fileName) {
  const ext = fileName.split('.').pop()?.toLowerCase()
  return MIME_TYPES[ext] || 'application/octet-stream'
}

/**
 * Calculate SHA-256 hash for file data
 * @param {string} filePath - Path to the file
 * @param {number} size - File size in bytes
 * @param {Function} readFile - Function to read file content
 * @returns {Promise<string|null>} Hash string or null
 */
export async function calculateFileHash(filePath, size, readFile) {
  try {
    // Skip hash calculation for large files or if size exceeds limit
    if (size > FILE_LIMITS.MAX_HASH_SIZE) {
      return 'file-too-large'
    }

    const fileData = await readFile(filePath)
    let dataToHash

    if (typeof fileData === 'string') {
      const encoder = new TextEncoder()
      dataToHash = encoder.encode(fileData)
    } else if (fileData instanceof Uint8Array) {
      dataToHash = fileData
    } else {
      return null
    }

    const hashBuffer = await crypto.subtle.digest('SHA-256', dataToHash)
    const hashArray = Array.from(new Uint8Array(hashBuffer))
    return hashArray.map(b => b.toString(16).padStart(2, '0')).join('')
  } catch (error) {
    console.error('Error calculating file hash:', error)
    return null
  }
}

/**
 * Validate store name
 * @param {string} name - Store name
 * @param {number} minLength - Minimum length requirement
 * @returns {boolean} Is valid
 */
export function validateStoreName(name, minLength = 2) {
  if (!name || typeof name !== 'string') return false
  const trimmed = name.trim()
  if (trimmed.length < minLength) return false
  // Prevent JSON-like strings
  if (trimmed.startsWith('{') || trimmed.startsWith('[')) return false
  return true
}

/**
 * Format item count text
 * @param {number} count - Number of items
 * @returns {string} Formatted text
 */
export function formatItemCount(count) {
  return `${count} ${count === 1 ? 'item' : 'items'}`
}

/**
 * Get friendly file type from MIME type
 * @param {string} mimeType - MIME type string
 * @returns {string} Friendly file type name
 */
export function getFriendlyFileType(mimeType) {
  if (!mimeType) return 'File'

  // Check for common patterns
  if (mimeType.startsWith('image/')) return 'Image'
  if (mimeType.startsWith('video/')) return 'Video'
  if (mimeType.startsWith('audio/')) return 'Audio'
  if (mimeType.startsWith('text/')) {
    // Special cases for text types
    if (mimeType === 'text/html') return 'HTML'
    if (mimeType === 'text/css') return 'CSS'
    if (mimeType === 'text/javascript') return 'JavaScript'
    if (mimeType === 'text/typescript') return 'TypeScript'
    if (mimeType === 'text/markdown') return 'Markdown'
    if (mimeType === 'text/x-vue') return 'Vue'
    return 'Text'
  }

  // Specific MIME types
  const typeMap = {
    'application/pdf': 'PDF',
    'application/zip': 'ZIP',
    'application/x-tar': 'TAR',
    'application/gzip': 'GZIP',
    'application/json': 'JSON',
    'application/xml': 'XML',
    'application/msword': 'Word',
    'application/vnd.openxmlformats-officedocument.wordprocessingml.document': 'Word',
    'application/vnd.ms-excel': 'Excel',
    'application/vnd.openxmlformats-officedocument.spreadsheetml.sheet': 'Excel',
    'application/vnd.ms-powerpoint': 'PowerPoint',
    'application/vnd.openxmlformats-officedocument.presentationml.presentation': 'PowerPoint',
    'application/octet-stream': 'Binary'
  }

  return typeMap[mimeType] || 'File'
}

/**
 * Escape HTML special characters to prevent XSS
 * @param {string} text - Text to escape
 * @returns {string} Escaped text
 */
export function escapeHtml(text) {
  if (!text) return ''
  const escapeMap = {
    '&': '&amp;',
    '<': '&lt;',
    '>': '&gt;',
    '"': '&quot;',
    "'": '&#39;'
  }
  return String(text).replace(/[&<>"']/g, (char) => escapeMap[char])
}

/**
 * Highlight search query matches in text (XSS-safe)
 * @param {string} text - Text to highlight
 * @param {string} query - Search query
 * @returns {string} HTML string with highlighted matches
 */
export function highlightSearchMatch(text, query) {
  if (!text) return ''
  if (!query) return escapeHtml(text)

  const escaped = escapeHtml(text)
  const escapedQuery = escapeHtml(query)
  const regex = new RegExp(`(${escapedQuery.replace(/[.*+?^${}()|[\]\\]/g, '\\$&')})`, 'gi')
  return escaped.replace(regex, '<mark>$1</mark>')
}
