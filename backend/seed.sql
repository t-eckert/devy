INSERT INTO "user" (id, username, email, github_username)
VALUES 
	('20bf70c8-d1c3-4d5c-9a50-6ca87eea205c', 't-eckert', 'name@email.com', 't-eckert'),
	('e3d9727f-1bfd-4a56-b875-45d877ecf115', 'analogue', 'analogue@email.com', 'analogue'),
	('8f85a118-4933-4c61-8708-716ffa35cc49', 'taylorschwift', 'taylorschwift@email.com', 'taylorschwift'),
	('7cd19c80-d72b-4fc2-a818-de809361b278', 'aaronjohnson', 'a.johnson@email.com', 'ajohnson'),
	('c5f1aae6-bc85-44e9-a4c2-ad89c201fdc6', 's-miller', 'susan.miller@email.com', 'susanm');

-- Add an admin user
INSERT INTO "user" (id, username, email, github_username, role)
VALUES 
	('8ff8f972-e62d-4498-a5c8-a33824d5fc83', 't-admin', 'admin@email.com', null, 'admin');

INSERT INTO "profile" (id, user_id, display_name, bio, avatar_url, twitter_username, github_username)
VALUES 
    ('e2f0fa7e-4517-4ac8-bbc6-73067d3feed4', '20bf70c8-d1c3-4d5c-9a50-6ca87eea205c', 'Tobias Eckert', 'I am a software engineer.', 'https://avatars.githubusercontent.com/u/1234567?v=4', 't_eckert', 't-eckert'),
    ('dcb4d8b8-a2be-41c9-afea-135541da371f', 'e3d9727f-1bfd-4a56-b875-45d877ecf115', 'Anna Logue', 'I like to build backend stuff.', null, null, null),
    ('183c1fe7-9d39-4a64-9f4c-abd61584e8f7', '8f85a118-4933-4c61-8708-716ffa35cc49', 'Taylor Schwift', 'Coding enthusiast and traveler.', 'https://avatars.example.com/johnsmith', 'ts_coder', 'taylorschwift'),
    ('778f1ba2-dac9-4d6b-ae2e-d8252bab0119', '7cd19c80-d72b-4fc2-a818-de809361b278', 'Aaron Johnson', 'Loves writing and sharing stories.', 'https://avatars.example.com/marydoe', null, 'ajohnson'),
    ('e0aa560b-1d80-4e2e-81f7-208b2ff463d5', 'c5f1aae6-bc85-44e9-a4c2-ad89c201fdc6', 'Susan Miller', 'Web developer and adventure seeker.', 'https://avatars.example.com/alicej', 'alice_dev', 'susanm');


INSERT INTO "blog" (id, profile_id, name, slug)
VALUES 
	('3c55b723-dc5d-4f9b-aac6-60d63b7e7733', 'e2f0fa7e-4517-4ac8-bbc6-73067d3feed4', 'Machine Learning', 'machine-learning'),
	('2d0be777-545f-4dbe-95c9-768670e6caba', 'e2f0fa7e-4517-4ac8-bbc6-73067d3feed4', 'DevOops', 'devoops');

INSERT INTO "post" (id, blog_id, title, slug, body)
VALUES
	('2d0be777-545f-4dbe-95c9-768670e6caba', '3c55b723-dc5d-4f9b-aac6-60d63b7e7733', 'Designing Distributed Systems with Kubernetes', 'designing-distributed-systems-with-kubernetes', '*Distributed systems have become increasingly popular in order to handle the scalability and complexity requirements of modern applications. Kubernetes, an open-source container orchestration platform, has emerged as one of the leading solutions for designing and managing distributed systems. In this article, we will explore the various considerations and best practices for designing distributed systems with Kubernetes.*

## Understanding Distributed Systems

Before delving into the details of designing distributed systems with Kubernetes, it is important to understand the concept of distributed systems. A distributed system is an application that runs on multiple machines or nodes, working together to achieve a common goal. Such systems provide benefits like increased fault tolerance, scalability, and reliability.

## Key Considerations for Designing Distributed Systems

When designing distributed systems with Kubernetes, there are several important considerations to keep in mind:

### 1. Containerization

Containerization plays a crucial role in designing distributed systems with Kubernetes. Containers provide an isolated and lightweight environment for running applications. By containerizing your applications, you can ensure consistency across different nodes and simplify the deployment process.

### 2. Service Discovery

Service discovery is a critical aspect of distributed systems, as it enables different components to locate and communicate with each other. Kubernetes offers built-in service discovery through its DNS-based service discovery mechanism. Leveraging this feature can simplify the design and implementation of your distributed system.

### 3. High Availability and Fault Tolerance

In distributed systems, ensuring high availability and fault tolerance is essential. Kubernetes offers a range of features to achieve this, such as replica sets, which ensure that a specified number of identical pods are running at all times. Additionally, Kubernetes provides mechanisms for automatic scaling and load balancing, further enhancing the fault tolerance of your system.

### 4. State Management

Managing state in distributed systems can be challenging. Kubernetes provides several approaches for handling stateful applications, such as StatefulSets and Persistent Volumes. These features enable you to manage and persist data in a distributed manner, ensuring data consistency and resilience.

### 5. Monitoring and Observability

Monitoring and observability are key aspects of designing distributed systems. Kubernetes offers features like metrics collection, logging, and tracing, which enable you to gain insights into the health and performance of your system. Leveraging these capabilities can help in identifying and resolving issues proactively.

## Best Practices for Designing Distributed Systems with Kubernetes

While designing distributed systems with Kubernetes, it is important to follow some best practices to ensure optimal performance and reliability:

- **Utilize Kubernetes namespaces** to logically isolate different components of your system and prevent naming conflicts.
- **Leverage Kubernetes deployments** to manage the lifecycle of your applications, ensuring seamless updates and rollbacks.
- **Use labels and selectors** effectively to organize and monitor your distributed system, making it easier to manage and scale.
- **Implement resource limits and requests** to ensure fair resource allocation and prevent the degradation of performance due to resource starvation.
- **Regularly test your distributed system** to validate its behavior under different conditions and identify any potential bottlenecks or limitations.

## Conclusion

Designing distributed systems with Kubernetes requires careful consideration of containerization, service discovery, fault tolerance, state management, and observability. By following best practices and leveraging the capabilities of Kubernetes, you can design and manage distributed systems that are scalable, reliable, and resilient.')
	();
