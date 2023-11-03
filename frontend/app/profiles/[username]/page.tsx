import api from "@/lib/api"
import Profile from "@/models/Profile"
import User from "@/models/User"
import Member from "@/components/member"
import Blog from "@/models/Blog"
import Post from "@/models/Post"
import PostCollection from "@/components/post-collection"
import Json from "@/debug/json"
import Card from "@/components/card"

interface Props {
  params: {
    username: string
  }
}

export default async function ProfilePage({ params }: Props) {
  const profile = await api.get<Profile>(`/v1/profiles/${params.username}`, 60)
  const user = await api.get<User>(`/v1/users/${params.username}`, 60)
  const blogs = await api.get<Blog[]>(
    `/v1/profiles/${params.username}/blogs`,
    60
  )
  const posts = await api.get<Post[]>(
    `/v1/profiles/${params.username}/posts`,
    60
  )
  const likes = await api.get<Post[]>(
    `/v1/profiles/${params.username}/likes`,
    60
  )

  return (
    <main className="mx-auto my-8 flex flex-row items-start px-2 w-full max-w-6xl gap-4 sm:gap-16">
      <div>
        <Member profile={profile} user={user} blogs={blogs} />
      </div>

      <div>
        <Card className="px-4 py-3 flex flex-col gap-2 w-full max-w-3xl">
          <h1 className="text-xl font-semibold">About</h1>

          <p className="text-sm">
            Lorem ipsum dolor sit amet, consectetur adipiscing elit. Phasellus ullamcorper lacus eget ipsum venenatis venenatis. Phasellus non ante risus. Aenean ante ex, tincidunt sit amet ex nec, tristique tristique tellus. Aenean laoreet aliquet dui, sit amet congue quam ullamcorper eget. Vestibulum facilisis varius interdum. Lorem ipsum dolor sit amet, consectetur adipiscing elit. In elementum sapien mi, tristique mollis purus accumsan in. Praesent interdum ligula mi, et facilisis metus luctus vitae. Nunc ultricies laoreet nulla, sit amet ultrices risus finibus vel.
          </p>

          <p className="text-sm">
            Praesent vel est mauris. Suspendisse in mi eget purus molestie sollicitudin quis ac mi. Nunc auctor, ante sed tincidunt scelerisque, odio massa tempus ante, lobortis viverra purus odio eu metus. Nam non dui non nunc vehicula fringilla. Nam fermentum massa vel tortor sollicitudin porttitor. Phasellus blandit nulla justo. Aenean euismod est semper tortor interdum vulputate. Ut eget eros eget quam consectetur rutrum. Sed tincidunt arcu cursus rhoncus rutrum. Etiam varius orci quis turpis finibus rhoncus. Curabitur mollis diam id est mollis varius.
          </p>
        </Card>

        <div>
          <div className="py-4">
            <h1 className="font-semibold">Posts by {profile.displayName}</h1>
          </div>
          <PostCollection posts={posts} />
        </div>
        <div>
          <div className="py-4">
            <h1 className="font-semibold">Liked by {profile.displayName}</h1>
          </div>
          <PostCollection posts={likes} />
        </div>
      </div>
    </main>
  )
}
