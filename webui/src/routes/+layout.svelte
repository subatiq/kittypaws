<script lang="ts">
	import '../app.postcss';
	import { AppShell, AppBar } from '@skeletonlabs/skeleton';
	import { AppRail, AppRailAnchor } from '@skeletonlabs/skeleton';
	import { page } from '$app/stores';
    import Fa from 'svelte-fa';
    import { faPaw, faPlug } from '@fortawesome/free-solid-svg-icons';

	// Floating UI for Popups
	import { computePosition, autoUpdate, flip, shift, offset, arrow } from '@floating-ui/dom';
	import { storePopup } from '@skeletonlabs/skeleton';
	storePopup.set({ computePosition, autoUpdate, flip, shift, offset, arrow });
</script>

<!-- App Shell -->
<AppShell>
	<svelte:fragment slot="header">
		<!-- App Bar -->
		<AppBar>
			<svelte:fragment slot="lead">
				<strong class="flex flex-row items-center gap-3 text-lg font-mono font-light uppercase"><img class="h-12" src="logo.svg">Kittypaws</strong>
			</svelte:fragment>
			<svelte:fragment slot="trail">
				<a
					class="btn btn-sm variant-ghost-surface"
					href="https://github.com/subatiq/kittypaws"
					target="_blank"
					rel="noreferrer"
				>
					GitHub
				</a>
			</svelte:fragment>
		</AppBar>
	</svelte:fragment>
	<svelte:fragment slot="sidebarLeft">
	<AppRail>
		<AppRailAnchor href="/" selected={$page.url.pathname === '/'}><div class="flex flex-row justify-center text-3xl text-white"><Fa icon={faPaw}/></div></AppRailAnchor>
		<AppRailAnchor href="/plugins" selected={$page.url.pathname === '/plugins'}
			><div class="flex flex-row justify-center text-3xl text-white"><Fa icon={faPlug}/></div></AppRailAnchor
		>
	</AppRail>
	</svelte:fragment>

	<!-- Page Route Content -->

    <div class="m-10">
    <slot />
    </div>
</AppShell>
