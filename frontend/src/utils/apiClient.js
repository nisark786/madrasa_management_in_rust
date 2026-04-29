import { useAuthStore } from '../stores/authStore';

const API_BASE_URL = import.meta.env.VITE_API_URL || 'http://localhost:8080/api/v1';

export const apiClient = {
  async request(endpoint, options = {}) {
    const state = useAuthStore.getState();
    
    const headers = {
      'Content-Type': 'application/json',
      ...options.headers,
    };

    if (state.tokens?.access) {
      headers['Authorization'] = `Bearer ${state.tokens.access}`;
    }

    try {
      const response = await fetch(`${API_BASE_URL}${endpoint}`, {
        ...options,
        headers,
      });

      // Handle 401 - token expired
      if (response.status === 401 && state.tokens?.refresh) {
        try {
          await state.refreshToken();
          const newState = useAuthStore.getState();
          headers['Authorization'] = `Bearer ${newState.tokens.access}`;
          return fetch(`${API_BASE_URL}${endpoint}`, {
            ...options,
            headers,
          }).then(res => res.json());
        } catch (error) {
          state.logout();
          throw error;
        }
      }

      if (!response.ok) {
        throw new Error(`API error: ${response.status}`);
      }

      return await response.json();
    } catch (error) {
      console.error('API request failed:', error);
      throw error;
    }
  },

  get(endpoint, options = {}) {
    return this.request(endpoint, { ...options, method: 'GET' });
  },

  post(endpoint, data, options = {}) {
    return this.request(endpoint, {
      ...options,
      method: 'POST',
      body: JSON.stringify(data),
    });
  },

  put(endpoint, data, options = {}) {
    return this.request(endpoint, {
      ...options,
      method: 'PUT',
      body: JSON.stringify(data),
    });
  },

  patch(endpoint, data, options = {}) {
    return this.request(endpoint, {
      ...options,
      method: 'PATCH',
      body: JSON.stringify(data),
    });
  },

  delete(endpoint, options = {}) {
    return this.request(endpoint, { ...options, method: 'DELETE' });
  },
};
