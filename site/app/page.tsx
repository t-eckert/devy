import Feed from "sections/Feed"
import Intro from "sections/Intro"

const HomePage: React.FC = () => {
	return (
		<div>
			<main className="px-2 max-w-3xl md:mt-12 mx-auto flex flex-col-reverse gap-12 md:flex-row md:gap-4">
				<div className="flex flex-col">
					<Feed />
				</div>
				<Intro />
			</main>
		</div>
	)
}

export default HomePage
