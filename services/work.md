# Register a new user
curl -X POST http://localhost:8080/api/v1/auth/register \
  -H "Content-Type: application/json" \
  -d '{
    "username": "john",
    "password": "john123",
    "role": "user"
  }'

# Register an admin user
curl -X POST http://localhost:5100/api/v1/auth/register \
  -H "Content-Type: application/json" \
  -d '{
    "username": "admin",
    "password": "admin123",
    "role": "admin"
  }'

# Login as admin (default user)
curl -X POST http://localhost:5100/api/v1/auth/login \
  -H "Content-Type: application/json" \
  -d '{
    "username": "admin",
    "password": "admin123"
  }'


curl -X POST http://localhost:5100/api/v1/auth/login   -H "Content-Type: application/json"   -d '{                                                                                                                  "username": "admin",
    "password": "admin123"
  }'
{"token":"eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJzdWIiOiJhZG1pbiIsInJvbGUiOiJhZG1pbiIsImV4cCI6MTc2Mjk5NjYxNX0.P_dZR7m3VguYOEkve8JmxRRHcFBgfxbBPNyM57XoXFc","username":"admin","role":"admin"}


curl -X POST http://localhost:8080/api/v1/networks \
  -H "Content-Type: application/json" \
  -H "Authorization: Bearer eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJzdWIiOiJhZG1pbiIsInJvbGUiOiJhZG1pbiIsImV4cCI6MTc2Mjk5NzMyOX0.RrQ0ZxuO_dvDyeFCUfwU2iR1vLH5xc3P6oW7WdAV7_s" \
  -d '{
    "name": "My Company Network",
    "type": "company",
    "contact_email": "admin@company.com",
    "phone_number": "+1234567890",
    "address": "123 Business Ave"
  }'