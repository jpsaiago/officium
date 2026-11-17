import type {
	Application,
	ApplicationStatus,
	NewApplication,
	UpdateApplication,
} from "../types/application";
import { invoke } from "@tauri-apps/api/core";

export const applications: Application[] = $state([]);

export const applicationsState = $state({
	loading: false,
	error: null as string | null,
});

export async function loadApplications() {
	applicationsState.loading = true;
	applicationsState.error = null;
	try {
		const result = await invoke<Application[]>("list_applications");
		applications.splice(0, applications.length, ...result);
	} catch (e) {
		applicationsState.error = String(e);
		applications.splice(
			0,
			applications.length,
			{
				id: 1,
				company: "Acme Corp",
				role: "Frontend Engineer",
				status: "applied",
				job_url: null,
				location: "Remote",
				work_mode: "remote",
				contract_type: "pj",
				salary: null,
				applied_on: "2026-11-10",
				notes: null,
				created_at: "",
				updated_at: "",
			},
			{
				id: 2,
				company: "Globex",
				role: "Backend Engineer",
				status: "applied",
				job_url: null,
				location: "São Paulo",
				work_mode: "hybrid",
				contract_type: "clt",
				salary: null,
				applied_on: "2026-11-09",
				notes: null,
				created_at: "",
				updated_at: "",
			},
		);
	} finally {
		applicationsState.loading = false;
	}
}

export async function addApplication(payload: NewApplication) {
	try {
		const created = await invoke<Application>("create_application", {
			payload,
		});
		applications.unshift(created);
		return created;
	} catch {
		const fake: Application = {
			id: Date.now(),
			company: payload.company,
			role: payload.role,
			status: payload.status ?? "applied",
			job_url: payload.job_url ?? null,
			location: payload.location ?? null,
			work_mode: payload.work_mode ?? null,
			contract_type: payload.contract_type ?? null,
			salary: payload.salary ?? null,
			applied_on: payload.applied_on ?? null,
			notes: payload.notes ?? null,
			created_at: "",
			updated_at: "",
		};
		applications.unshift(fake);
		return fake;
	}
}

export async function removeApplication(id: number) {
	await invoke("delete_application", { id });
	const index = applications.findIndex((application) => application.id === id);
	if (index !== -1) applications.splice(index, 1);
}

export async function updateApplicationStatus(
	id: number,
	status: ApplicationStatus,
) {
	await updateApplication(id, { status });
}

export async function updateApplication(
	id: number,
	payload: UpdateApplication,
) {
	const updated = await invoke<Application>("update_application", {
		id,
		payload,
	});
	const index = applications.findIndex((application) => application.id === id);
	if (index !== -1) applications[index] = updated;
	return updated;
}
