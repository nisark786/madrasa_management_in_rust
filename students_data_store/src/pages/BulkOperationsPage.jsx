import { useState, useRef } from 'react';
import api from '../api/client';
import Sidebar from '../components/layout/Sidebar';
import { 
  Upload, 
  Download, 
  Trash2, 
  RefreshCw,
  AlertCircle,
  CheckCircle,
  FileText,
  Loader,
  FileDown
} from 'lucide-react';

export default function BulkOperationsPage() {
  const [activeTab, setActiveTab] = useState('export'); // 'export' or 'import'
  const [loading, setLoading] = useState(false);
  const [importing, setImporting] = useState(false);
  const [selectedStudents, setSelectedStudents] = useState(new Set());
  const [importResult, setImportResult] = useState(null);
  const fileInputRef = useRef(null);

  // Handle export CSV
  const handleExportCSV = async () => {
    setLoading(true);
    try {
      const response = await api.get('/bulk/students/export/csv', {
        responseType: 'blob'
      });
      
      // Create download link
      const url = window.URL.createObjectURL(new Blob([response.data]));
      const link = document.createElement('a');
      link.href = url;
      link.setAttribute('download', `students_export_${new Date().toISOString().split('T')[0]}.csv`);
      document.body.appendChild(link);
      link.click();
      link.parentNode.removeChild(link);
      window.URL.revokeObjectURL(url);
    } catch (err) {
      alert('Failed to export students: ' + (err.response?.data?.detail || err.message));
    } finally {
      setLoading(false);
    }
  };

  // Handle download template
  const handleDownloadTemplate = async () => {
    setLoading(true);
    try {
      const response = await api.get('/bulk/students/import-template', {
        responseType: 'blob'
      });
      
      const url = window.URL.createObjectURL(new Blob([response.data]));
      const link = document.createElement('a');
      link.href = url;
      link.setAttribute('download', 'students_template.csv');
      document.body.appendChild(link);
      link.click();
      link.parentNode.removeChild(link);
      window.URL.revokeObjectURL(url);
    } catch (err) {
      alert('Failed to download template: ' + (err.response?.data?.detail || err.message));
    } finally {
      setLoading(false);
    }
  };

  // Handle file selection and import
  const handleFileChange = async (e) => {
    const file = e.target.files?.[0];
    if (!file) return;

    if (!file.name.endsWith('.csv')) {
      alert('Please select a CSV file');
      return;
    }

    setImporting(true);
    setImportResult(null);

    try {
      const formData = new FormData();
      formData.append('file', file);

      const response = await api.post('/bulk/students/import/csv', formData, {
        headers: { 'Content-Type': 'multipart/form-data' }
      });

      setImportResult(response.data);
    } catch (err) {
      alert('Failed to import: ' + (err.response?.data?.detail || err.message));
    } finally {
      setImporting(false);
      if (fileInputRef.current) fileInputRef.current.value = '';
    }
  };

  return (
    <div className="flex min-h-screen bg-gray-50/50 font-sans antialiased">
      <Sidebar />
      <main className="flex-1 p-8 md:p-12 overflow-y-auto min-w-0">
        <div className="flex flex-col gap-8 max-w-5xl mx-auto">
          
          {/* Header */}
          <div className="flex flex-wrap items-center justify-between gap-4 pb-6 border-b border-gray-200">
            <div className="flex items-center gap-4">
              <div className="bg-emerald-600 p-3 rounded-2xl shadow-lg shadow-emerald-100/50">
                <Upload className="w-6 h-6 text-white" />
              </div>
              <div>
                <h1 className="text-2xl font-bold text-gray-900 leading-tight tracking-tight">Bulk Operations</h1>
                <p className="text-sm text-gray-400 font-medium">Import, export, and bulk manage students</p>
              </div>
            </div>
          </div>

          {/* Tabs */}
          <div className="flex gap-2 border-b border-gray-200">
            <button
              onClick={() => setActiveTab('export')}
              className={`px-4 py-3 text-sm font-semibold border-b-2 transition-colors ${
                activeTab === 'export'
                  ? 'border-emerald-600 text-emerald-600'
                  : 'border-transparent text-gray-500 hover:text-gray-900'
              }`}
            >
              <FileDown className="w-4 h-4 inline mr-2" />
              Export
            </button>
            <button
              onClick={() => setActiveTab('import')}
              className={`px-4 py-3 text-sm font-semibold border-b-2 transition-colors ${
                activeTab === 'import'
                  ? 'border-emerald-600 text-emerald-600'
                  : 'border-transparent text-gray-500 hover:text-gray-900'
              }`}
            >
              <Upload className="w-4 h-4 inline mr-2" />
              Import
            </button>
          </div>

          {/* Export Tab */}
          {activeTab === 'export' && (
            <div className="bg-white rounded-2xl shadow-sm border border-gray-200 p-8 space-y-6">
              <div className="flex items-center gap-4 p-4 bg-blue-50 border border-blue-200 rounded-xl">
                <AlertCircle className="w-5 h-5 text-blue-600 flex-shrink-0" />
                <p className="text-sm text-blue-900">
                  Export all students to a CSV file for backup or external processing.
                </p>
              </div>

              <div className="flex gap-3">
                <button
                  onClick={handleExportCSV}
                  disabled={loading}
                  className="px-6 py-3 bg-emerald-600 text-white rounded-lg text-sm font-bold hover:bg-emerald-700 transition-colors disabled:opacity-50 flex items-center gap-2"
                >
                  {loading ? (
                    <Loader className="w-4 h-4 animate-spin" />
                  ) : (
                    <Download className="w-4 h-4" />
                  )}
                  Export All Students as CSV
                </button>
              </div>

              <div className="bg-gray-50 p-4 rounded-lg border border-gray-200 text-sm text-gray-600">
                <p className="font-medium mb-2">What will be exported:</p>
                <ul className="list-disc list-inside space-y-1 text-xs">
                  <li>Student name, email, contact information</li>
                  <li>Class, roll number, admission number</li>
                  <li>Address and enrollment date</li>
                  <li>Active status and notes</li>
                  <li>Creation and update timestamps</li>
                </ul>
              </div>
            </div>
          )}

          {/* Import Tab */}
          {activeTab === 'import' && (
            <div className="space-y-6">
              {/* Template Download */}
              <div className="bg-white rounded-2xl shadow-sm border border-gray-200 p-8 space-y-4">
                <div className="flex items-center justify-between">
                  <div>
                    <h3 className="text-lg font-bold text-gray-900">Download CSV Template</h3>
                    <p className="text-sm text-gray-500 mt-1">Download a template to see the required format</p>
                  </div>
                  <button
                    onClick={handleDownloadTemplate}
                    disabled={loading}
                    className="px-4 py-2 bg-blue-100 text-blue-700 hover:bg-blue-200 rounded-lg text-sm font-medium transition-colors disabled:opacity-50 flex items-center gap-2"
                  >
                    {loading ? (
                      <Loader className="w-4 h-4 animate-spin" />
                    ) : (
                      <FileText className="w-4 h-4" />
                    )}
                    Download Template
                  </button>
                </div>
              </div>

              {/* Import Section */}
              <div className="bg-white rounded-2xl shadow-sm border border-gray-200 p-8 space-y-6">
                <div className="flex items-center gap-4 p-4 bg-purple-50 border border-purple-200 rounded-xl">
                  <AlertCircle className="w-5 h-5 text-purple-600 flex-shrink-0" />
                  <p className="text-sm text-purple-900">
                    Required columns: first_name, last_name, email
                  </p>
                </div>

                <div className="border-2 border-dashed border-gray-300 rounded-xl p-8 text-center">
                  <input
                    ref={fileInputRef}
                    type="file"
                    accept=".csv"
                    onChange={handleFileChange}
                    disabled={importing}
                    className="hidden"
                  />
                  <button
                    onClick={() => fileInputRef.current?.click()}
                    disabled={importing}
                    className="mx-auto mb-4 px-6 py-3 bg-emerald-600 text-white rounded-lg text-sm font-bold hover:bg-emerald-700 transition-colors disabled:opacity-50 flex items-center gap-2"
                  >
                    {importing ? (
                      <>
                        <Loader className="w-4 h-4 animate-spin" />
                        Importing...
                      </>
                    ) : (
                      <>
                        <Upload className="w-4 h-4" />
                        Select CSV File
                      </>
                    )}
                  </button>
                  <p className="text-sm text-gray-600">or drag and drop your CSV file here</p>
                </div>

                {/* Import Result */}
                {importResult && (
                  <div className="space-y-4 p-4 bg-gray-50 rounded-lg border border-gray-200">
                    <div className="flex items-center justify-between">
                      <h4 className="font-bold text-gray-900">Import Results</h4>
                      <span className={`px-3 py-1 rounded-full text-xs font-semibold ${
                        importResult.failed === 0
                          ? 'bg-green-100 text-green-700'
                          : 'bg-amber-100 text-amber-700'
                      }`}>
                        {importResult.successful > 0 ? '✓ Complete' : 'No records imported'}
                      </span>
                    </div>

                    <div className="grid grid-cols-3 gap-4">
                      <div className="bg-white p-4 rounded-lg border border-gray-200">
                        <p className="text-xs text-gray-600 font-medium">Total Rows</p>
                        <p className="text-2xl font-bold text-gray-900">{importResult.total_rows}</p>
                      </div>
                      <div className="bg-white p-4 rounded-lg border border-green-200">
                        <p className="text-xs text-green-600 font-medium flex items-center gap-1">
                          <CheckCircle className="w-3 h-3" />
                          Successful
                        </p>
                        <p className="text-2xl font-bold text-green-700">{importResult.successful}</p>
                      </div>
                      <div className="bg-white p-4 rounded-lg border border-red-200">
                        <p className="text-xs text-red-600 font-medium flex items-center gap-1">
                          <AlertCircle className="w-3 h-3" />
                          Failed
                        </p>
                        <p className="text-2xl font-bold text-red-700">{importResult.failed}</p>
                      </div>
                    </div>

                    {importResult.errors && importResult.errors.length > 0 && (
                      <div className="bg-red-50 p-4 rounded-lg border border-red-200">
                        <p className="text-sm font-bold text-red-900 mb-2">Errors:</p>
                        <div className="space-y-1 max-h-40 overflow-y-auto">
                          {importResult.errors.map((err, idx) => (
                            <p key={idx} className="text-xs text-red-800">{err}</p>
                          ))}
                        </div>
                      </div>
                    )}
                  </div>
                )}
              </div>
            </div>
          )}
        </div>
      </main>
    </div>
  );
}
