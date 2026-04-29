import { useAuthStore } from '../stores/authStore';
import { useEffect, useState } from 'react';

export const useAuth = () => {
  const { user, isAuthenticated, tokens } = useAuthStore();
  return { user, isAuthenticated, tokens };
};

export const useProtectedRoute = (requiredRoles = []) => {
  const { user, isAuthenticated } = useAuthStore();
  const [isAuthorized, setIsAuthorized] = useState(false);

  useEffect(() => {
    if (!isAuthenticated) {
      setIsAuthorized(false);
      return;
    }

    if (requiredRoles.length === 0) {
      setIsAuthorized(true);
      return;
    }

    setIsAuthorized(requiredRoles.includes(user?.role));
  }, [user, isAuthenticated, requiredRoles]);

  return { isAuthorized, user, isAuthenticated };
};
