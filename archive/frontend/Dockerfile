FROM node:22-alpine AS builder
WORKDIR /app
COPY ./frontend/package*.json .
RUN npm ci -f
COPY ./frontend/ .
RUN npm run build

FROM node:22-alpine
WORKDIR /app
COPY --from=builder /app/build build/
COPY --from=builder /app/node_modules node_modules/
COPY --from=builder /app/package.json .
EXPOSE 3000
ENV NODE_ENV=production
CMD [ "node", "build" ]
