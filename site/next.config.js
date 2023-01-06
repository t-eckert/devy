/** @type {import('next').NextConfig} */
const nextConfig = {
  reactStrictMode: true,
  swcMinify: true,
  async redirects() {
    return [
      {
        source: "/:user",
        destination: "/:user/profile",
        permanent: true,
      },
    ]
  },
}

module.exports = nextConfig
