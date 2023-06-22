const PrismaClient = require("@prisma/client").PrismaClient
const Order = require("@prisma/client").Order

const prisma = new PrismaClient()

const userEntries: any = [
	{
		id: 0,
		username: "johndoe",
		name: "John Doe",
		email: "john@doe.com",
	},
	{
		id: 1,
		username: "sunnydae",
		name: "Sunny Dae",
		email: "shine@gmail.com",
	},
	{
		id: 2,
		username: "rmarz",
		name: "R. Marz",
		email: "rmarz@hotmail.com",
	},
]

const blogEntries: any = [
	{
		id: 0,
		slug: "ml-blog",
		name: "Machine Learning Blog",
		authorId: 0,
	},
	{
		id: 1,
		slug: "web-dev-blog",
		name: "Web Development Blog",
		authorId: 1,
	},
	{
		id: 2,
		slug: "bata-dlog",
		name: "Data Blog",
		authorId: 0,
	},
	{
		id: 3,
		slug: "security-blog",
		name: "Security Blog",
		authorId: 1,
	},
]

const postEntries: any = [
	{
		id: 0,
		slug: "introduction-to-machine-learning",
		blogSlug: blogEntries[0].slug,
		authorUsername: userEntries[0].username,
		title: "Introduction to Machine Learning",
		created: new Date("2022-03-15"),
		updated: new Date("2022-03-15"),
		markdown:
			"# Introduction to Machine Learning\n\nMachine learning is a powerful field of study that enables computers to learn from data and make predictions or decisions without being explicitly programmed. It has gained tremendous popularity and is revolutionizing various industries, from healthcare and finance to self-driving cars and virtual assistants.\n\n## What is Machine Learning?\n\nAt its core, machine learning is about building algorithms and models that can learn patterns and relationships from data. These models are trained using historical or labeled data, allowing them to identify and generalize patterns, and make accurate predictions on new, unseen data.\n\n## Types of Machine Learning\n\nThere are several types of machine learning algorithms, each designed for different tasks:\n\n1. **Supervised Learning**: In this type, the model learns from labeled data, where each input has a corresponding desired output. It learns to predict the output for new inputs based on the patterns observed in the training data.\n\n2. **Unsupervised Learning**: Here, the model learns from unlabeled data, finding patterns and structures within the data itself. Unsupervised learning is useful for tasks such as clustering, anomaly detection, and dimensionality reduction.\n\n3. **Reinforcement Learning**: This type involves an agent interacting with an environment and learning through trial and error. The agent receives rewards or penalties based on its actions, enabling it to learn optimal strategies over time.\n\n## Applications of Machine Learning\n\nMachine learning has diverse applications across various industries:\n\n- **Healthcare**: It aids in diagnosing diseases, predicting patient outcomes, and personalizing treatment plans.\n\n- **Finance**: It enables fraud detection, algorithmic trading, and risk assessment.\n\n- **Transportation**: Machine learning is essential in self-driving cars for object detection, path planning, and decision-making.\n\n- **Natural Language Processing**: It powers virtual assistants, language translation, sentiment analysis, and chatbots.\n\n## Steps in a Typical Machine Learning Workflow\n\nA typical machine learning workflow involves several key steps:\n\n1. **Data Collection**: Gathering relevant data that represents the problem at hand.\n\n2. **Data Preprocessing**: Cleaning and transforming the data to remove noise, handle missing values, and normalize the features.\n\n3. **Feature Selection/Engineering**: Identifying the most informative features or creating new ones to enhance the model's performance.\n\n4. **Model Selection**: Choosing an appropriate machine learning algorithm that suits the problem and data characteristics.\n\n5. **Model Training**: Training the selected model using the labeled or unlabeled data.\n\n6. **Model Evaluation**: Assessing the model's performance on a separate dataset to gauge its accuracy and generalization ability.\n\n7. **Model Deployment**: Integrating the trained model into a production environment for real-time predictions.\n\n## Conclusion\n\nMachine learning is an exciting and rapidly evolving field that offers immense potential for solving complex problems and making data-driven decisions. By harnessing the power of data, machine learning algorithms can uncover valuable insights and improve processes across numerous industries. Whether it's predicting customer behavior, analyzing medical images, or enhancing user experiences, machine learning continues to shape our world in profound ways.\n",
	},
	{
		id: 1,
		slug: "building-scalable-web-applications",
		blogSlug: blogEntries[1].slug,
		authorUsername: userEntries[1].username,
		title: "Building Scalable Web Applications",
		created: new Date("2022-07-02"),
		updated: new Date("2022-07-02"),
		markdown:
			"# Building Scalable Web Applications\n\nIn today's digital world, building web applications that can handle a large number of users and scale effectively is crucial. In this blog post, we will discuss various strategies and best practices for building scalable web applications.\n\n...",
	},
	{
		id: 2,
		slug: "data-visualization-with-d3",
		blogSlug: blogEntries[2].slug,
		authorUsername: userEntries[0].username,
		title: "Data Visualization with D3",
		created: new Date("2022-09-21"),
		updated: new Date("2022-09-21"),
		markdown:
			"# Data Visualization with D3\n\nData visualization is a powerful way to represent and communicate complex data effectively. In this blog post, we will explore how to use D3.js, a popular JavaScript library, to create interactive and visually appealing data visualizations.\n\n...",
	},
	{
		id: 3,
		slug: "getting-started-with-python",
		blogSlug: blogEntries[2].slug,
		authorUsername: userEntries[0].username,
		title: "Getting Started with Python",
		created: new Date("2022-10-10"),
		updated: new Date("2022-10-10"),
		markdown:
			"# Getting Started with Python\n\nPython is a versatile programming language that is widely used for various purposes, including web development, data analysis, and artificial intelligence. In this blog post, we will cover the basics of Python programming and get you started on your Python journey.\n\n...",
	},
	{
		id: 4,
		slug: "secure-coding-practices",
		blogSlug: blogEntries[3].slug,
		authorUsername: userEntries[1].username,
		title: "Secure Coding Practices",
		created: new Date("2022-11-28"),
		updated: new Date("2022-11-28"),
		markdown:
			"# Secure Coding Practices\n\nWriting secure code is of utmost importance in today's digital landscape. In this blog post, we will discuss essential secure coding practices that can help protect your applications from common vulnerabilities and attacks.\n\n...",
	},
	{
		id: 5,
		slug: "understanding-algorithms",
		blogSlug: blogEntries[0].slug,
		authorUsername: userEntries[0].username,
		title: "Understanding Algorithms",
		created: new Date("2023-01-05"),
		updated: new Date("2023-01-05"),
		markdown:
			"# Understanding Algorithms\n\nAlgorithms form the foundation of computer science and are essential for solving complex problems efficiently. In this blog post, we will dive into various algorithms, their analysis, and their practical applications.\n\n...",
	},
]

const feedEntries: any = [
	{
		id: 0,
		slug: "new",
		name: "New",
		publishOrder: Order.DESC,
	},
	{
		id: 1,
		slug: "popular",
		name: "Popular",
		publishOrder: Order.NONE,
	},
]

// For the seed, all posts are in all feeds.
const postInFeedEntries: any = postEntries.flatMap(
	(post: any, postIndex: number) =>
		feedEntries.map((feed: any, feedIndex: number) => ({
			id: postIndex * feedEntries.length + feedIndex,
			postId: post.id,
			feedId: feed.id,
		}))
)

async function seed() {
	const users = await prisma.user.createMany({
		data: userEntries,
		skipDuplicates: true,
	})
	console.log("Created Users, skipping duplicates")
	console.dir(users)

	const blogs = await prisma.blog.createMany({
		data: blogEntries,
		skipDuplicates: true,
	})
	console.log("Created Blogs, skipping duplicates")
	console.dir(blogs)

	const posts = await prisma.post.createMany({
		data: postEntries,
		skipDuplicates: true,
	})
	console.log("Created Posts, skipping duplicates")
	console.dir(posts)

	const feeds = await prisma.feed.createMany({
		data: feedEntries,
		skipDuplicates: true,
	})
	console.log("Created Feeds, skipping duplicates")
	console.dir(feeds)

	const postsInFeeds = await prisma.postInFeed.createMany({
		data: postInFeedEntries,
		skipDuplicates: true,
	})
	console.log("Created PostsInFeeds, skipping duplicates")
	console.dir(postsInFeeds)
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
