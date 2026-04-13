import { useState, useEffect, useCallback } from 'react';
import { FileText, Plus, Edit2, Trash2, Download, Settings } from 'lucide-react';
import api from '../api/client';
import { useAuthStore } from '../store/authStore';

export default function ReportsPage() {
  const [activeTab, setActiveTab] = useState('templates'); // templates, generate
  const [templates, setTemplates] = useState([]);
  const [availableFields, setAvailableFields] = useState([]);
  const [loading, setLoading] = useState(false);
  const [error, setError] = useState('');

  // Template form state
  const [editingTemplate, setEditingTemplate] = useState(null);
  const [formData, setFormData] = useState({
    name: '',
    description: '',
    fields: [],
    export_format: 'excel',
    filters: {},
    group_by: '',
    include_summary: false,
    sort_by: 'first_name',
    sort_order: 'asc',
    is_default: false,
  });

  const [showDeleteConfirm, setShowDeleteConfirm] = useState(null);
  const { hasPermission } = useAuthStore();

  // Load templates
  const loadTemplates = useCallback(async () => {
    setLoading(true);
    try {
      const response = await api.get('/api/v1/reports/templates');
      setTemplates(response.data);
    } catch (err) {
      setError('Failed to load templates');
    } finally {
      setLoading(false);
    }
  }, []);

  // Load available fields
  const loadAvailableFields = useCallback(async () => {
    try {
      const response = await api.get('/api/v1/reports/available-fields');
      setAvailableFields(response.data.fields);
    } catch (err) {
      console.error('Failed to load fields:', err);
    }
  }, []);

  useEffect(() => {
    if (!hasPermission('students:read')) {
      setError('You do not have permission to access this page');
      return;
    }
    loadTemplates();
    loadAvailableFields();
  }, [hasPermission, loadTemplates, loadAvailableFields]);

  // Handle form submission
  const handleSaveTemplate = async (e) => {
    e.preventDefault();
    setLoading(true);

    try {
      if (editingTemplate) {
        await api.put(`/api/v1/reports/templates/${editingTemplate.id}`, formData);
      } else {
        await api.post('/api/v1/reports/templates', formData);
      }
      loadTemplates();
      resetForm();
    } catch (err) {
      setError(err.response?.data?.detail || 'Failed to save template');
    } finally {
      setLoading(false);
    }
  };

  // Handle edit template
  const handleEditTemplate = (template) => {
    setEditingTemplate(template);
    setFormData({
      name: template.name,
      description: template.description || '',
      fields: template.fields,
      export_format: template.export_format,
      filters: template.filters,
      group_by: template.group_by || '',
      include_summary: template.include_summary,
      sort_by: template.sort_by,
      sort_order: template.sort_order,
      is_default: template.is_default,
    });
    setActiveTab('templates');
  };

  // Handle delete template
  const handleDeleteTemplate = async (id) => {
    try {
      await api.delete(`/api/v1/reports/templates/${id}`);
      loadTemplates();
      setShowDeleteConfirm(null);
    } catch (err) {
      setError('Failed to delete template');
    }
  };

  // Handle generate report
  const handleGenerateReport = async (templateId) => {
    setLoading(true);
    try {
      const response = await api.post(
        `/api/v1/reports/templates/${templateId}/generate`,
        { template_id: templateId },
        { responseType: 'blob' }
      );
      
      // Get filename from response headers
      const contentDisposition = response.headers['content-disposition'];
      const filename = contentDisposition
        ? contentDisposition.split('filename=')[1].replace(/"/g, '')
        : `report_${Date.now()}.xlsx`;
      
      // Download file
      const url = window.URL.createObjectURL(response.data);
      const link = document.createElement('a');
      link.href = url;
      link.setAttribute('download', filename);
      document.body.appendChild(link);
      link.click();
      document.body.removeChild(link);
    } catch (err) {
      setError('Failed to generate report');
    } finally {
      setLoading(false);
    }
  };

  // Reset form
  const resetForm = () => {
    setEditingTemplate(null);
    setFormData({
      name: '',
      description: '',
      fields: [],
      export_format: 'excel',
      filters: {},
      group_by: '',
      include_summary: false,
      sort_by: 'first_name',
      sort_order: 'asc',
      is_default: false,
    });
  };

  // Handle field toggle
  const toggleField = (fieldName) => {
    setFormData(prev => ({
      ...prev,
      fields: prev.fields.includes(fieldName)
        ? prev.fields.filter(f => f !== fieldName)
        : [...prev.fields, fieldName]
    }));
  };

  return (
    <div className="min-h-screen bg-gray-50 py-8">
      <div className="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8">
        {/* Header */}
        <div className="mb-8">
          <h1 className="text-3xl font-bold text-gray-900 mb-2">Reports & Export</h1>
          <p className="text-gray-600">Create, manage, and generate student reports</p>
        </div>

        {error && (
          <div className="mb-6 p-4 bg-red-50 border border-red-200 rounded-lg">
            <div className="text-red-700">{error}</div>
          </div>
        )}

        {/* Tabs */}
        <div className="mb-6 border-b border-gray-200">
          <div className="flex gap-8">
            <button
              onClick={() => setActiveTab('templates')}
              className={`pb-3 px-1 border-b-2 font-medium transition-colors ${
                activeTab === 'templates'
                  ? 'border-indigo-600 text-indigo-600'
                  : 'border-transparent text-gray-600 hover:text-gray-900'
              }`}
            >
              Report Templates
            </button>
            <button
              onClick={() => setActiveTab('generate')}
              className={`pb-3 px-1 border-b-2 font-medium transition-colors ${
                activeTab === 'generate'
                  ? 'border-indigo-600 text-indigo-600'
                  : 'border-transparent text-gray-600 hover:text-gray-900'
              }`}
            >
              Quick Export
            </button>
          </div>
        </div>

        {/* Templates Tab */}
        {activeTab === 'templates' && (
          <div className="grid grid-cols-1 lg:grid-cols-3 gap-6">
            {/* Template List */}
            <div className="lg:col-span-2">
              <div className="space-y-4">
                {templates.map(template => (
                  <div
                    key={template.id}
                    className="bg-white rounded-lg shadow p-6 border border-gray-200 hover:shadow-lg transition-shadow"
                  >
                    <div className="flex items-start justify-between gap-4 mb-4">
                      <div className="flex-1">
                        <h3 className="text-lg font-bold text-gray-900">{template.name}</h3>
                        {template.description && (
                          <p className="text-sm text-gray-600 mt-1">{template.description}</p>
                        )}
                      </div>
                      <div className="flex gap-2">
                        <button
                          onClick={() => handleEditTemplate(template)}
                          className="p-2 text-indigo-600 hover:bg-indigo-50 rounded-lg transition-colors"
                          title="Edit template"
                        >
                          <Edit2 className="w-4 h-4" />
                        </button>
                        <button
                          onClick={() => setShowDeleteConfirm(template.id)}
                          className="p-2 text-red-600 hover:bg-red-50 rounded-lg transition-colors"
                          title="Delete template"
                        >
                          <Trash2 className="w-4 h-4" />
                        </button>
                      </div>
                    </div>

                    <div className="grid grid-cols-2 sm:grid-cols-4 gap-4 text-sm mb-4">
                      <div>
                        <div className="text-gray-500 text-xs uppercase font-semibold">Format</div>
                        <div className="text-gray-900 font-medium capitalize">{template.export_format}</div>
                      </div>
                      <div>
                        <div className="text-gray-500 text-xs uppercase font-semibold">Fields</div>
                        <div className="text-gray-900 font-medium">{template.fields.length}</div>
                      </div>
                      <div>
                        <div className="text-gray-500 text-xs uppercase font-semibold">Sort By</div>
                        <div className="text-gray-900 font-medium text-xs">{template.sort_by}</div>
                      </div>
                      <div>
                        <div className="text-gray-500 text-xs uppercase font-semibold">Status</div>
                        <div className="text-gray-900 font-medium">
                          {template.is_default ? '★ Default' : 'Custom'}
                        </div>
                      </div>
                    </div>

                    <button
                      onClick={() => handleGenerateReport(template.id)}
                      disabled={loading}
                      className="w-full px-4 py-2 bg-indigo-600 text-white rounded-lg hover:bg-indigo-700 transition-colors font-medium disabled:opacity-50 flex items-center justify-center gap-2"
                    >
                      <Download className="w-4 h-4" />
                      Generate Report
                    </button>
                  </div>
                ))}

                {templates.length === 0 && !editingTemplate && (
                  <div className="text-center py-12 bg-white rounded-lg border border-gray-200">
                    <FileText className="w-12 h-12 text-gray-300 mx-auto mb-4" />
                    <p className="text-gray-500">No templates yet. Create one to get started.</p>
                  </div>
                )}
              </div>
            </div>

            {/* Template Form */}
            <div className="bg-white rounded-lg shadow p-6 border border-gray-200">
              <h2 className="text-lg font-bold text-gray-900 mb-6 flex items-center gap-2">
                <Settings className="w-5 h-5 text-indigo-600" />
                {editingTemplate ? 'Edit Template' : 'New Template'}
              </h2>

              <form onSubmit={handleSaveTemplate} className="space-y-4">
                {/* Name */}
                <div>
                  <label className="block text-sm font-medium text-gray-700 mb-1">
                    Template Name *
                  </label>
                  <input
                    type="text"
                    value={formData.name}
                    onChange={(e) => setFormData(prev => ({ ...prev, name: e.target.value }))}
                    className="w-full px-3 py-2 border border-gray-300 rounded-lg focus:ring-indigo-500 focus:border-indigo-500"
                    required
                  />
                </div>

                {/* Description */}
                <div>
                  <label className="block text-sm font-medium text-gray-700 mb-1">
                    Description
                  </label>
                  <textarea
                    value={formData.description}
                    onChange={(e) => setFormData(prev => ({ ...prev, description: e.target.value }))}
                    rows={2}
                    className="w-full px-3 py-2 border border-gray-300 rounded-lg focus:ring-indigo-500 focus:border-indigo-500"
                  />
                </div>

                {/* Export Format */}
                <div>
                  <label className="block text-sm font-medium text-gray-700 mb-1">
                    Export Format *
                  </label>
                  <select
                    value={formData.export_format}
                    onChange={(e) => setFormData(prev => ({ ...prev, export_format: e.target.value }))}
                    className="w-full px-3 py-2 border border-gray-300 rounded-lg focus:ring-indigo-500 focus:border-indigo-500"
                    required
                  >
                    <option value="csv">CSV</option>
                    <option value="excel">Excel</option>
                    <option value="pdf">PDF</option>
                  </select>
                </div>

                {/* Fields Selection */}
                <div>
                  <label className="block text-sm font-medium text-gray-700 mb-2">
                    Fields to Include ({formData.fields.length}) *
                  </label>
                  <div className="max-h-48 overflow-y-auto border border-gray-300 rounded-lg p-2 space-y-1">
                    {availableFields.map(field => (
                      <label key={field.name} className="flex items-center gap-2 cursor-pointer p-1 hover:bg-gray-50 rounded">
                        <input
                          type="checkbox"
                          checked={formData.fields.includes(field.name)}
                          onChange={() => toggleField(field.name)}
                          className="w-4 h-4 text-indigo-600 border-gray-300 rounded"
                        />
                        <span className="text-sm text-gray-700">{field.label}</span>
                      </label>
                    ))}
                  </div>
                </div>

                {/* Sort By */}
                <div>
                  <label className="block text-sm font-medium text-gray-700 mb-1">
                    Sort By
                  </label>
                  <select
                    value={formData.sort_by}
                    onChange={(e) => setFormData(prev => ({ ...prev, sort_by: e.target.value }))}
                    className="w-full px-3 py-2 border border-gray-300 rounded-lg focus:ring-indigo-500 focus:border-indigo-500"
                  >
                    {availableFields.map(field => (
                      <option key={field.name} value={field.name}>{field.label}</option>
                    ))}
                  </select>
                </div>

                {/* Sort Order */}
                <div>
                  <label className="block text-sm font-medium text-gray-700 mb-1">
                    Sort Order
                  </label>
                  <div className="flex gap-2">
                    {['asc', 'desc'].map(order => (
                      <label key={order} className="flex items-center gap-2 cursor-pointer">
                        <input
                          type="radio"
                          name="sort_order"
                          value={order}
                          checked={formData.sort_order === order}
                          onChange={(e) => setFormData(prev => ({ ...prev, sort_order: e.target.value }))}
                          className="w-4 h-4"
                        />
                        <span className="text-sm text-gray-700 capitalize">{order === 'asc' ? '↑ Ascending' : '↓ Descending'}</span>
                      </label>
                    ))}
                  </div>
                </div>

                {/* Default Template */}
                <div className="flex items-center gap-2">
                  <input
                    type="checkbox"
                    id="is_default"
                    checked={formData.is_default}
                    onChange={(e) => setFormData(prev => ({ ...prev, is_default: e.target.checked }))}
                    className="w-4 h-4"
                  />
                  <label htmlFor="is_default" className="text-sm font-medium text-gray-700 cursor-pointer">
                    Set as default template
                  </label>
                </div>

                {/* Submit Buttons */}
                <div className="flex gap-2 pt-4">
                  <button
                    type="submit"
                    disabled={loading || !formData.name || formData.fields.length === 0}
                    className="flex-1 px-4 py-2 bg-indigo-600 text-white rounded-lg hover:bg-indigo-700 transition-colors font-medium disabled:opacity-50"
                  >
                    {editingTemplate ? 'Update' : 'Create'}
                  </button>
                  {editingTemplate && (
                    <button
                      type="button"
                      onClick={resetForm}
                      className="flex-1 px-4 py-2 bg-gray-200 text-gray-700 rounded-lg hover:bg-gray-300 transition-colors font-medium"
                    >
                      Cancel
                    </button>
                  )}
                </div>
              </form>
            </div>
          </div>
        )}

        {/* Delete Confirmation */}
        {showDeleteConfirm && (
          <div className="fixed inset-0 bg-black/50 flex items-center justify-center z-50">
            <div className="bg-white rounded-lg shadow-lg max-w-md w-full mx-4 p-6">
              <h3 className="text-lg font-bold text-gray-900 mb-4">Delete Template?</h3>
              <p className="text-gray-600 mb-6">
                This action cannot be undone. The template will be permanently deleted.
              </p>
              <div className="flex gap-3">
                <button
                  onClick={() => setShowDeleteConfirm(null)}
                  className="flex-1 px-4 py-2 border border-gray-300 text-gray-700 rounded-lg hover:bg-gray-50 font-medium"
                >
                  Cancel
                </button>
                <button
                  onClick={() => handleDeleteTemplate(showDeleteConfirm)}
                  className="flex-1 px-4 py-2 bg-red-600 text-white rounded-lg hover:bg-red-700 font-medium"
                >
                  Delete
                </button>
              </div>
            </div>
          </div>
        )}
      </div>
    </div>
  );
}
