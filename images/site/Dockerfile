# Use Node.js LTS version as base image
FROM node:lts AS builder

# Set the working directory inside the container
WORKDIR /app

# Copy package.json and package-lock.json to the working directory
COPY package.json package-lock.json ./

# Install dependencies
RUN npm install

# Copy the rest of the application code
COPY . .

# Build the SvelteKit application
RUN npm run build

# Stage 2: Serve the application using a lightweight HTTP server
FROM node:lts-slim AS runner

# Set the working directory inside the container
WORKDIR /app

# Copy built files from the previous stage
COPY --from=builder /app/build /app/build

# Install `sirv-cli` globally to serve the application
RUN npm install -g sirv-cli

# Expose the port on which the application will run
EXPOSE 5000

# Command to serve the application
CMD ["sirv", "build"]
