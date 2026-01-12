# 🛡️ AEGIS FINTECH V1

> **Status:** 🟢 **Pilot Ready / Reference Implementation**  
> *Next-Generation Compliance Infrastructure for DeFi*

![License](https://img.shields.io/badge/license-MIT-blue.svg) ![Rust](https://img.shields.io/badge/backend-Rust-orange.svg) ![Next.js](https://img.shields.io/badge/frontend-Next.js_16-black.svg) ![Solidity](https://img.shields.io/badge/contracts-Solidity-363636.svg)

---

## 📖 Executive Summary

**Aegis Fintech V1** (formerly PROXY) is a digital "Compliance Guardrail" designed to bridge traditional banking standards with decentralized finance (DeFi). 

In the current "wild west" of crypto, ensuring **Know-Your-Customer (KYC)** compliance and **Anti-Money Laundering (AML)** checks is difficult. Aegis solves this by introducing a **Soulbound Token (SBT)** architectue. Legal entities are verified off-chain, and then issued a non-transferable identity token on-chain. This allows smart contracts to enforce regulatory rules programmatically (e.g., "Only allow transactions from Verified US Investors").

### 🎯 Key Capabilities
*   **Identity Verification**: Onboard verified entities and issue Soulbound Tokens (SBTs).
*   **Regulatory Guardrails**: On-chain and off-chain rule engines to reject non-compliant transactions.
*   **Bank-Grade Security**: High-performance Rust backend using Axum and SQLx.
*   **Modern Dashboard**: A reactive, premium UI built with Next.js 16 and Tailwind CSS v4.

---

## 🏗️ Architecture

Aegis follows a hybrid **Web 2.5** architecture, keeping sensitive data (PII) in a secure off-chain database while using the blockchain for public trust and settlement.

```mermaid
graph TD
    User[User / Admin] -->|HTTPS| Frontend[Next.js Dashboard]
    Frontend -->|JSON/REST| Backend[Rust API (Axum)]
    Backend -->|SQL| DB[(PostgreSQL Vault)]
    Backend -->|RPC| Blockchain[Ethereum / Hardhat Node]
    Blockchain -->|Events| Backend
    
    subgraph "On-Chain World"
    Blockchain
    SBT[Soulbound Identity Token]
    end
    
    subgraph "Off-Chain Secure Zone"
    Backend
    DB
    end
```

---

## 🛠️ Tech Stack

### Service Layer (Backend)
*   **Language**: Rust (Edition 2021)
*   **Framework**: [Axum 0.7](https://github.com/tokio-rs/axum)
*   **Database**: PostgreSQL with [SQLx 0.7](https://github.com/launchbadge/sqlx)
*   **Blockchain Client**: Ethers.rs 2.0
*   **Security**: Argon2 for hashing, JWT for auth.

### User Interface (Frontend - `dashboard/`)
*   **Framework**: [Next.js 16](https://nextjs.org/) (App Router)
*   **Language**: TypeScript
*   **Styling**: [Tailwind CSS v4](https://tailwindcss.com/) & Tailwind Merge
*   **Motion**: Framer Motion 12
*   **Icons**: Lucide React

### Protocol Layer (Smart Contracts)
*   **Language**: Solidity
*   **Framework**: Hardhat
*   **Standard**: ERC-721 / Soulbound Implementation (OpenZeppelin)

---

## 🚀 Getting Started

 For a detailed step-by-step guide including command line outputs, please refer to [QUICKSTART.md](QUICKSTART.md).

### Prerequisites
*   **Docker Desktop** (Running)
*   **Node.js 18+**
*   **Git**

### 1️⃣ Start the Local Blockchain
In your first terminal:
```powershell
npx hardhat node
```
*Creates a local Ethereum network with seeded accounts.*

### 2️⃣ Deploy Contracts & Start Infrastructure
In a second terminal:
```powershell
# Deploy Smart Contracts
npx hardhat run scripts/deploy.js --network localhost

# Start Backend, DB, and Frontend
docker-compose up --build
```

### 3️⃣ Verify System
Once running, the services will be available at:
*   **Frontend**: [http://localhost:3001](http://localhost:3001)
*   **Backend API**: [http://localhost:8080](http://localhost:8080)
*   **Database**: Port 5432

**Default Admin Credentials:**
*   You may need to register an admin user via API if the DB is fresh (see Quickstart Step 4).
*   Standard Dev Creds: `admin` / `password123`

---

## 📂 Project Structure

```bash
AEGIS_FINTECH_V1/
├── contracts/          # Solidity Smart Contracts (Identity, Access)
├── dashboard/          # Next.js Frontend Application
├── scripts/            # Deployment & Maintenance Scripts
├── src/                # Rust Backend Application Code
├── GUIDES/             # Documentation & Guides
├── docker-compose.yml  # Container Orchestration
├── Cargo.toml          # Rust Dependencies
└── hardhat.config.js   # Blockchain Development Config
```

---

## ⚠️ Disclaimer
This code is provided as a **Reference Implementation** for educational and portfolio purposes. While it uses production-grade tools (Rust, SQLx), it has **NOT** undergone a formal security audit. 

**DO NOT use in a production environment with real funds without a comprehensive security review.**

---

## 📜 License
Distributed under the MIT License. See `LICENSE` for more information.
