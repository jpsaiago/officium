<script lang="ts">
	import type {
		ApplicationStatus,
		ContractType,
		WorkMode,
	} from "../types/application";
	import { addApplication } from "../state/applications.svelte";
	import { CONTRACT_TYPE_LABELS, WORK_MODE_LABELS } from "../styles/tagStyles";
	import {
		APPLICATION_STATUSES,
		CONTRACT_TYPES,
		WORK_MODES,
	} from "../types/application";

	let { open = $bindable(false) } = $props();

	function todayISODate(): string {
		const now = new Date();
		const year = now.getFullYear();
		const month = String(now.getMonth() + 1).padStart(2, "0");
		const day = String(now.getDate()).padStart(2, "0");
		return `${year}-${month}-${day}`;
	}

	function emptyForm() {
		return {
			company: "",
			role: "",
			status: "applied" as ApplicationStatus,
			jobUrl: "",
			location: "",
			workMode: "" as WorkMode | "",
			contractType: "" as ContractType | "",
			salary: "",
			appliedOn: todayISODate(),
			notes: "",
		};
	}

	let form = $state(emptyForm());
	let dialogEl: HTMLDialogElement | undefined = $state();

	$effect(() => {
		if (!dialogEl) return;
		if (open && !dialogEl.open) dialogEl.showModal();
		if (!open && dialogEl.open) dialogEl.close();
	});

	function closeModal() {
		open = false;
	}

	function handleDialogClick(event: MouseEvent) {
		// A click that lands on the backdrop (not any child) reports the
		// dialog itself as the target, since the dialog's padding is 0 and
		// the inner card fills its content box exactly.
		if (event.target === dialogEl) closeModal();
	}

	async function handleSubmit(event: Event) {
		event.preventDefault();
		if (!form.company || !form.role) return;

		await addApplication({
			company: form.company,
			role: form.role,
			status: form.status,
			job_url: form.jobUrl || null,
			location: form.location || null,
			work_mode: form.workMode || null,
			contract_type: form.contractType || null,
			salary: form.salary || null,
			applied_on: form.appliedOn || null,
			notes: form.notes || null,
		});

		form = emptyForm();
		closeModal();
	}
</script>

<dialog
	bind:this={dialogEl}
	onclose={() => (open = false)}
	onclick={handleDialogClick}
	class="m-auto max-h-[85vh] w-full max-w-lg overflow-y-auto border-0 bg-transparent p-0 backdrop:bg-slate-900/50"
>
	<form
		onsubmit={handleSubmit}
		class="flex flex-col gap-4 rounded-sm bg-white p-6 shadow-xl dark:bg-slate-800"
	>
		<div class="flex items-center justify-between">
			<h2 class="text-lg font-semibold text-slate-900 dark:text-white">
				Add application
			</h2>
			<button
				type="button"
				onclick={closeModal}
				aria-label="Close"
				class="cursor-pointer text-slate-400 hover:text-slate-600 dark:text-slate-500 dark:hover:text-slate-300"
			>
				✕
			</button>
		</div>

		<div class="grid gap-4 sm:grid-cols-2">
			<label
				class="flex flex-col gap-1 text-sm font-medium text-slate-600 dark:text-slate-300"
			>
				Company *
				<input
					bind:value={form.company}
					required
					class="rounded-md border border-slate-300 bg-white px-3 py-2 text-sm text-slate-900 outline-none focus:border-blue-400 focus:ring-2 focus:ring-blue-100 dark:border-slate-600 dark:bg-slate-900 dark:text-slate-100 dark:focus:border-blue-500 dark:focus:ring-blue-900/40"
				/>
			</label>
			<label
				class="flex flex-col gap-1 text-sm font-medium text-slate-600 dark:text-slate-300"
			>
				Role *
				<input
					bind:value={form.role}
					required
					class="rounded-md border border-slate-300 bg-white px-3 py-2 text-sm text-slate-900 outline-none focus:border-blue-400 focus:ring-2 focus:ring-blue-100 dark:border-slate-600 dark:bg-slate-900 dark:text-slate-100 dark:focus:border-blue-500 dark:focus:ring-blue-900/40"
				/>
			</label>

			<label
				class="flex flex-col gap-1 text-sm font-medium text-slate-600 dark:text-slate-300"
			>
				Status
				<select
					bind:value={form.status}
					class="rounded-md border border-slate-300 bg-white px-3 py-2 text-sm text-slate-900 capitalize outline-none focus:border-blue-400 focus:ring-2 focus:ring-blue-100 dark:border-slate-600 dark:bg-slate-900 dark:text-slate-100 dark:focus:border-blue-500 dark:focus:ring-blue-900/40"
				>
					{#each APPLICATION_STATUSES as status (status)}
						<option value={status}>{status}</option>
					{/each}
				</select>
			</label>
			<label
				class="flex flex-col gap-1 text-sm font-medium text-slate-600 dark:text-slate-300"
			>
				Date applied
				<input
					type="date"
					bind:value={form.appliedOn}
					class="rounded-md border border-slate-300 bg-white px-3 py-2 text-sm text-slate-900 scheme-light outline-none focus:border-blue-400 focus:ring-2 focus:ring-blue-100 dark:border-slate-600 dark:bg-slate-900 dark:text-slate-100 dark:scheme-dark dark:focus:border-blue-500 dark:focus:ring-blue-900/40 [&::-webkit-calendar-picker-indicator]:cursor-pointer [&::-webkit-calendar-picker-indicator]:rounded [&::-webkit-calendar-picker-indicator]:p-1 [&::-webkit-calendar-picker-indicator]:opacity-60 [&::-webkit-calendar-picker-indicator]:transition-opacity hover:[&::-webkit-calendar-picker-indicator]:opacity-100"
				/>
			</label>

			<label
				class="flex flex-col gap-1 text-sm font-medium text-slate-600 dark:text-slate-300"
			>
				Work mode
				<select
					bind:value={form.workMode}
					class="rounded-md border border-slate-300 bg-white px-3 py-2 text-sm text-slate-900 outline-none focus:border-blue-400 focus:ring-2 focus:ring-blue-100 dark:border-slate-600 dark:bg-slate-900 dark:text-slate-100 dark:focus:border-blue-500 dark:focus:ring-blue-900/40"
				>
					<option value="">—</option>
					{#each WORK_MODES as mode (mode)}
						<option value={mode}>{WORK_MODE_LABELS[mode]}</option>
					{/each}
				</select>
			</label>
			<label
				class="flex flex-col gap-1 text-sm font-medium text-slate-600 dark:text-slate-300"
			>
				Contract type
				<select
					bind:value={form.contractType}
					class="rounded-md border border-slate-300 bg-white px-3 py-2 text-sm text-slate-900 outline-none focus:border-blue-400 focus:ring-2 focus:ring-blue-100 dark:border-slate-600 dark:bg-slate-900 dark:text-slate-100 dark:focus:border-blue-500 dark:focus:ring-blue-900/40"
				>
					<option value="">—</option>
					{#each CONTRACT_TYPES as type (type)}
						<option value={type}>{CONTRACT_TYPE_LABELS[type]}</option>
					{/each}
				</select>
			</label>

			<label
				class="flex flex-col gap-1 text-sm font-medium text-slate-600 dark:text-slate-300"
			>
				Location
				<input
					bind:value={form.location}
					placeholder="São Paulo, SP"
					class="rounded-md border border-slate-300 bg-white px-3 py-2 text-sm text-slate-900 outline-none focus:border-blue-400 focus:ring-2 focus:ring-blue-100 dark:border-slate-600 dark:bg-slate-900 dark:text-slate-100 dark:focus:border-blue-500 dark:focus:ring-blue-900/40"
				/>
			</label>
			<label
				class="flex flex-col gap-1 text-sm font-medium text-slate-600 dark:text-slate-300"
			>
				Salary
				<input
					bind:value={form.salary}
					placeholder="R$ 12.000/mo"
					class="rounded-md border border-slate-300 bg-white px-3 py-2 text-sm text-slate-900 outline-none focus:border-blue-400 focus:ring-2 focus:ring-blue-100 dark:border-slate-600 dark:bg-slate-900 dark:text-slate-100 dark:focus:border-blue-500 dark:focus:ring-blue-900/40"
				/>
			</label>

			<label
				class="flex flex-col gap-1 text-sm font-medium text-slate-600 sm:col-span-2 dark:text-slate-300"
			>
				Job URL
				<input
					bind:value={form.jobUrl}
					type="url"
					placeholder="https://..."
					class="rounded-md border border-slate-300 bg-white px-3 py-2 text-sm text-slate-900 outline-none focus:border-blue-400 focus:ring-2 focus:ring-blue-100 dark:border-slate-600 dark:bg-slate-900 dark:text-slate-100 dark:focus:border-blue-500 dark:focus:ring-blue-900/40"
				/>
			</label>

			<label
				class="flex flex-col gap-1 text-sm font-medium text-slate-600 sm:col-span-2 dark:text-slate-300"
			>
				Notes
				<textarea
					bind:value={form.notes}
					rows="3"
					class="resize-none rounded-md border border-slate-300 bg-white px-3 py-2 text-sm text-slate-900 outline-none focus:border-blue-400 focus:ring-2 focus:ring-blue-100 dark:border-slate-600 dark:bg-slate-900 dark:text-slate-100 dark:focus:border-blue-500 dark:focus:ring-blue-900/40"
				></textarea>
			</label>
		</div>

		<div class="flex justify-end gap-2 pt-2">
			<button
				type="button"
				onclick={closeModal}
				class="cursor-pointer rounded-md border border-slate-300 px-4 py-2 text-sm font-semibold text-slate-600 transition-colors hover:bg-slate-100 dark:border-slate-600 dark:text-slate-300 dark:hover:bg-slate-700"
			>
				Cancel
			</button>
			<button
				type="submit"
				class="cursor-pointer rounded-md bg-blue-600 px-4 py-2 text-sm font-semibold text-white shadow-sm transition-colors hover:bg-blue-700"
			>
				Add job
			</button>
		</div>
	</form>
</dialog>
