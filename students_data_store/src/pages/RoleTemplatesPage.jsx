import { useState, useEffect } from 'react';
import api from '../api/client';
import RoleTemplateFormModal from '../components/admin/RoleTemplateFormModal';
import Sidebar from '../components/layout/Sidebar';
import { 
  Layout, 
  Plus, 
  Pencil, 
  Trash2,
  Eye,
  Lock,
  Users,
  CheckCircle,
  AlertCircle
} from 'lucide-react';

const iconMap = {
  'eye': Eye,
  'edit': Pencil,
  'users': Users,
  'list': Lock,
  'book': Layout,
};

const colorMap = {
  'blue': 'bg-blue-100 text-blue-700 border-blue-200',
  'green': 'bg-green-100 text-green-700 border-green-200',
  'purple': 'bg-purple-100 text-purple-700 border-purple-200',
  'orange': 'bg-orange-100 text-orange-700 border-orange-200',
  'red': 'bg-red-100 text-red-700 border-red-200',
};

export default function RoleTemplatesPage() {
  const [templates, setTemplates] = useState([]);
  const [loading, setLoading] = useState(true);
  const [showModal, setShowModal] = useState(false);
  const [selectedTemplate, setSelectedTemplate] = useState(null);

  const fetchTemplates = () => {
    setLoading(true);
    api.get('/role-templates/')
      .then(({ data }) => setTemplates(data))
      .catch((err) => console.error("Failed to load templates", err))
      .finally(() => setLoading(false));
  };

  useEffect(() => {
    fetchTemplates();
  }, []);

  const handleAdd = () => {
    setSelectedTemplate(null);
    setShowModal(true);
  };

  const handleEdit = (template) => {
    setSelectedTemplate(template);
    setShowModal(true);
  };

  const handleDelete = async (id, name) => {
    if (!window.confirm(`Are you sure you want to delete template "${name}"?`)) return;
    try {
      await api.delete(`/role-templates/${id}`);
      fetchTemplates();
    } catch (err) {
      alert(err.response?.data?.detail || "Failed to delete template");
    }
  };

  const handleSave = () => {
    setShowModal(false);
    fetchTemplates();
  };

  return (
    <div className="flex min-h-screen bg-gray-50/50 font-sans antialiased">
      <Sidebar />
      <main className="flex-1 p-8 md:p-12 overflow-y-auto min-w-0">
        <div className="flex flex-col gap-8 max-w-7xl mx-auto">
          
          <div className="flex flex-wrap items-center justify-between gap-4 pb-6 border-b border-gray-200">
            <div className="flex items-center gap-4">
              <div className="bg-cyan-600 p-3 rounded-2xl shadow-lg shadow-cyan-100/50">
                <Layout className="w-6 h-6 text-white" />
              </div>
              <div>
                <h1 className="text-2xl font-bold text-gray-900 leading-tight tracking-tight">Role Templates</h1>
                <p className="text-sm text-gray-400 font-medium">Create and manage predefined role presets for quick role creation</p>
              </div>
            </div>
            <button 
              className="px-5 py-2.5 bg-cyan-600 text-white rounded-xl text-sm font-bold shadow-lg shadow-cyan-100 hover:bg-cyan-700 hover:shadow-cyan-200 transition-all active:scale-95 flex items-center gap-2"
              onClick={handleAdd}
            >
              <Plus className="w-4 h-4" strokeWidth={3} />
              New Template
            </button>
          </div>

          {loading ? (
            <div className="flex justify-center items-center h-64">
              <div className="animate-spin rounded-full h-12 w-12 border-b-2 border-cyan-600"></div>
            </div>
          ) : templates.length === 0 ? (
            <div className="bg-white rounded-2xl shadow-sm border border-gray-200 p-12 text-center">
              <AlertCircle className="w-12 h-12 text-gray-300 mx-auto mb-4" />
              <p className="text-gray-500 font-medium">No role templates yet. Create one to get started!</p>
            </div>
          ) : (
            <div className="grid gap-6 md:grid-cols-2 lg:grid-cols-3">
              {templates.map((template) => {
                const IconComponent = iconMap[template.icon] || Layout;
                const colorClass = colorMap[template.color] || colorMap['blue'];
                
                return (
                  <div 
                    key={template.id} 
                    className="bg-white rounded-2xl shadow-sm border border-gray-200 hover:shadow-md hover:border-cyan-200 transition-all p-6 flex flex-col gap-4"
                  >
                    <div className="flex items-start justify-between">
                      <div className={`p-3 rounded-xl border ${colorClass}`}>
                        <IconComponent className="w-6 h-6" strokeWidth={2} />
                      </div>
                      <div className="flex items-center gap-2">
                        {template.is_system_template && (
                          <span className="px-2 py-1 bg-blue-100 text-blue-700 text-xs font-semibold rounded-lg">System</span>
                        )}
                        {!template.is_active && (
                          <span className="px-2 py-1 bg-gray-100 text-gray-700 text-xs font-semibold rounded-lg">Inactive</span>
                        )}
                      </div>
                    </div>

                    <div className="flex-1">
                      <h3 className="text-lg font-bold text-gray-900 mb-1">{template.display_name}</h3>
                      <p className="text-sm text-gray-500 line-clamp-2">{template.description || "No description"}</p>
                    </div>

                    <div className="py-3 border-t border-gray-100">
                      <p className="text-xs font-semibold text-gray-600 mb-2">Permissions ({template.permission_count})</p>
                      <div className="flex flex-wrap gap-1">
                        {template.permissions.slice(0, 3).map((perm) => (
                          <span 
                            key={perm.id} 
                            className="inline-flex items-center px-2 py-1 bg-gray-100 text-gray-700 text-xs font-medium rounded-lg"
                          >
                            {perm.module}:{perm.action}
                          </span>
                        ))}
                        {template.permission_count > 3 && (
                          <span className="inline-flex items-center px-2 py-1 bg-gray-100 text-gray-700 text-xs font-medium rounded-lg">
                            +{template.permission_count - 3}
                          </span>
                        )}
                      </div>
                    </div>

                    <div className="flex gap-2 pt-3 border-t border-gray-100">
                      <button
                        onClick={() => handleEdit(template)}
                        className="flex-1 px-3 py-2 bg-blue-50 text-blue-700 hover:bg-blue-100 rounded-lg text-sm font-medium transition-all flex items-center justify-center gap-2"
                        disabled={template.is_system_template}
                      >
                        <Pencil className="w-3.5 h-3.5" />
                        Edit
                      </button>
                      <button
                        onClick={() => handleDelete(template.id, template.display_name)}
                        className="flex-1 px-3 py-2 bg-red-50 text-red-700 hover:bg-red-100 rounded-lg text-sm font-medium transition-all flex items-center justify-center gap-2"
                        disabled={template.is_system_template}
                      >
                        <Trash2 className="w-3.5 h-3.5" />
                        Delete
                      </button>
                    </div>
                  </div>
                );
              })}
            </div>
          )}
        </div>
      </main>

      {showModal && (
        <RoleTemplateFormModal
          template={selectedTemplate}
          onClose={() => setShowModal(false)}
          onSave={handleSave}
        />
      )}
    </div>
  );
}
