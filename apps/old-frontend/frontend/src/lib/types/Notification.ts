export default interface Notification {
	id: string;
	title: string;
	message: string;
	type: 'info' | 'success' | 'warning' | 'error';
	primaryAction?: {
		text: string;
		href: string;
	};
	secondaryAction?: {
		text: string;
		href: string;
	};
}
