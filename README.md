<img width="1885" height="1003" alt="image" src="https://github.com/user-attachments/assets/d44783ef-7059-4cc0-b1d4-1ba2b6912c1a" />








# 📦 Decentralized File Storage (Soroban Smart Contract)

## 📌 Project Description

This project is a decentralized file storage smart contract built using Soroban (Stellar Smart Contracts). It allows users to securely store, retrieve, and manage file metadata on-chain without relying on centralized servers.

Instead of storing actual files (which is inefficient on-chain), this system stores file hashes and metadata, ensuring data integrity and decentralization.

---

## 🚀 What It Does

- Stores file metadata (file name, hash, timestamp, owner)
- Ensures only the owner can delete their files
- Allows retrieval of stored file data
- Uses blockchain for tamper-proof record keeping

---

## ✨ Features

- 🔐 **Ownership Control** – Only the uploader can delete files  
- 📁 **File Metadata Storage** – Stores file hash instead of raw file  
- ⛓️ **Decentralized** – No centralized server required  
- ⚡ **Lightweight** – Efficient on-chain storage using Soroban  
- 🕒 **Timestamp Tracking** – Records upload time  
- 🔎 **Easy Retrieval** – Fetch file details anytime  

---

## 🛠️ Tech Stack

- Soroban (Stellar Smart Contracts)
- Rust
- Stellar SDK

---

## 📂 Smart Contract Functions

### 1. `store_file`
Stores file metadata on-chain.

**Inputs:**
- `user` – Address of uploader  
- `file_id` – Unique identifier  
- `file_hash` – Hash of the file  
- `file_name` – Name of the file  

---

### 2. `get_file`
Retrieves stored file metadata.

---

### 3. `delete_file`
Deletes file metadata (only by owner).

---

## 🌐 Deployed Smart Contract Link
https://stellar.expert/explorer/testnet/contract/CCYIRODULFMLFYBGJARDN7TPENMYOBNKKAM3R5MRHRHKSD4DGF6CYCWV
