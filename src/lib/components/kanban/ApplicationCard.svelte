<script lang="ts">
	import type { Application } from "../../types/application";
	import { STATUS_STYLES } from "../../styles/statusStyles";
	import {
		CONTRACT_TYPE_CHIP,
		CONTRACT_TYPE_LABELS,
		WORK_MODE_CHIP,
		WORK_MODE_LABELS,
	} from "../../styles/tagStyles";

	let {
		application,
		isDragging,
		ondragstart,
		ondragend,
		onremove,
	}: {
		application: Application;
		isDragging: boolean;
		ondragstart: (event: DragEvent) => void;
		ondragend: () => void;
		onremove: () => void;
	} = $props();
</script>

<div
	class="w-full cursor-grab rounded-xs border bg-white p-3 shadow-sm transition-colors duration-200 active:cursor-grabbing dark:bg-slate-800 {STATUS_STYLES[
		application.status
	].border}"
	class:opacity-40={isDragging}
	draggable="true"
	role="listitem"
	{ondragstart}
	{ondragend}
>
	<div class="flex items-start justify-between gap-2">
		<p class="font-medium text-slate-800 dark:text-slate-100">
			{application.company}
		</p>
		<button
			type="button"
			class="text-xs text-slate-400 hover:text-red-500 dark:text-slate-500 dark:hover:text-red-400"
			onclick={onremove}
		>
			✕
		</button>
	</div>
	<p class="text-sm text-slate-500 dark:text-slate-400">{application.role}</p>

	{#if application.work_mode || application.contract_type}
		<div class="mt-2 flex flex-wrap gap-1">
			{#if application.work_mode}
				<span class={WORK_MODE_CHIP[application.work_mode]}>
					{WORK_MODE_LABELS[application.work_mode]}
				</span>
			{/if}
			{#if application.contract_type}
				<span class={CONTRACT_TYPE_CHIP[application.contract_type]}>
					{CONTRACT_TYPE_LABELS[application.contract_type]}
				</span>
			{/if}
		</div>
	{/if}

	{#if application.applied_on}
		<p class="mt-1 text-xs text-slate-400 dark:text-slate-500">
			{application.applied_on}
		</p>
	{/if}
</div>
