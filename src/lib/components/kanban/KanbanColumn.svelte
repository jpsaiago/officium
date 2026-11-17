<script lang="ts">
	import type { Application, ApplicationStatus } from "../../types/application";
	import { STATUS_STYLES } from "../../styles/statusStyles";
	import ApplicationCard from "./ApplicationCard.svelte";

	let {
		status,
		label,
		items,
		draggingId,
		isDragOver,
		ondragover,
		ondragleave,
		ondrop,
		oncarddragstart,
		oncarddragend,
		oncardremove,
	}: {
		status: ApplicationStatus;
		label: string;
		items: Application[];
		draggingId: number | null;
		isDragOver: boolean;
		ondragover: (event: DragEvent) => void;
		ondragleave: () => void;
		ondrop: () => void;
		oncarddragstart: (event: DragEvent, id: number) => void;
		oncarddragend: () => void;
		oncardremove: (id: number) => void;
	} = $props();
</script>

<div
	class="flex h-full min-w-64 flex-1 shrink-0 flex-col rounded-sm bg-slate-100 p-2 transition-colors dark:bg-mist-900"
	class:bg-slate-200={isDragOver}
	class:dark:bg-slate-700={isDragOver}
	role="list"
	aria-label={label}
	{ondragover}
	{ondragleave}
	{ondrop}
>
	<h2
		class="mb-2 flex shrink-0 items-center justify-between px-1 text-sm font-semibold"
	>
		<span
			class="inline-flex items-center gap-2 rounded-full px-2 py-0.5 {STATUS_STYLES[
				status
			].badge}"
		>
			<span class="size-1.5 rounded-full {STATUS_STYLES[status].dot}"></span>
			{label}
		</span>
		<span class="text-slate-400 dark:text-slate-500">({items.length})</span>
	</h2>

	<div class="flex min-h-16 flex-1 flex-col gap-2 overflow-y-auto pr-1">
		{#each items as application (application.id)}
			<ApplicationCard
				{application}
				isDragging={draggingId === application.id}
				ondragstart={(event) => oncarddragstart(event, application.id)}
				ondragend={oncarddragend}
				onremove={() => oncardremove(application.id)}
			/>
		{/each}
	</div>
</div>
