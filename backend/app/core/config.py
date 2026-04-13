from pydantic_settings import BaseSettings
from pydantic import Field, field_validator
import sys


class Settings(BaseSettings):
    # ═══════════════════════════════════════════════════════════════
    # Database Configuration - REQUIRED
    # ═══════════════════════════════════════════════════════════════
    DATABASE_URL: str = Field(
        ...,
        description="PostgreSQL async connection string",
        examples=["postgresql+asyncpg://user:password@localhost:5432/students_db"]
    )
    
    # Database Connection Pool Configuration
    DB_POOL_SIZE: int = 20                    # Connections to keep in pool
    DB_MAX_OVERFLOW: int = 10                 # Additional connections during peaks
    DB_POOL_TIMEOUT: int = 10                 # Timeout in seconds to get connection
    DB_POOL_RECYCLE: int = 900                # Recycle connections every 15 minutes
    
    # ═══════════════════════════════════════════════════════════════
    # Redis Configuration - REQUIRED
    # ═══════════════════════════════════════════════════════════════
    REDIS_URL: str = Field(
        default="redis://redis:6379",
        description="Redis connection URL for caching"
    )
    REDIS_MAX_CONNECTIONS: int = 20
    
    # ═══════════════════════════════════════════════════════════════
    # Security Settings - CRITICAL: Never use default in production
    # ═══════════════════════════════════════════════════════════════
    SECRET_KEY: str = Field(
        ...,
        min_length=32,
        description="Cryptographically secure random key (32+ characters)",
        examples=["3P_4k9-x8zL2mQ1vF6bH7jY0cN5sT2wR3u4pE9dX"]
    )
    
    ALGORITHM: str = Field(
        default="HS256",
        description="JWT algorithm (HS256 for symmetric, RS256 for asymmetric)"
    )
    
    # ═══════════════════════════════════════════════════════════════
    # Token Expiration
    # ═══════════════════════════════════════════════════════════════
    ACCESS_TOKEN_EXPIRE_MINUTES: int = 15
    REFRESH_TOKEN_EXPIRE_DAYS: int = 7
    
    # ═══════════════════════════════════════════════════════════════
    # Frontend Configuration
    # ═══════════════════════════════════════════════════════════════
    FRONTEND_URL: str = Field(
        default="http://localhost:5173",
        description="Frontend URL for CORS and redirects"
    )
    
    ALLOWED_ORIGINS: str = Field(
        default="http://localhost:5173,http://localhost:3000",
        description="Comma-separated list of allowed origins for CORS"
    )
    
    # ═══════════════════════════════════════════════════════════════
    # Rate Limiting (requests per minute)
    # ═══════════════════════════════════════════════════════════════
    LOGIN_RATE_LIMIT: int = 5
    API_RATE_LIMIT: int = 60
    
    # ═══════════════════════════════════════════════════════════════
    # Admin Configuration
    # ═══════════════════════════════════════════════════════════════
    ADMIN_EMAIL: str = Field(
        ...,
        description="Email for initial admin account",
        examples=["admin@example.com"]
    )
    
    ADMIN_PASSWORD: str = Field(
        ...,
        description="Password for initial admin account (required, must be strong)",
        examples=["your-secure-password-here"]
    )
    
    # ═══════════════════════════════════════════════════════════════
    # Email / SMTP Configuration
    # ═══════════════════════════════════════════════════════════════
    SMTP_HOST: str = Field(
        default="smtp.gmail.com",
        description="SMTP server hostname"
    )
    SMTP_PORT: int = Field(
        default=587,
        description="SMTP server port (587 for TLS, 465 for SSL)"
    )
    SMTP_USER: str = Field(
        default="",
        description="SMTP username (often same as from_email)"
    )
    SMTP_PASSWORD: str = Field(
        default="",
        description="SMTP password (Gmail App Password for Gmail)"
    )
    FROM_EMAIL: str = Field(
        default="noreply@studentsds.com",
        description="Email address to send from"
    )
    FROM_NAME: str = Field(
        default="Students Data Store",
        description="Display name for emails"
    )
    
    # Email Settings
    EMAIL_ENABLED: bool = True
    EMAIL_VERIFICATION_REQUIRED: bool = False
    
    # ═══════════════════════════════════════════════════════════════
    # Environment & Logging
    # ═══════════════════════════════════════════════════════════════
    ENVIRONMENT: str = Field(
        default="development",
        description="Environment name (development, staging, production)"
    )
    DEBUG: bool = Field(
        default=False,
        description="Enable debug mode (should be False in production)"
    )
    LOG_LEVEL: str = Field(
        default="INFO",
        description="Logging level (DEBUG, INFO, WARNING, ERROR, CRITICAL)"
    )
    
    # ═══════════════════════════════════════════════════════════════
    # Jaeger / OpenTelemetry Tracing
    # ═══════════════════════════════════════════════════════════════
    JAEGER_ENABLED: bool = True
    JAEGER_HOST: str = "jaeger"
    JAEGER_PORT: int = 6831
    
    # ═══════════════════════════════════════════════════════════════
    # Google Drive Configuration
    # ═══════════════════════════════════════════════════════════════
    GOOGLE_DRIVE_ENABLED: bool = False
    GOOGLE_CREDENTIALS_PATH: str = "google-credentials.json"
    GOOGLE_DRIVE_FOLDER_ID: str = ""
    GOOGLE_DRIVE_BACKUP_FOLDER_NAME: str = "Students Data Store Backups"

    class Config:
        env_file = ".env"
        case_sensitive = True
        extra = "forbid"  # Reject unknown environment variables
    
    # ═══════════════════════════════════════════════════════════════
    # VALIDATORS - Ensure configuration is valid
    # ═══════════════════════════════════════════════════════════════
    
    @field_validator('SECRET_KEY', mode='after')
    @classmethod
    def validate_secret_key(cls, v: str) -> str:
        """
        CRITICAL FIX #1: Validate SECRET_KEY
        - Must be at least 32 characters (256 bits)
        - Cannot be the default insecure value
        """
        if not v or len(v) < 32:
            raise ValueError(
                '❌ SECURITY ERROR: SECRET_KEY must be at least 32 characters.\n'
                '   Generate one: python -c "import secrets; print(secrets.token_urlsafe(32))"\n'
                '   Then set it in your .env file.'
            )
        
        # Reject known insecure defaults
        insecure_defaults = [
            "supersecretkey-change-in-production",
            "change-me",
            "secret",
            "12345678",
            "supersecret",
        ]
        
        if v.lower() in insecure_defaults:
            raise ValueError(
                '❌ SECURITY ERROR: SECRET_KEY contains an insecure default value!\n'
                '   Generate a new secure key: python -c "import secrets; print(secrets.token_urlsafe(32))"\n'
                '   Update your .env file with the generated key.'
            )
        
        return v
    
    @field_validator('DATABASE_URL', mode='after')
    @classmethod
    def validate_database_url(cls, v: str) -> str:
        """
        Validate DATABASE_URL format
        """
        if not v:
            raise ValueError('DATABASE_URL is required')
        
        if not v.startswith('postgresql+asyncpg://'):
            raise ValueError(
                '❌ DATABASE_URL must use async driver: postgresql+asyncpg://\n'
                f'   Current value: {v}\n'
                f'   Expected format: postgresql+asyncpg://user:password@host:5432/dbname'
            )
        
        return v
    
    @field_validator('ADMIN_EMAIL', mode='after')
    @classmethod
    def validate_admin_email(cls, v: str) -> str:
        """
        Validate ADMIN_EMAIL is a valid email
        """
        if not v or '@' not in v or '.' not in v:
            raise ValueError(
                '❌ ADMIN_EMAIL must be a valid email address\n'
                f'   Current value: {v}\n'
                f'   Example: admin@example.com'
            )
        
        return v
    
    @field_validator('ADMIN_PASSWORD', mode='after')
    @classmethod
    def validate_admin_password(cls, v: str) -> str:
        """
        CRITICAL FIX #2: Validate ADMIN_PASSWORD
        - Must be provided (no defaults)
        - Must not be the default value
        - Should be strong (enforced by user)
        """
        if not v:
            raise ValueError(
                '❌ SECURITY ERROR: ADMIN_PASSWORD is required but not set.\n'
                '   Generate a strong password:\n'
                '   python -c "import secrets; print(secrets.token_urlsafe(16))"\n'
                '   Then set ADMIN_PASSWORD in your .env file.'
            )
        
        # Reject known insecure defaults
        insecure_defaults = [
            "change-me-in-production",
            "change-me",
            "admin",
            "password",
            "12345678",
            "admin123",
            "test",
            "demo",
        ]
        
        if v.lower() in insecure_defaults:
            raise ValueError(
                '❌ SECURITY ERROR: ADMIN_PASSWORD contains an insecure default value!\n'
                '   Generate a new secure password:\n'
                '   python -c "import secrets; print(secrets.token_urlsafe(16))"\n'
                '   Update ADMIN_PASSWORD in your .env file with the generated password.'
            )
        
        return v
    
    @field_validator('ENVIRONMENT', mode='after')
    @classmethod
    def validate_environment(cls, v: str) -> str:
        """
        Validate ENVIRONMENT is a known value
        """
        valid_envs = ['development', 'staging', 'production', 'testing']
        if v not in valid_envs:
            raise ValueError(
                f'❌ ENVIRONMENT must be one of: {", ".join(valid_envs)}\n'
                f'   Current value: {v}'
            )
        
        return v
    
    @field_validator('LOG_LEVEL', mode='after')
    @classmethod
    def validate_log_level(cls, v: str) -> str:
        """
        Validate LOG_LEVEL is a known value
        """
        valid_levels = ['DEBUG', 'INFO', 'WARNING', 'ERROR', 'CRITICAL']
        if v.upper() not in valid_levels:
            raise ValueError(
                f'❌ LOG_LEVEL must be one of: {", ".join(valid_levels)}\n'
                f'   Current value: {v}'
            )
        
        return v.upper()


# ═══════════════════════════════════════════════════════════════
# Initialize Settings and validate configuration
# ═══════════════════════════════════════════════════════════════
try:
    settings = Settings()
    print("✅ Configuration validated successfully")
except Exception as e:
    print(f"\n{str(e)}\n")
    sys.exit(1)

