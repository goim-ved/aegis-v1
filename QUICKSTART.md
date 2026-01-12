# AEGIS_MVP: Operational "From Zero" Guide

This guide assumes you have closed all terminals and want to verify the system from a clean slate.

## Prerequisites
- Docker Desktop (Running)
- Node.js (Installed)
- 3 Separate Terminal Windows

---

## 🛑 Step 1: The Blockchain (Terminal 1)
This terminal runs your local copy of the Ethereum network.

1. Open **Terminal 1**.
2. Run the Hardhat Node:
   ```powershell
   cd C:\PROJECTS\PROXY_FINTECH_V1
   npx hardhat node
   ```
   *Keep this running. It will print "Started HTTP and WebSocket JSON-RPC server at..."*

---

## 📝 Step 2: Deploy Contracts (Terminal 2)
You need to deploy the smart contract to your fresh local blockchain so the backend can talk to it.

1. Open **Terminal 2**.
2. Run the deploy script:
   ```powershell
   cd C:\PROJECTS\PROXY_FINTECH_V1
   npx hardhat run scripts/deploy.js --network localhost
   ```
   *You should see: "AegisID deployed to: 0x5FbDB..."*

---

## 🐳 Step 3: The Infrastructure (Terminal 2 - Continued)
Now that the blockchain is ready, launch the full application stack (Database, Backend, Frontend).

1. In the same **Terminal 2**, run:
   ```powershell
   docker-compose up --build
   ```
   *Wait for the logs. You should eventually see:*
   - `aegis_fintech_v1-db-1 | database system is ready to accept connections`
   - `aegis_fintech_v1-backend-1 | listening on 0.0.0.0:3000`
   - `aegis_fintech_v1-frontend-1 | Ready in ...ms`

---

## 👤 Step 4: Create Admin User (Terminal 3)
Now we create the initial administrator account.

1. Open **Terminal 3**.
2. Run the Health Check (to ensure system is up):
   ```powershell
   curl http://localhost:8080/health
   ```
   *Expected Output: `AEGIS_FINTECH_V1 SYSTEMS OPERATIONAL (DB CONNECTED)`*

3. Register the Admin **(Copy this exact PowerShell command)**:
   ```powershell
   Invoke-RestMethod -Uri "http://localhost:8080/api/auth/register" -Method Post -ContentType "application/json" -Body '{"username":"admin","password":"password123"}'
   ```
   *(If you receive an error saying the user exists, that's fine—it means your database data persisted).*

---

## 🚀 Step 5: Verification (Browser)
You are now ready to verify the full flow.

1. Open your browser to: **[http://localhost:3001/login](http://localhost:3001/login)**
2. Log in with:
   - **Username:** `admin`
   - **Password:** `password123`
3. You should be redirected to the **Dashboard**.
   - You will see a list of Legal Entities (initially empty).
   - Click **"Mint Soulbound Token"** (if entities exist) or check the database seeding logic if you want dummy data.

**🎉 Mission Complete: The system is live, integrated, and verified.**
