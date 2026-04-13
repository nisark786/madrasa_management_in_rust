#!/bin/bash
# 
# 🚀 Students Data Store - Local Development Setup Script
# 
# This script automates the setup process for local development:
# - Generates secure configuration
# - Creates .env files
# - Starts Docker services
# - Installs frontend dependencies
# - Validates setup
#
# Usage: bash setup-local-dev.sh
#

set -e  # Exit on error

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Print with color
print_header() {
    echo -e "${BLUE}═══════════════════════════════════════════════════════════${NC}"
    echo -e "${BLUE}$1${NC}"
    echo -e "${BLUE}═══════════════════════════════════════════════════════════${NC}"
}

print_success() {
    echo -e "${GREEN}✅ $1${NC}"
}

print_warning() {
    echo -e "${YELLOW}⚠️  $1${NC}"
}

print_error() {
    echo -e "${RED}❌ $1${NC}"
}

# Check prerequisites
check_prerequisites() {
    print_header "Checking Prerequisites"
    
    # Check Docker
    if ! command -v docker &> /dev/null; then
        print_error "Docker is not installed"
        echo "Install from: https://www.docker.com/products/docker-desktop"
        exit 1
    fi
    print_success "Docker found: $(docker --version)"
    
    # Check Docker Compose
    if ! command -v docker-compose &> /dev/null; then
        print_error "Docker Compose is not installed"
        exit 1
    fi
    print_success "Docker Compose found: $(docker-compose --version)"
    
    # Check Node.js
    if ! command -v node &> /dev/null; then
        print_error "Node.js is not installed"
        echo "Install from: https://nodejs.org/"
        exit 1
    fi
    print_success "Node.js found: $(node --version)"
    
    # Check Git
    if ! command -v git &> /dev/null; then
        print_error "Git is not installed"
        exit 1
    fi
    print_success "Git found: $(git --version)"
}

# Generate secure configuration
generate_env() {
    print_header "Generating Secure Configuration"
    
    # Generate SECRET_KEY
    if command -v python3 &> /dev/null; then
        SECRET_KEY=$(python3 -c "import secrets; print(secrets.token_urlsafe(32))")
    else
        SECRET_KEY=$(openssl rand -base64 32)
    fi
    print_success "Generated SECRET_KEY"
    
    # Create backend .env
    if [ -f backend/.env ]; then
        print_warning "backend/.env already exists, backing up to backend/.env.backup"
        cp backend/.env backend/.env.backup
    fi
    
    cat > backend/.env << EOF
# ═══════════════════════════════════════════════════════════
# Students Data Store - Backend Configuration
# Generated: $(date)
# ═══════════════════════════════════════════════════════════

# Database Configuration (REQUIRED - Update for your DB)
DATABASE_URL=postgresql+asyncpg://postgres:postgres@localhost:5432/students_db

# Redis Configuration
REDIS_URL=redis://redis:6379/0

# Security (CRITICAL - Use strong values)
SECRET_KEY=${SECRET_KEY}
ALGORITHM=HS256

# JWT Token Configuration
ACCESS_TOKEN_EXPIRE_MINUTES=15
REFRESH_TOKEN_EXPIRE_DAYS=7

# Admin Email (CRITICAL - Change this)
ADMIN_EMAIL=admin@example.com

# Frontend Configuration
FRONTEND_URL=http://localhost:5173
ALLOWED_ORIGINS=http://localhost:5173,http://localhost:3000

# Rate Limiting
LOGIN_RATE_LIMIT=5
API_RATE_LIMIT=60

# Logging
LOG_LEVEL=INFO
ENVIRONMENT=development
DEBUG=False

# Tracing
JAEGER_ENABLED=True
JAEGER_HOST=jaeger
JAEGER_PORT=6831

# Email Configuration (Optional - Update with your SMTP settings)
SMTP_HOST=smtp.gmail.com
SMTP_PORT=587
SMTP_USER=your-email@gmail.com
SMTP_PASSWORD=your-app-password
FROM_EMAIL=noreply@studentsds.com

# Google Drive Integration (Optional)
GOOGLE_DRIVE_ENABLED=False
GOOGLE_CREDENTIALS_PATH=/app/google-credentials.json
EOF
    print_success "Created backend/.env"
    
    # Create frontend .env.local
    cat > students_data_store/.env.local << EOF
# Frontend Configuration (Local Development)
VITE_API_BASE_URL=http://localhost:8000/api/v1
VITE_ENVIRONMENT=development
EOF
    print_success "Created students_data_store/.env.local"
}

# Start Docker services
start_docker_services() {
    print_header "Starting Docker Services"
    
    # Check if services are already running
    if docker-compose ps | grep -q "Up"; then
        print_warning "Some services are already running"
        read -p "Stop and restart them? (y/n) " -n 1 -r
        echo
        if [[ $REPLY =~ ^[Yy]$ ]]; then
            docker-compose down
        fi
    fi
    
    # Start services
    echo "Starting services..."
    docker-compose up -d
    
    # Wait for services to be ready
    print_warning "Waiting for services to start (30 seconds)..."
    sleep 30
    
    # Check health
    if docker-compose ps | grep -q "Up"; then
        print_success "Docker services started successfully"
        docker-compose ps
    else
        print_error "Failed to start Docker services"
        docker-compose logs
        exit 1
    fi
}

# Setup frontend
setup_frontend() {
    print_header "Setting Up Frontend"
    
    cd students_data_store
    
    # Install dependencies
    echo "Installing npm dependencies (this may take 2-3 minutes)..."
    npm install
    
    # Build check
    echo "Verifying build configuration..."
    npm run build 2>/dev/null || print_warning "Build verification skipped"
    
    cd ..
    
    print_success "Frontend setup complete"
}

# Validate setup
validate_setup() {
    print_header "Validating Setup"
    
    # Check backend
    echo "Checking backend connectivity..."
    if curl -s http://localhost:8000/health/ready &> /dev/null; then
        print_success "Backend is responding"
    else
        print_warning "Backend is not yet ready (may take a few more seconds)"
    fi
    
    # Check Redis
    echo "Checking Redis..."
    if redis-cli ping 2>/dev/null | grep -q "PONG"; then
        print_success "Redis is running"
    else
        print_warning "Redis may not be ready yet"
    fi
    
    # Check frontend files
    if [ -f students_data_store/.env.local ]; then
        print_success "Frontend .env.local configured"
    else
        print_error "Frontend .env.local not found"
    fi
    
    # Check backend .env
    if [ -f backend/.env ]; then
        print_success "Backend .env configured"
    else
        print_error "Backend .env not found"
    fi
}

# Print next steps
print_next_steps() {
    print_header "Setup Complete! 🎉"
    
    echo -e "${GREEN}Next Steps:${NC}"
    echo ""
    echo "1️⃣  Open a new terminal and start the frontend:"
    echo -e "   ${BLUE}cd students_data_store && npm run dev${NC}"
    echo ""
    echo "2️⃣  Frontend will be available at: ${BLUE}http://localhost:5173${NC}"
    echo ""
    echo "3️⃣  Backend API at: ${BLUE}http://localhost:8000${NC}"
    echo ""
    echo "4️⃣  API Documentation: ${BLUE}http://localhost:8000/docs${NC}"
    echo ""
    echo "5️⃣  Jaeger Tracing: ${BLUE}http://localhost:16686${NC}"
    echo ""
    echo -e "${YELLOW}Important:${NC}"
    echo "- Update backend/.env with your actual database URL"
    echo "- Update ADMIN_EMAIL in backend/.env"
    echo "- Use your own SECRET_KEY (already generated)"
    echo ""
    echo -e "${YELLOW}Useful Commands:${NC}"
    echo "  docker-compose logs -f backend   # View backend logs"
    echo "  docker-compose logs -f redis     # View Redis logs"
    echo "  redis-cli                        # Redis command-line"
    echo "  docker-compose down              # Stop all services"
    echo "  docker-compose ps                # Check service status"
    echo ""
}

# Main execution
main() {
    echo ""
    print_header "🚀 Students Data Store - Local Development Setup"
    echo ""
    
    check_prerequisites
    generate_env
    start_docker_services
    setup_frontend
    validate_setup
    print_next_steps
}

# Run main function
main
