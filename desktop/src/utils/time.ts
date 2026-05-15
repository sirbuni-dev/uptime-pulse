/**
 * Formats a UTC ISO-8601 timestamp string into a human-readable local time.
 * Uses the system timezone and locale automatically via Intl.DateTimeFormat.
 *
 * Input:  "2026-05-15T04:43:24Z"
 * Output: "May 15, 2026, 07:43:24 AM"  (example for UTC+3)
 */
export function formatDateTime(utcStr: string): string {
  return new Date(utcStr).toLocaleString(undefined, {
    year:   'numeric',
    month:  'short',
    day:    'numeric',
    hour:   '2-digit',
    minute: '2-digit',
    second: '2-digit',
    hour12: false,
  })
}
