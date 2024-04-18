<script lang="ts">
	import { faPaw } from '@fortawesome/free-solid-svg-icons';
	import { ProgressRadial } from '@skeletonlabs/skeleton';
	import Fa from 'svelte-fa';

	let running = false;
</script>

<h1 class="uppercase">Runner</h1>
<hr />
<div class="my-10 w-96 flex flex-col gap-5">
	{#if !running}
		<label class="label">
			<span>Staging duration</span>
			<input class="input" type="text" placeholder="P1DT30M" />
		</label>
		<label class="config">
        <span>Configuration</span>
        <textarea class="textarea font-mono" rows="20" placeholder="plugins:
- name: deathloop
  frequency: once" />
		</label>

		<a href="/" class="btn variant-filled-primary uppercase" on:click={() => (running = true)}
			><Fa class="mx-2" icon={faPaw}/>Start destruction</a
		>
	{:else}
		<div class="flex flex-row gap-5">
			<div class="flex flex-col gap-5 items-center">
				<aside class="alert variant-ghost">
					<!-- Icon -->
					<div class="alert-message flex flex-col items-center">
						<p>Running tests...</p>
						<ProgressRadial
							...
							stroke={100}
							meter="stroke-primary-500"
							track="stroke-primary-500/30"
							strokeLinecap="butt"
						/>
						<p class="text-xl">24:05:30</p>
					</div>
				</aside>
				<a href="/" class="btn w-full variant-filled-error uppercase" on:click={() => (running = false)}
					>Stop tests</a
				>
			</div>
			<div class="flex flex-col gap-2">
				<aside class="alert variant-ghost">
					<div class="flex flex-row gap-5 w-full justify-between">
						<div>Deathloop</div>
						<span class="badge variant-filled-success">100%</span>
					</div>
				</aside>

				<aside class="alert variant-ghost">
					<div class="flex flex-row gap-5 w-full justify-between">
						<div>Dropper</div>
						<span class="badge variant-filled-success">100%</span>
					</div>
				</aside>
				<aside class="alert variant-ghost">
					<div class="flex flex-row gap-5 w-full justify-between">
						<div>Timeburglar</div>
						<span class="badge variant-filled-warning">99.8%</span>
					</div>
				</aside>
			</div>
		</div>
	{/if}
</div>
