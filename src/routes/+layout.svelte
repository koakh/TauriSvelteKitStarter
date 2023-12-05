<main class="noselect">
	<slot />
</main>

<style>
	.noselect {
		/* iOS Safari */
		-webkit-touch-callout: none;
		/* Safari */
		-webkit-user-select: none;
		/* Konqueror HTML */
		-khtml-user-select: none;
		/* Firefox */
		-moz-user-select: none;
		/* Internet Explorer/Edge */
		-ms-user-select: none;
		/* Non-prefixed version, currently supported by Chrome and Opera */
		user-select: none;
	}
</style>
