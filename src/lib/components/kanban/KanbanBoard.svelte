<script lang="ts">
	import type { ApplicationStatus } from "../../types/application";
	import {
		applications,
		removeApplication,
		updateApplicationStatus,
	} from "../../state/applications.svelte";
	import { APPLICATION_STATUSES } from "../../types/application";
	import KanbanColumn from "./KanbanColumn.svelte";

	const COLUMN_LABELS: Record<ApplicationStatus, string> = {
		saved: "Saved",
		applied: "Applied",
		interviewing: "Interviewing",
		offer: "Offer",
		rejected: "Rejected",
		withdrawn: "Withdrawn",
	};

	const columns = $derived(
		APPLICATION_STATUSES.map((status) => ({
			status,
			items: applications.filter(
				(application) => application.status === status,
			),
		})),
	);

	// Tracked outside the browser's native drag payload so we don't have to
	// round-trip the id through dataTransfer, which Firefox/WebKit handle
	// inconsistently on dragover.
	let draggingId: number | null = $state(null);
	let dragOverStatus: ApplicationStatus | null = $state(null);

	function handleDragStart(event: DragEvent, id: number) {
		draggingId = id;
		// WebView2/Chromium show the "no-drop" cursor and reject the drop
		// unless effectAllowed/dropEffect are explicitly set, even though
		// dataTransfer itself isn't used to carry the payload (see comment above).
		if (event.dataTransfer) {
			event.dataTransfer.effectAllowed = "move";
		}
	}

	function handleDragEnd() {
		draggingId = null;
		dragOverStatus = null;
	}

	function handleDragOver(event: DragEvent, status: ApplicationStatus) {
		event.preventDefault();
		if (event.dataTransfer) {
			event.dataTransfer.dropEffect = "move";
		}
		dragOverStatus = status;
	}

	function handleDrop(status: ApplicationStatus) {
		if (draggingId !== null) {
			updateApplicationStatus(draggingId, status);
		}
		draggingId = null;
		dragOverStatus = null;
	}

	async function handleRemove(id: number) {
		await removeApplication(id);
	}
</script>

<div class="flex h-full gap-4 overflow-x-auto pb-4">
	{#each columns as column (column.status)}
		<KanbanColumn
			status={column.status}
			label={COLUMN_LABELS[column.status]}
			items={column.items}
			{draggingId}
			isDragOver={dragOverStatus === column.status}
			ondragover={(event) => handleDragOver(event, column.status)}
			ondragleave={() => (dragOverStatus = null)}
			ondrop={() => handleDrop(column.status)}
			oncarddragstart={handleDragStart}
			oncarddragend={handleDragEnd}
			oncardremove={handleRemove}
		/>
	{/each}
</div>
