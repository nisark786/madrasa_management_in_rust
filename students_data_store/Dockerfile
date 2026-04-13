# ═══════════════════════════════════════════════════════════════
# HIGH PRIORITY FIX #7: Frontend Production Build
# ═══════════════════════════════════════════════════════════════
# Multi-stage build that:
# 1. Builds optimized production bundle with React + Vite
# 2. Serves static files with 'serve' (lightweight production server)
# 3. Removes dev dependencies for smaller image
# ═══════════════════════════════════════════════════════════════

# Stage 1: Build stage
FROM node:20-alpine AS builder

WORKDIR /app

# Copy package files
COPY package*.json ./

# Install dependencies (all deps needed for build)
RUN npm install

# Copy source code
COPY . .

# Build production bundle
# Vite will create optimized dist/ folder with:
# - Minified JavaScript
# - CSS bundling
# - Asset optimization
# - Source maps for debugging
RUN npm run build

# ─────────────────────────────────────────────────────────────────
# Stage 2: Production runtime
FROM node:20-alpine

WORKDIR /app

# Install 'serve' - lightweight production HTTP server
# - Handles routing for SPA
# - Caches static files
# - Gzip compression
# - ~20MB total image size
RUN npm install -g serve

# Copy only the built artifacts from builder
COPY --from=builder /app/dist ./dist

# Copy public assets if they exist
COPY --from=builder /app/public ./public 2>/dev/null || true

# Expose port (same as dev but for production)
EXPOSE 5173

# ─────────────────────────────────────────────────────────────────
# Production health check
# Verifies the app is serving requests
HEALTHCHECK --interval=30s --timeout=10s --start-period=5s --retries=3 \
    CMD wget --no-verbose --tries=1 --spider http://localhost:5173 || exit 1

# ─────────────────────────────────────────────────────────────────
# Start production server
# - Serves dist/ on port 5173
# - Single Page App routing enabled
# - No debug output
CMD ["serve", "-s", "dist", "-l", "5173"]
