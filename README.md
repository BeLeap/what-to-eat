# What-to-Eat

This Discord bot let you know what to eat.

## Deployment

Currently deploy on kubernetes cluster supported.

### Kubernetes

1. Add secret(`DISCORD_BOT`, `APPLICATION_ID`) using name `what-to-eat-cred`
2. `kubectl apply -f ./deployment/what-to-eat-deployment.yml`
