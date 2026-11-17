<script lang="ts">
	import AddApplicationModal from "$lib/components/AddApplicationModal.svelte";
	import KanbanBoard from "$lib/components/kanban/KanbanBoard.svelte";
	import {
		applications,
		applicationsState,
		loadApplications,
	} from "$lib/state/applications.svelte";
	import { themeState, toggleTheme } from "$lib/state/theme.svelte";
	import { onMount } from "svelte";
	import "../app.css";

	let isAddModalOpen = $state(false);

	onMount(() => {
		loadApplications();
	});
</script>

<main
	class="flex h-screen flex-col bg-slate-50 text-slate-900 transition-colors dark:bg-mist-950 dark:text-slate-100"
>
	<div class="flex min-h-0 flex-1 flex-col px-6 py-8">
		<header class="mb-8 flex items-center justify-between">
			<div>
				<h1
					class="text-3xl font-bold tracking-tight text-slate-900 dark:text-white"
				>
					Job Applications
				</h1>
				<p class="mt-1 text-sm text-slate-500 dark:text-slate-400">
					Track every application from first save to offer.
				</p>
			</div>

			<div class="flex items-center gap-3">
				<button
					type="button"
					onclick={() => (isAddModalOpen = true)}
					class="cursor-pointer rounded-md bg-blue-600 px-4 py-2 text-sm font-semibold text-white shadow-sm transition-colors hover:bg-blue-700"
				>
					+ Add application
				</button>

				<button
					type="button"
					onclick={toggleTheme}
					aria-label="Toggle dark mode"
					class="flex size-10 cursor-pointer items-center justify-center rounded-full border border-slate-200 bg-white text-slate-600 shadow-sm transition-colors hover:bg-slate-100 dark:border-slate-700 dark:bg-slate-800 dark:text-slate-300 dark:hover:bg-slate-700"
				>
					{#if themeState.theme === "dark"}
						☀️
					{:else}
						🌙
					{/if}
				</button>
			</div>
		</header>

		{#if applicationsState.error}
			<p
				class="mb-4 rounded-md border border-red-200 bg-red-50 px-4 py-2 text-sm text-red-600 dark:border-red-900/50 dark:bg-red-900/20 dark:text-red-400"
			>
				{applicationsState.error}
			</p>
		{/if}

		{#if applicationsState.loading}
			<p class="text-slate-500 dark:text-slate-400">Loading...</p>
		{:else if applications.length === 0}
			<p class="text-slate-500 dark:text-slate-400">No jobs tracked yet.</p>
		{:else}
			<div class="min-h-0 flex-1">
				<KanbanBoard />
			</div>
		{/if}
	</div>
</main>

<AddApplicationModal bind:open={isAddModalOpen} />
