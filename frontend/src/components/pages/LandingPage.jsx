import { useNavigate } from 'react-router-dom';
import { BookOpen, Users, BarChart3, Zap, ArrowRight, Check } from 'lucide-react';

const LandingPage = () => {
  const navigate = useNavigate();

  const features = [
    {
      icon: Users,
      title: 'Student Management',
      description: 'Efficiently manage student enrollment, attendance, and performance tracking.',
    },
    {
      icon: BookOpen,
      title: 'Quranic Studies',
      description: 'Track Quran memorization progress with structured curriculum and assessments.',
    },
    {
      icon: BarChart3,
      title: 'Analytics & Reports',
      description: 'Comprehensive dashboards and reports for administrators and staff.',
    },
    {
      icon: Zap,
      title: 'Real-time Updates',
      description: 'Instant notifications and live updates across the entire system.',
    },
  ];

  const benefits = [
    'Multi-tenant architecture for multiple madrasas',
    'Role-based access control (Admin, Manager, Staff, Student)',
    'JWT authentication with secure token management',
    'PostgreSQL with Row-Level Security',
    'RESTful API with comprehensive endpoints',
    'Responsive React frontend with modern UI',
  ];

  return (
    <div className="min-h-screen bg-white">
      {/* Navigation */}
      <nav className="sticky top-0 z-50 bg-white border-b border-gray-200 shadow-sm">
        <div className="max-w-6xl mx-auto px-4 sm:px-6 lg:px-8">
          <div className="flex justify-between items-center h-16">
            <div className="flex items-center gap-2 text-2xl font-bold text-blue-600">
              <BookOpen size={32} />
              <span className="hidden sm:inline">Madrasa Management</span>
            </div>
            <div className="flex gap-3 items-center">
              <button 
                className="px-4 py-2 rounded-lg font-semibold text-gray-700 bg-gray-100 hover:bg-gray-200 transition-colors"
                onClick={() => navigate('/login')}
              >
                Login
              </button>
              <button 
                className="px-4 py-2 rounded-lg font-semibold text-white bg-blue-600 hover:bg-blue-700 transition-colors"
                onClick={() => navigate('/register')}
              >
                Get Started
              </button>
            </div>
          </div>
        </div>
      </nav>

      {/* Hero Section */}
      <section className="py-20 sm:py-32 bg-gradient-to-b from-blue-50 to-white">
        <div className="max-w-6xl mx-auto px-4 sm:px-6 lg:px-8">
          <div className="grid md:grid-cols-2 gap-12 items-center">
            <div>
              <h1 className="text-4xl sm:text-5xl lg:text-6xl font-bold text-gray-900 leading-tight mb-6">
                Modern Madrasa Management System
              </h1>
              <p className="text-lg sm:text-xl text-gray-600 mb-8 leading-relaxed">
                Streamline student enrollment, track Quranic progress, and manage finances 
                with our comprehensive, cloud-based solution designed specifically for Islamic institutions.
              </p>
              <div className="flex flex-col sm:flex-row gap-4">
                <button 
                  className="px-8 py-3 rounded-lg font-semibold text-white bg-blue-600 hover:bg-blue-700 transition-colors flex items-center justify-center gap-2"
                  onClick={() => navigate('/register')}
                >
                  Start Free Trial <ArrowRight size={20} />
                </button>
                <button 
                  className="px-8 py-3 rounded-lg font-semibold text-blue-600 border-2 border-blue-600 hover:bg-blue-50 transition-colors"
                  onClick={() => navigate('/login')}
                >
                  Sign In
                </button>
              </div>
            </div>
            <div className="hidden md:flex flex-col items-center justify-center h-96 bg-white rounded-lg border-2 border-gray-200 text-gray-400">
              <BookOpen size={120} />
              <p className="mt-4 text-lg font-medium">Madrasa Management Dashboard</p>
            </div>
          </div>
        </div>
      </section>

      {/* Features Section */}
      <section className="py-20 bg-white">
        <div className="max-w-6xl mx-auto px-4 sm:px-6 lg:px-8">
          <h2 className="text-4xl font-bold text-center text-gray-900 mb-4">Powerful Features</h2>
          <p className="text-center text-xl text-gray-600 mb-16">
            Everything you need to run your Madrasa efficiently
          </p>
          <div className="grid sm:grid-cols-2 lg:grid-cols-4 gap-8">
            {features.map((feature, idx) => {
              const Icon = feature.icon;
              return (
                <div key={idx} className="p-6 border border-gray-200 rounded-lg hover:border-blue-600 hover:shadow-lg transition-all transform hover:-translate-y-1">
                  <div className="w-14 h-14 bg-gradient-to-br from-blue-600 to-blue-700 rounded-lg flex items-center justify-center text-white mb-4">
                    <Icon size={32} />
                  </div>
                  <h3 className="text-xl font-bold text-gray-900 mb-2">{feature.title}</h3>
                  <p className="text-gray-600">{feature.description}</p>
                </div>
              );
            })}
          </div>
        </div>
      </section>

      {/* Benefits Section */}
      <section className="py-20 bg-gradient-to-b from-blue-50 to-white">
        <div className="max-w-6xl mx-auto px-4 sm:px-6 lg:px-8">
          <h2 className="text-4xl font-bold text-gray-900 mb-4">Why Choose Us</h2>
          <div className="grid md:grid-cols-2 gap-12 items-center">
            <div>
              <p className="text-lg text-gray-600 mb-8">
                Built with modern technology stack for reliability and scalability:
              </p>
              <ul className="space-y-4">
                {benefits.map((benefit, idx) => (
                  <li key={idx} className="flex items-center gap-3 text-gray-700">
                    <Check size={24} className="text-green-500 flex-shrink-0" />
                    <span>{benefit}</span>
                  </li>
                ))}
              </ul>
              <button 
                className="mt-8 px-8 py-3 rounded-lg font-semibold text-white bg-blue-600 hover:bg-blue-700 transition-colors"
                onClick={() => navigate('/register')}
              >
                Get Started Now
              </button>
            </div>
            <div className="bg-white rounded-lg shadow-lg p-8 border border-gray-200">
              <div className="space-y-6">
                <div className="text-center">
                  <div className="text-4xl font-bold text-blue-600 mb-2">10K+</div>
                  <div className="text-gray-600">Students Managed</div>
                </div>
                <div className="text-center">
                  <div className="text-4xl font-bold text-blue-600 mb-2">50+</div>
                  <div className="text-gray-600">Madrasas</div>
                </div>
                <div className="text-center">
                  <div className="text-4xl font-bold text-blue-600 mb-2">99.9%</div>
                  <div className="text-gray-600">Uptime</div>
                </div>
              </div>
            </div>
          </div>
        </div>
      </section>

      {/* CTA Section */}
      <section className="py-20 bg-gradient-to-r from-blue-600 to-blue-700 text-white text-center">
        <div className="max-w-2xl mx-auto px-4">
          <h2 className="text-4xl font-bold mb-4">Ready to Transform Your Madrasa?</h2>
          <p className="text-xl mb-8 opacity-90">Join hundreds of Islamic institutions using our platform</p>
          <button 
            className="px-8 py-4 rounded-lg font-semibold text-blue-600 bg-white hover:bg-gray-100 transition-colors inline-flex items-center justify-center gap-2"
            onClick={() => navigate('/register')}
          >
            Start Your Free Trial Today
          </button>
        </div>
      </section>

      {/* Footer */}
      <footer className="bg-gray-900 text-white py-12">
        <div className="max-w-6xl mx-auto px-4 sm:px-6 lg:px-8">
          <div className="grid sm:grid-cols-2 md:grid-cols-4 gap-8 mb-8">
            <div>
              <h4 className="font-bold mb-4">About</h4>
              <ul className="space-y-2 text-gray-400">
                <li><a href="#about" className="hover:text-white transition">About Us</a></li>
                <li><a href="#blog" className="hover:text-white transition">Blog</a></li>
                <li><a href="#careers" className="hover:text-white transition">Careers</a></li>
              </ul>
            </div>
            <div>
              <h4 className="font-bold mb-4">Product</h4>
              <ul className="space-y-2 text-gray-400">
                <li><a href="#features" className="hover:text-white transition">Features</a></li>
                <li><a href="#pricing" className="hover:text-white transition">Pricing</a></li>
                <li><a href="#security" className="hover:text-white transition">Security</a></li>
              </ul>
            </div>
            <div>
              <h4 className="font-bold mb-4">Support</h4>
              <ul className="space-y-2 text-gray-400">
                <li><a href="#docs" className="hover:text-white transition">Documentation</a></li>
                <li><a href="#contact" className="hover:text-white transition">Contact</a></li>
                <li><a href="#faq" className="hover:text-white transition">FAQ</a></li>
              </ul>
            </div>
            <div>
              <h4 className="font-bold mb-4">Legal</h4>
              <ul className="space-y-2 text-gray-400">
                <li><a href="#privacy" className="hover:text-white transition">Privacy</a></li>
                <li><a href="#terms" className="hover:text-white transition">Terms</a></li>
                <li><a href="#cookies" className="hover:text-white transition">Cookies</a></li>
              </ul>
            </div>
          </div>
          <div className="text-center pt-8 border-t border-gray-800 text-gray-400">
            <p>&copy; 2026 Madrasa Management System. All rights reserved.</p>
          </div>
        </div>
      </footer>
    </div>
  );
};

export default LandingPage;
