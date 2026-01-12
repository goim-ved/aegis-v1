$ErrorActionPreference = "Stop"

Write-Host "Checking system health..."
try {
    $response = Invoke-RestMethod -Uri "http://localhost:8080/health" -Method Get
    Write-Host "System Status: $response"
} catch {
    Write-Host "System is not ready yet. Please ensure Docker containers are running and Backend is listening on port 8080."
    Write-Host "Error: $_"
    exit 1
}

Write-Host "Creating Admin User..."
try {
    $admin = Invoke-RestMethod -Uri "http://localhost:8080/api/auth/register" -Method Post -ContentType "application/json" -Body '{"username":"admin","password":"password123"}'
    Write-Host "Admin user created successfully!"
    Write-Host "Response: $($admin | ConvertTo-Json -Depth 5)"
} catch {
    Write-Host "Failed to create admin user (or user already exists)."
    Write-Host "Error: $_"
}

Write-Host "-------------------------------------------"
Write-Host "You can now login at: http://localhost:3001/login"
Write-Host "Username: admin"
Write-Host "Password: password123"
