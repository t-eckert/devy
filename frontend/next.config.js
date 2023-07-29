const mode = process.env.NODE_ENV

const API =
  mode === "production" ? "https://devy-api.fly.dev/" : "http://localhost:8080"

/** @type {import('next').NextConfig} */
const nextConfig = {
  async rewrites() {
    return [
      {
        source: "/api/:path*",
        destination: `${API}/:path*`,
      },
    ]
  },
}

module.exports = nextConfig
