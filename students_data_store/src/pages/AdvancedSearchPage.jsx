import { useState, useEffect, useCallback } from 'react';
import { Search, Filter, X, Save, Trash2, ChevronDown } from 'lucide-react';
import api from '../api/client';
import { useAuthStore } from '../store/authStore';

export default function AdvancedSearchPage() {
  // State for search/filters
  const [searchQuery, setSearchQuery] = useState('');
  const [filters, setFilters] = useState({
    class_name: '',
    city: '',
    state: '',
    is_active: null,
  });
  const [sorting, setSorting] = useState({ sort_by: 'first_name', sort_order: 'asc' });
  const [pagination, setPagination] = useState({ page: 1, page_size: 20 });

  // State for results
  const [results, setResults] = useState(null);
  const [loading, setLoading] = useState(false);
  const [error, setError] = useState('');

  // State for suggestions
  const [suggestions, setSuggestions] = useState({
    city: [],
    class_name: [],
    state: [],
  });
  const [activeSuggestion, setActiveSuggestion] = useState(null);

  // State for saved searches
  const [savedSearches, setSavedSearches] = useState([]);
  const [showSaveDialog, setShowSaveDialog] = useState(false);
  const [saveName, setSaveName] = useState('');
  const [saveDescription, setSaveDescription] = useState('');

  const { hasPermission } = useAuthStore();

  // Check permission
  useEffect(() => {
    if (!hasPermission('students:read')) {
      setError('You do not have permission to access this page');
    }
  }, [hasPermission]);

  // Load saved searches
  const loadSavedSearches = useCallback(async () => {
    try {
      const response = await api.get('/api/v1/search/saved');
      setSavedSearches(response.data);
    } catch (err) {
      console.error('Failed to load saved searches:', err);
    }
  }, []);

  useEffect(() => {
    loadSavedSearches();
  }, [loadSavedSearches]);

  // Perform search
  const performSearch = useCallback(async () => {
    setLoading(true);
    setError('');
    try {
      const response = await api.post('/api/v1/search/students', {
        query: searchQuery || null,
        filters: Object.fromEntries(Object.entries(filters).filter(([, v]) => v !== '' && v !== null)),
        sort_by: sorting.sort_by,
        sort_order: sorting.sort_order,
        page: pagination.page,
        page_size: pagination.page_size,
      });
      setResults(response.data);
      setPagination(prev => ({ ...prev, page: response.data.page }));
    } catch (err) {
      setError(err.response?.data?.detail || 'Search failed');
      setResults(null);
    } finally {
      setLoading(false);
    }
  }, [searchQuery, filters, sorting, pagination.page_size]);

  // Get suggestions
  const getSuggestions = useCallback(async (field, query) => {
    if (!query || query.length < 2) {
      setSuggestions(prev => ({ ...prev, [field]: [] }));
      return;
    }

    try {
      const response = await api.get(`/api/v1/search/suggestions/${field}`, {
        params: { query, limit: 10 }
      });
      setSuggestions(prev => ({ ...prev, [field]: response.data.suggestions }));
    } catch (err) {
      console.error('Failed to get suggestions:', err);
    }
  }, []);

  // Handle filter change with debounce for suggestions
  const handleFilterChange = (field, value) => {
    setFilters(prev => ({ ...prev, [field]: value }));
    if (['city', 'class_name', 'state'].includes(field)) {
      getSuggestions(field, value);
    }
    setPagination(prev => ({ ...prev, page: 1 }));
  };

  // Handle search input change
  const handleSearchChange = (e) => {
    setSearchQuery(e.target.value);
    setPagination(prev => ({ ...prev, page: 1 }));
  };

  // Handle pagination
  const handlePageChange = (newPage) => {
    setPagination(prev => ({ ...prev, page: newPage }));
  };

  // Save search
  const handleSaveSearch = async () => {
    if (!saveName.trim()) {
      setError('Search name is required');
      return;
    }

    try {
      await api.post('/api/v1/search/saved', {
        name: saveName,
        description: saveDescription,
        filters,
        sort_by: sorting.sort_by,
        sort_order: sorting.sort_order,
      });
      setSaveName('');
      setSaveDescription('');
      setShowSaveDialog(false);
      loadSavedSearches();
    } catch (err) {
      setError(err.response?.data?.detail || 'Failed to save search');
    }
  };

  // Load saved search
  const handleLoadSavedSearch = async (savedSearch) => {
    setFilters(savedSearch.filters);
    setSorting({ sort_by: savedSearch.sort_by, sort_order: savedSearch.sort_order });
    setPagination(prev => ({ ...prev, page: 1 }));
  };

  // Delete saved search
  const handleDeleteSavedSearch = async (searchId) => {
    if (!window.confirm('Delete this saved search?')) return;

    try {
      await api.delete(`/api/v1/search/saved/${searchId}`);
      loadSavedSearches();
    } catch (err) {
      setError(err.response?.data?.detail || 'Failed to delete search');
    }
  };

  // Clear filters
  const handleClearFilters = () => {
    setSearchQuery('');
    setFilters({ class_name: '', city: '', state: '', is_active: null });
    setSorting({ sort_by: 'first_name', sort_order: 'asc' });
    setPagination(prev => ({ ...prev, page: 1 }));
  };

  return (
    <div className="min-h-screen bg-gray-50 py-8">
      <div className="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8">
        {/* Header */}
        <div className="mb-8">
          <h1 className="text-3xl font-bold text-gray-900 mb-2">Advanced Search</h1>
          <p className="text-gray-600">Search and filter students by multiple criteria</p>
        </div>

        {error && (
          <div className="mb-6 p-4 bg-red-50 border border-red-200 rounded-lg flex items-start gap-3">
            <div className="text-red-600 font-semibold">Error</div>
            <div className="text-red-700 flex-1">{error}</div>
          </div>
        )}

        <div className="grid grid-cols-1 lg:grid-cols-4 gap-6">
          {/* Sidebar - Filters & Saved Searches */}
          <div className="lg:col-span-1 space-y-6">
            {/* Saved Searches */}
            {savedSearches.length > 0 && (
              <div className="bg-white rounded-lg shadow p-6 border border-gray-200">
                <h2 className="text-lg font-bold text-gray-900 mb-4 flex items-center gap-2">
                  <Search className="w-5 h-5 text-indigo-600" />
                  Saved Searches
                </h2>
                <div className="space-y-2">
                  {savedSearches.map(search => (
                    <div
                      key={search.id}
                      className="p-3 bg-gray-50 rounded-lg hover:bg-indigo-50 transition-colors"
                    >
                      <div className="flex items-start justify-between gap-2 mb-2">
                        <button
                          onClick={() => handleLoadSavedSearch(search)}
                          className="flex-1 text-left text-sm font-medium text-indigo-600 hover:text-indigo-700"
                        >
                          {search.name}
                        </button>
                        <button
                          onClick={() => handleDeleteSavedSearch(search.id)}
                          className="text-red-500 hover:text-red-700 transition-colors p-1"
                        >
                          <Trash2 className="w-4 h-4" />
                        </button>
                      </div>
                      {search.description && (
                        <p className="text-xs text-gray-600">{search.description}</p>
                      )}
                    </div>
                  ))}
                </div>
              </div>
            )}

            {/* Filter Panel */}
            <div className="bg-white rounded-lg shadow p-6 border border-gray-200">
              <h2 className="text-lg font-bold text-gray-900 mb-4 flex items-center gap-2">
                <Filter className="w-5 h-5 text-indigo-600" />
                Filters
              </h2>

              <div className="space-y-4">
                {/* Class Name */}
                <div>
                  <label className="block text-sm font-medium text-gray-700 mb-1">
                    Class
                  </label>
                  <input
                    type="text"
                    value={filters.class_name}
                    onChange={(e) => handleFilterChange('class_name', e.target.value)}
                    placeholder="e.g., 10A"
                    className="w-full px-3 py-2 border border-gray-300 rounded-lg focus:ring-indigo-500 focus:border-indigo-500"
                  />
                  {suggestions.class_name.length > 0 && (
                    <div className="mt-1 bg-white border border-gray-300 rounded-lg shadow-sm max-h-40 overflow-y-auto">
                      {suggestions.class_name.map((s, i) => (
                        <button
                          key={i}
                          onClick={() => handleFilterChange('class_name', s)}
                          className="w-full text-left px-3 py-2 hover:bg-gray-100 text-sm"
                        >
                          {s}
                        </button>
                      ))}
                    </div>
                  )}
                </div>

                {/* City */}
                <div>
                  <label className="block text-sm font-medium text-gray-700 mb-1">
                    City
                  </label>
                  <input
                    type="text"
                    value={filters.city}
                    onChange={(e) => handleFilterChange('city', e.target.value)}
                    placeholder="City name"
                    className="w-full px-3 py-2 border border-gray-300 rounded-lg focus:ring-indigo-500 focus:border-indigo-500"
                  />
                  {suggestions.city.length > 0 && (
                    <div className="mt-1 bg-white border border-gray-300 rounded-lg shadow-sm max-h-40 overflow-y-auto">
                      {suggestions.city.map((s, i) => (
                        <button
                          key={i}
                          onClick={() => handleFilterChange('city', s)}
                          className="w-full text-left px-3 py-2 hover:bg-gray-100 text-sm"
                        >
                          {s}
                        </button>
                      ))}
                    </div>
                  )}
                </div>

                {/* State */}
                <div>
                  <label className="block text-sm font-medium text-gray-700 mb-1">
                    State
                  </label>
                  <input
                    type="text"
                    value={filters.state}
                    onChange={(e) => handleFilterChange('state', e.target.value)}
                    placeholder="State name"
                    className="w-full px-3 py-2 border border-gray-300 rounded-lg focus:ring-indigo-500 focus:border-indigo-500"
                  />
                  {suggestions.state.length > 0 && (
                    <div className="mt-1 bg-white border border-gray-300 rounded-lg shadow-sm max-h-40 overflow-y-auto">
                      {suggestions.state.map((s, i) => (
                        <button
                          key={i}
                          onClick={() => handleFilterChange('state', s)}
                          className="w-full text-left px-3 py-2 hover:bg-gray-100 text-sm"
                        >
                          {s}
                        </button>
                      ))}
                    </div>
                  )}
                </div>

                {/* Active Status */}
                <div>
                  <label className="block text-sm font-medium text-gray-700 mb-1">
                    Status
                  </label>
                  <select
                    value={filters.is_active === null ? '' : filters.is_active ? 'true' : 'false'}
                    onChange={(e) => handleFilterChange('is_active', e.target.value === '' ? null : e.target.value === 'true')}
                    className="w-full px-3 py-2 border border-gray-300 rounded-lg focus:ring-indigo-500 focus:border-indigo-500"
                  >
                    <option value="">All</option>
                    <option value="true">Active</option>
                    <option value="false">Inactive</option>
                  </select>
                </div>

                {/* Clear Filters */}
                <button
                  onClick={handleClearFilters}
                  className="w-full mt-6 px-4 py-2 bg-gray-100 text-gray-700 rounded-lg hover:bg-gray-200 transition-colors font-medium flex items-center justify-center gap-2"
                >
                  <X className="w-4 h-4" />
                  Clear Filters
                </button>
              </div>
            </div>
          </div>

          {/* Main Content - Search & Results */}
          <div className="lg:col-span-3 space-y-6">
            {/* Search Bar */}
            <div className="bg-white rounded-lg shadow p-6 border border-gray-200">
              <div className="flex gap-3">
                <div className="flex-1 relative">
                  <Search className="absolute left-3 top-3 w-5 h-5 text-gray-400" />
                  <input
                    type="text"
                    value={searchQuery}
                    onChange={handleSearchChange}
                    onKeyPress={(e) => e.key === 'Enter' && performSearch()}
                    placeholder="Search by name, email, admission no., roll no..."
                    className="w-full pl-10 pr-4 py-2 border border-gray-300 rounded-lg focus:ring-indigo-500 focus:border-indigo-500"
                  />
                </div>
                <button
                  onClick={performSearch}
                  disabled={loading}
                  className="px-6 py-2 bg-indigo-600 text-white rounded-lg hover:bg-indigo-700 transition-colors font-medium disabled:opacity-50"
                >
                  {loading ? 'Searching...' : 'Search'}
                </button>
              </div>
            </div>

            {/* Sorting & Save Controls */}
            {results && (
              <div className="bg-white rounded-lg shadow p-4 border border-gray-200 flex items-center justify-between">
                <div className="flex items-center gap-4">
                  <select
                    value={sorting.sort_by}
                    onChange={(e) => setSorting(prev => ({ ...prev, sort_by: e.target.value }))}
                    className="px-3 py-2 border border-gray-300 rounded-lg focus:ring-indigo-500 focus:border-indigo-500 text-sm"
                  >
                    <option value="first_name">First Name</option>
                    <option value="last_name">Last Name</option>
                    <option value="email">Email</option>
                    <option value="class_name">Class</option>
                    <option value="created_at">Created Date</option>
                  </select>

                  <button
                    onClick={() => setSorting(prev => ({
                      ...prev,
                      sort_order: prev.sort_order === 'asc' ? 'desc' : 'asc'
                    }))}
                    className="px-3 py-2 border border-gray-300 rounded-lg hover:bg-gray-50 text-sm font-medium"
                  >
                    {sorting.sort_order === 'asc' ? '↑ Ascending' : '↓ Descending'}
                  </button>
                </div>

                <button
                  onClick={() => setShowSaveDialog(true)}
                  className="px-4 py-2 bg-indigo-100 text-indigo-700 rounded-lg hover:bg-indigo-200 transition-colors font-medium flex items-center gap-2"
                >
                  <Save className="w-4 h-4" />
                  Save Search
                </button>
              </div>
            )}

            {/* Results */}
            {results && (
              <div className="space-y-4">
                <div className="text-sm text-gray-600">
                  Found <span className="font-bold text-gray-900">{results.total}</span> results
                </div>

                <div className="bg-white rounded-lg shadow border border-gray-200 overflow-hidden">
                  <div className="overflow-x-auto">
                    <table className="w-full">
                      <thead className="bg-gray-50 border-b border-gray-200">
                        <tr>
                          <th className="px-6 py-3 text-left text-xs font-bold text-gray-700 uppercase">Name</th>
                          <th className="px-6 py-3 text-left text-xs font-bold text-gray-700 uppercase">Email</th>
                          <th className="px-6 py-3 text-left text-xs font-bold text-gray-700 uppercase">Class</th>
                          <th className="px-6 py-3 text-left text-xs font-bold text-gray-700 uppercase">City</th>
                          <th className="px-6 py-3 text-left text-xs font-bold text-gray-700 uppercase">Status</th>
                        </tr>
                      </thead>
                      <tbody>
                        {results.students.map((student) => (
                          <tr key={student.id} className="border-b border-gray-100 hover:bg-gray-50 transition-colors">
                            <td className="px-6 py-4 text-sm font-medium text-gray-900">
                              {student.first_name} {student.last_name}
                            </td>
                            <td className="px-6 py-4 text-sm text-gray-600">{student.email}</td>
                            <td className="px-6 py-4 text-sm text-gray-600">{student.class_name}</td>
                            <td className="px-6 py-4 text-sm text-gray-600">{student.city || '—'}</td>
                            <td className="px-6 py-4 text-sm">
                              <span className={`inline-flex items-center px-2.5 py-0.5 rounded-full text-xs font-bold ${
                                student.is_active
                                  ? 'bg-green-100 text-green-700'
                                  : 'bg-gray-100 text-gray-600'
                              }`}>
                                {student.is_active ? 'Active' : 'Inactive'}
                              </span>
                            </td>
                          </tr>
                        ))}
                      </tbody>
                    </table>
                  </div>

                  {results.students.length === 0 && (
                    <div className="text-center py-8 text-gray-500">
                      No students found matching your criteria
                    </div>
                  )}
                </div>

                {/* Pagination */}
                {results.total_pages > 1 && (
                  <div className="flex items-center justify-center gap-2">
                    <button
                      onClick={() => handlePageChange(results.page - 1)}
                      disabled={!results.has_previous}
                      className="px-4 py-2 border border-gray-300 rounded-lg disabled:opacity-50 disabled:cursor-not-allowed hover:bg-gray-50"
                    >
                      Previous
                    </button>
                    <span className="text-sm text-gray-600">
                      Page {results.page} of {results.total_pages}
                    </span>
                    <button
                      onClick={() => handlePageChange(results.page + 1)}
                      disabled={!results.has_next}
                      className="px-4 py-2 border border-gray-300 rounded-lg disabled:opacity-50 disabled:cursor-not-allowed hover:bg-gray-50"
                    >
                      Next
                    </button>
                  </div>
                )}
              </div>
            )}

            {!results && !loading && !error && (
              <div className="text-center py-12">
                <Search className="w-12 h-12 text-gray-300 mx-auto mb-4" />
                <p className="text-gray-500">Enter search criteria and click Search to get started</p>
              </div>
            )}
          </div>
        </div>

        {/* Save Search Dialog */}
        {showSaveDialog && (
          <div className="fixed inset-0 bg-black/50 flex items-center justify-center z-50">
            <div className="bg-white rounded-lg shadow-lg max-w-md w-full mx-4 p-6">
              <h3 className="text-lg font-bold text-gray-900 mb-4">Save Search</h3>

              <div className="space-y-4 mb-6">
                <div>
                  <label className="block text-sm font-medium text-gray-700 mb-1">
                    Search Name *
                  </label>
                  <input
                    type="text"
                    value={saveName}
                    onChange={(e) => setSaveName(e.target.value)}
                    placeholder="e.g., Class 10A Students"
                    className="w-full px-3 py-2 border border-gray-300 rounded-lg focus:ring-indigo-500 focus:border-indigo-500"
                  />
                </div>

                <div>
                  <label className="block text-sm font-medium text-gray-700 mb-1">
                    Description
                  </label>
                  <textarea
                    value={saveDescription}
                    onChange={(e) => setSaveDescription(e.target.value)}
                    placeholder="Optional description"
                    rows={3}
                    className="w-full px-3 py-2 border border-gray-300 rounded-lg focus:ring-indigo-500 focus:border-indigo-500"
                  />
                </div>
              </div>

              <div className="flex gap-3">
                <button
                  onClick={() => setShowSaveDialog(false)}
                  className="flex-1 px-4 py-2 border border-gray-300 text-gray-700 rounded-lg hover:bg-gray-50 font-medium"
                >
                  Cancel
                </button>
                <button
                  onClick={handleSaveSearch}
                  className="flex-1 px-4 py-2 bg-indigo-600 text-white rounded-lg hover:bg-indigo-700 font-medium"
                >
                  Save
                </button>
              </div>
            </div>
          </div>
        )}
      </div>
    </div>
  );
}
