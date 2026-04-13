import { useState, useEffect } from 'react';
import api from '../../api/client';
import { X, Loader } from 'lucide-react';

export default function RoleTemplateFormModal({ template, onClose, onSave }) {
  const [formData, setFormData] = useState({
    name: '',
    display_name: '',
    description: '',
    icon: 'eye',
    color: 'blue',
    permission_ids: [],
  });

  const [allPermissions, setAllPermissions] = useState([]);
  const [loading, setLoading] = useState(false);
  const [fetchingPerms, setFetchingPerms] = useState(true);

  const icons = ['eye', 'edit', 'users', 'list', 'book'];
  const colors = ['blue', 'green', 'purple', 'orange', 'red'];

  useEffect(() => {
    // Fetch all permissions
    api.get('/permissions/')
      .then(({ data }) => setAllPermissions(data))
      .catch(err => console.error('Failed to fetch permissions', err))
      .finally(() => setFetchingPerms(false));

    // If editing, populate form
    if (template) {
      setFormData({
        name: template.name,
        display_name: template.display_name,
        description: template.description || '',
        icon: template.icon || 'eye',
        color: template.color || 'blue',
        permission_ids: template.permissions.map(p => p.id),
      });
    }
  }, [template]);

  const handlePermissionChange = (permId) => {
    setFormData(prev => {
      const newIds = prev.permission_ids.includes(permId)
        ? prev.permission_ids.filter(id => id !== permId)
        : [...prev.permission_ids, permId];
      return { ...prev, permission_ids: newIds };
    });
  };

  const handleSubmit = async (e) => {
    e.preventDefault();
    setLoading(true);

    try {
      if (template) {
        // Update
        await api.patch(`/role-templates/${template.id}`, {
          display_name: formData.display_name,
          description: formData.description,
          icon: formData.icon,
          color: formData.color,
          permission_ids: formData.permission_ids,
        });
      } else {
        // Create
        await api.post('/role-templates/', formData);
      }
      onSave();
    } catch (err) {
      alert(err.response?.data?.detail || `Failed to ${template ? 'update' : 'create'} template`);
    } finally {
      setLoading(false);
    }
  };

  return (
    <div className="fixed inset-0 bg-black/40 backdrop-blur-sm flex items-center justify-center z-50 p-4">
      <div className="bg-white rounded-2xl shadow-2xl max-w-2xl w-full max-h-[90vh] overflow-y-auto">
        
        <div className="sticky top-0 bg-white border-b border-gray-200 p-6 flex items-center justify-between">
          <h2 className="text-xl font-bold text-gray-900">
            {template ? 'Edit Template' : 'Create Role Template'}
          </h2>
          <button 
            onClick={onClose}
            className="p-2 hover:bg-gray-100 rounded-lg transition-colors"
          >
            <X className="w-5 h-5 text-gray-500" />
          </button>
        </div>

        <form onSubmit={handleSubmit} className="p-6 space-y-6">
          
          {/* Basic Info */}
          <div className="space-y-4">
            <h3 className="text-sm font-semibold text-gray-900">Basic Information</h3>
            
            <div>
              <label className="block text-sm font-medium text-gray-700 mb-2">Name *</label>
              <input
                type="text"
                value={formData.name}
                onChange={(e) => setFormData(prev => ({ ...prev, name: e.target.value }))}
                disabled={!!template}
                placeholder="e.g., viewer, editor"
                className="w-full px-4 py-2 border border-gray-300 rounded-lg text-sm focus:outline-none focus:ring-2 focus:ring-cyan-500 disabled:bg-gray-100 disabled:text-gray-500"
                required
              />
              {template && <p className="text-xs text-gray-500 mt-1">Cannot change name when editing</p>}
            </div>

            <div>
              <label className="block text-sm font-medium text-gray-700 mb-2">Display Name *</label>
              <input
                type="text"
                value={formData.display_name}
                onChange={(e) => setFormData(prev => ({ ...prev, display_name: e.target.value }))}
                placeholder="e.g., Student Viewer"
                className="w-full px-4 py-2 border border-gray-300 rounded-lg text-sm focus:outline-none focus:ring-2 focus:ring-cyan-500"
                required
              />
            </div>

            <div>
              <label className="block text-sm font-medium text-gray-700 mb-2">Description</label>
              <textarea
                value={formData.description}
                onChange={(e) => setFormData(prev => ({ ...prev, description: e.target.value }))}
                placeholder="Describe what this template is for"
                rows="3"
                className="w-full px-4 py-2 border border-gray-300 rounded-lg text-sm focus:outline-none focus:ring-2 focus:ring-cyan-500 resize-none"
              />
            </div>
          </div>

          {/* Styling */}
          <div className="space-y-4">
            <h3 className="text-sm font-semibold text-gray-900">Styling</h3>
            
            <div className="grid grid-cols-2 gap-4">
              <div>
                <label className="block text-sm font-medium text-gray-700 mb-2">Icon</label>
                <select
                  value={formData.icon}
                  onChange={(e) => setFormData(prev => ({ ...prev, icon: e.target.value }))}
                  className="w-full px-4 py-2 border border-gray-300 rounded-lg text-sm focus:outline-none focus:ring-2 focus:ring-cyan-500"
                >
                  {icons.map(icon => (
                    <option key={icon} value={icon}>{icon}</option>
                  ))}
                </select>
              </div>

              <div>
                <label className="block text-sm font-medium text-gray-700 mb-2">Color</label>
                <select
                  value={formData.color}
                  onChange={(e) => setFormData(prev => ({ ...prev, color: e.target.value }))}
                  className="w-full px-4 py-2 border border-gray-300 rounded-lg text-sm focus:outline-none focus:ring-2 focus:ring-cyan-500"
                >
                  {colors.map(color => (
                    <option key={color} value={color}>{color}</option>
                  ))}
                </select>
              </div>
            </div>
          </div>

          {/* Permissions */}
          <div className="space-y-4">
            <h3 className="text-sm font-semibold text-gray-900">Permissions</h3>
            
            {fetchingPerms ? (
              <div className="flex justify-center py-8">
                <Loader className="w-5 h-5 animate-spin text-cyan-600" />
              </div>
            ) : (
              <div className="space-y-3 max-h-64 overflow-y-auto bg-gray-50 p-4 rounded-lg border border-gray-200">
                {allPermissions.length === 0 ? (
                  <p className="text-sm text-gray-500">No permissions available</p>
                ) : (
                  allPermissions.map(perm => (
                    <label key={perm.id} className="flex items-center gap-3 cursor-pointer p-2 hover:bg-gray-100 rounded-lg transition-colors">
                      <input
                        type="checkbox"
                        checked={formData.permission_ids.includes(perm.id)}
                        onChange={() => handlePermissionChange(perm.id)}
                        className="w-4 h-4 rounded border-gray-300 text-cyan-600 focus:ring-cyan-500"
                      />
                      <div className="flex-1">
                        <p className="text-sm font-medium text-gray-900">{perm.name}</p>
                        <p className="text-xs text-gray-500">{perm.description}</p>
                      </div>
                    </label>
                  ))
                )}
              </div>
            )}
          </div>

          {/* Actions */}
          <div className="flex gap-3 pt-4 border-t border-gray-200">
            <button
              type="button"
              onClick={onClose}
              className="px-5 py-2.5 rounded-lg border border-gray-300 text-sm font-medium text-gray-700 hover:bg-gray-50 transition-colors"
            >
              Cancel
            </button>
            <button
              type="submit"
              disabled={loading}
              className="flex-1 px-5 py-2.5 bg-cyan-600 text-white rounded-lg text-sm font-bold hover:bg-cyan-700 transition-colors disabled:opacity-50 flex items-center justify-center gap-2"
            >
              {loading && <Loader className="w-4 h-4 animate-spin" />}
              {template ? 'Update Template' : 'Create Template'}
            </button>
          </div>
        </form>
      </div>
    </div>
  );
}
