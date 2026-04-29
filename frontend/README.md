# Madrasa Management System - React Frontend

Modern, responsive React frontend for the Madrasa Management System built with latest technologies.

## ✅ Latest Dependencies Installed

| Package | Version | Purpose |
|---------|---------|---------|
| **React** | 19.2.5 | UI Framework |
| **React DOM** | 19.2.5 | React Renderer |
| **React Router DOM** | 7.14.2 | Client-side Routing |
| **Zustand** | 5.0.12 | State Management |
| **Tailwind CSS** | 4.2.4 | Utility-first CSS |
| **@tailwindcss/vite** | 4.2.4 | Vite Plugin for Tailwind |
| **Lucide React** | 1.14.0 | Icon Library |
| **Vite** | 8.0.10 | Build Tool |
| **@vitejs/plugin-react** | 6.0.1 | React Plugin for Vite |

## 🚀 Quick Start

### Installation

All dependencies are pre-installed. To reinstall:

```bash
npm install
```

### Development Server

```bash
npm run dev
```

Starts dev server on `http://localhost:3000`

### Build for Production

```bash
npm run build
```

Creates optimized production build in `dist/` folder.

### Preview Production Build

```bash
npm run preview
```

## 📁 Project Structure

```
frontend/
├── src/
│   ├── components/
│   │   ├── layout/
│   │   │   └── ProtectedRoute.jsx
│   │   └── pages/
│   │       ├── LandingPage.jsx
│   │       ├── LoginPage.jsx
│   │       ├── RegisterPage.jsx
│   │       └── DashboardPage.jsx
│   ├── stores/
│   │   └── authStore.js (Zustand)
│   ├── utils/
│   │   └── apiClient.js
│   ├── hooks/
│   │   └── useAuth.js
│   ├── App.jsx
│   ├── App.css
│   ├── main.jsx
│   └── index.css (Tailwind directives)
├── .env (dev environment)
├── .env.production (prod environment)
├── vite.config.js (Vite + Tailwind config)
├── Dockerfile (multi-stage build)
└── package.json
```

## 🎨 Tailwind CSS Integration

### Implementation Method

This project uses the official **@tailwindcss/vite** plugin for optimal performance:

1. **vite.config.js** - Configured with Tailwind plugin
2. **index.css** - Imports Tailwind directives
3. **All components** - Use Tailwind utility classes

### Example Usage

```jsx
<div className="max-w-6xl mx-auto px-4 py-8 bg-gradient-to-r from-blue-600 to-blue-700 rounded-lg">
  <h1 className="text-3xl font-bold text-white mb-4">Welcome</h1>
  <button className="px-6 py-3 bg-white text-blue-600 rounded-lg font-semibold hover:bg-gray-100 transition">
    Click me
  </button>
</div>
```

## 🔐 Authentication

Uses **Zustand** with localStorage persistence:

- Auto token refresh on expiration
- JWT access/refresh token flow
- Protected routes with role-based access
- Demo credentials: `admin@example.com` / `password`

## 📡 API Integration

**apiClient.js** provides axios-like interface:

```javascript
// GET request
const data = await apiClient.get('/endpoint');

// POST request
const data = await apiClient.post('/endpoint', { key: 'value' });

// PUT request
const data = await apiClient.put('/endpoint', { key: 'value' });

// DELETE request
await apiClient.delete('/endpoint');
```

## 🎯 Pages

### Landing Page
- Hero section with CTA
- Features showcase (4 cards)
- Benefits section with statistics
- Footer with links

### Login Page
- Email/password authentication
- Demo credentials auto-filled
- Error handling
- Loading states

### Register Page
- Madrasa name setup
- Email/password with confirmation
- Form validation
- Error messages

### Dashboard
- Role-based layouts (Admin, Manager, Staff, Student)
- Quick stats cards
- Responsive sidebar navigation
- User info & logout

## 🐳 Docker Integration

Dockerfile uses multi-stage build:
- **Build stage**: Node 18 Alpine + npm install + npm run build
- **Runtime stage**: Node 18 Alpine + serve (static server)

### Build Docker Image

```bash
docker build -t madrasa-frontend .
```

### Run Docker Container

```bash
docker run -p 3000:3000 madrasa-frontend
```

## 🔗 Docker Compose

Frontend is integrated in main `docker-compose.yml`:

```bash
cd ../madrasa_management_in_rust
docker-compose build
docker-compose up --watch
```

## 📱 Responsive Breakpoints

- **Mobile**: < 640px (sm)
- **Tablet**: 640px - 1024px (md, lg)
- **Desktop**: > 1024px (xl)

All components are fully responsive with Tailwind's mobile-first approach.

## ⚙️ Environment Variables

### Development (.env)
```
VITE_API_URL=http://localhost:8080/api/v1
```

### Production (.env.production)
```
VITE_API_URL=http://localhost/api/v1
```

## 🎛️ Tailwind CSS Configuration

Tailwind runs through Vite plugin with:
- Built-in color palette
- Responsive design utilities
- Dark mode support (ready)
- Custom animations
- Automatic class detection

No separate config file needed - plugin handles it automatically!

## 🔧 Customization

### Add Custom Colors

Edit `tailwind.config.js` (if created):

```javascript
module.exports = {
  theme: {
    extend: {
      colors: {
        'custom-blue': '#1e40af',
      }
    }
  }
}
```

### Add Custom Fonts

Use `@import` in `index.css`:

```css
@import url('https://fonts.googleapis.com/css2?family=Inter:wght@400;600;700&display=swap');

@import "tailwindcss";
```

## 📚 Resources

- [React Docs](https://react.dev)
- [React Router Docs](https://reactrouter.com)
- [Tailwind CSS Docs](https://tailwindcss.com/docs)
- [Zustand Docs](https://github.com/pmndrs/zustand)
- [Lucide Icons](https://lucide.dev)
- [Vite Docs](https://vite.dev)

## ✨ Features

✅ Modern React 19 with hooks
✅ Type-safe Zustand state management
✅ Tailwind CSS for styling (latest v4.2.4)
✅ Responsive design (mobile-first)
✅ JWT authentication with refresh flow
✅ Protected routes with role-based access
✅ API client with auto-token refresh
✅ Beautiful UI components
✅ Docker support (multi-stage build)
✅ HMR (Hot Module Replacement) in development

## 🚀 Deployment

### Build for Production

```bash
npm run build
```

### Docker Build & Push

```bash
docker build -t your-registry/madrasa-frontend:latest .
docker push your-registry/madrasa-frontend:latest
```

### Docker Compose Production

```bash
docker-compose -f docker-compose.yml up -d
```

## 📝 License

MIT
