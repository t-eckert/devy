// A Translator can translate an object from its Prisma representation
// to its Model representation and vice versa.
export default interface Translator<P, M> {
	toModel: (prisma: P) => M
	toPrisma: (model: M) => P
}
