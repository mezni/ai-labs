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
{"token":"eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJzdWIiOiJhZG1pbiIsInJvbGUiOiJhZG1pbiIsImV4cCI6MTc2Mjk4MDI1M30.FV_7J6ln7dgBAqqdWIJtXsr8Cyu86d4y9fsGzg9OgAg","username":"admin","role":"admin"}


curl -X POST http://localhost:8080/api/v1/networks \
  -H "Content-Type: application/json" \
  -H "Authorization: Bearer eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJzdWIiOiJhZG1pbiIsInJvbGUiOiJhZG1pbiIsImV4cCI6MTc2Mjk4MDI1M30.FV_7J6ln7dgBAqqdWIJtXsr8Cyu86d4y9fsGzg9OgAg" \
  -d '{
    "name": "My Company Network",
    "type": "company",
    "contact_email": "admin@company.com",
    "phone_number": "+1234567890",
    "address": "123 Business Ave"
  }'