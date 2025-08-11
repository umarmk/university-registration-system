# University Registration System v2.0.0

A modern, full-stack student registration system built with Rust (Actix Web) backend and Next.js frontend, featuring PostgreSQL database and professional UI/UX design.

## Features

### Student Management

- **Complete CRUD Operations**: Create, read, update, and delete student records
- **Professional Form Design**: Modern modal-based forms with real-time validation
- **Rich Data Display**: Student avatars, contact information, and course badges
- **Responsive Tables**: Mobile-friendly data tables with pagination

### Authentication & Security

- **JWT-based Authentication**: Secure token-based authentication system
- **Role-based Access Control**: Admin and user roles with appropriate permissions
- **User Registration**: Complete signup flow with email validation
- **Session Management**: Secure session handling with NextAuth.js
- **CORS Protection**: Properly configured cross-origin resource sharing

### Modern UI/UX

- **Professional Design**: Clean, academic-focused interface design
- **Responsive Layout**: Works seamlessly on desktop, tablet, and mobile
- **Loading States**: Professional spinners and feedback indicators
- **Error Handling**: Comprehensive error messages and validation
- **Empty States**: Friendly messages when no data is available
- **Accessibility**: WCAG-compliant design with proper ARIA labels

## Technology Stack

### Backend (Rust/Actix Web)

- **RESTful API**: Complete student management endpoints
- **Diesel ORM**: Type-safe database operations with PostgreSQL
- **JWT Authentication**: Secure token-based authentication
- **Actix Web**: High-performance async web framework
- **Database Migrations**: Automated schema management
- **CORS Middleware**: Cross-origin request handling
- **Logging**: Comprehensive request/response logging

### Frontend (Next.js 14)

- **React 18**: Modern React with hooks and TypeScript
- **Next.js API Routes**: Server-side API handling
- **Tailwind CSS**: Utility-first CSS framework for styling
- **NextAuth.js**: Authentication and session management
- **React Hook Form**: Form validation and state management
- **React Query**: Data fetching and caching
- **Responsive Design**: Mobile-first approach

## Quick Start

### Prerequisites

- **Rust** (1.70+) and Cargo
- **PostgreSQL** (13+)
- **Node.js** (18+) and npm/yarn
- **Docker** (optional, for containerization)

### 1. Clone and Setup

```bash
git clone https://github.com/umarmk/university-registration-system.git
cd university-registration-system
```

### 2. Database Setup

```bash
# Install Diesel CLI
cargo install diesel_cli --no-default-features --features postgres

# Create database and run migrations
diesel setup
diesel migration run
```

### 3. Backend Setup

```bash
# Create .env file (copy from .env.example)
cp .env.example .env

# Start the Rust backend
cargo run
```

The backend will be available at `http://localhost:8081`

### 4. Frontend Setup

```bash
# Navigate to frontend directory
cd frontend

# Install dependencies
npm install

# Create environment file (copy from .env.local.example)
cp .env.local.example .env.local

# Start the Next.js frontend
npm run dev
```

The frontend will be available at `http://localhost:3000`

### 5. Access the Application

1. **Landing Page**: Visit `http://localhost:3000`
2. **Register**: Create a new account at `/auth/register`
3. **Login**: Sign in at `/auth/login`
4. **Dashboard**: Manage students at `/dashboard`

## Docker Setup (Alternative)

```bash
# Build and run with Docker Compose
docker-compose up --build
```

## API Documentation

### Authentication Endpoints

- `POST /api/auth/register` - Register new user
- `POST /api/auth/login` - Login with email and password
- `GET /api/auth/session` - Get current session

### Student Management Endpoints

- `GET /api/v1/students` - Get all students (paginated)
- `POST /api/v1/students` - Create a new student
- `GET /api/v1/students/{id}` - Get a specific student
- `PUT /api/v1/students/{id}` - Update a student
- `DELETE /api/v1/students/{id}` - Delete a student

### Example API Usage

```bash
# Register a new user
curl -X POST http://localhost:8081/api/auth/register \
  -H "Content-Type: application/json" \
  -d '{"username":"admin","email":"admin@university.edu","password":"password123","firstName":"Admin","lastName":"User"}'

# Login
curl -X POST http://localhost:8081/api/auth/login \
  -H "Content-Type: application/json" \
  -d '{"email":"admin@university.edu","password":"password123"}'

# Get students (requires authentication)
curl -X GET http://localhost:8081/api/v1/students \
  -H "Authorization: Bearer YOUR_JWT_TOKEN"
```

## Project Structure

```
university-registration-system/
├── src/                    # Rust backend source
│   ├── handlers/          # API route handlers
│   ├── models/           # Database models
│   ├── schema.rs         # Database schema
│   └── main.rs          # Application entry point
├── frontend/             # Next.js frontend
│   ├── src/
│   │   ├── app/         # Next.js 14 app router
│   │   ├── components/  # React components
│   │   └── types/       # TypeScript definitions
│   └── public/          # Static assets
├── migrations/          # Database migrations
└── docker-compose.yml   # Docker configuration
```

## Contributing

1. Fork the repository
2. Create a feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'Add amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Acknowledgments

- Built with [Actix Web](https://actix.rs/) for the backend
- Frontend powered by [Next.js](https://nextjs.org/)
- Styled with [Tailwind CSS](https://tailwindcss.com/)
- Database management with [Diesel](https://diesel.rs/)
