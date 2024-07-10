FROM node:lts AS builder
WORKDIR /app
COPY package.json package-lock.json ./
RUN npm install
COPY . .
RUN npm run build

FROM node:lts-slim AS runner
WORKDIR /app
COPY --from=builder /app/build /app/build
RUN npm install -g sirv-cli
EXPOSE 5000
CMD ["sirv", "build"]
