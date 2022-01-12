# What-to-Eat

This Discord bot let you know what to eat.

## Deployment

Currently deploy on kubernetes cluster supported.

### Kubernetes

1. Add secret(`DISCORD_BOT`, `APPLICATION_ID`) using name `what-to-eat-cred`
  ```bash
  kubectl create secret generic what-to-eat-cred \
    --from-literal DISCORD_BOT=xxxxxxxxxxx \
    --from-literal APPLICATION_ID=nnnnnnnnnn
  ```
2. Create Deployment
  ```bash
  kubectl apply -f ./deployment/what-to-eat-deployment.yml
  ```
