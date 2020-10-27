![Ellipticoin Logo](README/ellipticoin.png)

###A Docker-based Miner for Ellipticoin
---

### How to run Ellipticoin Miner for Docker:
1. run the command: **mv .env.sample .env**
2. Edit **.env**:


> RUST_BACKTRACE=0 \
HOST=IP:PORT or ExternalURL:PORT \
POSTGRES_DB=ellipticoind \
POSTGRES_USER=root \
POSTGRES_PASSWORD=\<password you make up> \
PRIVATE_KEY=\<you will input in upcoming steps> 
    
| note: set RUST_BACKTRACE=1 to print backtrace

3. run the command: **docker-compose build**
4. Copy & Paste **Private Key** that is written in the **build output** to **PRIVATE_KEY=** in **.env** . Save **public key** to somewhere safe.
5. run the command again: **docker-compose build --no-cache**
6. run the command: **docker-compose up**
---
