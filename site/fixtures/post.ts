import Post from "lib/post"

const posts: Post[] = [
  {
    id: "dca23e49-e712-4431-9601-62d9db080e31",
    title:
      "They Didn't Tell Me I Had to Eat the Two Pizzas: Stories of an IC Turned Manager",
    createdAt: "2020-01-01",
    updatedAt: "2023-01-04",
    slug: "they-didnt-tell-me-i-had-to-eat-the-two-pizzas",
    author: {
      id: "95781875-ba3f-417c-9b4b-dec6b6aea354",
      name: "Demari Williams",
      username: "d-williams",
    },
    tags: ["management", "humor"],
    likes: 300,
    markdown: `It was my first week as a manager and I was feeling overwhelmed. Not only was I
responsible for my own work, but I now had a team of engineers to manage as well.

One day, I received an email from my boss asking me to order lunch for a team 
meeting. No problem, I thought. I'll just order a few pizzas and everyone will be happy.

But when the pizzas arrived, I was in for a surprise. My boss had ordered two 
extra-large pizzas and told me to make sure everyone ate at least one slice. I couldn't believe it. They didn't tell me I had to eat the two pizzas!

I quickly realized that being a manager wasn't just about delegating tasks and 
managing schedules. It also meant taking on additional responsibilities and tasks that no one else wanted to do.

But as I continued in my role as a manager, I learned to embrace these unexpected
challenges and use them as opportunities to grow and learn. And while I may never
enjoy eating two extra-large pizzas in one sitting, I know that being a manager
means being ready for anything.`,
  },
  {
    id: "dca23e49-e712-4431-9601-62d9db080e34",
    title:
      "The Top 5 Machine Learning Libraries Every Data Scientist Should Know",
    createdAt: "2020-01-01",
    updatedAt: "2022-12-30",
    slug: "the-top-5-machine-learning-libraries-every-data-scientist-should-know",
    author: {
      id: "a5fb5b43-6e7c-4059-b973-e8f56b1e2d41",
      name: "Mason Kim",
      username: "mkim",
    },
    tags: ["machine learning", "data science"],
    likes: 432,
    markdown: `Machine learning is a rapidly growing field, and with so many libraries and frameworks available, it can be overwhelming to know where to start. In this post, we will highlight the top 5 machine learning libraries that every data scientist should be familiar with.

## 1. TensorFlow

[TensorFlow](https://www.tensorflow.org/) is an open-source library for machine learning developed by Google. It is widely used in industry and academia for a wide range of tasks such as image and speech recognition, natural language processing, and time series forecasting. TensorFlow offers a comprehensive and flexible ecosystem of tools, libraries, and community resources that make it easy to build and deploy machine learning models.

## 2. scikit-learn

[scikit-learn](https://scikit-learn.org/) is a simple and efficient library for machine learning in Python. It provides a range of supervised and unsupervised learning algorithms in Python, including support vector machines, decision trees, and random forests. scikit-learn is built on NumPy and SciPy, and it integrates well with other scientific Python libraries.

## 3. PyTorch

[PyTorch](https://pytorch.org/) is a popular open-source machine learning library developed by Facebook's AI research group. It is similar to TensorFlow in many ways, but it is known for its simplicity and ease of use. PyTorch provides a dynamic computational graph, which allows for easy debugging and modification of the model. It also offers support for CUDA, which makes it well-suited for deep learning tasks on GPUs.

## 4. Keras

[Keras](https://keras.io/) is a high-level neural networks API, written in Python and capable of running on top of TensorFlow, CNTK, or Theano. It is designed to make building and experimenting with deep learning models as fast and easy as possible. Keras is user-friendly, modular, and extensible, making it a great choice for both beginners and experienced data scientists.

## 5. R's caret package

[caret](http://topepo.github.io/caret/) is a powerful and easy-to-use package for machine learning in R. It provides a consistent interface to over 200 different algorithms, including popular methods such as random forests, gradient boosting, and neural networks. caret also includes functionality for pre-processing, feature selection, and model evaluation, making it a comprehensive tool for data scientists working in R.

In conclusion, mastering these five libraries will give you a solid foundation for tackling a wide range of machine learning tasks. Each library has its own strengths and weaknesses, and the best one for a particular task will depend on the specific problem and the resources available.`,
  },
  {
    id: "dca23e49-e712-4431-9601-62d9db080e33",
    createdAt: "2020-01-01",
    updatedAt: "2022-12-30",
    title: "How to Build a Scalable Serverless Architecture on AWS",
    slug: "how-to-build-a-scalable-serverless-architecture-on-aws",
    author: {
      id: "c6e8f3f3-3b0c-4f9d-8a7c-bbe4fad6e497",
      name: "Ashley Rodriguez",
      username: "arodriguez",
    },
    tags: ["serverless", "AWS"],
    likes: 271,
    markdown: "",
  },
  {
    id: "f24d66e-7f32-4d21-9fca-61c2e7bce653",
    createdAt: "2020-01-01",
    updatedAt: "2022-12-30",
    title: "The Future of Virtual Reality: Predictions for 2022 and Beyond",
    slug: "the-future-of-virtual-reality-predictions-for-2022-and-beyond",
    author: {
      id: "8f0b2eea-9f7b-43a9-8a64-1b78a4b4e4c0",
      name: "Samantha Kim",
      username: "skim",
    },
    tags: ["virtual reality", "predictions"],
    likes: 389,
    markdown: "",
  },
  {
    id: "f24f66e-9f32-4d21-9fca-61c2e7bce653",
    createdAt: "2020-01-01",
    updatedAt: "2022-12-30",
    title:
      "The Pros and Cons of Flutter vs. React Native for Mobile App Development",
    slug: "the-pros-and-cons-of-flutter-vs-react-native-for-mobile-app-development",
    author: {
      id: "eebdf74a-d07d-4d65-a7aa-6e5f6cd75c1d",
      name: "Emily Lee",
      username: "elee",
    },
    tags: ["mobile app development", "Flutter", "React Native"],
    likes: 309,
    markdown: `Mobile app development is a constantly evolving field, and with new frameworks and technologies emerging all the time, it can be difficult to decide which one to use for your project. Two of the most popular options for cross-platform mobile app development are Flutter and React Native. In this post, we will explore the pros and cons of each framework to help you make an informed decision.

## Flutter

### Flutter Pros
- **Hot Reload**: One of the most significant advantages of Flutter is its ability to hot reload, which allows developers to see the changes they make in the code instantly, without having to stop and restart the app. This feature can significantly speed up the development process.
- **Customizable UI**: Flutter's widgets are completely customizable, which means that developers have complete control over the look and feel of the app. This can be especially useful for creating unique and visually striking apps.
- **Dart Language**: Flutter uses the Dart programming language, which is easy to learn, especially for developers who are already familiar with Java or JavaScript.

### Flutter Cons
- **Limited third-party libraries**: Compared to React Native, Flutter has a relatively small ecosystem of third-party libraries, which can make it difficult to find pre-built solutions for certain tasks.
- **Steep learning curve**: While the Dart language is easy to learn, the framework itself has a steeper learning curve than React Native. 
- **Performance**: Although Flutter’s performance is generally good, it may not be as smooth as natively developed apps for iOS and Android in some cases.

## React Native

### React Native Pros
- **Large Community**: React Native has a large and active community, which means that developers have access to a wide range of tutorials, resources, and third-party libraries.
- **JavaScript Language**: React Native uses JavaScript, which is one of the most popular programming languages in the world. This means that developers who are already familiar with JavaScript can easily pick up React Native.
- **Familiarity with React**: React Native is based on React, a JavaScript library for building user interfaces. Developers who are already familiar with React will find it easier to learn React Native.

### React Native Cons
- **Limited Customization**: React Native offers limited customization options for the UI, which can make it difficult to create truly unique and visually striking apps.
- **Performance**: Although React Native's performance is generally good, it may not be as smooth as natively developed apps for iOS and Android in some cases.
- **Bridge**: React Native uses a bridge to communicate with native components, which can lead to performance issues, especially in complex apps.

## Conclusion

Both Flutter and React Native have their own set of pros and cons. Flutter offers a more customizable UI and hot reload feature, but has a smaller ecosystem of third-party libraries and a steeper learning curve. React Native, on the other hand, has a larger community and is based on JavaScript, but offers limited customization options and may have performance issues. Ultimately, the decision will depend on the specific requirements of your project and the skills of your development team.`,
  },
  {
    id: "f24f66e-7f32-4d21-9fca-61c9e7bce653",
    createdAt: "2020-01-01",
    updatedAt: "2022-12-30",
    title: "3 Ways Artificial Intelligence is Changing Healthcare",
    slug: "3-ways-artificial-intelligence-is-changing-healthcare",
    author: {
      id: "9f2c9e9b-7b34-4e8a-a497-f1b3e6b3c1e3",
      name: "David Kim",
      username: "dkim",
    },
    tags: ["artificial intelligence", "healthcare"],
    likes: 256,
    markdown: "",
  },
  {
    id: "f28f66e-7f32-4d21-9fca-61c2e7bce653",
    createdAt: "2020-01-01",
    updatedAt: "2022-12-30",
    title:
      "The Benefits and Challenges of Using Microservices in Your Architecture",
    slug: "the-benefits-and-challenges-of-using-microservices-in-your-architecture",
    author: {
      id: "a7b5edc4-a9f9-4eee-b7b7-da8e3a3a80e3",
      name: "Sophia Williams",
      username: "swilliams",
    },
    tags: ["microservices", "architecture"],
    likes: 187,
    markdown: `Microservices architecture is a popular approach for building scalable and resilient systems by breaking down monolithic applications into smaller, independently deployable services. In this post, we will explore the benefits and challenges of using microservices in your architecture.

## Benefits

### Scalability

One of the biggest advantages of microservices is the ability to scale individual services independently. This means that if one service is experiencing high traffic, it can be scaled up without affecting the rest of the system. This can also help to minimize downtime, as a problem with one service will not bring down the entire system.

### Flexibility

Microservices architecture allows teams to choose the best technology stack for each service, rather than being locked into a single technology. This can lead to more efficient development, as teams can work with the tools and languages they are most comfortable with.

### Resilience

By breaking down a monolithic application into smaller, independently deployable services, microservices architecture can help to make systems more resilient. If one service goes down, the rest of the system can continue to function, minimizing downtime.

### Faster Deployment

Since each service is independently deployable, it is possible to deploy updates to one service without affecting the rest of the system. This can lead to faster deployment times, as teams can work on different services in parallel, and deploy them independently.

## Challenges

### Complexity

While microservices architecture can bring many benefits, it also adds complexity to the system. With more services and inter-service communication, it can be more difficult to understand how the system as a whole functions. This can make it more difficult to troubleshoot and debug issues.

### Testing

Testing microservices can be more challenging than testing monolithic applications. With more services, there are more potential points of failure, which can make it more difficult to write comprehensive test cases.

### Network Latency

Microservices architecture often involves inter-service communication, which can add network latency to the system. This can be especially problematic for services that need to communicate with each other frequently.

### Service Discovery

In microservices architecture, services need to be able to discover each other in order to communicate. This can be a challenge, especially in large systems with many services.

## Conclusion

Microservices architecture can bring many benefits to your system, including scalability, flexibility, and resilience. However, it also brings its own set of challenges, including complexity, testing, network latency and service discovery. Careful planning, testing and monitoring are crucial for successful implementation of microservices.`,
  },
  {
    id: "f24f66e-7f32-4d22-9fca-61c2e7bce653",
    createdAt: "2020-01-01",
    updatedAt: "2022-12-30",
    title: "An Introduction to Quantum Computing: What It Is and How It Works",
    slug: "an-introduction-to-quantum-computing-what-it-is-and-how-it-works",
    author: {
      id: "3c13f937-6e8a-4c7a-bdfd-b7eb96384699",
      name: "Daniel Kim",
      username: "dkim",
    },
    tags: ["quantum computing"],
    likes: 341,
    markdown: "",
  },
]

const fixtures = {
  posts,
  postsMetadata: posts.map((post) => ({
    id: post.id,
    createdAt: post.createdAt,
    updatedAt: post.updatedAt,
    title: post.title,
    slug: post.slug,
    author: post.author,
    tags: post.tags,
    likes: post.likes,
  })),
}

export default fixtures
