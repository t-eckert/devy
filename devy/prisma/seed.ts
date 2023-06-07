const PrismaClient = require("@prisma/client").PrismaClient

const prisma = new PrismaClient()

async function seed() {
	const dillPickles = await prisma.user.create({
		data: {
			id: 0,
			email: "dill@liminal.com",
			name: "Dill Pickles",
			username: "dillman",
		},
	})
	console.log("Created Dill Pickles")
	console.dir(dillPickles)
}

seed()
	.then(async () => {
		await prisma.$disconnect()
	})
	.catch(async (e) => {
		console.error(e)
		await prisma.$disconnect()
		process.exit(1)
	})
