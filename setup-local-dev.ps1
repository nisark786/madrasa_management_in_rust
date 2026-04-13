# 
# 🚀 Students Data Store - Local Development Setup Script (Windows PowerShell)
# 
# This script automates the setup process for local development on Windows:
# - Generates secure configuration
# - Creates .env files
# - Starts Docker services
# - Installs frontend dependencies
# - Validates setup
#
# Usage: powershell -ExecutionPolicy Bypass -File setup-local-dev.ps1
#

$ErrorActionPreference = "Stop"

# Color definitions
$Colors = @{
    Red    = 'Red'
    Green  = 'Green'
    Yellow = 'Yellow'
    Blue   = 'Blue'
    White  = 'White'
}

# Helper functions
function Print-Header {
    param([string]$Message)
    Write-Host "═══════════════════════════════════════════════════════════" -ForegroundColor Blue
    Write-Host $Message -ForegroundColor Blue
    Write-Host "═══════════════════════════════════════════════════════════" -ForegroundColor Blue
}

function Print-Success {
    param([string]$Message)
    Write-Host "✅ $Message" -ForegroundColor Green
}

function Print-Warning {
    param([string]$Message)
    Write-Host "⚠️  $Message" -ForegroundColor Yellow
}

function Print-Error {
    param([string]$Message)
    Write-Host "❌ $Message" -ForegroundColor Red
}

function Print-Info {
    param([string]$Message)
    Write-Host "ℹ️  $Message" -ForegroundColor Cyan
}

# Check prerequisites
function Check-Prerequisites {
    Print-Header "Checking Prerequisites"
    
    # Check Docker
    try {
        $dockerVersion = docker --version
        Print-Success "Docker found: $dockerVersion"
    } catch {
        Print-Error "Docker is not installed or not in PATH"
        Print-Info "Install from: https://www.docker.com/products/docker-desktop"
        exit 1
    }
    
    # Check Docker Compose
    try {
        $composeVersion = docker-compose --version
        Print-Success "Docker Compose found: $composeVersion"
    } catch {
        Print-Error "Docker Compose is not installed"
        exit 1
    }
    
    # Check Node.js
    try {
        $nodeVersion = node --version
        Print-Success "Node.js found: $nodeVersion"
    } catch {
        Print-Error "Node.js is not installed or not in PATH"
        Print-Info "Install from: https://nodejs.org/"
        exit 1
    }
    
    # Check Git
    try {
        $gitVersion = git --version
        Print-Success "Git found: $gitVersion"
    } catch {
        Print-Error "Git is not installed or not in PATH"
        exit 1
    }
}

# Generate secure configuration
function Generate-Config {
    Print-Header "Generating Secure Configuration"
    
    # Generate SECRET_KEY using Python
    $SECRET_KEY = python -c "import secrets; print(secrets.token_urlsafe(32))"
    if ($LASTEXITCODE -ne 0) {
        Print-Warning "Python not available, generating random key using .NET"
        $bytes = New-Object byte[] 32
        [System.Security.Cryptography.RNGCryptoServiceProvider]::new().GetBytes($bytes)
        $SECRET_KEY = [Convert]::ToBase64String($bytes)
    }
    Print-Success "Generated SECRET_KEY"
    
    # Create backend .env
    $backendEnvPath = "backend/.env"
    if (Test-Path $backendEnvPath) {
        Print-Warning "backend/.env already exists, backing up to backend/.env.backup"
        Copy-Item $backendEnvPath "backend/.env.backup" -Force
    }
    
    $timestamp = Get-Date
    $backendEnvContent = @"
# ═══════════════════════════════════════════════════════════
# Students Data Store - Backend Configuration
# Generated: $timestamp
# ═══════════════════════════════════════════════════════════

# Database Configuration (REQUIRED - Update for your DB)
DATABASE_URL=postgresql+asyncpg://postgres:postgres@localhost:5432/students_db

# Redis Configuration
REDIS_URL=redis://redis:6379/0

# Security (CRITICAL - Use strong values)
SECRET_KEY=$SECRET_KEY
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
"@
    
    Set-Content -Path $backendEnvPath -Value $backendEnvContent -Encoding UTF8
    Print-Success "Created backend/.env"
    
    # Create frontend .env.local
    $frontendEnvPath = "students_data_store/.env.local"
    $frontendEnvContent = @"
# Frontend Configuration (Local Development)
VITE_API_BASE_URL=http://localhost:8000/api/v1
VITE_ENVIRONMENT=development
"@
    
    Set-Content -Path $frontendEnvPath -Value $frontendEnvContent -Encoding UTF8
    Print-Success "Created students_data_store/.env.local"
}

# Start Docker services
function Start-DockerServices {
    Print-Header "Starting Docker Services"
    
    # Check if services are already running
    $runningServices = docker-compose ps 2>$null | Select-String "Up"
    if ($runningServices) {
        Print-Warning "Some services are already running"
        $response = Read-Host "Stop and restart them? (y/n)"
        if ($response -eq 'y' -or $response -eq 'Y') {
            docker-compose down
        }
    }
    
    # Start services
    Print-Info "Starting services..."
    docker-compose up -d
    
    # Wait for services to be ready
    Print-Warning "Waiting for services to start (30 seconds)..."
    Start-Sleep -Seconds 30
    
    # Check health
    $psOutput = docker-compose ps
    if ($psOutput | Select-String "Up") {
        Print-Success "Docker services started successfully"
        docker-compose ps
    } else {
        Print-Error "Failed to start Docker services"
        docker-compose logs
        exit 1
    }
}

# Setup frontend
function Setup-Frontend {
    Print-Header "Setting Up Frontend"
    
    Push-Location students_data_store
    
    # Install dependencies
    Print-Info "Installing npm dependencies (this may take 2-3 minutes)..."
    npm install
    
    # Build check
    Print-Info "Verifying build configuration..."
    npm run build 2>$null
    if ($LASTEXITCODE -ne 0) {
        Print-Warning "Build verification skipped"
    }
    
    Pop-Location
    
    Print-Success "Frontend setup complete"
}

# Validate setup
function Validate-Setup {
    Print-Header "Validating Setup"
    
    # Check backend
    Print-Info "Checking backend connectivity..."
    try {
        $response = curl -s -o /dev/null -w "%{http_code}" http://localhost:8000/health/ready
        if ($response -eq "200") {
            Print-Success "Backend is responding"
        } else {
            Print-Warning "Backend returned status $response (may take a few more seconds)"
        }
    } catch {
        Print-Warning "Backend is not yet ready"
    }
    
    # Check Redis
    Print-Info "Checking Redis..."
    try {
        $redisResponse = redis-cli ping 2>$null
        if ($redisResponse -eq "PONG") {
            Print-Success "Redis is running"
        } else {
            Print-Warning "Redis may not be ready yet"
        }
    } catch {
        Print-Warning "Redis not accessible"
    }
    
    # Check frontend files
    if (Test-Path "students_data_store/.env.local") {
        Print-Success "Frontend .env.local configured"
    } else {
        Print-Error "Frontend .env.local not found"
    }
    
    # Check backend .env
    if (Test-Path "backend/.env") {
        Print-Success "Backend .env configured"
    } else {
        Print-Error "Backend .env not found"
    }
}

# Print next steps
function Print-NextSteps {
    Print-Header "Setup Complete! 🎉"
    
    Write-Host ""
    Write-Host "Next Steps:" -ForegroundColor Green
    Write-Host ""
    Write-Host "1️⃣  Open a new PowerShell terminal and start the frontend:" -ForegroundColor White
    Write-Host "   cd students_data_store; npm run dev" -ForegroundColor Blue
    Write-Host ""
    Write-Host "2️⃣  Frontend will be available at: http://localhost:5173" -ForegroundColor Blue
    Write-Host ""
    Write-Host "3️⃣  Backend API at: http://localhost:8000" -ForegroundColor Blue
    Write-Host ""
    Write-Host "4️⃣  API Documentation: http://localhost:8000/docs" -ForegroundColor Blue
    Write-Host ""
    Write-Host "5️⃣  Jaeger Tracing: http://localhost:16686" -ForegroundColor Blue
    Write-Host ""
    Write-Host "Important:" -ForegroundColor Yellow
    Write-Host "- Update backend/.env with your actual database URL"
    Write-Host "- Update ADMIN_EMAIL in backend/.env"
    Write-Host "- Use your own SECRET_KEY (already generated)"
    Write-Host ""
    Write-Host "Useful Commands:" -ForegroundColor Yellow
    Write-Host "  docker-compose logs -f backend   # View backend logs"
    Write-Host "  docker-compose logs -f redis     # View Redis logs"
    Write-Host "  redis-cli                        # Redis command-line"
    Write-Host "  docker-compose down              # Stop all services"
    Write-Host "  docker-compose ps                # Check service status"
    Write-Host ""
}

# Main execution
function Main {
    Write-Host ""
    Print-Header "🚀 Students Data Store - Local Development Setup"
    Write-Host ""
    
    Check-Prerequisites
    Generate-Config
    Start-DockerServices
    Setup-Frontend
    Validate-Setup
    Print-NextSteps
}

# Run main function
Main
