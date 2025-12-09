# Multi-Cloud Deployment Guide

This guide covers how to deploy the `artist-portfolio` to Koyeb, Oracle Cloud, and Google Cloud Run.

## Prerequisites
- Docker installed locally.
- Accounts on the respective cloud providers.
- The `standalone` binary built via Docker (or locally).

---

## 1. Koyeb Deployment

Koyeb is a developer-friendly serverless platform that can build directly from your GitHub repository using the Dockerfile.

1.  **Push your changes** to GitHub.
2.  **Login to Koyeb** and click **Create App**.
3.  Select **GitHub** as the deployment method.
4.  Choose your repository (`github-stef-portfolio`).
5.  **Builder**: Select **Dockerfile**.
6.  **Environment Variables**:
    - Add all variables from your `.env` / `Secrets.toml`:
        - `DATABASE_URL` (Your Aiven/Neon URL)
        - `CLOUDINARY_CLOUD_NAME`
        - `CLOUDINARY_API_KEY`
        - `CLOUDINARY_API_SECRET`
        - `APP_ENVIRONMENT` = `production`
7.  **Regions**: Choose a region close to your database (e.g., Frankfurt if using Aiven/Neon in Europe).
8.  **Instance Type**: The "Nano" or "Micro" instance is usually sufficient for free tier/low cost.
9.  Click **Deploy**.

---

## 2. Oracle Cloud (Always Free VM)

This guide assumes you are setting up a fresh "Always Free" VM (Ampere A1 or AMD Micro).

### Step 1: Create the Instance
1.  Log in to OCI Console.
2.  Go to **Compute** -> **Instances** -> **Create Instance**.
3.  **Image**: Canonical Ubuntu 22.04 or 24.04 (Minimal is fine).
4.  **Shape**:
    - **Ampere** (VM.Standard.A1.Flex): Select 1-4 OCPUs and up to 24GB RAM. (Best performance).
    - **AMD** (VM.Standard.E2.1.Micro): 1 OCPU, 1GB RAM. (Slower, but standard x86).
    - *Note*: If you choose Ampere (ARM), you must ensure your Docker image is built for ARM64 (`linux/arm64`). The provided Dockerfile supports multi-arch if built with `docker buildx`.
5.  **Networking**: Create a new VCN (Virtual Cloud Network) if you don't have one. Ensure "Assign a public IPv4 address" is checked.
6.  **SSH Keys**: Generate a key pair and save the private key (`.key`) securely.
7.  Click **Create**.

### Step 2: Network & Security Setup (Firewall)
By default, Oracle blocks most ports. You need to open port 80/443 (or 8000).

1.  Click on your instance -> **Subnet** (link) -> **Security Lists** -> **Default Security List**.
2.  **Add Ingress Rule**:
    - Source CIDR: `0.0.0.0/0`
    - IP Protocol: TCP
    - Destination Port Range: `80, 443, 8000`
    - Description: HTTP/HTTPS/App
3.  **Ubuntu Firewall (on the VM)**:
    SSH into your VM:
    ```bash
    ssh -i /path/to/private.key ubuntu@<public-ip>
    ```
    Run:
    ```bash
    sudo iptables -I INPUT 6 -m state --state NEW -p tcp --dport 80 -j ACCEPT
    sudo iptables -I INPUT 6 -m state --state NEW -p tcp --dport 443 -j ACCEPT
    sudo iptables -I INPUT 6 -m state --state NEW -p tcp --dport 8000 -j ACCEPT
    sudo netfilter-persistent save
    ```

### Step 3: Install Docker
On the VM:
```bash
# Update and install prerequisites
sudo apt-get update
sudo apt-get install -y ca-certificates curl gnupg

# Add Docker's official GPG key
sudo install -m 0755 -d /etc/apt/keyrings
curl -fsSL https://download.docker.com/linux/ubuntu/gpg | sudo gpg --dearmor -o /etc/apt/keyrings/docker.gpg
sudo chmod a+r /etc/apt/keyrings/docker.gpg

# Add the repository
echo \
  "deb [arch="$(dpkg --print-architecture)" signed-by=/etc/apt/keyrings/docker.gpg] https://download.docker.com/linux/ubuntu \
  "$(. /etc/os-release && echo "$VERSION_CODENAME")" stable" | \
  sudo tee /etc/apt/sources.list.d/docker.list > /dev/null

# Install Docker
sudo apt-get update
sudo apt-get install -y docker-ce docker-ce-cli containerd.io docker-buildx-plugin docker-compose-plugin

# Allow ubuntu user to run docker
sudo usermod -aG docker ubuntu
# Log out and log back in for this to take effect
exit
```

### Step 4: Deploy the App
Since this is a private project, you have two options:
1.  **Option A (Registry)**: Push your image to Docker Hub (public/private) or GitHub Container Registry (GHCR).
2.  **Option B (Build on VM)**: Clone the repo on the VM and build there. (Easiest for free tier).

**Using Option B:**
1.  SSH back in.
2.  Clone your repo:
    ```bash
    git clone https://github.com/rezgauche/github-stef-portfolio.git
    cd github-stef-portfolio
    ```
3.  Create `.env` file:
    ```bash
    nano .env
    # Paste your environment variables here
    ```
4.  Build and Run:
    ```bash
    docker compose up -d --build
    ```
    *Note*: You might need to create a `docker-compose.yml` file if you haven't yet (see below).

**Simple `docker-compose.yml`:**
```yaml
version: '3.8'
services:
  app:
    build: .
    ports:
      - "8000:8000"
    env_file:
      - .env
    restart: always
```

---

## 3. Google Cloud Run

Cloud Run is excellent for scaling to zero.

1.  **Install Google Cloud SDK** locally.
2.  **Authenticate**:
    ```bash
    gcloud auth login
    gcloud config set project <PROJECT_ID>
    ```
3.  **Build and Deploy**:
    ```bash
    gcloud run deploy artist-portfolio \
      --source . \
      --region us-central1 \
      --allow-unauthenticated \
      --set-env-vars "APP_ENVIRONMENT=production,DATABASE_URL=...,CLOUDINARY_CLOUD_NAME=..."
    ```
    *Note*: You can pass env vars individually or reference a secret manager.

4.  **Verify**: Google will provide a URL ending in `.run.app`.

---

## Summary
- **Koyeb**: Easiest "git push" deployment.
- **Oracle**: Most control, "free VPS", requires manual setup.
- **Google Cloud Run**: Best for intermittent traffic (scales to zero).
