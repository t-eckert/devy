<div class="grid grid-cols-1 sm:grid-cols-4 gap-2 sm:gap-10 mx-auto">
	<div>
		<slot name="sidebar" />
	</div>
	<div class="sm:col-start-2 sm:col-span-2">
		<slot />
	</div>
</div>
