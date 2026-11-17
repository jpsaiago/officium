import type { ContractType, WorkMode } from "../types/application";

const CHIP_BASE =
	"inline-flex items-center rounded-full px-2 py-0.5 text-[11px] font-medium";

export const WORK_MODE_LABELS: Record<WorkMode, string> = {
	remote: "Remote",
	hybrid: "Hybrid",
	onsite: "On-site",
};

export const WORK_MODE_CHIP: Record<WorkMode, string> = {
	remote: `${CHIP_BASE} bg-teal-100 text-teal-700 dark:bg-teal-900/40 dark:text-teal-300`,
	hybrid: `${CHIP_BASE} bg-violet-100 text-violet-700 dark:bg-violet-900/40 dark:text-violet-300`,
	onsite: `${CHIP_BASE} bg-orange-100 text-orange-700 dark:bg-orange-900/40 dark:text-orange-300`,
};

export const CONTRACT_TYPE_LABELS: Record<ContractType, string> = {
	pj: "PJ",
	clt: "CLT",
};

export const CONTRACT_TYPE_CHIP: Record<ContractType, string> = {
	pj: `${CHIP_BASE} bg-fuchsia-100 text-fuchsia-700 dark:bg-fuchsia-900/40 dark:text-fuchsia-300`,
	clt: `${CHIP_BASE} bg-cyan-100 text-cyan-700 dark:bg-cyan-900/40 dark:text-cyan-300`,
};
