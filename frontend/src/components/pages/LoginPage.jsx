import { useState } from 'react';
import { useNavigate } from 'react-router-dom';
import { useAuthStore } from '../../stores/authStore';
import { BookOpen, AlertCircle, Loader } from 'lucide-react';

const LoginPage = () => {
  const navigate = useNavigate();
  const { login, isLoading, error } = useAuthStore();
  const [formData, setFormData] = useState({
    email: '',
    password: '',
  });
  const [localError, setLocalError] = useState('');

  const handleChange = (e) => {
    const { name, value } = e.target;
    setFormData(prev => ({
      ...prev,
      [name]: value,
    }));
    setLocalError('');
  };

  const handleSubmit = async (e) => {
    e.preventDefault();
    try {
      await login(formData.email, formData.password);
      navigate('/dashboard');
    } catch (error) {
      setLocalError(error.message || 'Login failed. Please check your credentials.');
    }
  };

  return (
    <div className="min-h-screen flex items-center justify-center bg-gradient-to-br from-blue-50 to-indigo-100 p-4">
      <div className="w-full max-w-md bg-white rounded-xl shadow-lg p-8">
        {/* Header */}
        <div className="text-center mb-8">
          <div className="flex justify-center mb-4">
            <BookOpen size={40} className="text-blue-600" />
          </div>
          <h1 className="text-3xl font-bold text-gray-900">Madrasa Management</h1>
          <p className="text-gray-600 mt-2">Sign in to your account</p>
        </div>

        <form onSubmit={handleSubmit} className="space-y-6">
          {/* Error Alert */}
          {(error || localError) && (
            <div className="p-4 bg-red-50 border border-red-200 rounded-lg flex gap-3">
              <AlertCircle size={20} className="text-red-600 flex-shrink-0" />
              <span className="text-red-700">{error || localError}</span>
            </div>
          )}

          {/* Email Field */}
          <div>
            <label htmlFor="email" className="block text-sm font-semibold text-gray-900 mb-2">
              Email Address
            </label>
            <input
              type="email"
              id="email"
              name="email"
              value={formData.email}
              onChange={handleChange}
              placeholder="your@email.com"
              required
              disabled={isLoading}
              className="w-full px-4 py-3 border border-gray-300 rounded-lg focus:outline-none focus:ring-2 focus:ring-blue-500 focus:border-transparent disabled:bg-gray-100 transition"
            />
          </div>

          {/* Password Field */}
          <div>
            <label htmlFor="password" className="block text-sm font-semibold text-gray-900 mb-2">
              Password
            </label>
            <input
              type="password"
              id="password"
              name="password"
              value={formData.password}
              onChange={handleChange}
              placeholder="Enter your password"
              required
              disabled={isLoading}
              className="w-full px-4 py-3 border border-gray-300 rounded-lg focus:outline-none focus:ring-2 focus:ring-blue-500 focus:border-transparent disabled:bg-gray-100 transition"
            />
          </div>

          {/* Submit Button */}
          <button 
            type="submit" 
            disabled={isLoading}
            className="w-full py-3 rounded-lg font-semibold text-white bg-blue-600 hover:bg-blue-700 disabled:bg-gray-400 transition-colors flex items-center justify-center gap-2"
          >
            {isLoading ? (
              <>
                <Loader size={20} className="animate-spin" />
                Signing in...
              </>
            ) : (
              'Sign In'
            )}
          </button>

          {/* Footer Links */}
          <div className="space-y-2 text-center text-sm">
            <p>
              Don't have an account?{' '}
              <button 
                type="button"
                className="text-blue-600 font-semibold hover:underline"
                onClick={() => navigate('/register')}
              >
                Sign up here
              </button>
            </p>
            <p>
              <button 
                type="button"
                className="text-blue-600 font-semibold hover:underline"
                onClick={() => navigate('/forgot-password')}
              >
                Forgot password?
              </button>
            </p>
          </div>
        </form>

        {/* Demo Credentials */}
        <div className="mt-6 pt-6 border-t border-gray-200 text-center text-sm text-gray-600">
          <p className="font-semibold mb-2">Demo Credentials:</p>
          <p className="text-xs">Email: admin@example.com</p>
          <p className="text-xs">Password: password</p>
        </div>
      </div>
    </div>
  );
};

export default LoginPage;
