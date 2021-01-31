## Create public and private keys

```bash
# Luo yksityisen avimen
openssl genpkey -algorithm RSA -out private_key.pem -pkeyopt rsa_keygen_bits:2048

# Luo julkisen avaimen
openssl rsa -pubout -in private_key.pem -out public_key.pem

# Luo kubernetes tls secret for ingress
openssl req -x509 -nodes -days 365 -newkey rsa:2048 -keyout public.key -out private.cert -subj "/CN=${HOST}/O=${HOST}"

openssl req -x509 -new -nodes -key public.key -sha256 -days 1024 -out root.ca


kubectl create secret tls tls-secret --key public.key --cert private.cert
```
