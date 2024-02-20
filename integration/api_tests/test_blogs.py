from framework.config import api
from framework.utils import is_subset
from rich import print
import pytest

import httpx

path = "/blogs"
route = api + path


@pytest.mark.skip(reason="not implemented")
def test_get_blog_by_slug():
    expected = {
        "name": "IoT Insights",
        "slug": "iot-insights",
        "username": "cloudadmin",
        "displayName": "Cloud Admin",
        "description": None,
        "createdAt": "2023-10-06T03:10:37.37",
        "updatedAt": "2023-10-06T03:10:37.37",
    }

    r = httpx.get(api + "/blogs/iot-insights")

    assert r.status_code == 200
    assert is_subset(r.json(), expected)


@pytest.mark.skip(reason="not implemented")
def test_get_blog_posts_by_blog_slug():
    expected = [
        {
            "id": "e846050f-8e42-4d77-8e98-17185afe3d99",
            "slug": "iot-and-insights",
            "blogSlug": "iot-insights",
            "blogName": "IoT Insights",
            "authorName": "Cloud Admin",
            "authorUsername": "cloudadmin",
            "createdAt": "2023-10-06T03:10:37.37",
            "updatedAt": "2023-10-06T03:10:37.37",
            "title": "IoT and Insights",
            "content": "# IoT and Insights\n\nThe Internet of Things (IoT) has become one of the most transformative technologies in recent years, and it is revolutionizing the way businesses and individuals gather and utilize data. By connecting various devices and sensors to the internet, IoT enables the collection of vast amounts of information, which, when analyzed, can provide valuable insights that drive informed decision-making.\n\n## The Power of Data\n\nData has always been crucial in decision-making. However, IoT takes this to a whole new level by gathering real-time information from a multitude of sources. From wearable fitness devices to smart home appliances, IoT devices continuously generate a wealth of data that, when processed and analyzed, can unveil hidden patterns and offer valuable insights.\n\n## Improved Efficiency\n\nOne of the significant benefits of IoT lies in its ability to enhance efficiency across various sectors. With IoT-enabled devices, businesses can monitor and optimize their operations in real-time. For instance, in manufacturing, IoT sensors can track machine performance and detect anomalies, allowing for predictive maintenance and reducing unforeseen downtime.\n\n## Smarter Cities\n\nIoT has the potential to transform urban areas into smart cities. By integrating IoT devices in infrastructure and city services, municipalities can collect valuable data on traffic patterns, energy consumption, waste management, and more. This data can then be analyzed to improve resource allocation, reduce costs, and enhance the quality of life for residents.\n\n## Personalized Experiences\n\nIoT allows businesses to provide personalized experiences to their customers by leveraging data insights. Wearable devices and mobile apps can collect data on user preferences and behavior, enabling companies to tailor products and services to individual needs. This level of personalization not only improves customer satisfaction but also drives customer loyalty.\n\n## Data Security and Privacy\n\nAs the volume of data collected through IoT devices grows, ensuring data security and privacy becomes increasingly critical. IoT networks must implement robust security measures to protect sensitive information from unauthorized access or breaches. Additionally, individuals must also be aware of the data collected by IoT devices and have the necessary controls to safeguard their privacy.\n\n## The Future of IoT and Insights\n\nThe potential applications of IoT and insights are vast and ever-expanding. As more devices get connected to the internet, the amount of data generated will continue to multiply. This opens doors to more advanced analytics techniques, such as machine learning and artificial intelligence, allowing for even deeper and more accurate insights.\n\nIn conclusion, IoT and insights go hand in hand, enabling businesses and individuals to make data-driven decisions. The power of IoT lies not merely in collecting data but in the analysis of that data to gain valuable insights. As the IoT ecosystem evolves, we can anticipate more sophisticated technologies and greater opportunities for a smarter and more connected world.",
            "likes": 12,
        }
    ]

    r = httpx.get(route + "/iot-insights/posts")

    assert r.status_code == 200
    assert r.json() == expected


@pytest.mark.skip(reason="not implemented")
def test_get_blog_post_by_blog_and_post_slugs():
    expected = {
        "id": "e846050f-8e42-4d77-8e98-17185afe3d99",
        "slug": "iot-and-insights",
        "blogSlug": "iot-insights",
        "blogName": "IoT Insights",
        "authorName": "Cloud Admin",
        "authorUsername": "cloudadmin",
        "createdAt": "2023-10-06T03:10:37.37",
        "updatedAt": "2023-10-06T03:10:37.37",
        "title": "IoT and Insights",
        "content": "# IoT and Insights\n\nThe Internet of Things (IoT) has become one of the most transformative technologies in recent years, and it is revolutionizing the way businesses and individuals gather and utilize data. By connecting various devices and sensors to the internet, IoT enables the collection of vast amounts of information, which, when analyzed, can provide valuable insights that drive informed decision-making.\n\n## The Power of Data\n\nData has always been crucial in decision-making. However, IoT takes this to a whole new level by gathering real-time information from a multitude of sources. From wearable fitness devices to smart home appliances, IoT devices continuously generate a wealth of data that, when processed and analyzed, can unveil hidden patterns and offer valuable insights.\n\n## Improved Efficiency\n\nOne of the significant benefits of IoT lies in its ability to enhance efficiency across various sectors. With IoT-enabled devices, businesses can monitor and optimize their operations in real-time. For instance, in manufacturing, IoT sensors can track machine performance and detect anomalies, allowing for predictive maintenance and reducing unforeseen downtime.\n\n## Smarter Cities\n\nIoT has the potential to transform urban areas into smart cities. By integrating IoT devices in infrastructure and city services, municipalities can collect valuable data on traffic patterns, energy consumption, waste management, and more. This data can then be analyzed to improve resource allocation, reduce costs, and enhance the quality of life for residents.\n\n## Personalized Experiences\n\nIoT allows businesses to provide personalized experiences to their customers by leveraging data insights. Wearable devices and mobile apps can collect data on user preferences and behavior, enabling companies to tailor products and services to individual needs. This level of personalization not only improves customer satisfaction but also drives customer loyalty.\n\n## Data Security and Privacy\n\nAs the volume of data collected through IoT devices grows, ensuring data security and privacy becomes increasingly critical. IoT networks must implement robust security measures to protect sensitive information from unauthorized access or breaches. Additionally, individuals must also be aware of the data collected by IoT devices and have the necessary controls to safeguard their privacy.\n\n## The Future of IoT and Insights\n\nThe potential applications of IoT and insights are vast and ever-expanding. As more devices get connected to the internet, the amount of data generated will continue to multiply. This opens doors to more advanced analytics techniques, such as machine learning and artificial intelligence, allowing for even deeper and more accurate insights.\n\nIn conclusion, IoT and insights go hand in hand, enabling businesses and individuals to make data-driven decisions. The power of IoT lies not merely in collecting data but in the analysis of that data to gain valuable insights. As the IoT ecosystem evolves, we can anticipate more sophisticated technologies and greater opportunities for a smarter and more connected world.",
        "likes": 12,
    }

    r = httpx.get(route + "/iot-insights/posts/iot-and-insights")

    assert r.status_code == 200
    assert r.json() == expected


@pytest.mark.skip(reason="not implemented")
def test_create_blog():
    r = httpx.post(route, data={})
    print(r.status_code)


@pytest.mark.skip(reason="not implemented")
def test_update_blog():
    r = httpx.put(route + "/iot-insights", data={})
    print(r.status_code)


@pytest.mark.skip(reason="not implemented")
def test_delete_blog():
    # Create blog

    r = httpx.delete(route + "/iot-insights")
    print(r.status_code)
