# Learn REST API

A comprehensive Rust-based REST API for user management, featuring full CRUD operations, OpenAPI documentation, and PostgreSQL database integration.

## Overview

This project demonstrates a modern REST API implementation using Rust with Axum as the web framework. It includes:
- User management (Create, Read, Update, Delete operations)
- OpenAPI documentation with Swagger UI
- PostgreSQL database integration
- Docker and Docker Compose support
- Comprehensive error handling

## Technology Stack

### Core Dependencies

**Web Framework:** `axum = "0.8.9"`
- Modern, ergonomic web framework built on Tokio
- Excellent performance and type safety
- Built-in support for async/await

**HTTP Runtime:** `tokio = { version = "1.53.1", features = ["macros", "rt-multi-thread", "net", "rt"] }`
- Asynchronous runtime for high-performance networking
- Multi-threaded runtime for optimal CPU utilization

**Database ORM:** `sqlx = {version = "0.9.0", features = ["runtime-tokio", "postgres", "macros", "tls-native-tls"]}`
- Compile-time checked SQL queries
- Full PostgreSQL support with async/await
- Type-safe database access

**Data Serialization:** `serde = {version = "1.0.229", features = ["derive"]}`
- Powerful serialization/deserialization library
- Zero-cost serialization with compile-time optimizations
- Built-in support for all primitive types

**API Documentation:** `axum-swagger-ui = "0.3.0"`
- Interactive API documentation with Swagger UI
- Real-time API exploration and testing

### Key Dependencies (Runtime)

**Error Handling & Logging:**
- `tracing = "0.1.44"` - Structured logging
- `log = "0.4.34"` - Logging facade

**Database Drivers & Utilities:**
- `sqlx-core = "0.9.0"` - Core SQLx functionality
- `sqlx-postgres = "0.9.0"` - PostgreSQL driver
- `native-tls = "0.2.18"` - TLS/SSL support

**Async Utilities:**
- `futures-util = "0.3.34"` - Async utilities
- `tokio-stream = "0.1.19"` - Async stream processing

**HTTP & Networking:**
- `hyper = "1.12.0"` - HTTP/1 and HTTP/2 implementation
- `http-body = "1.1.0"` - HTTP body types
- `tower = "0.5.3"` - Middleware and service stack

**Security & Cryptography:**
- `openssl = "0.10.81"` - OpenSSL bindings
- `sha2 = "0.11.0"` - SHA-2 hash function

**Internationalization & Localization:**
- `idna = "1.1.0"` - IDNA encoding/decoding
- `unicode-bidi = "0.3.18"` - Bidirectional text support

**Performance & Memory Management:**
- `hashbrown = "0.17.1"` - Alternative HashMap implementation
- `smallvec = "1.16.2"` - Compact vectors

**Testing & Development:**
- `dotenvy = "0.15.7"` - Environment variable loading
- `tempfile = "3.27.0"` - Temporary file creation

## Project Structure

```
learn_rest/
├── src/
│   ├── main.rs              # Application entry point
│   ├── endpoint_functions.rs # API endpoints and business logic
│   └── openapi.json         # OpenAPI specification
├── migrations/              # Database migrations
│   └── 0001_users_table.sql  # Initial schema
├── Cargo.toml              # Project configuration and dependencies
├── Cargo.lock              # Locked dependency versions
├── .env                   # Environment variables
├── Dockerfile             # Production build configuration
├── docker-compose.yaml     # Docker orchestration
└── .gitignore             # Version control ignores
```

## API Documentation

### Base URL
```
http://localhost:8000
```

### Available Endpoints

#### 1. Health Check
- **GET** `/`
- Returns: `Welcome to User management api`

#### 2. Swagger UI
- **GET** `/swagger`
- Returns: Interactive API documentation

#### 3. OpenAPI Specification
- **GET** `/swagger/openapi.json`
- Returns: Complete OpenAPI 3.0 specification

#### 4. User Operations (CRUD)

**List All Users**
- **GET** `/users`
- Response: `200 OK` with array of users

**Create User**
- **POST** `/users`
- Request Body: `UserPayload` object
- Response: `201 Created` with created user

**Get User by ID**
- **GET** `/users/{id}`
- Path Parameter: `id` (integer)
- Response: `200 OK` with user or `404 Not Found`

**Update User**
- **PUT** `/users/{id}`
- Path Parameter: `id` (integer)
- Request Body: `UserPayload` object
- Response: `200 OK` with updated user

**Delete User**
- **DELETE** `/users/{id}`
- Path Parameter: `id` (integer)
- Response: `204 No Content` or `404 Not Found`

### Data Models

#### User (Response)
```json
{
  "id": 1,
  "name": "John Doe",
  "email": "john@example.com"
}
```

#### UserPayload (Request)
```json
{
  "name": "John Doe",
  "email": "john@example.com"
}
```

## Running the Application

### Prerequisites
- Rust 1.75+ with cargo
- PostgreSQL database

### Quick Start (Development)

1. **Clone the repository**
```bash
git clone <repository-url>
cd learn_rest
```

2. **Set up environment**
```bash
cp .env.example .env  # Create from template if needed
```

3. **Start development server**
```bash
cargo run
```

### Production Setup

#### Using Docker

1. **Build and run with Docker Compose**
```bash
docker-compose up --build
```

2. **Run directly with Docker**
```bash
docker build -t learn_rest .
docker run -p 8000:8000 learn_rest
```

#### Using Podman (alternative)
```bash
podman-compose up --build
```

### Database Operations

#### Manual Migration
```bash
# Run migrations using SQLx
cargo sqlx migrate
```

#### Database URL Configuration
The application uses PostgreSQL with the following connection string format:
```
DATABASE_URL=postgres://avnadmin:password@host:port/database?sslmode=require
```

## Development Workflow

### Building
```bash
# Build for release
cargo build --release

# Build for development
cargo build
```

### Testing
```bash
# Run tests
cargo test

# Run tests with verbose output
cargo test -- --nocapture
```

### Code Quality
```bash
# Format code
cargo fmt

# Lint code
cargo clippy
```

### Database Management

#### Creating Users
```bash
curl -X POST http://localhost:8000/users \
  -H "Content-Type: application/json" \
  -d '{"name": "John Doe", "email": "john@example.com"}'
```

#### Getting Users
```bash
curl http://localhost:8000/users
```

#### Updating Users
```bash
curl -X PUT http://localhost:8000/users/1 \
  -H "Content-Type: application/json" \
  -d '{"name": "Jane Doe", "email": "jane@example.com"}'
```

#### Deleting Users
```bash
curl -X DELETE http://localhost:8000/users/1
```

## Configuration

### Environment Variables

| Variable | Description | Default |
|----------|-------------|---------|
| `DATABASE_URL` | PostgreSQL connection string | Required |

### Database Schema

The application uses a simple `users` table:

```sql
CREATE TABLE users (
    id SERIAL PRIMARY KEY,
    name TEXT NOT NULL,
    email TEXT NOT NULL UNIQUE
);
```

## Architecture

### Main Components

1. **main.rs**
   - Application entry point
   - Database connection setup
   - Migration execution
   - Axum server initialization

2. **endpoint_functions.rs**
   - API route definitions
   - User management endpoints
   - Database query handlers
   - Request/response serialization

3. **openapi.json**
   - OpenAPI 3.0 specification
   - Complete API documentation
   - Request/response schemas

### Key Design Patterns

1. **State Management**
   - Database connection pool passed to all handlers via Axum State
   - Efficient connection reuse

2. **Error Handling**
   - Comprehensive error mapping to HTTP status codes
   - Consistent error responses

3. **Serialization**
   - serde with derives for type-safe serialization
   - Separate models for request/response

4. **Routing**
   - Axum Router with clear endpoint organization
   - RESTful URL patterns

## Performance Considerations

1. **Connection Pooling**
   - SQLx manages database connection pooling
   - Efficient resource utilization

2. **Async/Await**
   - Non-blocking I/O operations
   - High concurrency support

3. **Memory Management**
   - Zero-cost abstractions through Rust's type system
   - Efficient data structures (smallvec, hashbrown)

4. **Caching**
   - Consider Redis for frequently accessed data
   - Database query optimization

## Security Best Practices

1. **Database Security**
   - SSL/TLS encryption for database connections
   - Strong passwords in environment variables
   - Regular database backups

2. **API Security**
   - Input validation and sanitization
   - Rate limiting (consider adding in production)
   - CORS configuration

3. **Environment Security**
   - Never commit database credentials
   - Use `.env` files for sensitive data
   - Consider secret management solutions

## Testing Strategy

### Unit Tests
- Test individual functions and modules
- Mock database connections
- Verify data transformation logic

### Integration Tests
- Test full API endpoints
- Database integration
- End-to-end workflow testing

### Performance Testing
- Load testing with concurrent requests
- Database performance benchmarks
- Memory usage analysis

## Troubleshooting

### Common Issues

1. **Database Connection Errors**
   ```
   Error: connection refused
   ```
   - Ensure PostgreSQL is running
   - Check database credentials
   - Verify network connectivity

2. **Migration Errors**
   ```
   Error: relation "users" already exists
   ```
   - Database already migrated
   - Reset database or use `--force` flag

3. **API Not Responding**
   - Check server logs
   - Verify port 8000 is available
   - Check firewall settings

### Debug Commands

```bash
# Enable debug logging
cargo run -- --log-level debug

# Connect to database
dbchecker <connection-string>

# Monitor database performance
pg_stat_activity
```

## Future Enhancements

1. **Authentication & Authorization**
   - JWT tokens for API protection
   - Role-based access control
   - OAuth2 integration

2. **Advanced Features**
   - Search and filtering capabilities
   - Pagination for large datasets
   - Background jobs processing

3. **Observability**
   - Distributed tracing
   - Metrics collection
   - Alerting integration

4. **Additional Endpoints**
   - Bulk operations
   - Export/import functionality
   - Webhooks support

## License

This project is licensed under the terms of the MIT License.

## Acknowledgments

- The Rust community for creating such an amazing language
- Axum and SQLx teams for excellent libraries
- Docker and containerization technologies
- PostgreSQL for reliable database operations

## Contact

For questions or issues, please check the repository issues or documentation.
