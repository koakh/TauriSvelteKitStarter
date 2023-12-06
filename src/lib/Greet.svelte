<script>
	import { invoke } from '@tauri-apps/api/tauri';

	let name = '';
	let greetMsg = '';

	async function greet() {
		greetMsg = await invoke('greet', { name });
	}
</script>

<h1 class="layout-header">Tauri + SvelteKit + Tailwind Starter Project</h1>
<div class="mb-4">
	<label class="form-input-label" for="username">Greet name</label>
	<input
		class="form-input-text"
		id="greet-input"
		type="text"
		placeholder="Enter a name..."
		bind:value={name}
	/>
</div>
<div class="mb-4">
	<button class="btn btn-blue" on:click={greet} disabled={!name}>Invoke tauri greet</button>
</div>
<div class="mb-4">
	<p>{greetMsg}</p>
</div>
