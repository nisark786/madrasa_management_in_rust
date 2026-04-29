import { create } from 'zustand';
import { persist } from 'zustand/middleware';

const API_BASE_URL = import.meta.env.VITE_API_URL || 'http://localhost:8080/api/v1';

const toTenantSlug = (nameOrSlug) =>
  String(nameOrSlug || '')
    .trim()
    .toLowerCase()
    .replace(/[^a-z0-9]+/g, '-')
    .replace(/^-+|-+$/g, '');

const readErrorMessage = async (response, fallback) => {
  try {
    const data = await response.json();
    if (data?.message) return data.message;
  } catch (_) {}
  return fallback;
};

export const useAuthStore = create(
  persist(
    (set, get) => ({
      // State
      user: null,
      tokens: null,
      isLoading: false,
      error: null,
      isAuthenticated: false,

      // Actions
      login: async (email, password) => {
        set({ isLoading: true, error: null });
        try {
          const response = await fetch(`${API_BASE_URL}/identity/login`, {
            method: 'POST',
            headers: { 'Content-Type': 'application/json' },
            body: JSON.stringify({ email, password }),
          });

          if (!response.ok) {
            throw new Error(await readErrorMessage(response, 'Login failed'));
          }

          const data = await response.json();
          set({
            user: {
              id: data.user_id,
              email: data.email,
              role: data.role,
              tenantId: data.tenant_id,
            },
            tokens: {
              access: data.access_token,
              refresh: data.refresh_token,
            },
            isAuthenticated: true,
            isLoading: false,
          });

          return data;
        } catch (error) {
          set({ 
            error: error.message, 
            isLoading: false,
            isAuthenticated: false,
          });
          throw error;
        }
      },

      register: async (email, password, tenantName) => {
        set({ isLoading: true, error: null });
        try {
          const tenantSlug = toTenantSlug(tenantName);
          const response = await fetch(`${API_BASE_URL}/identity/bootstrap`, {
            method: 'POST',
            headers: { 'Content-Type': 'application/json' },
            body: JSON.stringify({ 
              tenant_name: tenantName,
              tenant_slug: tenantSlug,
              email, 
              password,
            }),
          });

          if (!response.ok) {
            throw new Error(await readErrorMessage(response, 'Registration failed'));
          }

          const data = await response.json();
          set({
            user: {
              id: data.user_id,
              email: data.email,
              role: data.role,
              tenantId: data.tenant_id,
            },
            tokens: {
              access: data.access_token,
              refresh: data.refresh_token,
            },
            isAuthenticated: true,
            isLoading: false,
          });

          return data;
        } catch (error) {
          set({ 
            error: error.message, 
            isLoading: false,
          });
          throw error;
        }
      },

      refreshToken: async () => {
        try {
          const state = get();
          if (!state.tokens?.refresh) {
            throw new Error('No refresh token available');
          }

          const response = await fetch(`${API_BASE_URL}/identity/refresh`, {
            method: 'POST',
            headers: { 'Content-Type': 'application/json' },
            body: JSON.stringify({ refresh_token: state.tokens.refresh }),
          });

          if (!response.ok) {
            throw new Error(await readErrorMessage(response, 'Token refresh failed'));
          }

          const data = await response.json();
          set({
            tokens: {
              access: data.access_token,
              refresh: data.refresh_token,
            },
          });

          return data;
        } catch (error) {
          set({ 
            isAuthenticated: false,
            user: null,
            tokens: null,
          });
          throw error;
        }
      },

      logout: () => {
        set({
          user: null,
          tokens: null,
          isAuthenticated: false,
          error: null,
        });
      },

      clearError: () => {
        set({ error: null });
      },
    }),
    {
      name: 'auth-storage',
      partialize: (state) => ({
        user: state.user,
        tokens: state.tokens,
        isAuthenticated: state.isAuthenticated,
      }),
    }
  )
);
