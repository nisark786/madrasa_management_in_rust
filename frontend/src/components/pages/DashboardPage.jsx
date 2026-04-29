import { useAuth } from '../../hooks/useAuth';
import { useAuthStore } from '../../stores/authStore';
import { useNavigate } from 'react-router-dom';
import { LogOut, Menu, X, BarChart3, Users, BookOpen, DollarSign } from 'lucide-react';
import { useState } from 'react';

const DashboardPage = () => {
  const { user } = useAuth();
  const { logout } = useAuthStore();
  const navigate = useNavigate();
  const [sidebarOpen, setSidebarOpen] = useState(false);

  const handleLogout = () => {
    logout();
    navigate('/login');
  };

  const renderDashboardContent = () => {
    switch (user?.role) {
      case 'admin':
        return <AdminDashboard />;
      case 'manager':
        return <ManagerDashboard />;
      case 'staff':
        return <StaffDashboard />;
      case 'student':
        return <StudentDashboard />;
      default:
        return <DefaultDashboard />;
    }
  };

  return (
    <div className="min-h-screen bg-gray-50">
      {/* Top Navigation */}
      <header className="sticky top-0 z-50 bg-white border-b border-gray-200 shadow-sm">
        <div className="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8">
          <div className="flex justify-between items-center h-16 gap-4">
            <button 
              className="md:hidden p-2 hover:bg-gray-100 rounded-lg text-blue-600"
              onClick={() => setSidebarOpen(!sidebarOpen)}
            >
              {sidebarOpen ? <X size={24} /> : <Menu size={24} />}
            </button>
            <h1 className="text-2xl font-bold text-gray-900">Dashboard</h1>
            <div className="flex items-center gap-4">
              <div className="hidden sm:flex flex-col text-right">
                <span className="font-semibold text-gray-900 text-sm">{user?.email}</span>
                <span className="text-xs text-gray-600 capitalize">{user?.role}</span>
              </div>
              <button 
                className="p-2 hover:bg-gray-100 rounded-lg text-blue-600 transition"
                onClick={handleLogout}
                title="Logout"
              >
                <LogOut size={20} />
              </button>
            </div>
          </div>
        </div>
      </header>

      <div className="flex">
        {/* Sidebar */}
        <aside className={`fixed md:relative w-64 h-screen bg-white border-r border-gray-200 p-6 transition-transform duration-300 z-40 ${
          sidebarOpen ? 'translate-x-0' : '-translate-x-full md:translate-x-0'
        }`}>
          <nav className="space-y-2">
            <h3 className="text-xs font-bold text-gray-600 uppercase tracking-wider mb-6">Menu</h3>
            <ul className="space-y-2">
              <li><a href="#overview" className="block px-3 py-2 rounded-lg text-gray-700 hover:bg-blue-50 hover:text-blue-600 transition">Overview</a></li>
              <li><a href="#students" className="block px-3 py-2 rounded-lg text-gray-700 hover:bg-blue-50 hover:text-blue-600 transition">Students</a></li>
              <li><a href="#staff" className="block px-3 py-2 rounded-lg text-gray-700 hover:bg-blue-50 hover:text-blue-600 transition">Staff</a></li>
              <li><a href="#quran" className="block px-3 py-2 rounded-lg text-gray-700 hover:bg-blue-50 hover:text-blue-600 transition">Quran Progress</a></li>
              <li><a href="#finances" className="block px-3 py-2 rounded-lg text-gray-700 hover:bg-blue-50 hover:text-blue-600 transition">Finances</a></li>
              <li><a href="#reports" className="block px-3 py-2 rounded-lg text-gray-700 hover:bg-blue-50 hover:text-blue-600 transition">Reports</a></li>
              <li><a href="#settings" className="block px-3 py-2 rounded-lg text-gray-700 hover:bg-blue-50 hover:text-blue-600 transition">Settings</a></li>
            </ul>
          </nav>
        </aside>

        {/* Main Content */}
        <main className="flex-1 p-6 md:p-8">
          {renderDashboardContent()}
        </main>
      </div>
    </div>
  );
};

const DefaultDashboard = () => (
  <div>
    <h2 className="text-3xl font-bold text-gray-900 mb-8">Welcome to Madrasa Management System</h2>
    <div className="grid sm:grid-cols-2 lg:grid-cols-3 gap-6">
      <StatCard icon={Users} label="Total Students" value="245" color="bg-blue-600" />
      <StatCard icon={BookOpen} label="Quranic Progress" value="78%" color="bg-green-600" />
      <StatCard icon={BarChart3} label="Attendance" value="92%" color="bg-amber-600" />
    </div>
  </div>
);

const StatCard = ({ icon: Icon, label, value, color }) => (
  <div className="bg-white rounded-lg border border-gray-200 p-6 flex items-center gap-4 hover:shadow-lg transition">
    <div className={`${color} w-16 h-16 rounded-lg flex items-center justify-center text-white flex-shrink-0`}>
      <Icon size={32} />
    </div>
    <div>
      <span className="text-gray-600 text-sm">{label}</span>
      <span className="text-2xl font-bold text-gray-900">{value}</span>
    </div>
  </div>
);

const AdminDashboard = () => (
  <div>
    <h2 className="text-3xl font-bold text-gray-900 mb-8">Admin Dashboard</h2>
    <div className="grid sm:grid-cols-2 lg:grid-cols-3 gap-6">
      <DashboardCard title="System Overview" description="Manage all madrasas, users, and system settings." />
      <DashboardCard title="User Management" description="Manage administrators, managers, staff, and students." />
      <DashboardCard title="System Reports" description="Generate comprehensive system and usage reports." />
    </div>
  </div>
);

const ManagerDashboard = () => (
  <div>
    <h2 className="text-3xl font-bold text-gray-900 mb-8">Manager Dashboard</h2>
    <div className="grid sm:grid-cols-2 lg:grid-cols-3 gap-6">
      <DashboardCard title="Madrasa Statistics" description="Overview of your madrasa's performance and metrics." />
      <DashboardCard title="Staff Management" description="Manage staff members and their assignments." />
      <DashboardCard title="Financial Reports" description="View financial transactions and reports." />
    </div>
  </div>
);

const StaffDashboard = () => (
  <div>
    <h2 className="text-3xl font-bold text-gray-900 mb-8">Staff Dashboard</h2>
    <div className="grid sm:grid-cols-2 lg:grid-cols-3 gap-6">
      <DashboardCard title="My Classes" description="Manage your assigned classes and schedules." />
      <DashboardCard title="Attendance" description="Mark attendance and view reports." />
      <DashboardCard title="Student Progress" description="Track student progress and send updates." />
    </div>
  </div>
);

const StudentDashboard = () => (
  <div>
    <h2 className="text-3xl font-bold text-gray-900 mb-8">Student Dashboard</h2>
    <div className="grid sm:grid-cols-2 lg:grid-cols-3 gap-6">
      <DashboardCard title="My Progress" description="View your Quranic memorization progress." />
      <DashboardCard title="My Classes" description="View your class schedule and assignments." />
      <DashboardCard title="My Grades" description="Check your assessments and grades." />
    </div>
  </div>
);

const DashboardCard = ({ title, description }) => (
  <div className="bg-white rounded-lg border border-gray-200 p-6 hover:border-blue-600 hover:shadow-lg transition">
    <h3 className="text-xl font-bold text-gray-900 mb-2">{title}</h3>
    <p className="text-gray-600 mb-4">{description}</p>
    <button className="px-4 py-2 bg-blue-600 text-white rounded-lg font-semibold hover:bg-blue-700 transition">
      View Details
    </button>
  </div>
);

export default DashboardPage;
