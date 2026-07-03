export const API_URL = import.meta.env.VITE_API_URL ?? "http://localhost:8080";

export function authHeaders(token) {
  return token ? { Authorization: `Bearer ${token}` } : {};
}