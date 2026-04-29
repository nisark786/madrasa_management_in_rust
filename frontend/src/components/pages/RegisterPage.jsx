import { useState } from 'react';
import { useNavigate } from 'react-router-dom';
import { useAuthStore } from '../../stores/authStore';
import { BookOpen, AlertCircle, Loader } from 'lucide-react';

const RegisterPage = () => {
  const navigate = useNavigate();
  const { register, isLoading, error } = useAuthStore();
  const [formData, setFormData] = useState({
    email: '',
    password: '',
    confirmPassword: '',
    tenantName: '',
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

    if (formData.password !== formData.confirmPassword) {
      setLocalError('Passwords do not match');
      return;
    }

    if (formData.password.length < 8) {
      setLocalError('Password must be at least 8 characters');
      return;
    }

    try {
      await register(formData.email, formData.password, formData.tenantName);
      navigate('/dashboard');
    } catch (error) {
      setLocalError(error.message || 'Registration failed. Please try again.');
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
          <p className="text-gray-600 mt-2">Create your account</p>
        </div>

        <form onSubmit={handleSubmit} className="space-y-6">
          {/* Error Alert */}
          {(error || localError) && (
            <div className="p-4 bg-red-50 border border-red-200 rounded-lg flex gap-3">
              <AlertCircle size={20} className="text-red-600 flex-shrink-0" />
              <span className="text-red-700">{error || localError}</span>
            </div>
          )}

          {/* Madrasa Name */}
          <div>
            <label htmlFor="tenantName" className="block text-sm font-semibold text-gray-900 mb-2">
              Madrasa Name
            </label>
            <input
              type="text"
              id="tenantName"
              name="tenantName"
              value={formData.tenantName}
              onChange={handleChange}
              placeholder="Your Madrasa Name"
              required
              disabled={isLoading}
              className="w-full px-4 py-3 border border-gray-300 rounded-lg focus:outline-none focus:ring-2 focus:ring-blue-500 focus:border-transparent disabled:bg-gray-100 transition"
            />
          </div>

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
              placeholder="At least 8 characters"
              required
              disabled={isLoading}
              className="w-full px-4 py-3 border border-gray-300 rounded-lg focus:outline-none focus:ring-2 focus:ring-blue-500 focus:border-transparent disabled:bg-gray-100 transition"
            />
          </div>

          {/* Confirm Password Field */}
          <div>
            <label htmlFor="confirmPassword" className="block text-sm font-semibold text-gray-900 mb-2">
              Confirm Password
            </label>
            <input
              type="password"
              id="confirmPassword"
              name="confirmPassword"
              value={formData.confirmPassword}
              onChange={handleChange}
              placeholder="Re-enter your password"
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
                Creating account...
              </>
            ) : (
              'Create Account'
            )}
          </button>

          {/* Footer Links */}
          <div className="text-center text-sm">
            <p>
              Already have an account?{' '}
              <button 
                type="button"
                className="text-blue-600 font-semibold hover:underline"
                onClick={() => navigate('/login')}
              >
                Sign in here
              </button>
            </p>
          </div>
        </form>
      </div>
    </div>
  );
};

export default RegisterPage;
