const mode = process.env.NODE_ENV

const API =
  mode === "production" ? "https://api.devy.page/" : "http://localhost:8080"

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
