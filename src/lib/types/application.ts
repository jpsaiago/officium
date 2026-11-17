export type ApplicationStatus =
	"saved" | "applied" | "interviewing" | "offer" | "rejected" | "withdrawn";

export const APPLICATION_STATUSES: ApplicationStatus[] = [
	"saved",
	"applied",
	"interviewing",
	"offer",
	"rejected",
	"withdrawn",
];

export type WorkMode = "remote" | "hybrid" | "onsite";

export const WORK_MODES: WorkMode[] = ["remote", "hybrid", "onsite"];

// Brazil-specific hiring arrangement: PJ (pessoa jurídica, invoiced as a
// contractor) vs CLT (salaried employee under labor law).
export type ContractType = "pj" | "clt";

export const CONTRACT_TYPES: ContractType[] = ["pj", "clt"];

// Mirrors `Application` in src-tauri/src/models.rs. Field names and casing
// must match the Rust struct's serde output (plain snake_case, no rename_all).
export interface Application {
	id: number;
	company: string;
	role: string;
	status: ApplicationStatus;
	job_url: string | null;
	location: string | null;
	work_mode: WorkMode | null;
	contract_type: ContractType | null;
	salary: string | null;
	applied_on: string | null;
	notes: string | null;
	created_at: string;
	updated_at: string;
}

// Mirrors `NewApplication` in src-tauri/src/models.rs.
export interface NewApplication {
	company: string;
	role: string;
	status?: ApplicationStatus;
	job_url?: string | null;
	location?: string | null;
	work_mode?: WorkMode | null;
	contract_type?: ContractType | null;
	salary?: string | null;
	applied_on?: string | null;
	notes?: string | null;
}

// Mirrors `UpdateApplication` in src-tauri/src/models.rs.
export interface UpdateApplication {
	company?: string;
	role?: string;
	status?: ApplicationStatus;
	job_url?: string | null;
	location?: string | null;
	work_mode?: WorkMode | null;
	contract_type?: ContractType | null;
	salary?: string | null;
	applied_on?: string | null;
	notes?: string | null;
}
